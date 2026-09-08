use std::{
    collections::HashSet,
    io::{Cursor, Read as _},
    net::{IpAddr, ToSocketAddrs as _},
    sync::{Arc, Mutex, MutexGuard},
    task::Poll,
    thread,
    time::Duration,
};

use eframe::egui::{
    self, ColorImage,
    load::{
        Bytes, BytesLoadResult, BytesLoader, BytesPoll, ImageLoadResult, ImageLoader, ImagePoll,
        LoadError, SizeHint,
    },
};
use image::{GenericImageView as _, ImageFormat, ImageReader, Limits};
use url::{Host, Url};

mod resources;
use resources::{Cache, Cost, Permit, Textures};

#[derive(Clone)]
struct ImagePolicy {
    automatic: bool,
    approvals: HashSet<String>,
}
impl Default for ImagePolicy {
    fn default() -> Self {
        Self {
            automatic: true,
            approvals: HashSet::new(),
        }
    }
}
fn remote(uri: &str) -> bool {
    uri.get(..7)
        .is_some_and(|prefix| prefix.eq_ignore_ascii_case("http://"))
        || uri
            .get(..8)
            .is_some_and(|prefix| prefix.eq_ignore_ascii_case("https://"))
}
fn allowed(context: &egui::Context, uri: &str) -> bool {
    if !remote(uri) {
        return true;
    }
    context.data_mut(|data| {
        let policy =
            data.get_temp_mut_or_default::<ImagePolicy>(egui::Id::new("remote image policy"));
        policy.automatic || policy.approvals.contains(uri)
    })
}
#[must_use]
pub fn automatic_images(context: &egui::Context) -> bool {
    context.data_mut(|data| {
        data.get_temp_mut_or_default::<ImagePolicy>(egui::Id::new("remote image policy"))
            .automatic
    })
}
pub fn set_automatic_images(context: &egui::Context, automatic: bool) {
    context.data_mut(|data| {
        let policy =
            data.get_temp_mut_or_default::<ImagePolicy>(egui::Id::new("remote image policy"));
        policy.automatic = automatic;
        policy.approvals.clear();
    });
    let viewports = context.input(|input| input.raw.viewports.keys().copied().collect::<Vec<_>>());
    for id in viewports {
        context.request_repaint_of(id);
    }
}
pub fn image_placeholder(ui: &mut egui::Ui, uri: &str, alt: &str) -> bool {
    if allowed(ui.ctx(), uri) {
        return false;
    }
    ui.push_id(uri, |ui| {
        ui.horizontal_wrapped(|ui| {
            ui.weak(if alt.is_empty() { "Remote image" } else { alt });
            if ui.button("Load image").on_hover_text(uri).clicked() {
                ui.ctx().data_mut(|data| {
                    data.get_temp_mut_or_default::<ImagePolicy>(egui::Id::new(
                        "remote image policy",
                    ))
                    .approvals
                    .insert(uri.to_owned());
                });
                ui.ctx().request_repaint();
            }
        });
    });
    true
}

pub const MAX_REMOTE_BYTES: usize = 10 * 1024 * 1024;
pub const MAX_DECODED_PIXELS: u64 = 40_000_000;
const MAX_REDIRECTS: usize = 5;

#[derive(Clone)]
struct RemoteFile {
    bytes: Arc<[u8]>,
    mime: Option<String>,
}

type RemoteEntry = Poll<Result<RemoteFile, String>>;
impl Cost for RemoteEntry {
    const LIMIT: usize = 32 * 1024 * 1024;
    fn bytes(&self) -> usize {
        match self {
            Poll::Ready(Ok(file)) => file.bytes.len(),
            Poll::Ready(Err(error)) => error.len(),
            Poll::Pending => 0,
        }
    }
    fn pending(&self) -> bool {
        self.is_pending()
    }
}

#[derive(Default)]
pub struct LocalBytesLoader {
    cache: Arc<Mutex<Cache<String, RemoteEntry>>>,
}

impl LocalBytesLoader {
    pub const ID: &'static str = egui::generate_loader_id!(LocalBytesLoader);
}

impl BytesLoader for LocalBytesLoader {
    fn id(&self) -> &str {
        Self::ID
    }

