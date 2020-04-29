use futures::{future, Future};
use futures_util::future::TryFutureExt;
use std::collections::HashSet;
use std::ffi::CStr;
use std::process::Command;
use std::{thread, ptr};
use tokio::runtime::Runtime;
use warp::Filter;

#[allow(bad_style)]
pub mod fswatch_sys {
    include!(concat!(env!("OUT_DIR"), "/fswatch_sys.rs"));
}

fn cache_script(path: &str) {
    if let Err(e) = Command::new("cargo-script").args(&["script", "--build-only", &path]).spawn() {
        eprintln!("Failed to use cargo-script to compile-cache {}: {:?}", path, e);
    }
}

extern "C" fn process_script_changes(events_ptr: *const fswatch_sys::fsw_cevent, events_len: u32, _data: *mut std::ffi::c_void) {
    use fswatch_sys::{fsw_get_event_flag_name, fsw_event_flag_Created, fsw_event_flag_Updated};
    let events = unsafe { std::slice::from_raw_parts(events_ptr, events_len as usize) };
    let mut to_cache = HashSet::new();
    for event in events {
        let name = unsafe { CStr::from_ptr(event.path) }.to_string_lossy();
        let flags = unsafe { std::slice::from_raw_parts(event.flags, event.flags_num as usize) };
        let mut flags_pretty = String::new();
        for (i, flag) in flags.iter().enumerate() {
            flags_pretty += &unsafe { CStr::from_ptr(fsw_get_event_flag_name(*flag)) }.to_string_lossy();
            if i != flags.len()-1 {
                flags_pretty += ", ";
            }
        }
        println!("event, file {:?}, flags [{}]", name, flags_pretty);
        if name.ends_with(".rs") && (flags.contains(&fsw_event_flag_Created) || flags.contains(&fsw_event_flag_Updated)) {
            to_cache.insert(name.to_owned());
        }
    }
    println!("to_cache: {:?}", to_cache);
    for file in to_cache {
        cache_script(&file);
    }
}

fn setup_fswatch() {
    if unsafe { fswatch_sys::fsw_init_library() } != fswatch_sys::FSW_OK as i32 {
        eprintln!("fsw_init_library failed");
        return;
    }

    unsafe {
        use fswatch_sys::*;
        let handle = fsw_init_session(fsw_monitor_type_system_default_monitor_type);
        fsw_add_path(handle, b"cgiscripts\0" as *const u8 as *const i8);
        fsw_set_callback(handle, Some(process_script_changes), ptr::null_mut());
        fsw_start_monitor(handle);
    }
}

async fn handle_request(name: String) -> Result<String, Box<dyn std::error::Error>> {
    println!("received request for {:?}", name);
    if name.ends_with(".rs") {
        let output = tokio::task::spawn_blocking(move || {
            // TODO: prevent directory traversal
            Command::new("cargo-script").args(&["script", &format!("cgiscripts/{}", name)]).output()
        }).await??;
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err("404")?
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // TODO: populate cache on already-existing scripts

    thread::spawn(setup_fswatch);

    let wildcard = warp::path::param::<String>().and(warp::path::end()).and_then(|name| handle_request(name).map_err(|_| warp::reject::not_found()));
    let index = warp::path::end().and_then(|| handle_request("index.rs".into()).map_err(|_| warp::reject::not_found()));

    let router = wildcard.or(index);

    let mut runtime = Runtime::new()?;
    let ip = ([127, 0, 0, 1], 3000);
    runtime.block_on(warp::serve(router).run(ip));
    Ok(())
}
