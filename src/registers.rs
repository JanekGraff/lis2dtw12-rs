#![allow(non_upper_case_globals)]

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
