#[cfg(feature = "native")]
use core::fmt::Display;
use crate::hwa::EventStatus;
use crate::math::Real;
#[cfg(feature = "native")]
use strum::Display;
use strum::{AsRefStr, VariantNames};
use alloc::string::String;
#[cfg(feature = "with-motion")]
pub(crate) mod motion_planning;
#[cfg(feature = "with-motion")]
pub(crate) mod motion_timing;

mod processing;
pub use processing::*;

pub mod task_control;
#[cfg(feature = "with-printjob")]
pub mod task_printjob;
#[cfg(feature = "integration-test")]
pub mod task_integration;
#[cfg(feature = "with-motion")]
pub mod task_defer;
#[cfg(any(feature = "with-hot-end", feature = "with-hot-bed"))]
pub mod task_temperature;
#[cfg(feature = "with-motion")]
pub mod task_stepper;

#[allow(dead_code)]
#[derive(Clone, Default)]
#[cfg_attr(feature = "native", derive(Debug))]
pub struct S {
    pub(crate) ln: Option<u32>,
    pub(crate) s: Option<Real>,
}
#[allow(dead_code)]
#[derive(Clone, Default)]
#[cfg_attr(feature = "native", derive(Debug))]
pub struct XYZW {
    pub(crate) ln: Option<u32>,
    pub(crate) x: Option<Real>,
    pub(crate) y: Option<Real>,
    pub(crate) z: Option<Real>,
    pub(crate) w: Option<Real>,
}

#[cfg(feature = "with-defmt")]
impl crate::hwa::defmt::Format for XYZW {
    fn format(&self, fmt: crate::hwa::defmt::Formatter) {
        crate::hwa::defmt::write!(fmt, "XYZW {:?}", self.ln)
    }
}


#[allow(dead_code)]
#[derive(Clone, Default)]
#[cfg_attr(feature = "native", derive(Debug))]
pub struct XYZ {
    pub(crate) ln: Option<u32>,
    pub(crate) f: Option<Real>,
    pub(crate) x: Option<Real>,
    pub(crate) y: Option<Real>,
    pub(crate) z: Option<Real>,
}

#[cfg(feature = "with-defmt")]
impl crate::hwa::defmt::Format for XYZ {
    fn format(&self, fmt: crate::hwa::defmt::Formatter) {
        crate::hwa::defmt::write!(fmt, "XYZ {:?}", self.ln)
    }
}

#[cfg(feature = "native")]
impl Display for XYZ {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::write!(f,
               "X {} Y {} Z {} F {}",
               self.x.map_or_else(|| "NaN".to_string(), |v| v.to_string()),
               self.y.map_or_else(|| "NaN".to_string(), |v| v.to_string()),
               self.z.map_or_else(|| "NaN".to_string(), |v| v.to_string()),
               self.f.map_or_else(|| "NaN".to_string(), |v| v.to_string()),
        )
    }
}

#[allow(dead_code)]
#[derive(Clone, Default)]
#[cfg_attr(feature = "native", derive(Debug))]
pub struct XYZEFS {
    pub(crate) ln: Option<u32>,
    pub(crate) e: Option<Real>,
    pub(crate) f: Option<Real>,
    pub(crate) s: Option<Real>,
    pub(crate) x: Option<Real>,
    pub(crate) y: Option<Real>,
    pub(crate) z: Option<Real>,
}

impl XYZEFS {
    #[allow(unused)]
    pub fn with_x(&self, pos: i32) -> Self {
        Self {
            ln: self.ln,
            e: self.e,
            f: self.f,
            s: self.s,
            x: Some(Real::from_lit(pos as i64, 0)),
            y: self.y,
            z: self.z,
        }
    }
}

#[cfg(feature = "with-defmt")]
impl crate::hwa::defmt::Format for XYZEFS {
    fn format(&self, fmt: crate::hwa::defmt::Formatter) {
        crate::hwa::defmt::write!(fmt, "XYZEFS {:?}", self.ln)
    }
}

#[allow(unused)]
#[derive(Clone, VariantNames, AsRefStr, Default)]
#[cfg_attr(feature = "native", derive(Display))]
pub enum GCode {
    /// No Operation
    #[default]
    NOP,
    #[cfg(feature = "grbl-compat")]
    /// GRBL compat status
    STATUS,
    #[cfg(feature = "grbl-compat")]
    /// GRBL compat cmd
    GRBLCMD,
    /// List supported G-Codes
    G,
    /// Rapid move
    G0(XYZ),
    /// Linear move
    G1(XYZEFS),
    /// Dwell
    G4,
    /// Set coordinate system data
    G10, G11, // retraction
    G17, G18, G19, // CNC Plane selection
    G21, // Settings
    G22, G23, // Retraction