    fn load(&self, context: &egui::Context, uri: &str) -> BytesLoadResult {
        let url = Url::parse(uri).map_err(|_| LoadError::NotSupported)?;
        if url.scheme() != "file" {
            return Err(LoadError::NotSupported);
        }
        let path = url
            .to_file_path()
            .map_err(|()| LoadError::Loading("invalid local image path".to_owned()))?;
        if let Some(entry) = lock(&self.cache).get(uri).cloned() {
            return remote_poll(entry);
        }

        let Some(permit) = Permit::acquire(false) else {
            context.request_repaint_after(Duration::from_millis(50));
            return Ok(BytesPoll::Pending { size: None });
        };
        let ticket = lock(&self.cache).insert(uri.to_owned(), Poll::Pending);
        let cache = Arc::clone(&self.cache);
        let viewport = context.viewport_id();
        let context = context.clone();
        let key = uri.to_owned();
        thread::Builder::new()
            .name("FastMarkdownViewer local image".to_owned())
            .spawn(move || {
                let _permit = permit;
                let result = read_local_image(&path)
                    .map(|bytes| RemoteFile {
                        bytes: bytes.into(),
                        mime: mime_from_path(&path),
                    })
                    .map_err(|error| format!("could not read local image: {error}"));
                lock(&cache).complete(key, ticket, Poll::Ready(result));
                context.request_repaint_of(viewport);
            })
            .map_err(|error| {
                lock(&self.cache).remove(uri);
                LoadError::Loading(error.to_string())
            })?;
        Ok(BytesPoll::Pending { size: None })
    }

    fn forget(&self, uri: &str) {
        lock(&self.cache).remove(uri);
    }

    fn forget_all(&self) {
        lock(&self.cache).clear();
    }

    fn byte_size(&self) -> usize {
        lock(&self.cache)
            .values()
            .map(|entry| match entry {
                Poll::Ready(Ok(file)) => file.bytes.len(),
                Poll::Ready(Err(error)) => error.len(),
                Poll::Pending => 0,
            })
            .sum()
    }

    fn has_pending(&self) -> bool {
        lock(&self.cache).values().any(Poll::is_pending)
    }
}

fn mime_from_path(path: &std::path::Path) -> Option<String> {
    let extension = path.extension()?.to_str()?.to_ascii_lowercase();
    Some(
        match extension.as_str() {
            "png" => "image/png",
            "jpg" | "jpeg" => "image/jpeg",
            "webp" => "image/webp",
            "gif" => "image/gif",
            "svg" => "image/svg+xml",
            _ => return None,
        }
        .to_owned(),
    )
}

fn read_local_image(path: &std::path::Path) -> std::io::Result<Vec<u8>> {
    let mut bytes = Vec::new();
    std::fs::File::open(path)?
        .take((MAX_REMOTE_BYTES + 1) as u64)
        .read_to_end(&mut bytes)?;
    if bytes.len() > MAX_REMOTE_BYTES {
        return Err(std::io::Error::other("local image is larger than 10 MiB"));
    }
    Ok(bytes)
}

#[derive(Default)]
pub struct SafeBytesLoader {
    cache: Arc<Mutex<Cache<String, RemoteEntry>>>,
}

impl SafeBytesLoader {
    pub const ID: &'static str = egui::generate_loader_id!(SafeBytesLoader);
}

impl BytesLoader for SafeBytesLoader {
    fn id(&self) -> &str {
        Self::ID
    }

    fn load(&self, context: &egui::Context, uri: &str) -> BytesLoadResult {
        let url = match validate_remote_url(uri) {
            Ok(url) => url,
            Err(_) if !uri.starts_with("http://") && !uri.starts_with("https://") => {
                return Err(LoadError::NotSupported);
            }
            Err(error) => return Err(LoadError::Loading(error)),
        };

        if !allowed(context, uri) {
            return Err(LoadError::Loading("Remote image loading is off".into()));
        }
        if let Some(entry) = lock(&self.cache).get(uri).cloned() {
            return remote_poll(entry);
        }

        let Some(permit) = Permit::acquire(false) else {
            context.request_repaint_after(Duration::from_millis(50));
            return Ok(BytesPoll::Pending { size: None });
        };
        let ticket = lock(&self.cache).insert(uri.to_owned(), Poll::Pending);
        let cache = Arc::clone(&self.cache);
        let viewport = context.viewport_id();
        let context = context.clone();
        let key = uri.to_owned();
        thread::Builder::new()
            .name("FastMarkdownViewer image download".to_owned())
            .spawn(move || {
                let _permit = permit;
                let result = fetch_remote(url);
                lock(&cache).complete(key, ticket, Poll::Ready(result));
                context.request_repaint_of(viewport);
            })
            .map_err(|error| {
                lock(&self.cache).remove(uri);
                LoadError::Loading(error.to_string())
            })?;
        Ok(BytesPoll::Pending { size: None })
    }

