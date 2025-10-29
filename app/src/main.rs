use std::ffi::CStr;

use objc2::rc::Retained;
use objc2::runtime::{NSObject, NSObjectProtocol, ProtocolObject};
use objc2::{define_class, MainThreadMarker, MainThreadOnly, msg_send};
use objc2_app_kit::{NSApplication, NSApplicationDelegate, NSApplicationActivationPolicy};
use objc2_foundation::{NSArray, NSURL, NSString, NSNotification};

define_class!(
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    struct AppDelegate;

    unsafe impl NSObjectProtocol for AppDelegate {}

    unsafe impl NSApplicationDelegate for AppDelegate {
        #[unsafe(method(applicationDidFinishLaunching:))]
        fn did_finish_launching(&self, _notif: &NSNotification) {
            println!("App arrancada, esperando URLs o ficheros…");
        }

        // Para esquemas de URL (uds2://…)
        #[unsafe(method(application:openURLs:))]
        fn application_open_urls(&self, _app: &NSApplication, urls: &NSArray<NSURL>) {
            for url in urls {
                if let Some(nsstr) = url.absoluteString() {
                    let c_ptr = nsstr.UTF8String();
                    let s = unsafe { CStr::from_ptr(c_ptr) }
                        .to_string_lossy()
                        .into_owned();
                    std::fs::write("/tmp/url_opener_output.txt", s.as_bytes()).unwrap();
                }
            }
        }

        // Para ficheros/documentos (arrastrados al icono o “Abrir con…”)
        #[unsafe(method(application:openFile:))]
        fn application_open_file(&self, _app: &NSApplication, filename: &NSString) -> bool {
            let s = filename.to_string();
            std::fs::write("/tmp/url_opener_output.txt", s.as_bytes()).unwrap();
            true
        }

        #[unsafe(method(application:openFiles:))]
        fn application_open_files(&self, _app: &NSApplication, files: &NSArray<NSString>) {
            for f in files {
                let s = f.to_string();
                std::fs::write("/tmp/url_opener_output.txt", s.as_bytes()).unwrap();
            }
        }
    }
);

fn new_delegate(mtm: MainThreadMarker) -> Retained<AppDelegate> {
    let alloc = mtm.alloc::<AppDelegate>();
    unsafe { msg_send![alloc, init] }
}

fn main() {
    let mtm = MainThreadMarker::new().unwrap();
    let app = NSApplication::sharedApplication(mtm);

    let delegate = Box::leak(Box::new(new_delegate(mtm)));
    let proto: &ProtocolObject<dyn NSApplicationDelegate> = ProtocolObject::from_ref(&**delegate);
    app.setDelegate(Some(proto));

    // Mantener la app viva aunque no tenga ventanas
    app.setActivationPolicy(NSApplicationActivationPolicy::Accessory);

    app.run();
}
