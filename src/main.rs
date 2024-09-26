extern crate gtk;
extern crate webkit2gtk;

mod connections;
mod settings;
mod styles;
mod tabs;
mod webview;

use connections::get_webview;
use gtk::gdk::keys::constants;
use gtk::gdk_pixbuf::Pixbuf;

use gtk::{glib::Propagation, prelude::*, Box, Button, Entry, Notebook};
use gtk::{Image, Label, Popover, Switch};
use settings::{show_settings_window, Settings};
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;
use std::sync::atomic::{AtomicUsize, Ordering};
use styles::apply_css_style;
use tabs::add_tab;
use tokio;
use webkit2gtk::{CookieManagerExt, WebContext, WebContextExt, WebViewExt};
use webview::{change_webview_setting, WebviewSetting};

static WINDOW_COUNT: AtomicUsize = AtomicUsize::new(0);

#[tokio::main]
async fn main() {
    std::env::set_var("GDK_BACKEND", "x11");
    gtk::init().expect("Failed to initialize GTK.");
    create_window(None);

    gtk::main();
}

pub fn create_window(default_tab_url: Option<&str>) {
    WINDOW_COUNT.fetch_add(1, Ordering::SeqCst);

    let adblock_enabled = Rc::new(RefCell::new(true));

    let provider = gtk::CssProvider::new();

    let css = r#"
        * {
            font-family: "Hack", sans-serif;
        }
        "#;

    provider
        .load_from_data(css.as_bytes())
        .expect("Failed to load css");

    gtk::StyleContext::add_provider_for_screen(
        &gtk::gdk::Screen::default().unwrap(),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

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
    window.set_position(gtk::WindowPosition::Center);

    let path = PathBuf::from("/usr/share/pixmaps/myicon.png");
    let icon = Pixbuf::from_file(path).expect("Failed to load pixbuf");
    window.set_icon(Some(&icon));

    let hbox = Box::new(gtk::Orientation::Vertical, 0);
    let top_bar = Box::new(gtk::Orientation::Horizontal, 0);

    let control_buttons_box = Box::new(gtk::Orientation::Horizontal, 0);

    let back_button = create_button_with_icon("chevron-left");
    let forward_button = create_button_with_icon("chevron-right");
    let refresh_button = create_button_with_icon("rotate-cw");

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
        button { background: transparent; border: none; box-shadow: none; }
        button:hover { background: #2a2a2a; }
        ",
    );

    let search_bar = Entry::new();

    control_buttons_box.set_halign(gtk::Align::Start);

    let menu_buttons_box = Box::new(gtk::Orientation::Horizontal, 0);

    let new_tab_button = create_button_with_icon("plus");
    let download_button = create_button_with_icon("download");

    let menu_button = create_button_with_icon("shield-ban");
    let settings_button = create_button_with_icon("align-justify");

    let menu_popup = Popover::new(Some(&menu_button));

    apply_css_style(
        &[menu_popup.upcast_ref()],
        "popover { background: #2a2a2a; box-shadow: none; }",
    );

    let menu_popup_box = Box::new(gtk::Orientation::Vertical, 0);

    let path = PathBuf::from("/usr/share/pixmaps/monitor-cog.svg");
    let title_img = Pixbuf::from_file(path).expect("Failed to load icon");

    let title = Image::from_pixbuf(Some(&title_img));

    title.set_size_request(150, 150);

    let title_box = Box::new(gtk::Orientation::Horizontal, 0);
    title_box.pack_start(&title, true, true, 5);

    menu_popup_box.pack_start(&title_box, false, false, 5);

    let adblock_box = Box::new(gtk::Orientation::Horizontal, 0);
    let adblock_toggle_label = Label::new(Some("Adblock enabled"));
    let adblock_toggle = Switch::new();

    adblock_toggle.set_state(*adblock_enabled.clone().borrow());

    adblock_box.pack_start(&adblock_toggle_label, false, false, 5);
    adblock_box.pack_end(&adblock_toggle, false, false, 5);

    menu_popup_box.pack_start(&adblock_box, false, false, 5);

    let notebook = Notebook::new();
    notebook.set_action_widget(&new_tab_button, gtk::PackType::End);

    new_tab_button.show();

    notebook.set_show_border(false);
    notebook.set_border_width(0);

    for setting in [
        WebviewSetting::Javascript,
        WebviewSetting::WebGL,
        WebviewSetting::JsClipboardAccess,
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
                    WebviewSetting::JsClipboardAccess => {
                        println!(
                            "Js clipboard access is now {}",
                            if is_active { "enabled" } else { "disabled" }
                        );

                        change_webview_setting(
                            &webview,
                            WebviewSetting::JsClipboardAccess,
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

    menu_buttons_box.pack_start(&download_button, false, false, 5);
    menu_buttons_box.pack_start(&menu_button, false, false, 5);
    menu_buttons_box.pack_start(&settings_button, false, false, 5);

    apply_css_style(
        &[
            &new_tab_button.upcast_ref(),
            &download_button.upcast_ref(),
            &menu_button.upcast_ref(),
            &settings_button.upcast_ref(),
        ],
        "
        button { background: transparent; border: none; box-shadow: none; }
        button:hover { background: #2a2a2a; }
        ",
    );

    top_bar.pack_start(&control_buttons_box, false, false, 0);
    top_bar.pack_start(&search_bar, true, true, 5);
    top_bar.pack_end(&menu_buttons_box, false, false, 0);

    match default_tab_url {
        Some(url) => search_bar.set_text(&url),
        None => search_bar.set_text(""),
    }

    search_bar.set_halign(gtk::Align::Fill);
    search_bar.set_hexpand(true);

    hbox.pack_start(&top_bar, false, false, 5);

    hbox.pack_start(&notebook, true, true, 0);
    notebook.set_scrollable(true);

    match default_tab_url {
        Some(url) => add_tab(&notebook, &search_bar, Some(url)),
        None => add_tab(&notebook, &search_bar, None),
    }

    notebook.connect_drag_end(move |notebook, _| {
        match get_webview(&notebook) {
            Some(webview) => {
                let uri = webview.uri();

                match uri {
                    Some(string) => {
                        create_window(Some(&string));
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
        entry { background: #2a2a2a; border-color: #2d2d2d; margin-bottom: 5px; }
        notebook header.top { background: #202020; box-shadow: none; }
        notebook header.top action-widget { background: #2a2a2a; padding: 5px; box-shadow: none; }
        notebook header.top tabs { background: #202020; }
        notebook header.top tabs tab {
            min-height: 15px;
            min-width: 100px;
            background: transparent;
            border: none;
            border-radius: 7px;
            margin: 4px;
            padding: 2px;
            padding-left: 10px;
            padding-right: 10px;
            transition-duration: 300ms;
        }
        notebook header.top tabs tab:checked { background: #2a2a2a; }
        notebook header.top tabs tab.reorderable-page { box-shadow: none; }
        ",
    );

    window.set_child(Some(&hbox));

    connections::back_button_clicked(&notebook, &back_button);
    connections::forward_button_clicked(&notebook, &forward_button);
    connections::refresh_button_clicked(&notebook, &refresh_button);
    connections::new_tab_button_clicked(&notebook, &new_tab_button, &search_bar);
    connections::search_entry_activate(&search_bar, &notebook);
    connections::notebook_switch_page(&notebook, &search_bar, menu_popup_box);
    connections::settings_button_clicked(&settings_button, &notebook, &search_bar);
    connections::menu_button_clicked(&menu_popup, &menu_button);
    connections::adblock_toggle(&adblock_toggle, adblock_enabled, &notebook);

    window.connect_key_press_event(move |_, key| {
        let keyval = key.keyval();

        // for debugging
        // let state = key.state();
        // println!("Key: {:?}, State: {:?}", keyval, state);

        match keyval {
            constants::F1 => {
                add_tab(&notebook, &search_bar, None);
                Propagation::Stop
            }

            constants::F2 => {
                create_window(None);
                Propagation::Stop
            }

            constants::F12 => {
                show_settings_window();
                Propagation::Stop
            }

            _ => Propagation::Proceed,
        }
    });

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
        WebviewSetting::JsClipboardAccess => (
            "Js Can access clipboard",
            settings.javascript_can_access_clipboard,
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

pub fn create_button_with_icon(icon: &str) -> Button {
    let path = PathBuf::from(format!("/usr/share/pixmaps/{}.svg", icon));
    let pixbuf = Pixbuf::from_file(path).expect("Failed to load icon");
    let image = Image::from_pixbuf(Some(&pixbuf));
    let button = Button::new();
    button.set_image(Some(&image));

    button
}
