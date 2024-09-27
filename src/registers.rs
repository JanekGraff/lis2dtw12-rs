#![allow(non_upper_case_globals)]
#![allow(clippy::upper_case_acronyms)]

use core::default;

/// Operating Mode
#[derive(Debug, Copy, Clone, Default)]
pub enum Mode {
    /// Low power mode (12/14-bit resolution, depending on the set LowPowerMode)
    #[default]
    LowPower = 0b00,
    /// High-performance mode (14 Bit resolution)
    HighPerformance = 0b01,
    /// Single data conversion on-demand mode (12/14-bit resolution, depending on the set LowPowerMode)
    SingleDataConversion = 0b10,
}

/// Output Data Rate
/// Rates are shown as: <High-performance rate> / <Low-power rate>
#[derive(Debug, Copy, Clone, Default)]
pub enum OutputDataRate {
    /// Power-down mode
    #[default]
    PowerDown = 0b0000,
    /// 12.5 Hz / 1.6 Hz
    Hz1_6 = 0b0001,
    /// 12.5 Hz / 12.5 Hz
    Hz12_5 = 0b0010,
    /// 25 Hz / 25 Hz
    Hz25 = 0b0011,
    /// 50 Hz / 50 Hz
    Hz50 = 0b0100,
    /// 100 Hz / 100 Hz
    Hz100 = 0b0101,
    /// 200 Hz / 200 Hz
    Hz200 = 0b0110,
    /// 400 Hz / 200 Hz
    Hz400 = 0b0111,
    /// 800 Hz / 200 Hz
    Hz800 = 0b1000,
    /// 1600 Hz / 200 Hz
    Hz1600 = 0b1001,
}

/// Low Power Mode
#[derive(Debug, Copy, Clone, Default)]
pub enum LowPowerMode {
    /// Low-power mode 1 (12-bit resolution)
    #[default]
    Mode1 = 0b00,
    /// Low-power mode 2 (14-bit resolution)
    Mode2 = 0b01,
    /// Low-power mode 3 (14-bit resolution)
    Mode3 = 0b10,
    /// Low-power mode 4 (14-bit resolution)
    Mode4 = 0b11,
}

/// Digital filtering cutoff selection / Bandwidth selection
#[derive(Debug, Copy, Clone, Default)]
pub enum BandwidthSelection {
    /// ODR/2 (up to ODR = 800 Hz, 400 Hz when ODR = 1600 Hz)
    #[default]
    OdrDiv2 = 0b00,
    /// ODR/4 (HP/LP)
    OdrDiv4 = 0b01,
    /// ODR/10 (HP/LP)
    OdrDiv10 = 0b10,
    /// ODR/20 (HP/LP)
    OdrDiv20 = 0b11,
}

/// Full-scale selection
#[derive(Debug, Copy, Clone, Default)]
pub enum FullScale {
    /// ±2 g
    #[default]
    G2 = 0b00,
    /// ±4 g
    G4 = 0b01,
    /// ±8 g
    G8 = 0b10,
    /// ±16 g
    G16 = 0b11,
}

impl FullScale {
    pub(crate) fn convert_raw_i16_to_g(self, raw: i16) -> f32 {
        let factor = match self {
            // TODO: ?
            FullScale::G2 => 0.061,
            FullScale::G4 => 0.122,
            FullScale::G8 => 0.244,
            FullScale::G16 => 0.488,
        };
        // TODO: Need to factor in the resolution?
        (raw >> 2) as f32 * factor
    }
}

/// Fifo Mode
#[derive(Debug, Copy, Clone, Default)]
pub enum FifoMode {
    /// Bypass mode (FIFO turned off)
    #[default]
    Bypass = 0b000,
    /// FIFO mode: Stop collecting data when FIFO is full
    StopOnFifoFull = 0b001,
    /// Continuous-to-FIFO: Stream mode until trigger is deasserted, then FIFO mode (StopOnFifoFull)
    ContinuousToFifo = 0b011,
    /// Bypass-to-continuous: Bypass mode until trigger is deasserted, then Continuous mode
    BypassToContinuous = 0b100,
    /// Continuous mode: If FIFO is full, the new sample overwrites the older sample
    Continuous = 0b110,
}

