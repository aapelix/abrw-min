use gtk::{glib::Propagation, Box, Label, Orientation, Switch, Window, WindowType};
use gtk::{prelude::*, STYLE_PROVIDER_PRIORITY_APPLICATION};
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::fs;
use std::rc::Rc;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Settings {
    pub private_browsing: bool,
    pub enable_javascript: bool,
    pub enable_webgl: bool,
    pub offline_web_app_cache: bool,
    pub page_cache: bool,
    pub enable_media_capabilities: bool,
    pub do_not_track: bool,
    pub enable_local_storage: bool,
    pub enable_indexed_db: bool,
    pub media_playback_requires_user_gesture: bool,
}

impl Settings {
    pub fn save(&self) {
        let json_data = serde_json::to_string(self).expect("Failed to serialize settings.");
        fs::write("settings.json", json_data).expect("Failed to write settings to file.");
    }

    pub fn load() -> Settings {
        if let Ok(data) = fs::read_to_string("settings.json") {
            serde_json::from_str(&data).unwrap_or_default()
        } else {
            Settings::default()
        }
    }
}

pub fn show_settings_window() {
    let settings = Rc::new(RefCell::new(Settings::load())); // Load settings from file

    let window = Window::new(WindowType::Toplevel);
    window.set_title("Abrw Settings");
    window.set_default_size(500, 700);

    window.connect_button_press_event(|window, event| {
        if event.button() == 1 {
            window.begin_move_drag(
                event.button().try_into().unwrap(),
                event.root().0 as i32,
                event.root().1 as i32,
                event.time(),
            );
        }

        Propagation::Stop
    });

    let vbox = Box::new(Orientation::Vertical, 10);

    let css_provider = gtk::CssProvider::new();
    css_provider
        .load_from_data(
            b"

        .box {
            background: #212121;
        }
    ",
        )
        .expect("Failed to load css");
    let vbox_style = vbox.style_context();
    vbox_style.add_class("box");
    vbox_style.add_provider(&css_provider, STYLE_PROVIDER_PRIORITY_APPLICATION);

    let create_setting =
        |label: &str, get_value: fn(&Settings) -> bool, set_value: fn(&mut Settings, bool)| {
            let hbox = Box::new(Orientation::Horizontal, 0);
            let setting_label = Label::new(Some(label));
            let switch = Switch::new();
            let settings_clone = Rc::clone(&settings);
            switch.set_active(get_value(&settings_clone.borrow()));

            setting_label.set_halign(gtk::Align::Start);

            hbox.pack_start(&setting_label, true, true, 5);
            hbox.pack_end(&switch, false, false, 5);
            vbox.pack_start(&hbox, false, false, 5);

            switch.connect_active_notify(move |switch| {
                let mut settings = settings_clone.borrow_mut();
                set_value(&mut settings, switch.is_active());
                settings.save(); // Save settings to file
            });
        };

    create_setting(
        "Private browsing",
        |s| s.private_browsing,
        |s, v| s.private_browsing = v,
    );
    create_setting(
        "Enable JavaScript",
        |s| s.enable_javascript,
        |s, v| s.enable_javascript = v,
    );
    create_setting(
        "Enable WebGL",
        |s| s.enable_webgl,
        |s, v| s.enable_webgl = v,
    );
    create_setting(
        "Offline Web App Cache",
        |s| s.offline_web_app_cache,
        |s, v| s.offline_web_app_cache = v,
    );
    create_setting("Page Cache", |s| s.page_cache, |s, v| s.page_cache = v);
    create_setting(
        "Enable Media Capabilities",
        |s| s.enable_media_capabilities,
        |s, v| s.enable_media_capabilities = v,
    );
    create_setting(
        "Do Not Track",
        |s| s.do_not_track,
        |s, v| s.do_not_track = v,
    );
    create_setting(
        "Enable Local Storage",
        |s| s.enable_local_storage,
        |s, v| s.enable_local_storage = v,
    );
    create_setting(
        "Enable Indexed DB",
        |s| s.enable_indexed_db,
        |s, v| s.enable_indexed_db = v,
    );
    create_setting(
        "Media playback requires user gesture",
        |s| s.media_playback_requires_user_gesture,
        |s, v| s.media_playback_requires_user_gesture = v,
    );

    window.add(&vbox);
    window.show_all();
}

pub fn toggle_adblock(adblock_enabled: Rc<RefCell<bool>>) {
    let current_value = *adblock_enabled.borrow();
    *adblock_enabled.borrow_mut() = !current_value;

    if current_value {
        println!("Adblocker disabled.");
    } else {
        println!("Adblocker enabled.");
    }
}
