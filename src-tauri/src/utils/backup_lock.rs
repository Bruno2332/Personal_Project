use std::sync::atomic::{AtomicBool, Ordering};

static BUSY: AtomicBool = AtomicBool::new(false);

pub struct BackupGuard;

impl Drop for BackupGuard {
    fn drop(&mut self) {
        BUSY.store(false, Ordering::SeqCst);
    }
}

pub fn acquire() -> Result<BackupGuard, String> {
    BUSY.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
        .map_err(|_| "Já existe uma operação de backup em andamento.".to_string())?;

    Ok(BackupGuard)
}
