use crate::registers::*;

/// Struct representation of the Status register
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Status {
    /// FIFO threshold status
    ///
    /// false: FIFO filling is lower than the threshold level
    ///
    /// true: FIFO filling is equal or higher than the threshold level
    pub fifo_threshold: bool,
    /// Wake up event detection
    ///
    /// false: no wake-up event detected
    ///
    /// true: wake-up event detected
    pub wake_up_event: bool,
    /// Sleep event status
    ///
    /// false: no sleep event detected
    ///
    /// true: sleep event detected
    pub sleep_event: bool,
    /// Double-tap event status
    ///
    /// false: no tap event detected
    ///
    /// true: tap event detected
    pub double_tap_event: bool,
    /// Single-tap event status
    ///
    /// false: no tap event detected
    ///
    /// true: tap event detected
    pub single_tap_event: bool,
    /// Source of change in position (portrait/landscape/face-up/face-down)
    ///
    /// false: no change in position detected
    ///
    /// true: change in position detected
    pub position_change_event: bool,
    /// Free-fall event detection status
    ///
    /// false: no free-fall event detected
    ///
    /// true: free-fall event detected
    pub free_fall_event: bool,
    /// Data ready status
    ///
    /// false: no data is available
    ///
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
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct EventStatus {
    /// FIFO threshold status
    ///
    /// false: FIFO is not completely filled
    ///
    /// true: FIFO is overrun
    pub fifo_overrun: bool,
    /// Temperature data ready status
    ///
    /// false: data not available
    ///
    /// true: new set of data is available
    pub temperature_data_ready: bool,
    /// Sleep event status
    ///
    /// false: no sleep event detected
    ///
    /// true: sleep event detected
    pub sleep_event: bool,
    /// Double-tap event status
    ///
    /// false: no tap event detected
    ///
    /// true: tap event detected
    pub double_tap_event: bool,
    /// Single-tap event status
    ///
    /// false: no tap event detected
    ///
    /// true: tap event detected
    pub single_tap_event: bool,
    /// Source of change in position (portrait/landscape/face-up/face-down)
    ///
    /// false: no change in position detected
    ///
    /// true: change in position detected
    pub position_change_event: bool,
    /// Free-fall event detection status
    ///
    /// false: no free-fall event detected
    ///
    /// true: free-fall event detected
    pub free_fall_event: bool,
    /// Data ready status
    ///
    /// false: no data is available
    ///
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
#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct AccelerationData {
    /// X-axis acceleration
    pub x: f32,
    /// Y-axis acceleration
    pub y: f32,
    /// Z-axis acceleration
    pub z: f32,
}

/// RAW acceleration data
#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
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
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct FifoSamplesStatus {
    /// FIFO threshold status
    ///
    /// false: FIFO filling is lower than the threshold level
    ///
    /// true: FIFO filling is equal or higher than the threshold level
    pub threshold: bool,
    /// FIFO overrun status
    ///
    /// false: FIFO is not overrun
    ///
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
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct WakeUpSource {
    /// Free-fall event detection status
    ///
    /// false: no free-fall event detected
    ///
    /// true: free-fall event detected
    pub free_fall_event: bool,
    /// Sleep event status
    ///
    /// false: no sleep event detected
    ///
    /// true: sleep event detected
    pub sleep_event: bool,
    /// Wake up event detection
    ///
    /// false: no wake-up event detected
    ///
    /// true: wake-up event detected
    pub wake_up_event: bool,
    /// X-axis wake-up event detection
    ///
    /// false: no wake-up event detected
    ///
    /// true: wake-up event on X-axis detected
    pub x_wake_up_event: bool,
    /// Y-axis wake-up event detection
    ///
    /// false: no wake-up event detected
    ///
    /// true: wake-up event on Y-axis detected
    pub y_wake_up_event: bool,
    /// Z-axis wake-up event detection
    ///
    /// false: no wake-up event detected
    ///
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
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sign {
    /// Positive sign
    Positive,
    /// Negative sign
    Negative,
}

/// Tap source
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct TapSource {
    /// Tap event status
    ///
    /// false: no tap event detected
    ///
    /// true: tap event detected
    pub tap_event: bool,
    /// Single-tap event status
    ///
    /// false: no tap event detected
    ///
    /// true: tap event detected
    pub single_tap_event: bool,
    /// Double-tap event status
    ///
    /// false: no tap event detected
    ///
    /// true: tap event detected
    pub double_tap_event: bool,
    /// Tap sign
    pub tap_sign: Sign,
    /// X-axis tap event detection
    ///
    /// false: no tap event detected
    ///
    /// true: tap event on X-axis detected
    pub x_tap_event: bool,
    /// Y-axis tap event detection
    ///
    /// false: no tap event detected
    ///
    /// true: tap event on Y-axis detected
    pub y_tap_event: bool,
    /// Z-axis tap event detection
    ///
    /// false: no tap event detected
    ///
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

/// 6D source
#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct SixDSource {
    /// Source of change in position (portrait/landscape/face-up/face-down)
    ///
    /// false: no change in position detected
    ///
    /// true: change in position detected
    pub position_change_event: bool,
    /// ZH over threshold
    ///
    /// false: ZH is not over threshold
    ///
    /// true: ZH is over threshold
    pub zh_over_threshold: bool,
    /// ZL over threshold
    ///
    /// false: ZL is not over threshold
    ///
    /// true: ZL is over threshold
    pub zl_over_threshold: bool,
    /// YH over threshold
    ///
    /// false: YH is not over threshold
    ///
    /// true: YH is over threshold
    pub yh_over_threshold: bool,
    /// YL over threshold
    ///
    /// false: YL is not over threshold
    ///
    /// true: YL is over threshold
    pub yl_over_threshold: bool,
    /// XH over threshold
    ///
    /// false: XH is not over threshold
    ///
    /// true: XH is over threshold
    pub xh_over_threshold: bool,
    /// XL over threshold
    ///
    /// false: XL is not over threshold
    ///
    /// true: XL is over threshold
    pub xl_over_threshold: bool,
}

impl From<u8> for SixDSource {
    fn from(value: u8) -> Self {
        Self {
            position_change_event: value & IA_6D != 0,
            zh_over_threshold: value & ZH != 0,
            zl_over_threshold: value & ZL != 0,
            yh_over_threshold: value & YH != 0,
            yl_over_threshold: value & YL != 0,
            xh_over_threshold: value & XH != 0,
            xl_over_threshold: value & XL != 0,
        }
    }
}

/// Struct representation of the All Interrupt Sources register
///
/// This register is a combination of all interrupt sources
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct AllInterruptSources {
    /// Sleep change interrupt
    ///
    /// false: no sleep change interrupt
    ///
    /// true: sleep change interrupt
    pub sleep_change_interrupt: bool,
    /// 6D interrupt
    ///
    /// false: no 6D interrupt
    ///
    /// true: 6D interrupt
    pub six_d_interrupt: bool,
    /// Double-tap interrupt
    ///
    /// false: no double-tap interrupt
    ///
    /// true: double-tap interrupt
    pub double_tap_interrupt: bool,
    /// Single-tap interrupt
    ///
    /// false: no single-tap interrupt
    ///
    /// true: single-tap interrupt
    pub single_tap_interrupt: bool,
    /// Wake-up interrupt
    ///
    /// false: no wake-up interrupt
    ///
    /// true: wake-up interrupt
    pub wake_up_interrupt: bool,
    /// Free-fall interrupt
    ///
    /// false: no free-fall interrupt
    ///
    /// true: free-fall interrupt
    pub free_fall_interrupt: bool,
}

impl From<u8> for AllInterruptSources {
    fn from(value: u8) -> Self {
        Self {
            sleep_change_interrupt: value & ALL_INT_SLEEP_CHANGE_IA != 0,
            six_d_interrupt: value & ALL_INT_6D_IA != 0,
            double_tap_interrupt: value & ALL_INT_DOUBLE_TAP != 0,
            single_tap_interrupt: value & ALL_INT_SINGLE_TAP != 0,
            wake_up_interrupt: value & ALL_INT_WU_IA != 0,
            free_fall_interrupt: value & ALL_INT_FF_IA != 0,
        }
    }
}

/// Struct representation of the 5 SRC registers combined
///
/// can be read with [crate::Lis2dtw12::get_all_sources]
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct AllSources {
    /// Event status register (see [`EventStatus`](crate::register_data::EventStatus))
    pub event_status: EventStatus,
    /// Wake up source register (see [`WakeUpSource`](crate::register_data::WakeUpSource))
    pub wake_up_source: WakeUpSource,
    /// Tap source register (see [`TapSource`](crate::register_data::TapSource))
    pub tap_source: TapSource,
    /// 6D source register (see [`SixDSource`](crate::register_data::SixDSource))
    pub six_d_source: SixDSource,
    /// All interrupt sources register (see [`AllInterruptSources`](crate::register_data::AllInterruptSources))
    pub all_interrupt_sources: AllInterruptSources,
}

impl From<[u8; 5]> for AllSources {
    fn from(value: [u8; 5]) -> Self {
        Self {
            event_status: EventStatus::from(value[0]),
            wake_up_source: WakeUpSource::from(value[1]),
            tap_source: TapSource::from(value[2]),
            six_d_source: SixDSource::from(value[3]),
            all_interrupt_sources: AllInterruptSources::from(value[4]),
        }
    }
}
