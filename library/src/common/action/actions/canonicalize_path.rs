build_action! { CanonicalizePath (_par_prog, _src_prog, hook, arg_position, args, _act_args, do_return, return_value) {
        // TODO: Handle NULL dlopen
        let library: &str = &hook.library;
        let library_basename: &str = library.rsplit('/').next().unwrap_or(library);
        let symbol: &str = &hook.symbol;
        let file_index = arg_position.expect("WhiteBeam: Lost track of environment") as usize;
        let file_argument: crate::common::db::ArgumentRow = args[file_index].clone();
        let file_value = file_argument.real as *const libc::c_char;
        let file_osstring = unsafe { crate::common::convert::c_char_to_osstring(file_value) };
        let new_file_value: std::ffi::OsString = match (library_basename, symbol) {
            ("libdl.so.2", "dlopen") |
            ("libdl.so.2", "dlmopen") => {
                let lib_path: std::ffi::OsString = platform::get_lib_path();
                match platform::search_path(&file_osstring, &lib_path) {
                    Some(abspath) => abspath.as_os_str().to_owned(),
                    None => {
                        do_return = true;
                        return_value = 0;
                        return (hook, args, do_return, return_value);
                    }
                }
            },
            _ => {
                let env_path: std::ffi::OsString = platform::get_env_path();
                match platform::search_path(&file_osstring, &env_path) {
                    Some(abspath) => abspath.as_os_str().to_owned(),
                    None => {
                        unsafe { libc::exit(127) };
                    }
                }
            }
        };
        let new_file_value_cstring: Box<std::ffi::CString> = Box::new(crate::common::convert::osstr_to_cstring(&new_file_value).expect("WhiteBeam: Unexpected null reference"));
        args[file_index].datatype = String::from("String");
        args[file_index].real = Box::leak(new_file_value_cstring).as_ptr() as usize;
}}
