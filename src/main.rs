#![windows_subsystem = "windows"]
/**
    A very simple application that show your name in a message box.

    This demo shows how to use NWG without the NativeUi trait boilerplate.
    Note that this way of doing things is alot less extensible and cannot make use of native windows derive.

    See `basic` for the NativeUi version and `basic_d` for the derive version
*/
extern crate native_windows_gui as nwg;
use std::rc::Rc;
//for include the SwapMouseButton func
mod func;
pub use self::func::func::swap_mouse_button;

fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");

    let mut window = Default::default();
    let mut window_ico = Default::default(); //图标

    let mut embed = Default::default();
    let layout = Default::default();
    let mut combobox = Default::default();
    let mut label = Default::default();
    let data = vec!["Left", "Right"];

    //Menu
    let mut help_menu = Default::default();
    let mut about_menu = Default::default();
    //   let mut menu_sep = Default::default();
    //   let mut update_menu = Default::default();


    nwg::EmbedResource::builder()
        .build(&mut embed)
        .unwrap();

    nwg::Icon::builder()
        .source_embed(Some(&mut embed))
        .source_embed_str(Some("MAINICON"))
        .build(&mut window_ico)
        .unwrap();

    nwg::Window::builder()
        .icon(Some(&mut window_ico))
        .size((300, 115))
        .position((300, 300))
        .title("SwapMouseButton")
        .build(&mut window)
        .unwrap();

    //构建menu
    nwg::Menu::builder()
        .text("&Help")
        .parent(&mut window)
        .build(&mut help_menu)
        .unwrap();

    nwg::MenuItem::builder()
        .text("About")
        .parent(&mut help_menu)
        .build(&mut about_menu)
        .unwrap();
    /*
        nwg::MenuSeparator::builder()
            .parent(&mut help_menu)
            .build(&mut menu_sep)
            .unwrap();

        nwg::MenuItem::builder()
            .text("update check")
            .parent(&mut help_menu)
            .build(&mut update_menu)
            .unwrap();
    */
    nwg::ComboBox::builder()
        .size((100, 100))
        .collection(data)
        .selected_index(Some(0))
        .parent(&window)
        .build(&mut combobox)
        .unwrap();

    nwg::Label::builder()
        .text("Left->left-handed\nRight->right-handed")
        .parent(&window)
        .build(&mut label)
        .unwrap();

    nwg::GridLayout::builder()
        .parent(&window)
        .spacing(1)
        .child(0, 0, &combobox)
        .child(0, 1, &label)
        .build(&layout)
        .unwrap();

    let window = Rc::new(window);
    let events_window = window.clone();

    let handler = nwg::full_bind_event_handler(&window.handle, move |evt, _evt_data, handle| {
        use nwg::Event as E;

        match evt {
            E::OnWindowClose => {
                if &handle == &events_window as &nwg::Window {
                    nwg::modal_info_message(&events_window.handle, "Goodbye", &format!("Goodbye "));
                    nwg::stop_thread_dispatch();
                }
            }
            E::OnMenuItemSelected => {
                if &handle == &about_menu {
                    let about_msg = "If this parameter is Left, the left button generates right-button messages and the right button generates left-button messages. If this parameter is Right, the buttons are restored to their original meanings".to_string();
                    nwg::modal_info_message(
                        &events_window.handle,
                        "About",
                        &format!("SwapMouseButton\n Reverses or restores the meaning of the left and right mouse buttons!\n {}",about_msg),
                    );
                }
                /*
                if &handle == &update_menu {
                    nwg::modal_info_message(
                        &events_window.handle,
                        "update check",
                        &format!("This software write by sndnvaps!"),
                    );
                }
                */
            }

            E::OnComboxBoxSelection => {
                if &handle == &combobox {
                    if combobox.selection_string().eq(&Some("Left".to_string())) {
                        let err = swap_mouse_button(true);
                        unsafe {
                            err.unwrap_unchecked();
                        }
                    }
                    if combobox.selection_string().eq(&Some("Right".to_string())) {
                        swap_mouse_button(false).unwrap();
                    }
                }
            }
            _ => {}
        }
    });

    nwg::dispatch_thread_events();
    nwg::unbind_event_handler(&handler);
}