    fn forget(&self, uri: &str) {
        lock(&self.cache).remove(uri);
    }

    fn forget_all(&self) {
        lock(&self.cache).clear();
    }

    fn byte_size(&self) -> usize {
        lock(&self.cache)
            .values()
            .map(|entry| match entry {
                Poll::Ready(Ok(file)) => {
                    file.bytes.len() + file.mime.as_ref().map_or(0, String::len)
                }
                Poll::Ready(Err(error)) => error.len(),
                Poll::Pending => 0,
            })
            .sum()
    }

    fn has_pending(&self) -> bool {
        lock(&self.cache).values().any(Poll::is_pending)
    }
}

fn remote_poll(entry: RemoteEntry) -> BytesLoadResult {
    match entry {
        Poll::Ready(Ok(file)) => Ok(BytesPoll::Ready {
            size: None,
            bytes: Bytes::Shared(file.bytes),
            mime: file.mime,
        }),
        Poll::Ready(Err(error)) => Err(LoadError::Loading(error)),
        Poll::Pending => Ok(BytesPoll::Pending { size: None }),
    }
}

fn fetch_remote(mut url: Url) -> Result<RemoteFile, String> {
    let agent = ureq::AgentBuilder::new()
        .redirects(0)
        .timeout_connect(Duration::from_secs(8))
        .timeout_read(Duration::from_secs(20))
        .user_agent(concat!("FastMarkdownViewer/", env!("CARGO_PKG_VERSION")))
        .build();

    for redirect_count in 0..=MAX_REDIRECTS {
        validate_public_destination(&url)?;
        let response = match agent
            .get(url.as_str())
            .set(
                "Accept",
                "image/png,image/jpeg,image/webp,image/gif,image/svg+xml",
            )
            .call()
        {
            Ok(response) | Err(ureq::Error::Status(_, response)) => response,
            Err(ureq::Error::Transport(error)) => {
                return Err(format!("image request failed: {error}"));
            }
        };

        if (300..400).contains(&response.status()) {
            if redirect_count == MAX_REDIRECTS {
                return Err("image redirected too many times".to_owned());
            }
            let location = response
                .header("Location")
                .ok_or_else(|| "image redirect had no Location header".to_owned())?;
            url = url
                .join(location)
                .map_err(|error| format!("invalid image redirect: {error}"))?;
            validate_remote_url(url.as_str())?;
            continue;
        }

        if !(200..300).contains(&response.status()) {
            return Err(format!("image server returned HTTP {}", response.status()));
        }
        if response
            .header("Content-Length")
            .and_then(|length| length.parse::<usize>().ok())
            .is_some_and(|length| length > MAX_REMOTE_BYTES)
        {
            return Err("remote image is larger than 10 MiB".to_owned());
        }

        let mime = response.header("Content-Type").map(|mime| {
            mime.split(';')
                .next()
                .unwrap_or(mime)
                .trim()
                .to_ascii_lowercase()
        });
        let mut bytes = Vec::new();
        response
            .into_reader()
            .take((MAX_REMOTE_BYTES + 1) as u64)
            .read_to_end(&mut bytes)
            .map_err(|error| format!("could not read image response: {error}"))?;
        if bytes.len() > MAX_REMOTE_BYTES {
            return Err("remote image is larger than 10 MiB".to_owned());
        }
        return Ok(RemoteFile {
            bytes: bytes.into(),
            mime,
        });
    }

    Err("image redirected too many times".to_owned())
}

