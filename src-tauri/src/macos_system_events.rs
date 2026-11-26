use std::sync::OnceLock;
use std::thread;

use hades_core::LockReason;
use tauri::{AppHandle, Manager};

use crate::app_vault::AppVault;
use crate::auto_lock_runner::lock_now;

const EVENT_SLEEP: i32 = 1;
const EVENT_SCREEN_LOCK: i32 = 2;
const EVENT_USER_SWITCH: i32 = 3;

static APP_HANDLE: OnceLock<AppHandle> = OnceLock::new();

unsafe extern "C" {
    fn hades_system_events_start(callback: extern "C" fn(i32));
}

extern "C" fn handle_system_event(event: i32) {
    let reason = match event {
        EVENT_SLEEP => LockReason::Sleep,
        EVENT_SCREEN_LOCK => LockReason::ScreenLock,
        EVENT_USER_SWITCH => LockReason::UserSwitch,
        _ => return,
    };
    let Some(app) = APP_HANDLE.get().cloned() else {
        return;
    };
    thread::spawn(move || {
        let vault = app.state::<AppVault>();
        lock_now(&app, &vault, reason);
    });
}

pub fn start(app: AppHandle) {
    if APP_HANDLE.set(app).is_ok() {
        unsafe { hades_system_events_start(handle_system_event) };
    }
}
