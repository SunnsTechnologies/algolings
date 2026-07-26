//! Prevents a second `algolings watch` in the same repo from running
//! concurrently (a forgotten second terminal tab producing undefined
//! shared-state behavior — Architecture review finding, eng review).
//! Uses an OS-level advisory file lock (`std::fs::File::try_lock`), which
//! is released automatically when the guard is dropped or the process
//! exits, so there's no stale-lock cleanup to worry about.

use std::fs::{File, OpenOptions, TryLockError};
use std::path::Path;

#[derive(Debug)]
pub enum LockError {
    AlreadyRunning,
    Io(std::io::Error),
}

/// Holds the lock for as long as this guard is alive.
pub struct WatchLock {
    _file: File,
}

pub fn acquire_watch_lock(workspace_root: &Path) -> Result<WatchLock, LockError> {
    let lock_path = workspace_root.join(".algolings.lock");
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(false)
        .open(&lock_path)
        .map_err(LockError::Io)?;

    match file.try_lock() {
        Ok(()) => Ok(WatchLock { _file: file }),
        Err(TryLockError::WouldBlock) => Err(LockError::AlreadyRunning),
        Err(TryLockError::Error(e)) => Err(LockError::Io(e)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn acquires_the_lock_in_a_fresh_directory() {
        let dir = tempfile::tempdir().unwrap();
        let guard = acquire_watch_lock(dir.path());
        assert!(guard.is_ok());
    }

    #[test]
    fn a_second_acquire_while_the_first_guard_is_held_fails() {
        let dir = tempfile::tempdir().unwrap();
        let _first = acquire_watch_lock(dir.path()).unwrap();

        let second = acquire_watch_lock(dir.path());
        assert!(matches!(second, Err(LockError::AlreadyRunning)));
    }

    #[test]
    fn releasing_the_guard_allows_a_new_acquire() {
        let dir = tempfile::tempdir().unwrap();
        let first = acquire_watch_lock(dir.path()).unwrap();
        drop(first);

        let second = acquire_watch_lock(dir.path());
        assert!(second.is_ok());
    }
}
