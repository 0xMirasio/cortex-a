//! Virtualization Translation Control Register  - EL2
//!
use tock_registers::{
    interfaces::{Readable, Writeable},
    register_bitfields,
};

register_bitfields! {u64,
    pub VTCR_EL2 [

        
        SL2 OFFSET(33) NUMBITS(1) [
            Enable = 1,
            Disable = 0
        ],
        DS OFFSET(32) NUMBITS(1) [
            Enable = 1,
            Disable = 0
        ],
        RES2 OFFSET(31) NUMBITS(1) [
            Enable = 1,
            Disable = 0
        ],
        NSA OFFSET(30) NUMBITS(1) [
            Enable = 1,
            Disable = 0
        ],
        NSW OFFSET(29) NUMBITS(1) [
            Enable = 1,
            Disable = 0
        ],
        HWU62 OFFSET(28) NUMBITS(1) [
            Enable = 1,
            Disable = 0
        ],
        HWU61 OFFSET(27) NUMBITS(1) [
            Enable = 1,
            Disable = 0
        ],
        HWU60 OFFSET(26) NUMBITS(1) [
            Enable = 1,
            Disable = 0
        ],
        HWU59 OFFSET(25) NUMBITS(1) [
            Enable = 1,
            Disable = 0
        ],

        RES1 OFFSET(23) NUMBITS(2) [
            _reserved0 = 0b00,
            _reserved1 = 0b01,
            _reserved2 = 0b11,
        ],

        HD OFFSET(22) NUMBITS(1) [
            stage2_dirty_enabled = 1,
            stage2_dirty_disabled = 0
        ],

        HA OFFSET(21) NUMBITS(1) [
            stage2_accesflag_enabled = 1,
            stage2_accesflag_disabled = 0
        ],

        RES0 OFFSET(20) NUMBITS(1) [
            Enable = 1,
            Disable = 0
        ],
        VS OFFSET(19) NUMBITS(1) [
            used_16b = 1,
            ignored_8b = 0
        ],

        /// Physical Address Size.
        ///
        /// 000 32 bits, 4GiB.
        /// 001 36 bits, 64GiB.
        /// 010 40 bits, 1TiB.
        /// 011 42 bits, 4TiB.
        /// 100 44 bits, 16TiB.
        /// 101 48 bits, 256TiB.
        /// 110 52 bits, 4PB
        ///
        /// Other values are reserved.
        ///
        /// The reserved values behave in the same way as the 101 or 110 encoding, but software must
        /// not rely on this property as the behavior of the reserved values might change in a
        /// future revision of the architecture.
        ///
        /// The value 110 is permitted only if ARMv8.2-LPA is implemented and the translation
        /// granule size is 64KiB.
        ///
        /// In an implementation that supports 52-bit PAs, if the value of this field is not 110 ,
        /// then bits[51:48] of every translation table base address for the stage of translation
        /// controlled by TCR_EL2 are 0000.
        PS OFFSET(16) NUMBITS(3) [
            Bits_32 = 0b000,
            Bits_36 = 0b001,
            Bits_40 = 0b010,
            Bits_42 = 0b011,
            Bits_44 = 0b100,
            Bits_48 = 0b101,
            Bits_52 = 0b110
        ],

        /// Granule size for the TTBR0_EL2.
        ///
        /// 0b00 4KiB
        /// 0b01 64KiB
        /// 0b10 16KiB
        ///
        /// Other values are reserved.
        ///
        /// If the value is programmed to either a reserved value, or a size that has not been
        /// implemented, then the hardware will treat the field as if it has been programmed to an
        /// IMPLEMENTATION DEFINED choice of the sizes that has been implemented for all purposes
        /// other than the value read back from this register.
        ///
        /// It is IMPLEMENTATION DEFINED whether the value read back is the value programmed or the
        /// value that corresponds to the size chosen.
        TG0 OFFSET(14) NUMBITS(2) [
            KiB_4 = 0b00,
            KiB_64 = 0b01,
            KiB_16 = 0b10
        ],

        /// Shareability attribute for memory associated with translation table walks using
        /// TTBR0_EL2.
        ///
        /// 00 Non-shareable
        /// 01 Reserved
        /// 10 Outer Shareable
        /// 11 Inner Shareable
        ///
        /// Other values are reserved.
        SH0 OFFSET(12) NUMBITS(2) [
            None = 0b00,
            Outer = 0b10,
            Inner = 0b11
        ],

        /// outer cacheability attribute for memory associated with translation table walks using
        /// TTBR0_EL2.
        ///
        /// 00 Normal memory, outer Non-cacheable
        ///
        /// 01 Normal memory, outer Write-Back Read-Allocate Write-Allocate Cacheable
        ///
        /// 10 Normal memory, outer Write-Through Read-Allocate No Write-Allocate Cacheable
        ///
        /// 11 Normal memory, outer Write-Back Read-Allocate No Write-Allocate Cacheable
        ORGN0 OFFSET(10) NUMBITS(2) [
            NonCacheable = 0b00,
            WriteBack_ReadAlloc_WriteAlloc_Cacheable = 0b01,
            WriteThrough_ReadAlloc_NoWriteAlloc_Cacheable = 0b10,
            WriteBack_ReadAlloc_NoWriteAlloc_Cacheable = 0b11
        ],

        /// inner cacheability attribute for memory associated with translation table walks using
        /// TTBR0_EL2.
        ///
        /// 00 Normal memory, inner Non-cacheable
        ///
        /// 01 Normal memory, inner Write-Back Read-Allocate Write-Allocate Cacheable
        ///
        /// 10 Normal memory, inner Write-Through Read-Allocate No Write-Allocate Cacheable
        ///
        /// 11 Normal memory, inner Write-Back Read-Allocate No Write-Allocate Cacheable
        IRGN0 OFFSET(8) NUMBITS(2) [
            NonCacheable = 0b00,
            WriteBack_ReadAlloc_WriteAlloc_Cacheable = 0b01,
            WriteThrough_ReadAlloc_NoWriteAlloc_Cacheable = 0b10,
            WriteBack_ReadAlloc_NoWriteAlloc_Cacheable = 0b11
        ],


        SL0 OFFSET(6) NUMBITS(2) [
            start_lv0 = 0b11,
            start_lv1 = 0b01,
            start_lv2 = 0b10,
            start_lv3 = 0b00,
        ],


        /// The size offset of the memory region addressed by TTBR0_EL2. The region size is
        /// 2^(64-T0SZ) bytes.
        ///
        /// The maximum and minimum possible values for T0SZ depend on the level of translation
        /// table and the memory translation granule size, as described in the AArch64 Virtual
        /// Memory System Architecture chapter.
        T0SZ OFFSET(0) NUMBITS(6) []
    ]
}

pub struct Reg;

impl Readable for Reg {
    type T = u64;
    type R = VTCR_EL2::Register;

    sys_coproc_read_raw!(u64, "VTCR_EL2", "x");
}

impl Writeable for Reg {
    type T = u64;
    type R = VTCR_EL2::Register;

    sys_coproc_write_raw!(u64, "VTCR_EL2", "x");
}

pub const VTCR_EL2: Reg = Reg {};
