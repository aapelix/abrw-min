use dirs::config_dir;
use gtk::gio::ffi::{GAsyncResult, GCancellable};
use gtk::glib::ffi::g_bytes_new;
use gtk::glib::gobject_ffi::GObject;
use gtk::glib::{ffi::GError, translate::ToGlibPtr};
use gtk::prelude::{DialogExt, FileChooserExt, GtkWindowExt};
use reqwest::blocking::get;
use std::ffi::{c_void, CStr, CString};
use std::ptr::null;
use std::{
    fs::{create_dir_all, write, File},
    io::{BufReader, Read},
};
use webkit2gtk::{
    CookieManagerExt, Download, DownloadExt, SettingsExt, WebContext, WebContextExt, WebView,
    WebViewExt,
};
use webkit2gtk_sys::{
    webkit_settings_get_enable_javascript, webkit_user_content_filter_store_load,
    webkit_user_content_filter_store_load_finish, webkit_user_content_filter_store_new,
    webkit_user_content_filter_store_save, webkit_user_content_filter_store_save_finish,
    webkit_user_content_manager_add_filter, webkit_user_content_manager_remove_all_filters,
    WebKitUserContentFilterStore, WebKitUserContentManager,
};

#[derive(Clone)]
pub enum WebviewSetting {
    Javascript,
    WebGL,
    AutoMediaPlayback,
}

const DATA_URL: &'static str =
    "https://easylist-downloads.adblockplus.org/easylist_min_content_blocker.json";

use std::error::Error;

const BLOCK_LIST_IDENT: *const i8 = "blocklist\0".as_ptr() as *const i8;

use crate::settings::Settings;

static mut SHARED_CONTEXT: Option<WebContext> = None;