/// Validate the static policy for a remote image URL.
///
/// # Errors
///
/// Returns an explanation for malformed URLs, unsupported schemes,
/// credentials, missing hosts, or literal private-network destinations.
pub fn validate_remote_url(value: &str) -> Result<Url, String> {
    let url = Url::parse(value).map_err(|error| format!("invalid image URL: {error}"))?;
    if !matches!(url.scheme(), "http" | "https") {
        return Err("remote images must use HTTP or HTTPS".to_owned());
    }
    if !url.username().is_empty() || url.password().is_some() {
        return Err("remote image URLs cannot contain credentials".to_owned());
    }
    let host = url
        .host()
        .ok_or_else(|| "remote image URL has no host".to_owned())?;
    if match host {
        Host::Ipv4(address) => !is_public_ip(IpAddr::V4(address)),
        Host::Ipv6(address) => !is_public_ip(IpAddr::V6(address)),
        Host::Domain(domain) => {
            domain.eq_ignore_ascii_case("localhost") || domain.ends_with(".localhost")
        }
    } {
        return Err("private-network image URLs are blocked".to_owned());
    }
    Ok(url)
}

fn validate_public_destination(url: &Url) -> Result<(), String> {
    let host = url
        .host_str()
        .ok_or_else(|| "remote image URL has no host".to_owned())?;
    let port = url
        .port_or_known_default()
        .ok_or_else(|| "image URL has no port".to_owned())?;
    let addresses = (host, port)
        .to_socket_addrs()
        .map_err(|error| format!("could not resolve image host: {error}"))?
        .collect::<Vec<_>>();
    if addresses.is_empty() || addresses.iter().any(|address| !is_public_ip(address.ip())) {
        return Err("private-network image destinations are blocked".to_owned());
    }
    Ok(())
}

const fn is_public_ip(address: IpAddr) -> bool {
    match address {
        IpAddr::V4(address) => {
            !(address.is_private()
                || address.is_loopback()
                || address.is_link_local()
                || address.is_unspecified()
                || address.is_broadcast()
                || address.is_multicast()
                || address.is_documentation())
        }
        IpAddr::V6(address) => {
            !(address.is_loopback()
                || address.is_unspecified()
                || address.is_multicast()
                || address.is_unique_local()
                || address.is_unicast_link_local())
        }
    }
}

#[derive(Clone)]
enum DecodedEntry {
    Pending,
    Ready(Arc<ColorImage>),
    Failed(String),
}
impl Cost for DecodedEntry {
    const LIMIT: usize = 192 * 1024 * 1024;
    fn bytes(&self) -> usize {
        match self {
            Self::Ready(image) => image.pixels.len() * 4,
            Self::Failed(error) => error.len(),
            Self::Pending => 0,
        }
    }
    fn pending(&self) -> bool {
        matches!(self, Self::Pending)
    }
}

#[derive(Default)]
pub struct SafeImageLoader {
    cache: Arc<Mutex<Cache<(String, SizeHint), DecodedEntry>>>,
}

impl SafeImageLoader {
    pub const ID: &'static str = egui::generate_loader_id!(SafeImageLoader);
}

impl ImageLoader for SafeImageLoader {
    fn id(&self) -> &str {
        Self::ID
    }

    fn load(&self, context: &egui::Context, uri: &str, size_hint: SizeHint) -> ImageLoadResult {
        let uri = egui::decode_animated_image_uri(uri).map_or(uri, |(uri, _)| uri);
        let key = (uri.to_owned(), size_hint);
        if let Some(entry) = lock(&self.cache).get(&key).cloned() {
            return decoded_poll(entry);
        }

        match context.try_load_bytes(uri) {
            Ok(BytesPoll::Pending { size }) => Ok(ImagePoll::Pending { size }),
            Ok(BytesPoll::Ready { bytes, mime, .. }) => {
                let Some(permit) = Permit::acquire(true) else {
                    context.request_repaint_after(Duration::from_millis(50));
                    return Ok(ImagePoll::Pending { size: None });
                };
                let ticket = lock(&self.cache).insert(key.clone(), DecodedEntry::Pending);
                let pending_key = key.clone();
                let cache = Arc::clone(&self.cache);
                let viewport = context.viewport_id();
                let context = context.clone();
                thread::Builder::new()
                    .name("FastMarkdownViewer image decode".to_owned())
                    .spawn(move || {
                        let _permit = permit;
                        let result = decode_image(&key.0, &bytes, mime.as_deref(), key.1)
                            .map_or_else(DecodedEntry::Failed, |image| {
                                DecodedEntry::Ready(Arc::new(image))
                            });
                        lock(&cache).complete(key, ticket, result);
                        context.request_repaint_of(viewport);
                    })
                    .map_err(|error| {
                        lock(&self.cache).remove(&pending_key);
                        LoadError::Loading(error.to_string())
                    })?;
                Ok(ImagePoll::Pending { size: None })
            }
            Err(error) => Err(error),
        }
    }

