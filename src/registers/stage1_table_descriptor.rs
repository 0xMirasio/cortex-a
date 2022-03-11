//! STAGE1_PAGE_DESCRIPTOR

use tock_registers::{
    interfaces::{Readable, Writeable},
    register_bitfields,
};

register_bitfields! {u64,

    pub STAGE1_TABLE_DESCRIPTOR [
        
        NEXT_LEVEL_TABLE_ADDR_64KiB OFFSET(16) NUMBITS(32) [], // [47:16]

        TYPE  OFFSET(1) NUMBITS(1) [
            Block = 0,
            Table = 1
        ],

        VALID OFFSET(0) NUMBITS(1) [
            False = 0,
            True = 1
        ]

    ]
}

pub struct Reg;

impl Readable for Reg {
    type T = u64;
    type R = STAGE1_TABLE_DESCRIPTOR::Register;

    sys_coproc_read_raw!(u64, "STAGE1_TABLE_DESCRIPTOR", "x");
}

impl Writeable for Reg {
    type T = u64;
    type R = STAGE1_TABLE_DESCRIPTOR::Register;

    sys_coproc_write_raw!(u64, "STAGE1_TABLE_DESCRIPTOR", "x");
}

pub const STAGE1_TABLE_DESCRIPTOR: Reg = Reg {};