/// Thresholds for 4D/6D function @ FS = ±2 g
#[derive(Debug, Copy, Clone, Default)]
pub enum Threshold6D {
    /// 6 (80°)
    #[default]
    Deg80 = 0b00,
    /// 11 (70°)
    Deg70 = 0b01,
    /// 16 (60°)
    Deg60 = 0b10,
    /// 21 (50°)
    Deg50 = 0b11,
}

/// Tap Priority axis selection for tap detection
/// MAX_PRIO, MID_PRIO, MIN_PRIO
#[derive(Debug, Copy, Clone, Default)]
pub enum TapPriority {
    /// X, Y, Z
    #[default]
    XYZ = 0b000,
    /// Y, X, Z
    YXZ = 0b001,
    /// X, Z, Y
    XZY = 0b010,
    /// Z, Y, X
    ZYX = 0b011,
    /// X, Y, Z (alternative), same as XYZ
    XYZAlt = 0b100,
    /// Y, Z, X
    YZX = 0b101,
    /// Z, X, Y
    ZXY = 0b110,
    /// Z, Y, X (alternative), same as ZYX
    ZYXAlt = 0b111,
}

/// Free-fall Threshold @ FS = ±2 g
#[derive(Debug, Copy, Clone, Default)]
pub enum FreeFallThreshold {
    /// 5
    #[default]
    Ths5 = 0b000,
    /// 7
    Ths7 = 0b001,
    /// 8
    Ths8 = 0b010,
    /// 10
    Ths10 = 0b011,
    /// 11
    Ths11 = 0b100,
    /// 13
    Ths13 = 0b101,
    /// 15
    Ths15 = 0b110,
    /// 16
    Ths16 = 0b111,
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Copy, Clone)]
pub enum Register {
    OUT_T_L = 0x0D,
    OUT_T_H = 0x0E,
    WHO_AM_I = 0x0F,
    CTRL1 = 0x20,
    CTRL2 = 0x21,
    CTRL3 = 0x22,
    CTRL_INT1_PAD_CTRL = 0x23,
    CTRL_INT2_PAD_CTRL = 0x24,
    CTRL6 = 0x25,
    STATUS = 0x27,
    OUT_X_L = 0x28,
    OUT_X_H = 0x29,
    OUT_Y_L = 0x2A,
    OUT_Y_H = 0x2B,
    OUT_Z_L = 0x2C,
    OUT_Z_H = 0x2D,
    FIFO_CTRL = 0x2E,
    FIFO_SAMPLES = 0x2F,
    TAP_THS_X = 0x30,
    TAP_THS_Y = 0x31,
    TAP_THS_Z = 0x32,
    INT_DUR = 0x33,
    WAKE_UP_THS = 0x34,
    WAKE_UP_DUR = 0x35,
    FREE_FALL = 0x36,
    STATUS_DUP = 0x37,
    WAKE_UP_SRC = 0x38,
    TAP_SRC = 0x39,
    SIXD_SRC = 0x3A,
    ALL_INT_SRC = 0x3B,
    X_OFS_USR = 0x3C,
    Y_OFS_USR = 0x3D,
    Z_OFS_USR = 0x3E,
    CTRL7 = 0x3F,
}

impl Register {
    pub fn addr(self) -> u8 {
        self as u8
    }
}

// ----------------- Register Masks ----------------- //
// ------- CTRL1 ------- //
pub const ODR_MASK: u8 = 0b1111_0000;
pub const ODR_SHIFT: u8 = 4;
pub const MODE_MASK: u8 = 0b0000_1100;
pub const MODE_SHIFT: u8 = 2;
pub const LP_MODE_MASK: u8 = 0b0000_0011;
pub const LP_MODE_SHIFT: u8 = 0;

// ------- CTRL2 ------- //
pub const SOFT_RESET: u8 = 0b0100_0000;
pub const CS_PU_DISC: u8 = 0b0001_0000;
pub const BDU: u8 = 0b0000_1000;

