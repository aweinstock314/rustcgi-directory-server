#[allow(bad_style)]
pub mod fswatch_sys {
    include!(concat!(env!("OUT_DIR"), "/fswatch_sys.rs"));
}

fn main() {
    if unsafe { fswatch_sys::fsw_init_library() } != fswatch_sys::FSW_OK as i32 {
        eprintln!("fsw_init_library failed");
        return;
    }
}