pub fn create_webview() -> WebView {
    let context: &mut WebContext = unsafe {
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

    context.set_favicon_database_directory(Some("favicons"));

    context.connect_download_started(move |_view, download| {
        let download: &Download = download;

        let dialog = gtk::FileChooserDialog::new(
            Some("Save File"),
            Some(&gtk::Window::new(gtk::WindowType::Toplevel)),
            gtk::FileChooserAction::Save,
        );

        dialog.set_current_name("thisisnotworkingyet");

        let response = dialog.run();

        if response == gtk::ResponseType::Accept {
            let filename = dialog.filename().unwrap();
            download.set_destination(&filename.to_str().unwrap());
        }

        // Destroy the dialog

        dialog.close();
    });

    let webview: WebView = WebView::with_context(context);

    add_filter(&webview);

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

unsafe extern "C" fn filter_save_callback(
    caller: *mut GObject,
    res: *mut GAsyncResult,
    con_man_ptr: *mut c_void,
) {
    let filter_store = caller as *mut WebKitUserContentFilterStore;
    let mut error = null::<GError>() as *mut GError;
    let filter = webkit_user_content_filter_store_save_finish(filter_store, res, &mut error);

    if error.is_null() {
        let con_man = con_man_ptr as *mut WebKitUserContentManager;
        webkit_user_content_manager_add_filter(con_man, filter);
    } else {
        // Tried and failed. Give up
        let real_err = *error;
        let error_msg = real_err.message;
        println!(
            "GError: {}",
            CStr::from_ptr(error_msg).to_str().unwrap_or("")
        );

        println!("Failed to save and load filter list :(\nNo adblock for you, sorry!");
    }
}

unsafe extern "C" fn filter_load_callback(
    caller: *mut GObject,
    res: *mut GAsyncResult,
    con_man_ptr: *mut c_void,
) {
    let filter_store = caller as *mut WebKitUserContentFilterStore;
    let mut error = null::<GError>() as *mut GError;
    let filter = webkit_user_content_filter_store_load_finish(filter_store, res, &mut error);

    if error.is_null() {
        println!("Successfully loaded cached filter store.");
        let con_man = con_man_ptr as *mut WebKitUserContentManager;
        webkit_user_content_manager_add_filter(con_man, filter);
    } else {
        // We haven't saved the filter list before, so let's do that
        let real_err = *error;
        let error_msg = real_err.message;
        println!(
            "GError Warning: {}",
            CStr::from_ptr(error_msg).to_str().unwrap_or("")
        );

        let fl_buff = get_filter_list();
        if fl_buff.is_err() {
            println!(
                "Failed to load filter list! Error: {}.\nIgnoring.",
                fl_buff.as_ref().err().unwrap().to_string()
            );
            return;
        }
        let fl_buff = fl_buff.unwrap();
        let fl_data = fl_buff.as_ptr();
        let fl_arr = g_bytes_new(fl_data as *const c_void, fl_buff.len());

        webkit_user_content_filter_store_save(
            filter_store,
            BLOCK_LIST_IDENT,
            fl_arr,
            null::<GCancellable>() as *mut _,
            Some(filter_save_callback),
            con_man_ptr,
        );
    }
}

pub fn add_filter(web_view: &WebView) {
    println!("Addign filter to content manager");

    let con_man = web_view.user_content_manager();
    let con_man_ptr: *mut WebKitUserContentManager = con_man.as_ref().to_glib_none().0;

    // let script_source = r#" document.querySelectorAll('div[id*="ad"], div[class*="ad"]').forEach(el => el.style.display = 'none'); "#;

    // let user_script = UserScript::new(
    //     script_source,                                    // JavaScript code
    //     webkit2gtk::UserContentInjectedFrames::AllFrames, // Inject into all frames
    //     webkit2gtk::UserScriptInjectionTime::End,         // Inject after the document is loaded
    //     &[],                                              // Allow list (none in this case)
    //     &[],                                              // Block list (none in this case)
    // );

    // UserContentManagerExt::add_script(&con_man.unwrap(), &user_script);

    let filter_path = CString::new("filters").unwrap();
    let filter_store = unsafe { webkit_user_content_filter_store_new(filter_path.as_ptr()) };

    unsafe {
        webkit_user_content_filter_store_load(
            filter_store,
            BLOCK_LIST_IDENT,
            null::<GCancellable>() as *mut _,
            Some(filter_load_callback),
            con_man_ptr as *mut _,
        );
    }
}

fn get_filter_list() -> Result<Vec<u8>, Box<dyn Error>> {
    let file_name = save_filter_list_to_file()?;

    let filter_list = File::open(file_name)?;
    let mut filter_list_reader = BufReader::new(filter_list);
    let mut filter_list_buff = Vec::new();
    filter_list_reader.read_to_end(&mut filter_list_buff)?;
    Ok(filter_list_buff)
}

fn save_filter_list_to_file() -> Result<String, Box<dyn Error>> {
    let resp = download_filter_list()?;

    let mut conf = config_dir().unwrap();
    conf.push("swb");
    conf.push("adblock");

    if !conf.clone().exists() {
        create_dir_all(conf.clone())?;
    }

    let file_name = String::from(conf.as_os_str().to_str().unwrap()) + "/easylist.json";
    write(file_name.clone(), resp)?;

    Ok(file_name)
}

fn download_filter_list() -> Result<String, Box<dyn Error>> {
    let response = get(DATA_URL)?;
    let text = response.text()?;
    Ok(text)
}

pub fn toggle_content_filter(webview: &WebView, enable_filter: bool) {
    let con_man = webview.user_content_manager();
    let con_man_ptr: *mut WebKitUserContentManager = con_man.as_ref().to_glib_none().0;

    if !enable_filter {
        add_filter(webview);
    } else {
        unsafe { webkit_user_content_manager_remove_all_filters(con_man_ptr as *mut _) }
    }
}

pub fn get_webview_setting(webview: &WebView, setting: WebviewSetting) -> Option<bool> {
    let settings = WebViewExt::settings(webview).unwrap();

    match setting {
        WebviewSetting::Javascript => {
            return Some(unsafe {
                webkit_settings_get_enable_javascript(settings.to_glib_none().0) != 0
            });
        }
        WebviewSetting::WebGL => {
            return Some(unsafe {
                webkit_settings_get_enable_javascript(settings.to_glib_none().0) != 0
            });
        }
        WebviewSetting::AutoMediaPlayback => {
            return Some(unsafe {
                webkit_settings_get_enable_javascript(settings.to_glib_none().0) != 0
            });
        }
    }
}

pub fn change_webview_setting(webview: &WebView, setting: WebviewSetting, value: bool) {
    let settings = WebViewExt::settings(webview).unwrap();

    match setting {
        WebviewSetting::Javascript => {
            settings.set_enable_javascript(value);
        }
        WebviewSetting::WebGL => {
            settings.set_enable_webgl(value);
        }
        WebviewSetting::AutoMediaPlayback => {
            settings.set_media_playback_requires_user_gesture(value);
        }
    }
}
