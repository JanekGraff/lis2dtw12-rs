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
///
/// Rates are shown as: <High-performance rate> / <Low-power rate>
#[derive(Debug, Copy, Clone, Default)]
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
    pub(crate) fn convert_raw_i16_to_mg(
        self,
        raw: i16,
        set_mode: Mode,
        set_lp_mode: LowPowerMode,
    ) -> f32 {
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
///
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
