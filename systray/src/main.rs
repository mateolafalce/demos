use fltk::{app, enums::FrameType, prelude::*, *};

#[cfg(target_os = "windows")]
mod systray;

type HWND = *mut std::os::raw::c_void;
pub static mut WINDOW: HWND = std::ptr::null_mut();

fn main() {
    let app = app::App::default();
    let mut win = window::Window::default().with_size(400, 300);
    let mut frame = frame::Frame::new(10, 10, 380, 200, "");
    frame.set_frame(FrameType::EngravedBox);
    let mut but = button::Button::new(160, 220, 80, 40, "Click me!");
    win.end();
    win.show();

    but.set_callback(move |_| frame.set_label("Hello world!"));

    #[cfg(target_os = "windows")]
    {
        unsafe {
            WINDOW = win.raw_handle();
        }
        win.set_callback(|w| {
            // We intercept the closing of the window here
            unsafe {
                w.platform_hide();
            }
        });
        use crate::systray::NativeUi;
        systray::init().expect("Failed to init Native Windows GUI");
        let _ui = systray::SystemTray::build_ui(Default::default()).expect("Failed to build UI");
        systray::dispatch_thread_events_with_callback(move || {
            if win.shown() {
                app.run().unwrap();
            } else {
                app::sleep(0.030);
            }
        });
    }

    #[cfg(not(target_os = "windows"))]
    app.run().unwrap();
}
