use std::collections::HashSet;
use std::ffi::CStr;
use std::process::Command;
use std::ptr;

#[allow(bad_style)]
pub mod fswatch_sys {
    include!(concat!(env!("OUT_DIR"), "/fswatch_sys.rs"));
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
        if let Err(e) = Command::new("cargo-script").args(&["script", "--build-only", &file]).spawn() {
            eprintln!("Failed to use cargo-script to compile-cache {}: {:?}", file, e);
        }
    }
}

fn main() {
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
