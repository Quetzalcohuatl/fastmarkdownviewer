//! Bounded, lazy image work and caches. No worker exists until an image needs it.
use super::lock;
use std::{
    borrow::Borrow,
    collections::HashMap,
    hash::Hash,
    sync::atomic::{AtomicUsize, Ordering},
};

pub(super) trait Cost {
    fn bytes(&self) -> usize;
    fn pending(&self) -> bool {
        false
    }
    const LIMIT: usize;
}
struct Entry<V> {
    value: V,
    touched: u64,
    ticket: u64,
}
pub(super) struct Cache<K, V> {
    entries: HashMap<K, Entry<V>>,
    serial: u64,
    bytes: usize,
}
impl<K, V> Default for Cache<K, V> {
    fn default() -> Self {
        Self {
            entries: HashMap::new(),
            serial: 0,
            bytes: 0,
        }
    }
}
impl<K: Eq + Hash + Clone, V: Cost> Cache<K, V> {
    pub fn get<Q: ?Sized + Hash + Eq>(&mut self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
    {
        self.serial += 1;
        let entry = self.entries.get_mut(key)?;
        entry.touched = self.serial;
        Some(&entry.value)
    }
    pub fn insert(&mut self, key: K, value: V) -> u64 {
        self.remove(&key);
        self.serial += 1;
        let ticket = self.serial;
        self.bytes += value.bytes();
        self.entries.insert(
            key,
            Entry {
                value,
                touched: ticket,
                ticket,
            },
        );
        self.trim();
        ticket
    }
    pub fn complete(&mut self, key: K, ticket: u64, value: V) {
        if self
            .entries
            .get(&key)
            .is_some_and(|entry| entry.ticket == ticket)
        {
            self.insert(key, value);
        }
    }
    pub fn remove<Q: ?Sized + Hash + Eq>(&mut self, key: &Q)
    where
        K: Borrow<Q>,
    {
        if let Some(entry) = self.entries.remove(key) {
            self.bytes -= entry.value.bytes();
        }
    }
    pub fn clear(&mut self) {
        self.entries.clear();
        self.bytes = 0;
    }
    pub fn retain(&mut self, mut keep: impl FnMut(&K, &V) -> bool) {
        self.entries.retain(|key, entry| keep(key, &entry.value));
        self.bytes = self.entries.values().map(|entry| entry.value.bytes()).sum();
    }
    pub fn values(&self) -> impl Iterator<Item = &V> {
        self.entries.values().map(|entry| &entry.value)
    }
    fn trim(&mut self) {
        while self.bytes > V::LIMIT || self.entries.len() > 256 {
            let oldest = self
                .entries
                .iter()
                .filter(|(_, entry)| !entry.value.pending())
                .min_by_key(|(_, entry)| entry.touched)
                .map(|(key, _)| key.clone());
            let Some(oldest) = oldest else {
                break;
            };
            self.remove(&oldest);
        }
    }
}

static IO_JOBS: AtomicUsize = AtomicUsize::new(0);
static DECODE_JOBS: AtomicUsize = AtomicUsize::new(0);
pub(super) struct Permit(&'static AtomicUsize);
impl Permit {
    pub fn acquire(decode: bool) -> Option<Self> {
        let (counter, maximum) = if decode {
            (&DECODE_JOBS, 2)
        } else {
            (&IO_JOBS, 4)
        };
        Self::acquire_from(counter, maximum)
    }
    fn acquire_from(counter: &'static AtomicUsize, maximum: usize) -> Option<Self> {
        counter
            .fetch_update(Ordering::AcqRel, Ordering::Acquire, |active| {
                (active < maximum).then_some(active + 1)
            })
            .ok()?;
        Some(Self(counter))
    }
}
impl Drop for Permit {
    fn drop(&mut self) {
        self.0.fetch_sub(1, Ordering::Release);
    }
}

#[derive(Clone)]
struct Texture(eframe::egui::TextureHandle, eframe::egui::Vec2);
impl Cost for Texture {
    const LIMIT: usize = 192 * 1024 * 1024;
    fn bytes(&self) -> usize {
        self.0.byte_size()
    }
}
#[derive(Default)]
pub(super) struct Textures {
    cache: std::sync::Mutex<
        Cache<
            (
                String,
                eframe::egui::TextureOptions,
                eframe::egui::load::SizeHint,
            ),
            Texture,
        >,
    >,
}
impl eframe::egui::load::TextureLoader for Textures {
    fn id(&self) -> &'static str {
        "FastMarkdownViewer bounded textures"
    }
    fn load(
        &self,
        context: &eframe::egui::Context,
        uri: &str,
        options: eframe::egui::TextureOptions,
        size: eframe::egui::load::SizeHint,
    ) -> eframe::egui::load::TextureLoadResult {
        use eframe::egui::load::{ImagePoll, LoadError, SizedTexture, TexturePoll};
        if !uri.starts_with("file://") && !super::remote(uri) {
            return Err(LoadError::NotSupported);
        }
        if !super::allowed(context, uri) {
            return Err(LoadError::Loading("Remote image loading is off".into()));
        }
        // Raster pixels don't depend on layout size. SVGs retain a size-specific key.
        let hint = if url::Url::parse(uri)
            .is_ok_and(|url| url.path().to_ascii_lowercase().ends_with(".svg"))
        {
            size
        } else {
            eframe::egui::load::SizeHint::default()
        };
        let key = (uri.to_owned(), options, hint);
        if let Some(texture) = lock(&self.cache).get(&key).cloned() {
            return Ok(TexturePoll::Ready {
                texture: SizedTexture::new(texture.0.id(), texture.1),
            });
        }
        match context.try_load_image(uri, hint)? {
            ImagePoll::Pending { size } => Ok(TexturePoll::Pending { size }),
            ImagePoll::Ready { image } => {
                let source_size = image.source_size;
                let handle = context.load_texture(uri, image, options);
                let texture = SizedTexture::new(handle.id(), source_size);
                lock(&self.cache).insert(key, Texture(handle, source_size));
                // The GPU copy is now authoritative; don't keep raw + decoded copies too.
                for loader in context.loaders().bytes.lock().iter() {
                    loader.forget(uri);
                }
                for loader in context.loaders().image.lock().iter() {
                    loader.forget(uri);
                }
                Ok(TexturePoll::Ready { texture })
            }
        }
    }
    fn forget(&self, uri: &str) {
        lock(&self.cache).retain(|(key, _, _), _| key != uri);
    }
    fn forget_all(&self) {
        lock(&self.cache).clear();
    }
    fn byte_size(&self) -> usize {
        lock(&self.cache).bytes
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn worker_slots_are_bounded_and_released() {
        static ACTIVE: AtomicUsize = AtomicUsize::new(0);
        let first = Permit::acquire_from(&ACTIVE, 2).unwrap();
        let second = Permit::acquire_from(&ACTIVE, 2).unwrap();
        assert!(Permit::acquire_from(&ACTIVE, 2).is_none());
        drop(first);
        assert!(Permit::acquire_from(&ACTIVE, 2).is_some());
        drop(second);
        assert_eq!(ACTIVE.load(Ordering::Acquire), 0);
    }
    #[derive(Clone)]
    struct Item(usize, bool);
    impl Cost for Item {
        const LIMIT: usize = 10;
        fn bytes(&self) -> usize {
            self.0
        }
        fn pending(&self) -> bool {
            self.1
        }
    }
    #[test]
    fn budget_is_lru_and_stale_jobs_do_not_restore_forgotten_entries() {
        let mut cache = Cache::default();
        cache.insert("a", Item(4, false));
        cache.insert("b", Item(4, false));
        cache.get("a");
        cache.insert("c", Item(4, false));
        assert!(cache.get("b").is_none());
        assert!(cache.get("a").is_some());
        assert!(cache.bytes <= 10);
        let ticket = cache.insert("job", Item(0, true));
        cache.remove("job");
        cache.insert("job", Item(0, true));
        cache.complete("job", ticket, Item(5, false));
        assert!(cache.get("job").unwrap().pending());
        cache.clear();
        cache.complete("job", ticket, Item(5, false));
        assert!(cache.get("job").is_none());
    }
}