// ------- CTRL6 ------- //
pub const BW_FILT_MASK: u8 = 0b1100_0000;
pub const BW_FILT_SHIFT: u8 = 6;
pub const FS_MASK: u8 = 0b0011_0000;
pub const FS_SHIFT: u8 = 4;
pub const FDS: u8 = 0b0000_1000;
pub const LOW_NOISE: u8 = 0b0000_0100;

// ------- STATUS ------- //
pub const FIFO_THS: u8 = 0b1000_0000;
pub const WU_IA: u8 = 0b0100_0000;
pub const SLEEP_STATE: u8 = 0b0010_0000;
pub const DOUBLE_TAP: u8 = 0b0001_0000;
pub const SINGLE_TAP: u8 = 0b0000_1000;
pub const D6D_IA: u8 = 0b0000_0100;
pub const FF_IA: u8 = 0b0000_0010;
pub const DRDY: u8 = 0b0000_0001;

// ------- FIFO_CTRL ------- //
pub const FMODE_MASK: u8 = 0b1110_0000;
pub const FMODE_SHIFT: u8 = 5;
pub const FTH_MASK: u8 = 0b0001_1111;
pub const FTH_SHIFT: u8 = 0;

// ------- FIFO_SAMPLES ------- //
pub const FIFO_FTH: u8 = 0b1000_0000;
pub const FIFO_OVR: u8 = 0b0100_0000;
pub const FIFO_DIFF: u8 = 0b0011_1111;

// ------- TAP_THS_X/Y/Z  ------- //
pub const TAP_THS_MASK: u8 = 0b0001_1111;
pub const TAP_THS_SHIFT: u8 = 0;

// ------- TAP_THS_X ------- //
pub const EN_4D: u8 = 0b1000_0000;
pub const THS_6D_MASK: u8 = 0b0110_0000;
pub const THS_6D_SHIFT: u8 = 5;

// ------- TAP_THS_Y ------- //
pub const TAP_PRIOR_MASK: u8 = 0b1110_0000;
pub const TAP_PRIOR_SHIFT: u8 = 5;

// ------- TAP_THS_Z ------- //
pub const TAP_XYZ_MASK: u8 = 0b1110_0000;
pub const TAP_XYZ_SHIFT: u8 = 5;

// ------- INT_DUR ------- //
pub const LATENCY_MASK: u8 = 0b1111_0000;
pub const LATENCY_SHIFT: u8 = 4;
pub const QUIET_MASK: u8 = 0b0000_1100;
pub const QUIET_SHIFT: u8 = 2;
pub const SHOCK_MASK: u8 = 0b0000_0011;
pub const SHOCK_SHIFT: u8 = 0;

// ------- WAKE_UP_THS ------- //
pub const SINGLE_DOUBLE_TAP: u8 = 0b1000_0000;
pub const SLEEP_ON: u8 = 0b0100_0000;
pub const WK_THS_MASK: u8 = 0b0011_1111;
pub const WK_THS_SHIFT: u8 = 0;

// ------- WAKE_UP_DUR ------- //
pub const FF_DUR5: u8 = 0b1000_0000;
pub const WK_DUR_MASK: u8 = 0b0110_0000;
pub const WK_DUR_SHIFT: u8 = 5;
pub const STATIONARY: u8 = 0b0001_0000;
pub const SLEEP_DUR_MASK: u8 = 0b0000_1111;
pub const SLEEP_DUR_SHIFT: u8 = 0;

// ------- FREE_FALL ------- //
pub const FF_DUR_MASK: u8 = 0b1111_1000;
pub const FF_DUR_SHIFT: u8 = 3;
pub const FF_THS_MASK: u8 = 0b0000_0111;
pub const FF_THS_SHIFT: u8 = 0;

// ------- STATUS_DUP ------- //
pub const OVR: u8 = 0b1000_0000;
pub const DRDY_T: u8 = 0b0100_0000;
pub const SLEEP_STATE_IA: u8 = 0b0010_0000;
// remaining bits are the same as STATUS
