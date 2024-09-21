use crate::ads;
use crate::styles::apply_css_style;
use crate::webview::create_webview;
use adblock::{lists::FilterSet, Engine};
use gtk::prelude::*;
use gtk::{Box, Button, Entry, Label, Notebook};
use log::info;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use webkit2gtk::WebViewExt;

pub fn add_tab(
    notebook: &Notebook,
    search_entry: &Entry,
    filter_set: &Arc<Mutex<FilterSet>>,
    adblock_enabled: Rc<RefCell<bool>>,
) {
    let tab_box = Box::new(gtk::Orientation::Horizontal, 5);
    let tab_label = Label::new(Some("New tab"));

    let close_button = Button::with_label("x");

    apply_css_style(
        &[
            &close_button.upcast_ref()
        ],
        "
        button { background: transparent; border: none; min-width: 10px; min-height: 10px; box-shadow: none; }
        button:hover { background: #1a1a1a; }
        ",
    );

    close_button.set_size_request(10, 10);

    tab_box.pack_start(&tab_label, false, false, 0);
    tab_box.pack_start(&close_button, false, false, 0);

    let webview = create_webview();
    webview.load_uri("https://start.duckduckgo.com/");

    let search_entry_clone = search_entry.clone();
    webview.connect_notify_local(Some("uri"), move |webview, _| {
        if let Some(uri) = webview.uri() {
            search_entry_clone.set_text(&uri);
        }
    });

    let notebook_clone = notebook.clone();
    webview.connect_title_notify(move |webview| {
        let notebook = notebook_clone.clone();
        let webview = webview.clone();

        let title = webview
            .title()
            .map(|s| s.to_string())
            .unwrap_or_else(|| "Untitled".to_string());

        info!("Title changed {}", title);

        let current_page = notebook.current_page();

        if let Some(page) = notebook.nth_page(current_page) {
            if let Some(tab) = notebook.tab_label(&page) {
                if let Some(tab_box) = tab.downcast_ref::<gtk::Container>() {
                    for child in tab_box.children() {
                        if let Some(label) = child.downcast_ref::<gtk::Label>() {
                            label.set_label(&title);
                        }
                    }
                }
            }
        }
    });

    let tab_index = notebook.append_page(&webview, Some(&tab_box));

    webview.show();
    tab_label.show();
    close_button.show();

    notebook.set_current_page(Some(tab_index));
    notebook.set_tab_reorderable(&webview, true);
    notebook.set_tab_detachable(&webview, true);

    let notebook = notebook.clone();
    close_button.connect_clicked(move |_| {
        notebook.remove_page(Some(tab_index));
    });

    let engine = Engine::from_filter_set(filter_set.lock().unwrap().clone(), true);

    webview.connect_resource_load_started(move |webview, resource, request| {
        let should_block = *adblock_enabled.borrow();
        if should_block {
            ads::on_resource_load_started(webview, resource, request, &engine);
        }
    });

    search_entry.set_is_focus(true);
}
