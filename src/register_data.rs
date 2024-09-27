use crate::registers::*;

/// Struct representation of the Status register
#[derive(Debug, Copy, Clone)]
pub struct Status {
    /// FIFO threshold status
    /// false: FIFO filling is lower than the threshold level
    /// true: FIFO filling is equal or higher than the threshold level
    pub fifo_threshold: bool,
    /// Wake up event detection
    /// false: no wake-up event detected
    /// true: wake-up event detected
    pub wake_up_event: bool,
    /// Sleep event status
    /// false: no sleep event detected
    /// true: sleep event detected
    pub sleep_event: bool,
    /// Double-tap event status
    /// false: no tap event detected
    /// true: tap event detected
    pub double_tap_event: bool,
    /// Single-tap event status
    /// false: no tap event detected
    /// true: tap event detected
    pub single_tap_event: bool,
    /// Source of change in position (portrait/landscape/face-up/face-down)
    /// false: no change in position detected
    /// true: change in position detected
    pub position_change_event: bool,
    /// Free-fall event detection status
    /// false: no free-fall event detected
    /// true: free-fall event detected
    pub free_fall_event: bool,
    /// Data ready status
    /// false: no data is available
    /// true: X-, Y- and Z-axis new data available
    pub data_ready: bool,
}

impl From<u8> for Status {
    fn from(value: u8) -> Self {
        Self {
            fifo_threshold: value & FIFO_THS != 0,
            wake_up_event: value & WU_IA != 0,
            sleep_event: value & SLEEP_STATE != 0,
            double_tap_event: value & DOUBLE_TAP != 0,
            single_tap_event: value & SINGLE_TAP != 0,
            position_change_event: value & D6D_IA != 0,
            free_fall_event: value & FF_IA != 0,
            data_ready: value & DRDY != 0,
        }
    }
}

/// Struct representation of the Status DUP (Event status) register
#[derive(Debug, Copy, Clone)]
pub struct EventStatus {
    /// FIFO threshold status
    /// false: FIFO is not completely filled
    /// true: FIFO is overrun
    pub fifo_overrun: bool,
    /// Temperature data ready status
    /// false: data not available
    /// true: new set of data is available
    pub temperature_data_ready: bool,
    /// Sleep event status
    /// false: no sleep event detected
    /// true: sleep event detected
    pub sleep_event: bool,
    /// Double-tap event status
    /// false: no tap event detected
    /// true: tap event detected
    pub double_tap_event: bool,
    /// Single-tap event status
    /// false: no tap event detected
    /// true: tap event detected
    pub single_tap_event: bool,
    /// Source of change in position (portrait/landscape/face-up/face-down)
    /// false: no change in position detected
    /// true: change in position detected
    pub position_change_event: bool,
    /// Free-fall event detection status
    /// false: no free-fall event detected
    /// true: free-fall event detected
    pub free_fall_event: bool,
    /// Data ready status
    /// false: no data is available
    /// true: X-, Y- and Z-axis new data available
    pub data_ready: bool,
}

impl From<u8> for EventStatus {
    fn from(value: u8) -> Self {
        Self {
            fifo_overrun: value & OVR != 0,
            temperature_data_ready: value & DRDY_T != 0,
            sleep_event: value & SLEEP_STATE_IA != 0,
            double_tap_event: value & DOUBLE_TAP != 0,
            single_tap_event: value & SINGLE_TAP != 0,
            position_change_event: value & D6D_IA != 0,
            free_fall_event: value & FF_IA != 0,
            data_ready: value & DRDY != 0,
        }
    }
}

/// Acceleration data
pub struct AccelerationData {
    /// X-axis acceleration
    pub x: f32,
    /// Y-axis acceleration
    pub y: f32,
    /// Z-axis acceleration
    pub z: f32,
}

/// RAW acceleration data
pub struct RawAccelerationData {
    /// X-axis acceleration
    pub x: i16,
    /// Y-axis acceleration
    pub y: i16,
    /// Z-axis acceleration
    pub z: i16,
}

