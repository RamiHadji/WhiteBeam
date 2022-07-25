// TODO: Tests to ensure environment is not corrupted

whitebeam_test!("linux", execution_00_execve_simple {
    let pid = unsafe { libc::fork() };
    if pid == 0 {
        unsafe { libc::execve("/usr/bin/touch\0".as_ptr() as *const libc::c_char,
                              ["/usr/bin/touch\0".as_ptr() as *const libc::c_char, "/tmp/execve_test\0".as_ptr() as *const libc::c_char, std::ptr::null()].as_ptr(),
                              std::ptr::null()); }
    } else {
        let mut status = 0;
        unsafe { libc::waitpid(pid, &mut status, 0); }
        assert!(status == 0);
        let test_path = std::path::Path::new("/tmp/execve_test");
        assert!(test_path.exists());
        std::fs::remove_file(test_path).expect(&format!("WhiteBeam: Failed to remove {:?}", test_path));
    }
});

whitebeam_test!("linux", execution_01_execve_library_loaded {
    let pid = unsafe { libc::fork() };
    if pid == 0 {
        unsafe { libc::execve("/usr/bin/grep\0".as_ptr() as *const libc::c_char,
                              ["/usr/bin/grep\0".as_ptr() as *const libc::c_char, "-q\0".as_ptr() as *const libc::c_char, "libwhitebeam.so\0".as_ptr() as *const libc::c_char, "/proc/self/maps\0".as_ptr() as *const libc::c_char, std::ptr::null()].as_ptr(),
                              std::ptr::null()); }
    } else {
        let mut status = 0;
        unsafe { libc::waitpid(pid, &mut status, 0); }
        assert!(status == 0);
    }
});

whitebeam_test!("linux", execution_02_execl_simple {
    let pid = unsafe { libc::fork() };
    if pid == 0 {
        unsafe { libc::execl("/usr/bin/touch\0".as_ptr() as *const libc::c_char,
                             "/usr/bin/touch\0".as_ptr() as *const libc::c_char, "/tmp/execl_test\0".as_ptr() as *const libc::c_char, std::ptr::null() as *const libc::c_char); }
    } else {
        let mut status = 0;
        unsafe { libc::waitpid(pid, &mut status, 0); }
        assert!(status == 0);
        let test_path = std::path::Path::new("/tmp/execl_test");
        assert!(test_path.exists());
        std::fs::remove_file(test_path).expect(&format!("WhiteBeam: Failed to remove {:?}", test_path));
    }
});

whitebeam_test!("linux", execution_03_execle_simple {
    let pid = unsafe { libc::fork() };
    if pid == 0 {
        unsafe { libc::execle("/usr/bin/touch\0".as_ptr() as *const libc::c_char,
                              "/usr/bin/touch\0".as_ptr() as *const libc::c_char, "/tmp/execle_test\0".as_ptr() as *const libc::c_char, std::ptr::null() as *const libc::c_char,
                              std::ptr::null() as *const libc::c_char); }
    } else {
        let mut status = 0;
        unsafe { libc::waitpid(pid, &mut status, 0); }
        assert!(status == 0);
        let test_path = std::path::Path::new("/tmp/execle_test");
        assert!(test_path.exists());
        std::fs::remove_file(test_path).expect(&format!("WhiteBeam: Failed to remove {:?}", test_path));
    }
});

whitebeam_test!("linux", execution_04_execlp_simple {
    let pid = unsafe { libc::fork() };
    if pid == 0 {
        unsafe { libc::execlp("touch\0".as_ptr() as *const libc::c_char,
                              "touch\0".as_ptr() as *const libc::c_char, "/tmp/execlp_test\0".as_ptr() as *const libc::c_char, std::ptr::null() as *const libc::c_char); }
    } else {
        let mut status = 0;
        unsafe { libc::waitpid(pid, &mut status, 0); }
        assert!(status == 0);
        let test_path = std::path::Path::new("/tmp/execlp_test");
        assert!(test_path.exists());
        std::fs::remove_file(test_path).expect(&format!("WhiteBeam: Failed to remove {:?}", test_path));
    }
});

whitebeam_test!("linux", execution_05_execv_simple {
    let pid = unsafe { libc::fork() };
    if pid == 0 {
        unsafe { libc::execv("/usr/bin/touch\0".as_ptr() as *const libc::c_char,
                             ["/usr/bin/touch\0".as_ptr() as *const libc::c_char, "/tmp/execv_test\0".as_ptr() as *const libc::c_char, std::ptr::null()].as_ptr()); }
    } else {
        let mut status = 0;
        unsafe { libc::waitpid(pid, &mut status, 0); }
        assert!(status == 0);
        let test_path = std::path::Path::new("/tmp/execv_test");
        assert!(test_path.exists());
        std::fs::remove_file(test_path).expect(&format!("WhiteBeam: Failed to remove {:?}", test_path));
    }
});

