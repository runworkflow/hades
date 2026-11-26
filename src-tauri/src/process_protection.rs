pub fn start() {
    disable_core_dumps();
    #[cfg(not(debug_assertions))]
    deny_debugger_attach();
}

fn disable_core_dumps() {
    let no_dump = libc::rlimit {
        rlim_cur: 0,
        rlim_max: 0,
    };
    unsafe {
        libc::setrlimit(libc::RLIMIT_CORE, &no_dump);
    }
}

#[cfg(not(debug_assertions))]
fn deny_debugger_attach() {
    unsafe {
        libc::ptrace(libc::PT_DENY_ATTACH, 0, std::ptr::null_mut(), 0);
    }
}
