#![allow(non_upper_case_globals)]

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
