use std::sync::{Arc, Mutex, PoisonError};
use std::thread;
use std::time::Duration;

use arboard::{Clipboard, SetExtApple};
use zeroize::Zeroizing;

struct PendingClear {
    token: u64,
    value: Zeroizing<String>,
}

#[derive(Default)]
struct ClipboardState {
    last_token: u64,
    pending: Option<PendingClear>,
}

#[derive(Clone, Default)]
pub struct SecretClipboard {
    state: Arc<Mutex<ClipboardState>>,
}

impl SecretClipboard {
    pub fn copy(
        &self,
        value: Zeroizing<String>,
        clear_after: Duration,
    ) -> Result<(), arboard::Error> {
        Clipboard::new()?
            .set()
            .exclude_from_history()
            .text(value.as_str())?;

        let token = {
            let mut state = self.state.lock().unwrap_or_else(PoisonError::into_inner);
            state.last_token += 1;
            state.pending = Some(PendingClear {
                token: state.last_token,
                value,
            });
            state.last_token
        };

        let state = Arc::clone(&self.state);
        thread::spawn(move || {
            thread::sleep(clear_after);
            clear_pending(&state, Some(token));
        });
        Ok(())
    }

    pub fn clear_now(&self) {
        clear_pending(&self.state, None);
    }
}

fn clear_pending(state: &Mutex<ClipboardState>, only_token: Option<u64>) {
    let mut state = state.lock().unwrap_or_else(PoisonError::into_inner);
    let Some(pending) = state
        .pending
        .take_if(|pending| only_token.is_none_or(|token| pending.token == token))
    else {
        return;
    };

    let Ok(mut clipboard) = Clipboard::new() else {
        return;
    };
    let is_unchanged = clipboard
        .get_text()
        .map(Zeroizing::new)
        .is_ok_and(|current| current.as_str() == pending.value.as_str());
    if is_unchanged {
        let _ = clipboard.clear();
    }
}
