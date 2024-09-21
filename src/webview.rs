use webkit2gtk::{CookieManagerExt, SettingsExt, WebContext, WebContextExt, WebView, WebViewExt};

use crate::settings::Settings;

static mut SHARED_CONTEXT: Option<WebContext> = None;

pub fn create_webview() -> WebView {
    let context = unsafe {
        SHARED_CONTEXT.get_or_insert_with(|| {
            let context = WebContext::default().unwrap();
            let cookie_manager =
                WebContextExt::cookie_manager(&context).expect("Failed to init cookie manager");

            let storage_file_path = "cookies.sqlite";
            CookieManagerExt::set_persistent_storage(
                &cookie_manager,
                storage_file_path,
                webkit2gtk::CookiePersistentStorage::Sqlite,
            );

            context
        })
    };

    let webview = WebView::with_context(context);

    let settings = WebViewExt::settings(&webview).unwrap();
    let settings_json = Settings::load();

    settings.set_enable_developer_extras(true);
    settings.set_enable_smooth_scrolling(true);

    settings.set_enable_javascript(settings_json.enable_javascript);
    settings.set_enable_webgl(settings_json.enable_webgl);
    settings.set_enable_page_cache(settings_json.page_cache);
    settings.set_media_playback_requires_user_gesture(
        settings_json.media_playback_requires_user_gesture,
    );
    settings.set_user_agent(Some("aapelix/abrw"));

    webview.set_settings(&settings);

    return webview;
}
