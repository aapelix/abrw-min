use crate::webview::create_webview;
use crate::{create_window, styles::apply_css_style};
use gtk::prelude::*;
use gtk::{gio::SimpleAction, Box, Button, Entry, Label, Notebook};
use webkit2gtk::{
    ContextMenu, ContextMenuAction, ContextMenuExt, ContextMenuItem, ContextMenuItemExt,
    HitTestResultExt, WebViewExt,
};

pub fn add_tab(notebook: &Notebook, search_entry: &Entry, uri: Option<&str>) {
    let tab_box = Box::new(gtk::Orientation::Horizontal, 5);
    let tab_label = Label::new(Some("New tab"));

    tab_box.set_size_request(-1, 15);

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
    tab_box.pack_end(&close_button, false, false, 0);

    let webview = create_webview();

    match uri {
        Some(uri) => {
            webview.load_uri(uri);
        }
        None => {
            webview.load_html(r##"
                <!DOCTYPE html>
                <html lang="en">
                <head>
                    <meta charset="UTF-8">
                    <meta name="viewport" content="width=device-width, initial-scale=1.0">
                    <title>Home Page</title>
                    <style>
                        body, html {
                            height: 100%;
                            margin: 0;
                            display: flex;
                            justify-content: center;
                            align-items: center;
                            background-color: #202020;
                        }
                        .container {
                            text-align: center;
                        }
                        svg {
                            width: 100px;
                            height: 100px;
                            margin-bottom: 20px;
                        }
                        .darkened {
                            color: #888;
                        }
                        .hidden {
                            display: none;
                        }
                        h1 {
                            color: #f1f1f1;
                        }
                    </style>
                </head>
                <body>
                    <div class="container">
                        <div id="svg1" class="hidden">
                            <svg xmlns="http://www.w3.org/2000/svg" width="28" height="28" viewBox="0 0 24 24" fill="none" stroke="#ffffff" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-radar"><path d="M19.07 4.93A10 10 0 0 0 6.99 3.34"/><path d="M4 6h.01"/><path d="M2.29 9.62A10 10 0 1 0 21.31 8.35"/><path d="M16.24 7.76A6 6 0 1 0 8.23 16.67"/><path d="M12 18h.01"/><path d="M17.99 11.66A6 6 0 0 1 15.77 16.67"/><circle cx="12" cy="12" r="2"/><path d="m13.41 10.59 5.66-5.66"/></svg>
                        </div>
                        <div id="svg2" class="hidden">
                            <svg xmlns="http://www.w3.org/2000/svg" width="28" height="28" viewBox="0 0 24 24" fill="none" stroke="#ffffff" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-coffee"><path d="M10 2v2"/><path d="M14 2v2"/><path d="M16 8a1 1 0 0 1 1 1v8a4 4 0 0 1-4 4H7a4 4 0 0 1-4-4V9a1 1 0 0 1 1-1h14a4 4 0 1 1 0 8h-1"/><path d="M6 2v2"/></svg>
                        </div>
                        <div id="svg3" class="hidden">
                            <svg xmlns="http://www.w3.org/2000/svg" width="28" height="28" viewBox="0 0 24 24" fill="none" stroke="#ffffff" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-cookie"><path d="M12 2a10 10 0 1 0 10 10 4 4 0 0 1-5-5 4 4 0 0 1-5-5"/><path d="M8.5 8.5v.01"/><path d="M16 15.5v.01"/><path d="M12 12v.01"/><path d="M11 17v.01"/><path d="M7 14v.01"/></svg>
                        </div>

                        <div id="svg4" class="hidden">
                            <svg xmlns="http://www.w3.org/2000/svg" width="28" height="28" viewBox="0 0 24 24" fill="none" stroke="#ffffff" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-bomb"><circle cx="11" cy="13" r="9"/><path d="M14.35 4.65 16.3 2.7a2.41 2.41 0 0 1 3.4 0l1.6 1.6a2.4 2.4 0 0 1 0 3.4l-1.95 1.95"/><path d="m22 2-1.5 1.5"/></svg>
                        </div>

                        <div id="svg5" class="hidden">
                            <svg xmlns="http://www.w3.org/2000/svg" width="28" height="28" viewBox="0 0 24 24" fill="none" stroke="#ffffff" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-bowl-chopsticks"><path d="m13 2-3 11"/><path d="m22 2-8 11"/><ellipse cx="12" cy="12" rx="10" ry="5"/><path d="M22 12a10 10 0 0 1-20 0"/></svg>
                        </div>

                        <div id="svg6" class="hidden">
                            <svg xmlns="http://www.w3.org/2000/svg" width="28" height="28" viewBox="0 0 24 24" fill="none" stroke="#ffffff" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-bot"><path d="M12 8V4H8"/><rect width="16" height="12" x="4" y="8" rx="2"/><path d="M2 14h2"/><path d="M20 14h2"/><path d="M15 13v2"/><path d="M9 13v2"/></svg>
                        </div>

                        <div id="svg7" class="hidden">
                            <svg xmlns="http://www.w3.org/2000/svg" width="28" height="28" viewBox="0 0 24 24" fill="none" stroke="#ffffff" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-square-terminal"><path d="m7 11 2-2-2-2"/><path d="M11 13h4"/><rect width="18" height="18" x="3" y="3" rx="2" ry="2"/></svg>
                        </div>

                        <div id="svg8" class="hidden">
                            <svg xmlns="http://www.w3.org/2000/svg" width="28" height="28" viewBox="0 0 24 24" fill="none" stroke="#ffffff" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-antenna"><path d="M2 12 7 2"/><path d="m7 12 5-10"/><path d="m12 12 5-10"/><path d="m17 12 5-10"/><path d="M4.5 7h15"/><path d="M12 16v6"/></svg>
                        </div>

                        <div id="random-svg-container"></div>

                        <h1>aapelix/abrw</h1>
                        <p class="darkened">F1 to open new tab</p>
                        <p class="darkened">F2 to open new window</p>
                    </div>

                    <script>
                        const svgIds = ['svg1', 'svg2', 'svg3', 'svg4', 'svg5', 'svg6', 'svg7', 'svg8'];

                        function getRandomSVG() {
                            const randomIndex = Math.floor(Math.random() * svgIds.length);
                            return svgIds[randomIndex];
                        }

                        function showRandomSVG() {
                            const randomSVGId = getRandomSVG();
                            const randomSVG = document.getElementById(randomSVGId);
                            const container = document.getElementById('random-svg-container');

                            const clonedSVG = randomSVG.cloneNode(true);
                            clonedSVG.classList.remove('hidden');

                            container.appendChild(clonedSVG);
                        }

                        showRandomSVG();
                    </script>
                </body>
                </html>

                "##, None);
        }
    }

    let search_entry_clone = search_entry.clone();
    webview.connect_notify_local(Some("uri"), move |webview, _| {
        if let Some(uri) = webview.uri() {
            search_entry_clone.set_text(&uri);
        }
    });

    let max_length = 15;

    let notebook_clone = notebook.clone();
    webview.connect_title_notify(move |webview| {
        let notebook = notebook_clone.clone();
        let webview = webview.clone();

        let title = webview
            .title()
            .map(|s| s.to_string())
            .unwrap_or_else(|| "Untitled".to_string());

        let truncated_title: String = title.chars().take(max_length).collect();

        let final_title = if title.chars().count() > max_length {
            format!("{}...", truncated_title)
        } else {
            truncated_title
        };

        let current_page = notebook.current_page();

        if let Some(page) = notebook.nth_page(current_page) {
            if let Some(tab) = notebook.tab_label(&page) {
                if let Some(tab_box) = tab.downcast_ref::<gtk::Container>() {
                    for child in tab_box.children() {
                        if let Some(label) = child.downcast_ref::<gtk::Label>() {
                            label.set_label(&final_title);
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

    let notebook_webview = notebook.clone();
    let search_entry_clone = search_entry.clone();

    webview.connect_load_failed(move |webview, _event, uri, error| {
        if error.message().contains("Name or service not known") {
            let custom_error_page = format!(
                r#"
                        <html>
                        <head>
                            <title>404 - {}</title>
                            <style>
                                body {{
                                    background-color: #202020;
                                    color: white;
                                    display: flex;
                                    justify-content: center;
                                    align-items: center;
                                    height: 100vh;
                                    margin: 0;
                                    font-family: Arial, sans-serif;
                                    text-align: center;
                                }}
                                h1 {{
                                    font-size: 48px;
                                }}
                                p {{
                                    font-size: 24px;
                                }}
                            </style>
                        </head>
                        <body>
                            <div>
                                <svg xmlns="http://www.w3.org/2000/svg" width="300" height="300" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-cake-slice"><circle cx="9" cy="7" r="2"/><path d="M7.2 7.9 3 11v9c0 .6.4 1 1 1h16c.6 0 1-.4 1-1v-9c0-2-3-6-7-8l-3.6 2.6"/><path d="M16 13H3"/><path d="M16 17H3"/></svg>
                                <h1>404 Not Found!</h1>
                                <p>We can't find <strong>{}</strong></p>
                                <p>but here is some cake to cheer you up</p>
                            </div>
                        </body>
                        </html>
                        "#,
                uri, uri
            );
            webview.load_html(&custom_error_page, Some(uri));
            return true;
        }
        false
    });

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
                    create_window(Some(&link_uri));
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
