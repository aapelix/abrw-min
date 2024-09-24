use crate::webview::create_webview;
use crate::{create_window, styles::apply_css_style};
use gtk::{
    gdk_pixbuf::{InterpType, Pixbuf},
    gio::SimpleAction,
    Box, Button, Entry, Label, Notebook,
};
use gtk::{prelude::*, Image};
use log::info;
use std::path::PathBuf;
use webkit2gtk::{
    ContextMenu, ContextMenuAction, ContextMenuExt, ContextMenuItem, ContextMenuItemExt,
    HitTestResultExt, WebViewExt,
};

pub fn add_tab(notebook: &Notebook, search_entry: &Entry, uri: Option<&str>) {
    let tab_box = Box::new(gtk::Orientation::Horizontal, 5);
    let tab_label = Label::new(Some("New tab"));
    let path = PathBuf::from("/usr/share/pixmaps/myicon.png");
    let pixbuf_icon = Pixbuf::from_file(path).expect("Failed to create pixbuf");
    let scaled_pixbuf = &pixbuf_icon.scale_simple(25, 25, InterpType::Bilinear);

    tab_box.set_size_request(100, -1);

    let tab_favicon = Image::from_pixbuf(scaled_pixbuf.as_ref());

    tab_favicon.set_pixel_size(2000);

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

    tab_box.pack_start(&tab_favicon, false, false, 0);
    tab_box.pack_start(&tab_label, false, false, 0);
    tab_box.pack_start(&close_button, false, false, 0);

    let webview = create_webview();

    match uri {
        Some(uri) => {
            webview.load_uri(uri);
        }
        None => {
            webview.load_uri("https://start.duckduckgo.com/");
        }
    }

    let search_entry_clone = search_entry.clone();
    webview.connect_notify_local(Some("uri"), move |webview, _| {
        if let Some(uri) = webview.uri() {
            search_entry_clone.set_text(&uri);
        }
    });

    //
    // This probably need some work down here
    //

    let notebook_clone = notebook.clone();
    webview.connect_favicon_notify(move |webview| {
        println!("favicon changed");

        let notebook = notebook_clone.clone();
        let webview = webview.clone();

        let title = webview.favicon();

        let current_page = notebook.current_page();

        if let Some(page) = notebook.nth_page(current_page) {
            if let Some(tab) = notebook.tab_label(&page) {
                if let Some(tab_box) = tab.downcast_ref::<gtk::Container>() {
                    for child in tab_box.children() {
                        if let Some(image) = child.downcast_ref::<gtk::Image>() {
                            if let Some(favicon) = title.clone() {
                                let pixbuf_icon =
                                    gtk::gdk::pixbuf_get_from_surface(&favicon, 2, 2, 25, 25)
                                        .expect("Failed to create image from favicon");

                                let scaled_pixbuf =
                                    &pixbuf_icon.scale_simple(25, 25, InterpType::Bilinear);

                                image.set_from_pixbuf(scaled_pixbuf.as_ref());

                                image.set_pixel_size(2000);
                                image.show();
                            }
                        }
                    }
                }
            }
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
    tab_favicon.show();

    notebook.set_current_page(Some(tab_index));
    notebook.set_tab_reorderable(&webview, true);
    notebook.set_tab_detachable(&webview, true);

    let notebook_webview = notebook.clone();
    let search_entry_clone = search_entry.clone();

    webview.connect_context_menu(move |_webview, context_menu, _event, hit_test_result| {
        let menu: ContextMenu = context_menu.clone();

        for menu_item in menu.items() {
            let action = menu_item.stock_action();

            if action == ContextMenuAction::OpenLinkInNewWindow
                || action == ContextMenuAction::OpenLink
            {
                menu.remove(&menu_item);
            }
        }

        let open_link_in_new_tab_act = create_action_with_callback("open-link-in-new-tab", {
            let hit_test_result = hit_test_result.clone();

            let notebook_webview = notebook_webview.clone();
            let search_entry_clone = search_entry_clone.clone();

            move |_, _| {
                if let Some(link_uri) = hit_test_result.link_uri() {
                    add_tab(&notebook_webview, &search_entry_clone, Some(&link_uri));
                }
            }
        });

        let open_link_in_new_window_act = create_action_with_callback("open-link-in-new-window", {
            let hit_test_result = hit_test_result.clone();

            move |_, _| {
                if let Some(link_uri) = hit_test_result.link_uri() {
                    create_window(&link_uri);
                }
            }
        });

        let open_link_in_new_tab =
            ContextMenuItem::from_gaction(&open_link_in_new_tab_act, "Open Link in New Tab", None);

        let open_link_in_new_window = ContextMenuItem::from_gaction(
            &open_link_in_new_window_act,
            "Open Link in Window",
            None,
        );

        let separator = ContextMenuItem::new_separator();

        menu.prepend(&separator);
        menu.prepend(&open_link_in_new_window);
        menu.prepend(&open_link_in_new_tab);

        false
    });

    let notebook = notebook.clone();
    close_button.connect_clicked(move |_| {
        notebook.remove_page(Some(tab_index));
    });

    search_entry.set_is_focus(true);
}

fn create_action_with_callback<F>(name: &str, callback: F) -> SimpleAction
where
    F: Fn(&SimpleAction, Option<&gtk::glib::Variant>) + 'static, // Ensure the closure is `'static` for use in the signal
{
    let action = SimpleAction::new(name, None);

    action.connect_activate(callback);

    action
}
