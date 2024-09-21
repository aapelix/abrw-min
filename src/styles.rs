extern crate gtk;

use gtk::prelude::*;
use gtk::CssProvider;

pub fn apply_css_style(widgets: &[&gtk::Widget], css: &str) {
    let provider = CssProvider::new();
    provider.load_from_data(css.as_bytes()).unwrap();

    for widget in widgets {
        let context = widget.style_context();
        context.add_provider(&provider, gtk::STYLE_PROVIDER_PRIORITY_USER);
    }
}
