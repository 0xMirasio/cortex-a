//! VTTBR_EL2

use tock_registers::{
    interfaces::{Readable, Writeable},
    register_bitfields,
};

register_bitfields! {u64,
    pub VTTBR_EL2 [
        VMID OFFSET(48) NUMBITS(16) [],
        BADDR OFFSET(1) NUMBITS(47) [],
        CNP OFFSET(0) NUMBITS(1) [
            Enable = 1,
            Disable = 0
        ]
    ]
}

pub struct Reg;

impl Readable for Reg {
    type T = u64;
    type R = VTTBR_EL2::Register;

    sys_coproc_read_raw!(u64, "VTTBR_EL2", "x");
}

impl Writeable for Reg {
    type T = u64;
    type R = VTTBR_EL2::Register;

    sys_coproc_write_raw!(u64, "VTTBR_EL2", "x");
}

pub const VTTBR_EL2: Reg = Reg {};
