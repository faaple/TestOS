//! Synchronization and interior mutability primitives
//! 
//! Re-export the struct `UPSafeCell`.

mod up;

pub use up::UPSafeCell;
