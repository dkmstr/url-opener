use objc2::rc::Retained;
use objc2::runtime::{NSObject, NSObjectProtocol, ProtocolObject};
use objc2::{define_class, MainThreadMarker, MainThreadOnly, msg_send};
use objc2_app_kit::{NSApplication, NSApplicationDelegate};
use objc2_foundation::{NSArray, NSURL};
use fltk::{app, prelude::*, window::Window};

// Define AppDelegate en main thread y con protocolos correctos
define_class!(
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    struct AppDelegate;

    unsafe impl NSObjectProtocol for AppDelegate {}

    unsafe impl NSApplicationDelegate for AppDelegate {
        #[unsafe(method(application:openURLs:))]
        fn application_open_urls(
            &self,
            _app: &NSApplication,
            urls: &NSArray<NSURL>,
        ) {
            for url in urls {
                if let Some(s) = url.absoluteString() {
                    println!("Recibido URL: {}", s);
                }
            }
        }
    }
);

// Crea el delegado llamando al init de la propia clase
fn new_delegate(mtm: MainThreadMarker) -> Retained<AppDelegate> {
    let alloc = mtm.alloc::<AppDelegate>();
    unsafe {
        // Direct init on subclass; returns Retained<AppDelegate>
        msg_send![alloc, init]
    }
}

fn main() {
    let mtm = MainThreadMarker::new().expect("Debe ejecutarse en el hilo principal");

    // sharedApplication requiere main thread
    let app = NSApplication::sharedApplication(mtm);

    // Crear y registrar el delegate
    let delegate = new_delegate(mtm);
    let proto: &ProtocolObject<dyn NSApplicationDelegate> = ProtocolObject::from_ref(&*delegate);
    app.setDelegate(Some(proto));

    // Ventana FLTK mínima
    let fl_app = app::App::default();
    let mut win = Window::new(100, 100, 400, 300, "FLTK + objc2");
    win.end();
    win.show();
    fl_app.run().unwrap();
}
