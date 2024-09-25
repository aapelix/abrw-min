extern crate gtk;
extern crate webkit2gtk;

use gtk::{prelude::*, Box, Button, Entry, Label, Notebook, Popover, Switch};
use log::info;
use std::cell::RefCell;
use std::rc::Rc;
use url::Url;
use webkit2gtk::{WebView, WebViewExt};

use crate::create_window;
use crate::settings::toggle_adblock;
use crate::styles::apply_css_style;
use crate::webview::{change_webview_setting, get_webview_setting, WebviewSetting};
use crate::{settings::show_settings_window, tabs::add_tab};

pub fn get_webview(notebook: &Notebook) -> Option<WebView> {
    let current_page = notebook.current_page();

    if let Some(widget) = notebook.nth_page(current_page) {
        if let Some(webview) = widget.downcast_ref::<WebView>() {
            return Some(webview.clone());
        }
    }

    return None;
}

pub fn back_button_clicked(notebook: &Notebook, back_button: &Button) {
    back_button.connect_clicked({
        let notebook = notebook.clone();
        move |_| match get_webview(&notebook) {
            Some(webview) => {
                if webview.can_go_back() {
                    webview.go_back();
                }
            }
            None => {
                info!("Current tab doesn't have a webview")
            }
        }
    });
}

pub fn forward_button_clicked(notebook: &Notebook, forward_button: &Button) {
    forward_button.connect_clicked({
        let notebook = notebook.clone();
        move |_| match get_webview(&notebook) {
            Some(webview) => {
                if webview.can_go_forward() {
                    webview.go_forward();
                }
            }
            None => {
                info!("Current tab doesn't have a webview")
            }
        }
    });
}

pub fn refresh_button_clicked(notebook: &Notebook, refresh_button: &Button) {
    refresh_button.connect_clicked({
        let notebook = notebook.clone();
        move |_| match get_webview(&notebook) {
            Some(webview) => {
                webview.reload();
            }
            None => {
                info!("Current tab doesn't have a webview")
            }
        }
    });
}

pub fn new_tab_button_clicked(notebook: &Notebook, new_tab_button: &Button, search_entry: &Entry) {
    new_tab_button.connect_clicked({
        let notebook = notebook.clone();
        let search_entry = search_entry.clone();

        move |_| {
            add_tab(
                &notebook,
                &search_entry,
                Some("https://start.duckduckgo.com/"),
            )
        }
    });
}

pub fn search_entry_activate(search_entry: &Entry, notebook: &Notebook) {
    search_entry.connect_activate({
        let notebook = notebook.clone();

        move |search_entry| {
            let url = search_entry.text();

            if url.is_empty() {
                return;
            }

            match get_webview(&notebook) {
                Some(webview) => {
                    let url_str = url.as_str();

                    if let Ok(url) = Url::parse(url_str) {
                        if url.scheme() == "http" || url.scheme() == "https" {
                            if url.host_str() == Some("localhost") || url.path() == "/" {
                                info!("Local URL detected!");
                                webview.load_uri(&url_str);
                                return;
                            }
                        } else if url.scheme() == "file" {
                            info!("File URL detected!");
                            webview.load_uri(&url_str);
                            return;
                        }

                        webview.load_uri(&url_str);

                        return;
                    }

                    let domain_like = url_str.contains('.') && !url_str.contains(' ');

                    if domain_like {
                        info!("URL detected (no scheme)!");
                        webview.load_uri(&format!("https://{}", &url_str));

                        return;
                    }

                    info!("Search query detected");
                    let search_query = url.to_string().replace(" ", "+");
                    webview.load_uri(&format!("https://duckduckgo.com/?q={}", &search_query));
                    return;
                }
                None => info!("Current tab doesn't have a webview"),
            }
        }
    });
}