    fn forget(&self, uri: &str) {
        lock(&self.cache).retain(|(key, _), _| key != uri);
    }

    fn forget_all(&self) {
        lock(&self.cache).clear();
    }

    fn byte_size(&self) -> usize {
        lock(&self.cache)
            .values()
            .map(|entry| match entry {
                DecodedEntry::Ready(image) => image.pixels.len() * size_of::<egui::Color32>(),
                DecodedEntry::Failed(error) => error.len(),
                DecodedEntry::Pending => 0,
            })
            .sum()
    }

    fn has_pending(&self) -> bool {
        lock(&self.cache)
            .values()
            .any(|entry| matches!(entry, DecodedEntry::Pending))
    }
}

fn decoded_poll(entry: DecodedEntry) -> ImageLoadResult {
    match entry {
        DecodedEntry::Ready(image) => Ok(ImagePoll::Ready { image }),
        DecodedEntry::Failed(error) => Err(LoadError::Loading(error)),
        DecodedEntry::Pending => Ok(ImagePoll::Pending { size: None }),
    }
}

fn decode_image(
    uri: &str,
    bytes: &Bytes,
    mime: Option<&str>,
    size_hint: SizeHint,
) -> Result<ColorImage, String> {
    if is_svg(uri, mime, bytes) {
        return decode_svg(bytes, size_hint);
    }

    let format = image::guess_format(bytes).map_err(|error| error.to_string())?;
    if !matches!(
        format,
        ImageFormat::Png | ImageFormat::Jpeg | ImageFormat::Gif | ImageFormat::WebP
    ) {
        return Err("unsupported image format".to_owned());
    }
    let dimensions = ImageReader::with_format(Cursor::new(bytes.as_ref()), format)
        .into_dimensions()
        .map_err(|error| error.to_string())?;
    enforce_pixel_limit(u64::from(dimensions.0), u64::from(dimensions.1))?;

    let mut reader = ImageReader::with_format(Cursor::new(bytes.as_ref()), format);
    let mut limits = Limits::default();
    limits.max_image_width = Some(dimensions.0);
    limits.max_image_height = Some(dimensions.1);
    limits.max_alloc = Some(256 * 1024 * 1024);
    reader.limits(limits);
    let decoded = reader.decode().map_err(|error| error.to_string())?;
    let (width, height) = decoded.dimensions();
    enforce_pixel_limit(u64::from(width), u64::from(height))?;
    let rgba = decoded.into_rgba8();
    Ok(ColorImage::from_rgba_unmultiplied(
        [
            usize::try_from(width).map_err(|error| error.to_string())?,
            usize::try_from(height).map_err(|error| error.to_string())?,
        ],
        rgba.as_raw(),
    ))
}

// Check SVG dimensions before allocating its raster buffer. Match egui_extras' sizing.
#[allow(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_precision_loss
)]
fn decode_svg(bytes: &[u8], hint: SizeHint) -> Result<ColorImage, String> {
    let tree = resvg::usvg::Tree::from_data(bytes, &resvg::usvg::Options::default())
        .map_err(|error| error.to_string())?;
    let source = egui::vec2(tree.size().width(), tree.size().height());
    let size = match hint {
        SizeHint::Size {
            width,
            height,
            maintain_aspect_ratio: true,
        } => source * (width as f32 / source.x).min(height as f32 / source.y),
        SizeHint::Size {
            width,
            height,
            maintain_aspect_ratio: false,
        } => egui::vec2(width as f32, height as f32),
        SizeHint::Width(width) => source * (width as f32 / source.x),
        SizeHint::Height(height) => source * (height as f32 / source.y),
        SizeHint::Scale(scale) => source * scale.into_inner(),
    }
    .round();
    if !size.is_finite() || size.x < 1.0 || size.y < 1.0 {
        return Err("invalid SVG dimensions".into());
    }
    enforce_pixel_limit(size.x as u64, size.y as u64)?;
    let (width, height) = (size.x as u32, size.y as u32);
    let mut pixels = resvg::tiny_skia::Pixmap::new(width, height)
        .ok_or_else(|| "could not allocate SVG pixels".to_owned())?;
    resvg::render(
        &tree,
        resvg::usvg::Transform::from_scale(size.x / source.x, size.y / source.y),
        &mut pixels.as_mut(),
    );
    Ok(
        ColorImage::from_rgba_premultiplied([width as usize, height as usize], pixels.data())
            .with_source_size(source),
    )
}

