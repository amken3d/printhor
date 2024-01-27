use bitflags::bitflags;
mod plan;
pub use plan::*;

#[allow(unused)]
mod interpolators;

bitflags! {
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    pub struct StepperChannel: u8 {
        const X    = 0b00000001;
        const Y    = 0b00000010;
        const Z    = 0b00000100;
        #[cfg(feature="has-extruder")]
        const E    = 0b00001000;
    }
}

use printhor_hwa_common::EventStatus;
#[cfg_attr(all(feature = "defmt", feature = "native"), derive(defmt::Format))]
#[cfg_attr(feature = "native", derive(Debug))]
#[cfg_attr(feature = "native", derive(strum::Display))]
#[allow(unused)]
pub enum CodeExecutionSuccess {
    /// Immediately executed
    OK,
    /// Immediately executed and reported
    CONSUMED,
    /// Queued but assumed it will be executed not too long, so practically same as OK
    QUEUED,
    /// Executed but it will take time to get a final response. EventStatus contains the needed flags to wait for
    DEFERRED(EventStatus),
}

#[cfg_attr(all(feature = "defmt", feature = "native"), derive(defmt::Format))]
#[cfg_attr(feature = "native", derive(strum::Display))]
#[derive(Debug)]
#[allow(unused)]
pub enum CodeExecutionFailure {
    /// Cannot perform because there is the same or something else running
    BUSY,
    /// Generic internal error
    ERR,
    /// Cannot perform because requires homing before
    HomingRequired,
    /// Specific internal error: Numerical computation issue (division by 0, sqrt(x<0) or any other kind of ambiguity)
    NumericalError,
    /// The GCode is considered, but not yet implemented
    NotYetImplemented,
}
#[allow(unused)]
pub type CodeExecutionResult = Result<CodeExecutionSuccess, CodeExecutionFailure>;
