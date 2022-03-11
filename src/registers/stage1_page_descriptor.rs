//! STAGE1_PAGE_DESCRIPTOR

use tock_registers::{
    interfaces::{Readable, Writeable},
    register_bitfields,
};

register_bitfields! {u64,
    pub STAGE1_PAGE_DESCRIPTOR [
        
        /// Unprivileged execute-never.
        UXN      OFFSET(54) NUMBITS(1) [
            False = 0,
            True = 1
        ],

        /// Privileged execute-never.
        PXN      OFFSET(53) NUMBITS(1) [
            False = 0,
            True = 1
        ],

        /// Physical address of the next table descriptor (lvl2) or the page descriptor (lvl3).
        OUTPUT_ADDR_64KiB OFFSET(16) NUMBITS(32) [], // [47:16]

        /// Access flag.
        AF       OFFSET(10) NUMBITS(1) [
            False = 0,
            True = 1
        ],

        /// Shareability field.
        SH       OFFSET(8) NUMBITS(2) [
            OuterShareable = 0b10,
            InnerShareable = 0b11
        ],

        /// Access Permissions.
        AP       OFFSET(6) NUMBITS(2) [
            RW_EL1 = 0b00,
            RW_EL1_EL0 = 0b01,
            RO_EL1 = 0b10,
            RO_EL1_EL0 = 0b11
        ],

        /// Memory attributes index into the MAIR_EL1 register.
        AttrIndx OFFSET(2) NUMBITS(3) [],

        TYPE     OFFSET(1) NUMBITS(1) [
            Reserved_Invalid = 0,
            Page = 1
        ],

        VALID    OFFSET(0) NUMBITS(1) [
            False = 0,
            True = 1
        ]


    ]
}

pub struct Reg;

impl Readable for Reg {
    type T = u64;
    type R = STAGE1_PAGE_DESCRIPTOR::Register;

    sys_coproc_read_raw!(u64, "STAGE1_PAGE_DESCRIPTOR", "x");
}

impl Writeable for Reg {
    type T = u64;
    type R = STAGE1_PAGE_DESCRIPTOR::Register;

    sys_coproc_write_raw!(u64, "STAGE1_PAGE_DESCRIPTOR", "x");
}

pub const STAGE1_PAGE_DESCRIPTOR: Reg = Reg {};