fn is_svg(uri: &str, mime: Option<&str>, bytes: &[u8]) -> bool {
    mime.is_some_and(|mime| mime.eq_ignore_ascii_case("image/svg+xml"))
        || Url::parse(uri)
            .ok()
            .and_then(|url| {
                std::path::Path::new(url.path())
                    .extension()
                    .and_then(|extension| extension.to_str())
                    .map(str::to_owned)
            })
            .or_else(|| {
                std::path::Path::new(uri)
                    .extension()
                    .and_then(|extension| extension.to_str())
                    .map(str::to_owned)
            })
            .is_some_and(|extension| extension.eq_ignore_ascii_case("svg"))
        || std::str::from_utf8(bytes.get(..bytes.len().min(512)).unwrap_or(bytes))
            .is_ok_and(|start| start.contains("<svg"))
}

fn enforce_pixel_limit(width: u64, height: u64) -> Result<(), String> {
    if width
        .checked_mul(height)
        .is_none_or(|pixels| pixels > MAX_DECODED_PIXELS)
    {
        Err("decoded image is larger than 40 megapixels".to_owned())
    } else {
        Ok(())
    }
}

fn lock<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
    mutex
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
}

pub fn install(context: &egui::Context) {
    egui_extras::install_image_loaders(context);
    if !context.is_loader_installed(SafeBytesLoader::ID) {
        context.add_bytes_loader(Arc::new(SafeBytesLoader::default()));
    }
    if !context.is_loader_installed(SafeImageLoader::ID) {
        context.add_image_loader(Arc::new(SafeImageLoader::default()));
    }
    if !context.is_loader_installed(LocalBytesLoader::ID) {
        context.add_bytes_loader(Arc::new(LocalBytesLoader::default()));
    }
    if !context.is_loader_installed("FastMarkdownViewer bounded textures") {
        context.add_texture_loader(Arc::new(Textures::default()));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_http_and_https_only() {
        assert!(validate_remote_url("https://example.com/image.png").is_ok());
        assert!(validate_remote_url("http://example.com/image.png").is_ok());
        assert!(validate_remote_url("file:///secret.png").is_err());
        assert!(validate_remote_url("data:image/png;base64,AA==").is_err());
    }

    #[test]
    fn rejects_credentials_and_private_literal_hosts() {
        for value in [
            "https://user:pass@example.com/a.png",
            "http://127.0.0.1/a.png",
            "http://10.0.0.1/a.png",
            "http://[::1]/a.png",
            "http://localhost/a.png",
        ] {
            assert!(validate_remote_url(value).is_err(), "accepted {value}");
        }
    }

    #[test]
    fn enforces_decoded_pixel_limit() {
        assert!(enforce_pixel_limit(8_000, 5_000).is_ok());
        assert!(enforce_pixel_limit(8_001, 5_000).is_err());
    }

    #[test]
    fn svg_checks_raster_size_and_preserves_source_dimensions() {
        let svg = br#"<svg xmlns="http://www.w3.org/2000/svg" width="80" height="40"><rect width="80" height="40" fill="red"/></svg>"#;
        let image = decode_svg(svg, SizeHint::Width(160)).unwrap();
        assert_eq!(image.size, [160, 80]);
        assert_eq!(image.source_size, egui::vec2(80.0, 40.0));
        assert_eq!(image.pixels[0], egui::Color32::RED);
        assert!(
            decode_svg(svg, SizeHint::Width(100_000))
                .unwrap_err()
                .contains("40 megapixels")
        );
    }
}
