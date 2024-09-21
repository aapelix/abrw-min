extern crate gtk;
extern crate webkit2gtk;

use adblock::lists::FilterSet;
use gtk::{prelude::*, Button, Entry, Notebook, Popover, Switch};
use log::info;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use url::Url;
use webkit2gtk::{WebView, WebViewExt};

use crate::settings::toggle_adblock;
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

pub fn new_tab_button_clicked(
    notebook: &Notebook,
    new_tab_button: &Button,
    search_entry: &Entry,
    filter_set: &Arc<Mutex<FilterSet>>,
    adblock_enabled: Rc<RefCell<bool>>,
) {
    new_tab_button.connect_clicked({
        let notebook = notebook.clone();
        let search_entry = search_entry.clone();
        let filter_set = filter_set.clone();

        move |_| {
            add_tab(
                &notebook,
                &search_entry,
                &filter_set,
                adblock_enabled.clone(),
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

pub fn notebook_switch_page(notebook: &Notebook, search_entry: &Entry) {
    notebook.connect_switch_page({
        let search_entry = search_entry.clone();

        move |notebook, _, page_num| {
            if let Some(widget) = notebook.nth_page(Some(page_num)) {
                if let Some(webview) = widget.downcast_ref::<webkit2gtk::WebView>() {
                    if let Some(uri) = webview.uri() {
                        search_entry.set_text(&uri);
                    }
                }
            }
        }
    });
}

pub fn settings_button_clicked(settings_button: &Button) {
    settings_button.connect_clicked(move |_| show_settings_window());
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

pub fn adblock_toggle(adblock_toggle: &Switch, adblock_enabled: Rc<RefCell<bool>>) {
    adblock_toggle.connect_active_notify({
        move |_| {
            toggle_adblock(adblock_enabled.clone());
        }
    });
}
