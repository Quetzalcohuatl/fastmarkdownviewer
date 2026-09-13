//! Main-thread Apple-event adapter. The viewer consumes only safe Rust messages.
#![deny(unsafe_op_in_unsafe_fn)]

use objc2::rc::Retained;
use objc2::{DefinedClass, MainThreadOnly, define_class, msg_send, sel};
use objc2_foundation::{
    MainThreadMarker, NSAppleEventDescriptor, NSAppleEventManager, NSNotification,
    NSNotificationCenter, NSObject, NSObjectProtocol, ns_string,
};
use std::{cell::RefCell, path::PathBuf, rc::Rc};

pub enum Event {
    Open(Vec<PathBuf>),
    Quit,
}

#[derive(Clone, Default)]
pub struct Inbox(Rc<RefCell<Pending>>);

#[derive(Default)]
struct Pending {
    events: Vec<Event>,
    wake: Option<Box<dyn Fn()>>,
}

impl Inbox {
    pub fn drain(&self) -> Vec<Event> {
        std::mem::take(&mut self.0.borrow_mut().events)
    }
    pub fn set_wake(&self, wake: impl Fn() + 'static) {
        self.0.borrow_mut().wake = Some(Box::new(wake));
    }
    fn push(&self, event: Event) {
        let mut pending = self.0.borrow_mut();
        pending.events.push(event);
        if let Some(wake) = &pending.wake {
            wake();
        }
    }
}

define_class!(
    // SAFETY: NSObject has no subclass invariants; callbacks and ivars stay on the main thread.
    #[unsafe(super = NSObject)]
    #[thread_kind = MainThreadOnly]
    #[ivars = Inbox]
    struct Handler;
    unsafe impl NSObjectProtocol for Handler {}
    impl Handler {
        #[unsafe(method(installHandlers:))]
        fn will_finish(&self, _notification: &NSNotification) { self.register(); }

        // SAFETY: Apple-event selectors take two NSAppleEventDescriptor objects and return void.
        #[unsafe(method(openDocuments:withReplyEvent:))]
        fn open(&self, event: &NSAppleEventDescriptor, _reply: &NSAppleEventDescriptor) {
            // SAFETY: keyDirectObject ('----') is an AEKeyword/u32; Cocoa returns a nullable descriptor.
            let list: Option<Retained<NSAppleEventDescriptor>> = unsafe {
                msg_send![event, paramDescriptorForKeyword: u32::from_be_bytes(*b"----")]
            };
            let Some(list) = list else { return; };
            let paths = (1..=list.numberOfItems()).filter_map(|index| {
                let url = list.descriptorAtIndex(index)?.fileURLValue()?;
                if !url.isFileURL() { return None; }
                Some(PathBuf::from(url.path()?.to_string()))
            }).collect::<Vec<_>>();
            if !paths.is_empty() { self.ivars().push(Event::Open(paths)); }
        }

        #[unsafe(method(quitApplication:withReplyEvent:))]
        fn quit(&self, _event: &NSAppleEventDescriptor, _reply: &NSAppleEventDescriptor) {
            self.ivars().push(Event::Quit);
        }
    }
);

impl Handler {
    fn register(&self) {
        let manager = NSAppleEventManager::sharedAppleEventManager();
        // SAFETY: Both selectors above have Apple's required two-descriptor signature.
        // The owner retains self until after the event loop stops; four-character codes are u32.
        unsafe {
            let _: () = msg_send![&*manager, setEventHandler: self,
                andSelector: sel!(openDocuments:withReplyEvent:),
                forEventClass: u32::from_be_bytes(*b"aevt"), andEventID: u32::from_be_bytes(*b"odoc")];
            let _: () = msg_send![&*manager, setEventHandler: self,
                andSelector: sel!(quitApplication:withReplyEvent:),
                forEventClass: u32::from_be_bytes(*b"aevt"), andEventID: u32::from_be_bytes(*b"quit")];
        }
    }
}

/// Retain this guard on the main thread until the native event loop returns.
pub struct Registration(Retained<Handler>);

impl Registration {
    pub fn install() -> (Self, Inbox) {
        let mtm = MainThreadMarker::new().expect("Apple events require the main thread");
        let inbox = Inbox::default();
        let allocated = Handler::alloc(mtm).set_ivars(inbox.clone());
        // SAFETY: NSObject init completes initialization of our allocated subclass.
        let handler: Retained<Handler> = unsafe { msg_send![super(allocated), init] };
        // Register at will-finish-launching, after AppKit setup but before Finder's launch event.
        // We do not replace winit's application delegate or its lifecycle callbacks.
        unsafe {
            NSNotificationCenter::defaultCenter().addObserver_selector_name_object(
                &handler,
                sel!(installHandlers:),
                Some(ns_string!("NSApplicationWillFinishLaunchingNotification")),
                None,
            );
        }
        handler.register();
        (Self(handler), inbox)
    }
}

impl Drop for Registration {
    fn drop(&mut self) {
        // SAFETY: unregister the exact retained observer and our own event selectors before release.
        unsafe {
            NSNotificationCenter::defaultCenter().removeObserver(&self.0);
            let manager = NSAppleEventManager::sharedAppleEventManager();
            for id in [*b"odoc", *b"quit"] {
                let _: () = msg_send![&*manager, removeEventHandlerForEventClass: u32::from_be_bytes(*b"aevt"),
                    andEventID: u32::from_be_bytes(id)];
            }
        }
    }
}