pub fn notebook_switch_page(notebook: &Notebook, search_entry: &Entry, menu_popup_box: Box) {
    notebook.connect_switch_page({
        let search_entry = search_entry.clone();
        let menu_popup_box = menu_popup_box.clone();

        move |notebook, _, page_num| {
            if let Some(widget) = notebook.nth_page(Some(page_num)) {
                if let Some(webview) = widget.downcast_ref::<webkit2gtk::WebView>() {
                    if let Some(uri) = webview.uri() {
                        search_entry.set_text(&uri);
                    }

                    for child in menu_popup_box.children() {
                        if let Some(sub_box) = child.downcast_ref::<Box>() {
                            for sub_child in sub_box.children() {
                                if let Some(label) = sub_child.downcast_ref::<Label>() {
                                    let label_text = label.text();

                                    if label_text == "Javascript enabled" {
                                        for sub_sub_child in sub_box.children() {
                                            if let Some(_switch) =
                                                sub_sub_child.downcast_ref::<Switch>()
                                            {
                                                let toggle_state = get_webview_setting(
                                                    &webview,
                                                    WebviewSetting::Javascript,
                                                )
                                                .expect("Failed to get setting");

                                                change_webview_setting(
                                                    &webview,
                                                    WebviewSetting::Javascript,
                                                    toggle_state,
                                                );

                                                // now i would also need to set the state of the switch

                                                break;
                                            }
                                        }
                                    }

                                    if label_text == "WebGL enabled" {
                                        for sub_sub_child in sub_box.children() {
                                            if let Some(_switch) =
                                                sub_sub_child.downcast_ref::<Switch>()
                                            {
                                                let toggle_state = get_webview_setting(
                                                    &webview,
                                                    WebviewSetting::WebGL,
                                                )
                                                .expect("Failed to get setting");

                                                change_webview_setting(
                                                    &webview,
                                                    WebviewSetting::WebGL,
                                                    toggle_state,
                                                );

                                                // now i would also need to set the state of the switch

                                                break;
                                            }
                                        }
                                    }

                                    if label_text == "Auto Media Playback enabled" {
                                        for sub_sub_child in sub_box.children() {
                                            if let Some(_switch) =
                                                sub_sub_child.downcast_ref::<Switch>()
                                            {
                                                let toggle_state = get_webview_setting(
                                                    &webview,
                                                    WebviewSetting::AutoMediaPlayback,
                                                )
                                                .expect("Failed to get setting");

                                                change_webview_setting(
                                                    &webview,
                                                    WebviewSetting::AutoMediaPlayback,
                                                    toggle_state,
                                                );

                                                // now i would also need to set the state of the switch

                                                break;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    });
}

pub fn settings_button_clicked(
    settings_button: &Button,
    notebook: &Notebook,
    search_entry: &Entry,
) {
    let notebook = notebook.clone();
    let search_entry = search_entry.clone();
    settings_button.connect_clicked(move |button| {
        let popup = Popover::new(Some(button));

        apply_css_style(
            &[popup.upcast_ref()],
            "popover { background: #2a2a2a; box-shadow: none; padding: 7px; }",
        );

        let popup_box = Box::new(gtk::Orientation::Vertical, 0);

        create_button_with_callback("New tab", &popup_box, "F1", {
            let notebook = notebook.clone();
            let search_entry = search_entry.clone();

            move || {
                add_tab(
                    &notebook,
                    &search_entry,
                    Some("https://start.duckduckgo.com/"),
                );
            }
        });

        create_button_with_callback("New window", &popup_box, "F2", {
            move || {
                create_window("https://start.duckduckgo.com/");
            }
        });

        create_button_with_callback("Settings", &popup_box, "F12", || {
            show_settings_window();
        });

        popup.add(&popup_box);

        popup.popup();
        popup_box.show_all();
    });
}

pub fn menu_button_clicked(popup: &Popover, menu_button: &Button) {
    menu_button.connect_clicked({
        let popup: Popover = popup.clone();
        let menu_button = menu_button.clone();
        move |_| {
            popup.set_relative_to(Some(&menu_button));
            popup.popup();
        }
    });
}

pub fn adblock_toggle(
    adblock_toggle: &Switch,
    adblock_enabled: Rc<RefCell<bool>>,
    notebook: &Notebook,
) {
    adblock_toggle.connect_active_notify({
        let notebook = notebook.clone();
        move |_| match get_webview(&notebook) {
            Some(webview) => {
                toggle_adblock(adblock_enabled.clone(), &webview);
            }
            None => {
                info!("Current tab doesn't have a webview")
            }
        }
    });
}

fn create_button_with_callback<F>(label_text: &str, popup_box: &Box, hint: &str, callback: F)
where
    F: Fn() + 'static,
{
    let button = Button::new();

    let button_box = Box::new(gtk::Orientation::Horizontal, 5);

    let btn_label = Label::new(Some(label_text));
    let btn_hint = Label::new(Some(hint));

    button_box.pack_start(&btn_label, false, false, 5);
    button_box.pack_end(&btn_hint, false, false, 5);

    button.add(&button_box);

    button.set_width_request(300);
    btn_label.set_halign(gtk::Align::Start);
    btn_hint.set_halign(gtk::Align::End);

    apply_css_style(
        &[&button.upcast_ref(), &btn_hint.upcast_ref()],
        "
        button { background: transparent; border: none; box-shadow: none; }
        button:hover { background: #202020; }
        label { color: #5d5d5d; }
        ",
    );

    button.connect_clicked(move |_| {
        callback();
    });

    popup_box.pack_start(&button, false, false, 5);
}