/// FIFO Samples Status
#[derive(Debug, Copy, Clone)]
pub struct FifoSamplesStatus {
    /// FIFO threshold status
    /// false: FIFO filling is lower than the threshold level
    /// true: FIFO filling is equal or higher than the threshold level
    pub threshold: bool,
    /// FIFO overrun status
    /// false: FIFO is not overrun
    /// true: FIFO is overrun
    pub overrun: bool,
    /// Number of unread samples in FIFO
    pub samples: u8,
}

impl From<u8> for FifoSamplesStatus {
    fn from(value: u8) -> Self {
        Self {
            threshold: value & FIFO_FTH != 0,
            overrun: value & FIFO_OVR != 0,
            samples: value & FIFO_DIFF,
        }
    }
}

/// Wake-up source
#[derive(Debug, Copy, Clone)]
pub struct WakeUpSource {
    /// Free-fall event detection status
    /// false: no free-fall event detected
    /// true: free-fall event detected
    pub free_fall_event: bool,
    /// Sleep event status
    /// false: no sleep event detected
    /// true: sleep event detected
    pub sleep_event: bool,
    /// Wake up event detection
    /// false: no wake-up event detected
    /// true: wake-up event detected
    pub wake_up_event: bool,
    /// X-axis wake-up event detection
    /// false: no wake-up event detected
    /// true: wake-up event on X-axis detected
    pub x_wake_up_event: bool,
    /// Y-axis wake-up event detection
    /// false: no wake-up event detected
    /// true: wake-up event on Y-axis detected
    pub y_wake_up_event: bool,
    /// Z-axis wake-up event detection
    /// false: no wake-up event detected
    /// true: wake-up event on Z-axis detected
    pub z_wake_up_event: bool,
}

impl From<u8> for WakeUpSource {
    fn from(value: u8) -> Self {
        Self {
            free_fall_event: value & WAKE_UP_FF_IA != 0,
            sleep_event: value & WAKE_UP_SLEEP_STATE_IA != 0,
            wake_up_event: value & WAKE_UP_WU_IA != 0,
            x_wake_up_event: value & X_WU != 0,
            y_wake_up_event: value & Y_WU != 0,
            z_wake_up_event: value & Z_WU != 0,
        }
    }
}

/// Sign of the tap event
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Sign {
    /// Positive sign
    Positive,
    /// Negative sign
    Negative,
}

/// Tap source
#[derive(Debug, Copy, Clone)]
pub struct TapSource {
    /// Tap event status
    /// false: no tap event detected
    /// true: tap event detected
    pub tap_event: bool,
    /// Single-tap event status
    /// false: no tap event detected
    /// true: tap event detected
    pub single_tap_event: bool,
    /// Double-tap event status
    /// false: no tap event detected
    /// true: tap event detected
    pub double_tap_event: bool,
    /// Tap sign
    pub tap_sign: Sign,
    /// X-axis tap event detection
    /// false: no tap event detected
    /// true: tap event on X-axis detected
    pub x_tap_event: bool,
    /// Y-axis tap event detection
    /// false: no tap event detected
    /// true: tap event on Y-axis detected
    pub y_tap_event: bool,
    /// Z-axis tap event detection
    /// false: no tap event detected
    /// true: tap event on Z-axis detected
    pub z_tap_event: bool,
}

impl From<u8> for TapSource {
    fn from(value: u8) -> Self {
        Self {
            tap_event: value & TAP_IA != 0,
            single_tap_event: value & TAP_SRC_SINGLE_TAP != 0,
            double_tap_event: value & TAP_SRC_DOUBLE_TAP != 0,
            tap_sign: if value & TAP_SIGN == 0 {
                Sign::Positive
            } else {
                Sign::Negative
            },
            x_tap_event: value & X_TAP != 0,
            y_tap_event: value & Y_TAP != 0,
            z_tap_event: value & Z_TAP != 0,
        }
    }
}
