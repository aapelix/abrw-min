extern crate gtk;
extern crate webkit2gtk;

// mod ads;
mod connections;
mod settings;
mod styles;
mod tabs;
mod webview;

use connections::get_webview;
use gtk::gdk_pixbuf::Pixbuf;

use gtk::{glib::Propagation, prelude::*, Box, Button, Entry, Notebook};
use gtk::{Label, Popover, Switch};
use settings::Settings;
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;
use styles::apply_css_style;
use tabs::add_tab;
use tokio;
use webkit2gtk::{CookieManagerExt, WebContext, WebContextExt, WebViewExt};
use webview::{change_webview_setting, WebviewSetting};

use std::sync::atomic::{AtomicUsize, Ordering};

static WINDOW_COUNT: AtomicUsize = AtomicUsize::new(0);

#[tokio::main]
async fn main() {
    std::env::set_var("GDK_BACKEND", "x11");
    gtk::init().expect("Failed to initialize GTK.");
    create_window("https://start.duckduckgo.com/");

    gtk::main();
}

pub fn create_window(default_tab_url: &str) {
    WINDOW_COUNT.fetch_add(1, Ordering::SeqCst);

    let adblock_enabled = Rc::new(RefCell::new(true));

    let context = WebContext::default().unwrap();
    let cookie_manager = WebContextExt::cookie_manager(&context).unwrap();

    let storage_file_path = "cookies.sqlite";
    CookieManagerExt::set_persistent_storage(
        &cookie_manager,
        storage_file_path,
        webkit2gtk::CookiePersistentStorage::Sqlite,
    );

    let window = gtk::Window::new(gtk::WindowType::Toplevel);
    window.set_title("aapelix/abrw");
    window.set_default_size(1500, 900);

    let path = PathBuf::from("/usr/share/pixmaps/myicon.png");
    let icon = Pixbuf::from_file(path).expect("Failed to load pixbuf");
    window.set_icon(Some(&icon));

    let hbox = Box::new(gtk::Orientation::Vertical, 0);
    let top_bar = Box::new(gtk::Orientation::Horizontal, 0);

    let control_buttons_box = Box::new(gtk::Orientation::Horizontal, 0);

    let back_button = Button::with_label("<");
    let forward_button = Button::with_label(">");
    let refresh_button = Button::with_label("↺");

    control_buttons_box.pack_start(&back_button, false, false, 5);
    control_buttons_box.pack_start(&forward_button, false, false, 5);
    control_buttons_box.pack_start(&refresh_button, false, false, 5);

    apply_css_style(
        &[
            &back_button.upcast_ref(),
            &forward_button.upcast_ref(),
            &refresh_button.upcast_ref(),
        ],
        "
        button { background: transparent; border: none; min-width: 20px; min-height: 20px; box-shadow: none; }
        button:hover { background: #2a2a2a; }
        ",
    );

    back_button.set_size_request(25, 25);
    forward_button.set_size_request(25, 25);
    refresh_button.set_size_request(25, 25);

    let search_box = Box::new(gtk::Orientation::Vertical, 0);
    let search_bar = Entry::new();
    search_box.pack_start(&search_bar, true, true, 0);
    search_box.set_halign(gtk::Align::Center);

    search_bar.set_width_request(700);

    control_buttons_box.set_halign(gtk::Align::Start);

    let menu_buttons_box = Box::new(gtk::Orientation::Horizontal, 0);

    let new_tab_button = Button::with_label("+");
    let menu_button = Button::with_label("⋮");
    let settings_button = Button::with_label("⚙");

    let menu_popup = Popover::new(Some(&menu_button));

    let menu_popup_box = Box::new(gtk::Orientation::Vertical, 0);

    let adblock_box = Box::new(gtk::Orientation::Horizontal, 0);
    let adblock_toggle_label = Label::new(Some("Adblock enabled"));
    let adblock_toggle = Switch::new();

    adblock_toggle.set_state(*adblock_enabled.clone().borrow());

    adblock_box.pack_start(&adblock_toggle_label, false, false, 5);
    adblock_box.pack_end(&adblock_toggle, false, false, 5);

    menu_popup_box.pack_start(&adblock_box, false, false, 5);

    let notebook = Notebook::new();

    for setting in [
        WebviewSetting::Javascript,
        WebviewSetting::WebGL,
        WebviewSetting::AutoMediaPlayback,
    ]
    .iter()
    .cloned()
    {
        let toggle = create_toggle_switch(setting.clone(), {
            let notebook = notebook.clone();
            move |is_active| match get_webview(&notebook) {
                Some(webview) => match setting {
                    WebviewSetting::Javascript => {
                        println!(
                            "Javascript is now {}",
                            if is_active { "enabled" } else { "disabled" }
                        );

                        change_webview_setting(&webview, WebviewSetting::Javascript, is_active);
                    }
                    WebviewSetting::WebGL => {
                        println!(
                            "WebGL is now {}",
                            if is_active { "enabled" } else { "disabled" }
                        );

                        change_webview_setting(&webview, WebviewSetting::WebGL, is_active);
                    }
                    WebviewSetting::AutoMediaPlayback => {
                        println!(
                            "Auto Media Playback is now {}",
                            if is_active { "enabled" } else { "disabled" }
                        );

                        change_webview_setting(
                            &webview,
                            WebviewSetting::AutoMediaPlayback,
                            is_active,
                        );
                    }
                },
                None => {
                    println!("No webview")
                }
            }
        });
        menu_popup_box.pack_start(&toggle, false, false, 5);
    }

    menu_popup.add(&menu_popup_box);

    menu_popup_box.show_all();
    menu_popup_box.show();

    menu_buttons_box.pack_start(&new_tab_button, false, false, 5);
    menu_buttons_box.pack_start(&menu_button, false, false, 5);
    menu_buttons_box.pack_start(&settings_button, false, false, 5);

    apply_css_style(
        &[
            &new_tab_button.upcast_ref(),
            &menu_button.upcast_ref(),
            &settings_button.upcast_ref(),
        ],
        "
        button { background: transparent; border: none; min-width: 20px; min-height: 20px; box-shadow: none; }
        button:hover { background: #2a2a2a; }
        ",
    );

    new_tab_button.set_size_request(25, 25);
    menu_button.set_size_request(25, 25);
    settings_button.set_size_request(25, 25);

    top_bar.pack_start(&control_buttons_box, false, false, 0);
    top_bar.pack_start(&search_box, true, true, 5);
    top_bar.pack_end(&menu_buttons_box, false, false, 0);

    search_box.set_hexpand(true);
    search_bar.set_text(&default_tab_url);

    hbox.pack_start(&top_bar, false, false, 5);

    hbox.pack_start(&notebook, true, true, 0);
    notebook.set_scrollable(true);

    add_tab(&notebook, &search_bar, Some(&default_tab_url));

    notebook.connect_drag_end(move |notebook, _| {
        match get_webview(&notebook) {
            Some(webview) => {
                let uri = webview.uri();

                match uri {
                    Some(string) => {
                        create_window(&string);
                    }
                    None => {}
                }
            }
            None => {
                println!("No web view");
            }
        };
    });

    apply_css_style(
        &[
            &hbox.upcast_ref(),
            &search_bar.upcast_ref(),
            &notebook.upcast_ref(),
        ],
        "
        box { background: #202020; }
        entry { background: #2a2a2a; border-color: #2d2d2d; }
        notebook header.top tabs { background: #202020; }
        notebook header.top tabs tab { min-height: 1px; min-width: 100px; background: transparent; border: none; border-radius: 7px; margin: 4px; padding: 10px; transition-duration: 300ms; }
        notebook header.top tabs tab:checked { background: #2a2a2a }
        notebook header.top tabs tab.reorderable-page { border: none; }
        ",
    );

    window.set_child(Some(&hbox));

    connections::back_button_clicked(&notebook, &back_button);
    connections::forward_button_clicked(&notebook, &forward_button);
    connections::refresh_button_clicked(&notebook, &refresh_button);
    connections::new_tab_button_clicked(&notebook, &new_tab_button, &search_bar);
    connections::search_entry_activate(&search_bar, &notebook);
    connections::notebook_switch_page(&notebook, &search_bar, menu_popup_box);
    connections::settings_button_clicked(&settings_button);
    connections::menu_button_clicked(&menu_popup, &menu_button);
    connections::adblock_toggle(&adblock_toggle, adblock_enabled, &notebook);

    window.connect_delete_event(move |_, _| {
        WINDOW_COUNT.fetch_sub(1, Ordering::SeqCst);

        if WINDOW_COUNT.load(Ordering::SeqCst) == 0 {
            gtk::main_quit();
            Propagation::Stop
        } else {
            Propagation::Proceed
        }
    });

    // Show all widgets
    window.show_all();
}

fn create_toggle_switch<F>(setting: WebviewSetting, callback: F) -> Box
where
    F: Fn(bool) + 'static,
{
    let menu_popup_box = Box::new(gtk::Orientation::Horizontal, 5);
    let settings = Settings::load();

    let (label_text, toggle_state) = match setting {
        WebviewSetting::Javascript => ("Javascript enabled", settings.enable_javascript),
        WebviewSetting::WebGL => ("WebGL enabled", settings.enable_webgl),
        WebviewSetting::AutoMediaPlayback => (
            "Auto Media Playback enabled",
            settings.media_playback_requires_user_gesture,
        ),
    };

    let toggle_label = Label::new(Some(label_text));
    let toggle = Switch::new();
    toggle.set_active(toggle_state);

    toggle.connect_active_notify(move |switch| {
        let is_active = switch.is_active();
        callback(is_active);
    });

    menu_popup_box.pack_start(&toggle_label, false, false, 5);
    menu_popup_box.pack_end(&toggle, false, false, 5);

    menu_popup_box
}
