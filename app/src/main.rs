use std::io::Write;

use objc2::rc::Retained;
use objc2::runtime::{NSObject, NSObjectProtocol, ProtocolObject};
use objc2::{MainThreadMarker, MainThreadOnly, define_class, msg_send};
use objc2_app_kit::{NSApplication, NSApplicationActivationPolicy, NSApplicationDelegate};
use objc2_foundation::{NSArray, NSNotification, NSString, NSURL};

static LOGFILE: Lazy<std::path::PathBuf> =
    Lazy::new(|| std::env::temp_dir().join("url_opener_output.txt"));


fn log_message(message: &str) {
    // Open for appending, create if it doesn't exist
    let log_file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(LOGFILE.as_path());
    if let Ok(mut file) = log_file {
        let _ = writeln!(file, "{}", message);
    }   
}

define_class!(
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    struct AppDelegate;

    unsafe impl NSObjectProtocol for AppDelegate {}

    unsafe impl NSApplicationDelegate for AppDelegate {
        #[unsafe(method(applicationDidFinishLaunching:))]
        fn did_finish_launching(&self, _notif: &NSNotification) {
            log_message("Application did finish launching");
        }

        // Para esquemas de URL (uds2://…)
        #[unsafe(method(application:openURLs:))]
        fn application_open_urls(&self, _app: &NSApplication, urls: &NSArray<NSURL>) {
            log_message("URLs Opened");
            for url in urls {
                let s = url.absoluteString().unwrap_or_else(|| NSString::from_str("")).to_string();
                log_message(&s);
            }
        }

        // Para ficheros/documentos (arrastrados al icono o “Abrir con…”)
        #[unsafe(method(application:openFile:))]
        fn application_open_file(&self, _app: &NSApplication, filename: &NSString) -> bool {
            log_message("File Opened");
            let s = filename.to_string();
            log_message(&s);
            true
        }

        #[unsafe(method(application:openFiles:))]
        fn application_open_files(&self, _app: &NSApplication, files: &NSArray<NSString>) {
            log_message("Multiple Files Opened");
            for f in files {
                let s = f.to_string();
                log_message(&s);
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

    log_message("************* START *************");
    app.run();
}
