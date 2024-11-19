use crate::{
    INT1_6D, INT1_DIFF5, INT1_DRDY, INT1_FF, INT1_FTH, INT1_SINGLE_TAP, INT1_TAP, INT1_WU,
    INT2_BOOT, INT2_DIFF5, INT2_DRDY, INT2_DRDY_T, INT2_FTH, INT2_OVR, INT2_SLEEP_CHG,
    INT2_SLEEP_STATE,
};

/// Operating Mode
/// See the [datasheet](https://www.st.com/resource/en/datasheet/lis2dtw12.pdf) section 3.2.1 (Operating modes) for more info
#[derive(Debug, Copy, Clone, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mode {
    /// High performance mode (14-bit resolution)
    HighPerformance = 0b0100,
    /// Continuous conversion mode (14-bit resolution)
    ContinuousLowPower4 = 0b0011,
    /// Continuous conversion mode (14-bit resolution)
    ContinuousLowPower3 = 0b0010,
    /// Continuous conversion mode (14-bit resolution)
    ContinuousLowPower2 = 0b0001,
    #[default]
    /// Continuous conversion mode (12-bit resolution)
    ContinuousLowPower1 = 0b0000,
    /// Singe data conversion on demand mode (14-bit resolution)
    SingleConversionLowPower4 = 0b1011,
    /// Singe data conversion on demand mode (14-bit resolution)
    SingleConversionLowPower3 = 0b1010,
    /// Singe data conversion on demand mode (14-bit resolution)
    SingleConversionLowPower2 = 0b1001,
    /// Singe data conversion on demand mode (12-bit resolution)
    SingleConversionLowPower1 = 0b1000,
}

/// Output Data Rate
///
/// Rates are shown as: <High-performance rate> / <Low-power rate>
#[derive(Debug, Copy, Clone, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OutputDataRate {
    /// Power-down mode
    PowerDown = 0b0000,
    /// 12.5 Hz / 1.6 Hz
    Hz1_6 = 0b0001,
    /// 12.5 Hz / 12.5 Hz
    #[default]
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

/// Digital filtering cutoff selection / Bandwidth selection
#[derive(Debug, Copy, Clone, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
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
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
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
    pub(crate) fn convert_raw_i16_to_mg(self, raw: i16, set_mode: Mode) -> f32 {
        // mg/digit
        let factor = match self {
            FullScale::G2 => 0.244,
            FullScale::G4 => 0.488,
            FullScale::G8 => 0.976,
            FullScale::G16 => 1.952,
        };

        let aligned = match (set_mode, set_lp_mode) {
            // 12-bit resolution, only active on LowPowerMode::Mode1
            (Mode::LowPower | Mode::SingleDataConversion, LowPowerMode::Mode1) => (raw >> 4) as f32,
            // 14-bit resolution, active in every other mode
            (_, _) => (raw >> 2) as f32,
        };
        aligned * factor
    }
}

/// Fifo Mode
#[derive(Debug, Copy, Clone, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
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
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
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
///
/// MAX_PRIO, MID_PRIO, MIN_PRIO
#[derive(Debug, Copy, Clone, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
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
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
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

/// INT1 PAD Configuration
#[derive(Debug, Copy, Clone, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Int1PadConfig {
    /// 6D recognition interrupt
    pub int1_6d: bool,
    /// Single-tap recognition interrupt
    pub int1_single_tap: bool,
    /// Wake-up recognition interrupt
    pub int1_wu: bool,
    /// Free-fall recognition interrupt
    pub int1_ff: bool,
    /// Double-tap recognition interrupt
    pub int1_tap: bool,
    /// Fifo full recognition interrupt
    pub int1_diff5: bool,
    /// Fifo threshold interrupt
    pub int1_fth: bool,
    /// Data ready interrupt
    pub int1_drdy: bool,
}

impl From<u8> for Int1PadConfig {
    fn from(value: u8) -> Self {
        Self {
            int1_6d: value & INT1_6D != 0,
            int1_single_tap: value & INT1_SINGLE_TAP != 0,
            int1_wu: value & INT1_WU != 0,
            int1_ff: value & INT1_FF != 0,
            int1_tap: value & INT1_TAP != 0,
            int1_diff5: value & INT1_DIFF5 != 0,
            int1_fth: value & INT1_FTH != 0,
            int1_drdy: value & INT1_DRDY != 0,
        }
    }
}

impl From<Int1PadConfig> for u8 {
    fn from(value: Int1PadConfig) -> Self {
        let mut result = 0;
        if value.int1_6d {
            result |= INT1_6D;
        }
        if value.int1_single_tap {
            result |= INT1_SINGLE_TAP;
        }
        if value.int1_wu {
            result |= INT1_WU;
        }
        if value.int1_ff {
            result |= INT1_FF;
        }
        if value.int1_tap {
            result |= INT1_TAP;
        }
        if value.int1_diff5 {
            result |= INT1_DIFF5;
        }
        if value.int1_fth {
            result |= INT1_FTH;
        }
        if value.int1_drdy {
            result |= INT1_DRDY;
        }
        result
    }
}

/// INT2 PAD Configuration
#[derive(Debug, Copy, Clone, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Int2PadConfig {
    /// Route SLEEP_STATE to INT2 pad
    pub int2_sleep_state: bool,
    /// Route SLEEP_CHANGE to INT2 pad
    pub int2_sleep_chg: bool,
    /// Route BOOT_STATUS to INT2 pad
    pub int2_boot: bool,
    /// Route temperature data-ready to INT2 pad
    pub int2_drdy_t: bool,
    /// Route FIFO overrun to INT2 pad
    pub int2_ovr: bool,
    /// Route FIFO full recognition to INT2 pad
    pub int2_diff5: bool,
    /// Route FIFO threshold to INT2 pad
    pub int2_fth: bool,
    /// Route data-ready to INT2 pad
    pub int2_drdy: bool,
}

impl From<u8> for Int2PadConfig {
    fn from(value: u8) -> Self {
        Self {
            int2_sleep_state: value & INT2_SLEEP_STATE != 0,
            int2_sleep_chg: value & INT2_SLEEP_CHG != 0,
            int2_boot: value & INT2_BOOT != 0,
            int2_drdy_t: value & INT2_DRDY_T != 0,
            int2_ovr: value & INT2_OVR != 0,
            int2_diff5: value & INT2_DIFF5 != 0,
            int2_fth: value & INT2_FTH != 0,
            int2_drdy: value & INT2_DRDY != 0,
        }
    }
}

impl From<Int2PadConfig> for u8 {
    fn from(value: Int2PadConfig) -> Self {
        let mut result = 0;
        if value.int2_sleep_state {
            result |= INT2_SLEEP_STATE;
        }
        if value.int2_sleep_chg {
            result |= INT2_SLEEP_CHG;
        }
        if value.int2_boot {
            result |= INT2_BOOT;
        }
        if value.int2_drdy_t {
            result |= INT2_DRDY_T;
        }
        if value.int2_ovr {
            result |= INT2_OVR;
        }
        if value.int2_diff5 {
            result |= INT2_DIFF5;
        }
        if value.int2_fth {
            result |= INT2_FTH;
        }
        if value.int2_drdy {
            result |= INT2_DRDY;
        }
        result
    }
}
