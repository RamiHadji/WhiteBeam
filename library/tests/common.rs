pub fn load_whitebeam(test: &str) -> bool {
    // TODO: Cross platform
    let lib_path: std::path::PathBuf = std::path::PathBuf::from(format!("{}/target/release/libwhitebeam.so", env!("PWD")));
    assert!(lib_path.exists(), "WhiteBeam: libwhitebeam.so could not be found");
    let ld_audit: Option<std::ffi::OsString> = std::env::var_os("LD_AUDIT");
    // TODO: Check zeroth index of colon separated variable instead of checking if LD_AUDIT equals the library path
    if (ld_audit.is_none()) ||
       (ld_audit != Some(lib_path.as_os_str().to_os_string())) {
        // LD_AUDIT undefined. Restart program with LD_PRELOAD set to libwhitebeam.so
        let test_path = unsafe { std::ffi::CStr::from_ptr(libc::getauxval(libc::AT_EXECFN) as *const libc::c_char)};
        let test_path_str = test_path.to_str().expect("Failed to convert test path to &str");
        let exit_status_test = std::process::Command::new(test_path_str)
            .args(&["--test", test])
            // TODO: Gate behind verbose flag
            .stdout(std::process::Stdio::null())
            // Set LD_PRELOAD to test initialization of LD_AUDIT (/etc/ld.so.preload behavior)
            .env("LD_PRELOAD", lib_path.as_os_str())
            .status().expect("Failed to execute process");
        assert!(exit_status_test.success());
        return true;
    }
    return false;
}

#[macro_export]
macro_rules! whitebeam_test {
    ($os:expr, $func:ident $body:block) => {
        #[test]
        #[cfg(target_os = $os)]
        fn $func() {
            let in_parent_process: bool = crate::common::load_whitebeam(stringify!($func));
            if in_parent_process {
                return;
            }
            $body
        }
    };
}

pub fn load_sql(sql: &str) {
    use std::io::Write;
    let bin_path: std::path::PathBuf = std::path::PathBuf::from(format!("{}/target/release/whitebeam", env!("PWD")));
    assert!(bin_path.exists(), "WhiteBeam: whitebeam could not be found");
    let mut load_command = std::process::Command::new(bin_path)
            .args(&["--load", "-"])
            .env("WB_AUTH", "test")
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::null())
            .spawn().expect("Failed to execute process");
    let mut stdin = load_command.stdin.take().expect("Failed to capture stdin");
    write!(stdin, "{}", sql).expect("Failed to write to stdin");
    drop(stdin);
    match load_command.try_wait() {
        Ok(Some(_status)) => {},
        Ok(None) => {
            let _res = load_command.wait();
        },
        Err(_e) => {}
    }
}

pub fn toggle_hook(symbol: &str, enabled: bool) {
    assert!(symbol.chars().all(|c| c.is_ascii_alphanumeric() || c == '_'));
    // TODO: Cross platform
    let sql = String::from(format!("UPDATE Hook SET enabled = {} WHERE symbol = '{}';", enabled, symbol));
    load_sql(&sql);
}

pub fn is_hooked(library: &str, symbol: &str) -> bool {
    let is_hooked_addr: usize = unsafe { libc::dlsym(libc::RTLD_DEFAULT, "is_hooked\0".as_ptr() as *const libc::c_char) } as usize;
    assert!(is_hooked_addr != 0, "WhiteBeam: is_hooked not found in libwhitebeam.so, consider running: cargo run build library-test");
    let is_hooked_fn: unsafe extern "C" fn(library: *const libc::c_char, symbol: *const libc::c_char) -> libc::c_int = unsafe { std::mem::transmute(is_hooked_addr) };
    let mut library_string = String::from(library);
    library_string.push('\0');
    let mut symbol_string = String::from(symbol);
    symbol_string.push('\0');
    let is_hooked_result: libc::c_int = unsafe { is_hooked_fn(library_string.as_ptr() as *const libc::c_char, symbol_string.as_ptr() as *const libc::c_char) };
    return is_hooked_result == 1;
}