whitebeam_test!("linux", execution_06_execvp_simple {
    let pid = unsafe { libc::fork() };
    if pid == 0 {
        unsafe { libc::execvp("touch\0".as_ptr() as *const libc::c_char,
                              ["touch\0".as_ptr() as *const libc::c_char, "/tmp/execvp_test\0".as_ptr() as *const libc::c_char, std::ptr::null()].as_ptr()); }
    } else {
        let mut status = 0;
        unsafe { libc::waitpid(pid, &mut status, 0); }
        assert!(status == 0);
        let test_path = std::path::Path::new("/tmp/execvp_test");
        assert!(test_path.exists());
        std::fs::remove_file(test_path).expect(&format!("WhiteBeam: Failed to remove {:?}", test_path));
    }
});

whitebeam_test!("linux", execution_07_execvpe_simple {
    let pid = unsafe { libc::fork() };
    if pid == 0 {
        unsafe { libc::execvpe("touch\0".as_ptr() as *const libc::c_char,
                               ["touch\0".as_ptr() as *const libc::c_char, "/tmp/execvpe_test\0".as_ptr() as *const libc::c_char, std::ptr::null()].as_ptr(),
                               std::ptr::null()); }
    } else {
        let mut status = 0;
        unsafe { libc::waitpid(pid, &mut status, 0); }
        assert!(status == 0);
        let test_path = std::path::Path::new("/tmp/execvpe_test");
        assert!(test_path.exists());
        std::fs::remove_file(test_path).expect(&format!("WhiteBeam: Failed to remove {:?}", test_path));
    }
});

whitebeam_test!("linux", execution_08_fexecve_simple {
    let pid = unsafe { libc::fork() };
    if pid == 0 {
        let fd: libc::c_int = unsafe { libc::open("/usr/bin/touch\0".as_ptr() as *const libc::c_char, libc::O_RDONLY, 0) };
        unsafe { libc::fexecve(fd, ["/usr/bin/touch\0".as_ptr() as *const libc::c_char, "/tmp/fexecve_test\0".as_ptr() as *const libc::c_char, std::ptr::null()].as_ptr(), std::ptr::null()); }
    } else {
        let mut status = 0;
        unsafe { libc::waitpid(pid, &mut status, 0); }
        assert!(status == 0);
        let test_path = std::path::Path::new("/tmp/fexecve_test");
        assert!(test_path.exists());
        std::fs::remove_file(test_path).expect(&format!("WhiteBeam: Failed to remove {:?}", test_path));
    }
});

whitebeam_test!("linux", execution_09_posix_spawn_simple {
    let mut pid: libc::pid_t = 0;
    unsafe { libc::posix_spawn(&mut pid as *mut libc::pid_t,
                               "/usr/bin/touch\0".as_ptr() as *const libc::c_char,
                               std::ptr::null(),
                               std::ptr::null(),
                               ["/usr/bin/touch\0".as_ptr() as *const libc::c_char, "/tmp/posix_spawn_test\0".as_ptr() as *const libc::c_char, std::ptr::null()].as_ptr() as *const *mut libc::c_char,
                               std::ptr::null()); }
    let mut status = 0;
    unsafe { libc::waitpid(pid, &mut status, 0); }
    assert!(status == 0);
    let test_path = std::path::Path::new("/tmp/posix_spawn_test");
    assert!(test_path.exists());
    std::fs::remove_file(test_path).expect(&format!("WhiteBeam: Failed to remove {:?}", test_path));
});

whitebeam_test!("linux", execution_10_posix_spawnp_simple {
    let mut pid: libc::pid_t = 0;
    unsafe { libc::posix_spawnp(&mut pid as *mut libc::pid_t,
                                "touch\0".as_ptr() as *const libc::c_char,
                                std::ptr::null(),
                                std::ptr::null(),
                                ["touch\0".as_ptr() as *const libc::c_char, "/tmp/posix_spawnp_test\0".as_ptr() as *const libc::c_char, std::ptr::null()].as_ptr() as *const *mut libc::c_char,
                                std::ptr::null()); }
    let mut status = 0;
    unsafe { libc::waitpid(pid, &mut status, 0); }
    assert!(status == 0);
    let test_path = std::path::Path::new("/tmp/posix_spawnp_test");
    assert!(test_path.exists());
    std::fs::remove_file(test_path).expect(&format!("WhiteBeam: Failed to remove {:?}", test_path));
});

// Tests for:
// dlopen
// dlmopen
// kill