    /// Move to Origin (Home)
    G28(XYZW),
    /// Detailed Z-Probe
    G29,
    /// Set Z probe head offset
    #[strum(serialize = "G29.1")] G29_1,
    /// Set Z probe head offset calculated from tool head position
    #[strum(serialize = "G29.2")] G29_2,
    /// Single Z-Probe
    G30,
    /// Dock Sled
    G31,
    /// Undock Sled
    G32,
    #[strum(serialize = "G38.2")]
    G38_2, #[strum(serialize = "G38.3")] G38_3, #[strum(serialize = "G38.4")] G38_4, #[strum(serialize = "G38.5")] G38_5, G80, G81, G82, // Probing
    /// Set to Absolute Positioning
    G90,
    /// Set to Relative Positioning
    G91,
    /// Set position
    G92,
    #[strum(serialize = "G92.1")]
    G92_1,
    #[strum(serialize = "G92.2")]
    G92_2, // Positioning
    G93, G94, // Feed rate

    /// List supported M-Codes
    M,
    M0, M1, M2, // Program control
    M3, M4, M5, // CNC/Laser
    M6, M7, M8, M9, M10, M11, M13, M16, // CNC
    M17, M18, // Stepper motors
    /// List SD
    M20(Option<String>),
    M21,
    M22,
    /// Select SD file
    M23(Option<String>),
    /// Start/resume SD print
    M24,
    /// Pause SD print
    M25,
    M26,
    M27,
    /// Program Stop
    M30, M31, M32, M33, // SD
    M37, // Simulation mode
    /// Set Print Progress
    M73,
    /// Soft reset
    M79,
    /// ATX Power ON
    M80,
    /// ATX Power OFF
    M81,
    /// Settings
    M83,
    /// Disable steppers
    M84, M92,
    /// Show memory usage
    M100,
    /// Set Hotend Temperature
    M104(S),
    /// Get Hotend and/or Hotbed Temperature
    M105,
    /// Fan On
    M106,
    /// Fan Off
    M107,
    /// Wait for hotend temp
    M109(S),
    M110, // Settings
    /// Debug level
    M111,
    /// Full emergency stop
    M112,
    /// Get current position
    M114,
    /// Get Firmware Version and Capabilities
    M115,
    /// Wait
    M116,
    /// Display message
    M117,
    /// Echo message on host
    M118,
    /// Get Endstop Status
    M119,
    M120, M121, // Endstops get/set
    /// Set hotbed temperature
    M140(S),
    /// Wait for hotbed temperature
    M190,
    M200,
    /// Print / Travel Move Limits
    M201, M202,
    /// Set Max Feedrate
    M203, M204,
    /// Set Advanced Settings
    M205, M206, M207, M208, M209, M210, M211, M212, M218, // Settings
    /// Set Feedrate percentage
    M220(S),
    /// Set Flow Percentage
    M221(S),
    M290, // Babystepping
    M302, M305, M350, M360, // Settings
    /// Wait for moves and finish
    M400,
    M401, M402, // Probing
    M404, M407, // Settings
    M410, // quick stop
    M422, // Probe point
    M450, M451, M452, M453, // Modes
    M500, M501,
    /// Restore Default Settings
    M502, M504, M505, // EEProm/State
    M510, M511, M512, M513, // Password and locking
    /// Abort SD printing
    M524,
    M555, M563, M851,
    /// Report the status of position encoder modules.
    #[strum(serialize = "M862.1")] M862_1,
    /// Perform an axis continuity test for position encoder modules.
    #[strum(serialize = "M862.2")] M862_2,
    /// Perform steps-per-mm calibration for position encoder modules.
    #[strum(serialize = "M862.3")] M862_3,
    /// Set Lineal Advance Factor
    M900,
    /// Set motor current
    M907,
    M929 // Logging
}

#[cfg(feature = "defmt")]
impl crate::hwa::defmt::Format for GCode {
    fn format(&self, fmt: crate::hwa::defmt::Formatter) {
        let gcode_name: &str = self.as_ref();
        crate::hwa::defmt::write!(fmt, "{}", gcode_name)
    }
}

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
    /// Cannot perform because requires homing before
    PowerRequired,
    /// Specific internal error: Numerical computation issue (division by 0, sqrt(x<0) or any other kind of ambiguity)
    NumericalError,
    /// The GCode is considered, but not yet implemented
    NotYetImplemented,
}
#[allow(unused)]
pub type CodeExecutionResult = Result<CodeExecutionSuccess, CodeExecutionFailure>;
