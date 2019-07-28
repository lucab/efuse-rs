//! Software fuses
//!
//! This library provides boolean-like types that behave like software
//! fuses: they can be "zapped" once, after which they remain in the
//! toggled state forever.
//! It supports fuses with custom initial boolean state, and atomic fuses.
//!
//! ## Example
//!
//! ```rust
//! let initial_state = true;
//! let mut fuse = efuse::Fuse::new(initial_state);
//! assert_eq!(fuse.as_bool(), true);
//!
//! fuse.zap();
//! assert_eq!(fuse.is_zapped(), true);
//! assert_eq!(fuse.as_bool(), false);
//!
//! fuse.zap();
//! assert_eq!(fuse.as_bool(), false);
//!
//! let already_zapped = fuse.zap_once();
//! assert!(already_zapped.is_err());
//! ```

#![deny(missing_debug_implementations)]
#![deny(missing_docs)]
#![allow(clippy::trivially_copy_pass_by_ref)]

use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::SeqCst;

/// Attempted to `zap_once` an already zapped fuse.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct AlreadyZappedError;

/// Software fuse, with custom initial state.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Fuse {
    initial_state: bool,
    zapped: bool,
}

impl Fuse {
    /// Return a new fuse with the given initial state.
    pub fn new(initial_state: bool) -> Self {
        Self {
            initial_state,
            zapped: false,
        }
    }

    /// Return the initial state of this fuse.
    pub fn initial_state(&self) -> bool {
        self.initial_state
    }

    /// Return current fuse value as a boolean.
    pub fn as_bool(&self) -> bool {
        self.initial_state ^ self.zapped
    }

    /// Zap this fuse (unconditionally), toggling its value permanently.
    ///
    /// It returns the new value of this fuse.
    pub fn zap(&mut self) -> bool {
        self.zapped |= true;
        self.initial_state ^ true
    }

    /// Zap this fuse (conditionally), toggling its value permanently.
    ///
    /// If the fuse was already previously zapped, it returns an
    /// [`AlreadyZappedError`](struct.AlreadyZappedError.html) error.
    /// Otherwise, it returns the new value of this fuse.
    pub fn zap_once(&mut self) -> Result<bool, AlreadyZappedError> {
        if self.zapped {
            return Err(AlreadyZappedError);
        }
        Ok(self.zap())
    }

    /// Whether this fuse has already been zapped.
    pub fn is_zapped(&self) -> bool {
        self.zapped
    }
}

impl From<bool> for Fuse {
    fn from(b: bool) -> Self {
        Self {
            initial_state: b,
            zapped: false,
        }
    }
}

impl Into<bool> for Fuse {
    fn into(self) -> bool {
        self.initial_state ^ self.zapped
    }
}

/// Atomic software fuse, with custom initial state.
#[derive(Debug, Default)]
pub struct AtomicFuse {
    initial_state: bool,
    zapped: AtomicBool,
}

impl AtomicFuse {
    /// Return a new fuse with the given initial state.
    pub fn new(initial_state: bool) -> Self {
        Self {
            initial_state,
            zapped: AtomicBool::new(false),
        }
    }

    /// Return the initial state of this fuse.
    pub fn initial_state(&self) -> bool {
        self.initial_state
    }

    /// Return current fuse value as a boolean.
    pub fn as_bool(&self) -> bool {
        self.initial_state ^ self.zapped.load(SeqCst)
    }

    /// Zap this fuse (unconditionally), toggling its value permanently.
    ///
    /// It returns the new value of this fuse.
    pub fn zap(&self) -> bool {
        self.zapped.fetch_or(true, SeqCst);
        self.initial_state ^ true
    }

    /// Zap this fuse (conditionally), toggling its value permanently.
    ///
    /// If the fuse was already previously zapped, it returns an
    /// [`AlreadyZappedError`](struct.AlreadyZappedError.html) error.
    /// Otherwise, it returns the new value of this fuse.
    pub fn zap_once(&self) -> Result<bool, AlreadyZappedError> {
        if self.zapped.compare_and_swap(false, true, SeqCst) {
            return Err(AlreadyZappedError);
        }
        Ok(self.initial_state ^ true)
    }

    /// Whether this fuse has already been zapped.
    pub fn is_zapped(&self) -> bool {
        self.zapped.load(SeqCst)
    }
}

impl From<bool> for AtomicFuse {
    fn from(b: bool) -> Self {
        Self {
            initial_state: b,
            zapped: AtomicBool::new(false),
        }
    }
}

impl Into<bool> for AtomicFuse {
    fn into(self) -> bool {
        self.initial_state ^ self.zapped.into_inner()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_defaults() {
        {
            let fuse = Fuse::default();
            assert_eq!(fuse.as_bool(), false);
        }

        {
            let afuse = AtomicFuse::default();
            assert_eq!(afuse.as_bool(), false);
        }
    }

    #[test]
    fn test_zaps() {
        for init in vec![false, true] {
            {
                let mut fuse = Fuse::new(init);
                assert_eq!(fuse.as_bool(), init);
                let new1 = fuse.zap_once().unwrap();
                assert_eq!(new1, !init);
                assert_eq!(fuse.as_bool(), !init);
                fuse.zap_once().unwrap_err();
                assert_eq!(fuse.as_bool(), !init);
                let new2 = fuse.zap();
                assert_eq!(fuse.as_bool(), !init);
                assert_eq!(new2, !init);
            }

            {
                let afuse = AtomicFuse::new(init);
                assert_eq!(afuse.as_bool(), init);
                let new1 = afuse.zap_once().unwrap();
                assert_eq!(new1, !init);
                assert_eq!(afuse.as_bool(), !init);
                afuse.zap_once().unwrap_err();
                assert_eq!(afuse.as_bool(), !init);
                let new2 = afuse.zap();
                assert_eq!(afuse.as_bool(), !init);
                assert_eq!(new2, !init);
            }
        }
    }
}
