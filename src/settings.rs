use gtk::{prelude::*, STYLE_PROVIDER_PRIORITY_APPLICATION};
use gtk::{Box, Label, Orientation, Switch, Window, WindowType};
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::fs;
use std::rc::Rc;
use webkit2gtk::WebView;

use crate::styles::apply_css_style;
use crate::webview::toggle_content_filter;

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
    pub enable_html5_local_storage: bool,
    pub enable_html5_database: bool,
    pub enable_xss_auditor: bool,
    pub enable_hyperlink_auditing: bool,
    pub enable_dns_prefetching: bool,
    pub allow_modal_dialogs: bool,
    pub javascript_can_open_windows_automatically: bool,
    pub javascript_can_access_clipboard: bool,
    pub enable_site_specific_quirks: bool,
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
    let settings = Rc::new(RefCell::new(Settings::load()));

    let window = Window::new(WindowType::Toplevel);
    window.set_title("Abrw Settings");
    window.set_default_size(500, 700);

    let hbox = Box::new(Orientation::Horizontal, 10);

    apply_css_style(&[&hbox.upcast_ref()], "box { background: #202020 }");

    // Sidebar for categories
    let sidebar = Box::new(Orientation::Vertical, 10);
    let settings_stack = gtk::Stack::new();
    settings_stack.set_transition_type(gtk::StackTransitionType::SlideUpDown);

    let categories = vec![
        "General".to_string(),
        "Privacy".to_string(),
        "Advanced".to_string(),
    ];

    for category in categories.clone() {
        let button = gtk::Button::with_label(&category);
        let settings_stack_clone = settings_stack.clone();

        apply_css_style(
            &[&button.upcast_ref()],
            "button { background: transparent; box-shadow: none; }",
        );

        button.connect_clicked(move |_| {
            settings_stack_clone.set_visible_child_name(&category);
        });

        sidebar.pack_start(&button, false, false, 5);
    }

    let general_box = Box::new(Orientation::Vertical, 10);
    let privacy_box = Box::new(Orientation::Vertical, 10);
    let advanced_box = Box::new(Orientation::Vertical, 10);

    create_setting(
        "Enable JavaScript",
        |s| s.enable_javascript,
        |s, v| s.enable_javascript = v,
        &general_box,
        &settings,
    );

    create_setting(
        "Enable Local Storage",
        |s| s.enable_local_storage,
        |s, v| s.enable_local_storage = v,
        &general_box,
        &settings,
    );

    create_setting(
        "Private browsing",
        |s| s.private_browsing,
        |s, v| s.private_browsing = v,
        &privacy_box,
        &settings,
    );

    create_setting(
        "Do Not Track",
        |s| s.do_not_track,
        |s, v| s.do_not_track = v,
        &privacy_box,
        &settings,
    );

    create_setting(
        "Enable WebGL",
        |s| s.enable_webgl,
        |s, v| s.enable_webgl = v,
        &advanced_box,
        &settings,
    );

    create_setting(
        "Enable Html5 Local Storage",
        |s| s.enable_html5_local_storage,
        |s, v| s.enable_html5_local_storage = v,
        &advanced_box,
        &settings,
    );

    create_setting(
        "Enable Html5 Database",
        |s| s.enable_html5_database,
        |s, v| s.enable_html5_database = v,
        &advanced_box,
        &settings,
    );

    create_setting(
        "Enable XSS auditor",
        |s| s.enable_xss_auditor,
        |s, v| s.enable_xss_auditor = v,
        &advanced_box,
        &settings,
    );

    create_setting(
        "Enable Hyperlink Audting",
        |s| s.enable_hyperlink_auditing,
        |s, v| s.enable_hyperlink_auditing = v,
        &advanced_box,
        &settings,
    );

    create_setting(
        "Enable Dns Prefetching",
        |s| s.enable_dns_prefetching,
        |s, v| s.enable_dns_prefetching = v,
        &advanced_box,
        &settings,
    );

    create_setting(
        "Allow Modal Dialogs",
        |s| s.allow_modal_dialogs,
        |s, v| s.allow_modal_dialogs = v,
        &advanced_box,
        &settings,
    );

    create_setting(
        "JS Open Windows Automatically",
        |s| s.javascript_can_open_windows_automatically,
        |s, v| s.javascript_can_open_windows_automatically = v,
        &advanced_box,
        &settings,
    );

    create_setting(
        "JS Can access clipboard",
        |s| s.javascript_can_access_clipboard,
        |s, v| s.javascript_can_access_clipboard = v,
        &privacy_box,
        &settings,
    );

    create_setting(
        "Media playback requires user gesture",
        |s| s.media_playback_requires_user_gesture,
        |s, v| s.media_playback_requires_user_gesture = v,
        &advanced_box,
        &settings,
    );

    create_setting(
        "Enable Site Specific Quirks",
        |s| s.enable_site_specific_quirks,
        |s, v| s.enable_site_specific_quirks = v,
        &advanced_box,
        &settings,
    );

    settings_stack.add_named(&general_box, "General");
    settings_stack.add_named(&privacy_box, "Privacy");
    settings_stack.add_named(&advanced_box, "Advanced");

    // Pack the sidebar and settings stack into the main horizontal box
    hbox.pack_start(&sidebar, false, false, 5);
    hbox.pack_start(&settings_stack, true, true, 5);

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

    // Apply styles
    let vbox_style = sidebar.style_context();
    vbox_style.add_class("box");
    vbox_style.add_provider(&css_provider, STYLE_PROVIDER_PRIORITY_APPLICATION);

    window.add(&hbox);
    window.show_all();
}

// Adjusted create_setting function to accept a box parameter
fn create_setting(
    label: &str,
    get_value: fn(&Settings) -> bool,
    set_value: fn(&mut Settings, bool),
    parent_box: &Box,
    settings: &Rc<RefCell<Settings>>,
) {
    let hbox = Box::new(Orientation::Horizontal, 0);
    let setting_label = Label::new(Some(label));
    let switch = Switch::new();
    let settings_clone = Rc::clone(settings);
    switch.set_active(get_value(&settings_clone.borrow()));

    setting_label.set_halign(gtk::Align::Start);

    hbox.pack_start(&setting_label, true, true, 5);
    hbox.pack_end(&switch, false, false, 5);
    parent_box.pack_start(&hbox, false, false, 5);

    switch.connect_active_notify(move |switch| {
        let mut settings = settings_clone.borrow_mut();
        set_value(&mut settings, switch.is_active());
        settings.save(); // Save settings to file
    });
}

pub fn toggle_adblock(adblock_enabled: Rc<RefCell<bool>>, webview: &WebView) {
    let current_value = *adblock_enabled.borrow();
    *adblock_enabled.borrow_mut() = !current_value;

    if current_value {
        println!("Adblocker disabled.");
        toggle_content_filter(webview, current_value);
    } else {
        println!("Adblocker enabled.");
        toggle_content_filter(webview, current_value);
    }
}
