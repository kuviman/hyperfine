mod internal;

pub mod wallclocktimer;

use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(windows)] {
        mod windows_timer;
        pub use self::windows_timer::get_cpu_timer;
    } else {
        mod unix_timer;
        pub use self::unix_timer::get_cpu_timer;
    }
}
use std::process::Child;

pub type Scalar = f64;

/// Type alias for unit of time
pub type Second = Scalar;

/// Defines start functionality of a timer.
pub trait TimerStart {
    fn start() -> Self;
    fn start_for_process(process: &Child) -> Self;
}

/// Defines stop functionality of a timer.
pub trait TimerStop {
    type Result;
    fn stop(&self) -> Self::Result;
}
