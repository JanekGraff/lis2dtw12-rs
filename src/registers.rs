#![allow(non_upper_case_globals)]
#![allow(dead_code)]

#[allow(non_camel_case_types)]
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
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

// ------- WAKE_UP_SRC ------- //
pub const WAKE_UP_FF_IA: u8 = 0b0010_0000;
pub const WAKE_UP_SLEEP_STATE_IA: u8 = 0b0001_0000;
pub const WAKE_UP_WU_IA: u8 = 0b0000_1000;
pub const X_WU: u8 = 0b0000_0100;
pub const Y_WU: u8 = 0b0000_0010;
pub const Z_WU: u8 = 0b0000_0001;

// ------- TAP_SRC ------- //
pub const TAP_IA: u8 = 0b0100_0000;
pub const TAP_SRC_SINGLE_TAP: u8 = 0b0010_0000;
pub const TAP_SRC_DOUBLE_TAP: u8 = 0b0001_0000;
pub const TAP_SIGN: u8 = 0b0000_1000;
pub const X_TAP: u8 = 0b0000_0100;
pub const Y_TAP: u8 = 0b0000_0010;
pub const Z_TAP: u8 = 0b0000_0001;

// ------- SIXD_SRC ------- //
pub const IA_6D: u8 = 0b0100_0000;
pub const ZH: u8 = 0b0010_0000;
pub const ZL: u8 = 0b0001_0000;
pub const YH: u8 = 0b0000_1000;
pub const YL: u8 = 0b0000_0100;
pub const XH: u8 = 0b0000_0010;
pub const XL: u8 = 0b0000_0001;

// ------- ALL_INT_SRC ------- //
pub const ALL_INT_SLEEP_CHANGE_IA: u8 = 0b0010_0000;
pub const ALL_INT_6D_IA: u8 = 0b0001_0000;
pub const ALL_INT_DOUBLE_TAP: u8 = 0b0000_1000;
pub const ALL_INT_SINGLE_TAP: u8 = 0b0000_0100;
pub const ALL_INT_WU_IA: u8 = 0b0000_0010;
pub const ALL_INT_FF_IA: u8 = 0b0000_0001;

// ------- CTRL7 ------- //
pub const DRDY_PULSED: u8 = 0b1000_0000;
pub const INT2_ON_INT1: u8 = 0b0100_0000;
pub const INTERRUPTS_ENABLE: u8 = 0b0010_0000;
pub const USR_OFF_ON_OUT: u8 = 0b0001_0000;
pub const USR_OFF_ON_WU: u8 = 0b0000_1000;
pub const USR_OFF_W: u8 = 0b0000_0100;
pub const HP_REF_MODE: u8 = 0b0000_0010;
pub const LPASS_ON6D: u8 = 0b0000_0001;
