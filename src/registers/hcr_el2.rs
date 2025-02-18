// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (c) 2018-2022 by the author(s)
//
// Author(s):
//   - Andre Richter <andre.o.richter@gmail.com>
//   - Bradley Landherr <landhb@users.noreply.github.com>

//! Hypervisor Configuration Register - EL2
//!
//! Provides configuration controls for virtualization, including defining
//! whether various Non-secure operations are trapped to EL2.

use tock_registers::{
    interfaces::{Readable, Writeable},
    register_bitfields,
};

register_bitfields! {u64,
    pub HCR_EL2 [


        NV2  OFFSET(45) NUMBITS(1) [
            Disable = 0,
            Enable = 1
        ],

        NV1  OFFSET(43) NUMBITS(1) [
            Disable = 0,
            Enable = 1
        ],

        NV  OFFSET(42) NUMBITS(1) [
            Disable = 0,
            Enable = 1
        ],

        /// Execution state control for lower Exception levels:
        ///
        /// 0 Lower levels are all AArch32.
        /// 1 The Execution state for EL1 is AArch64. The Execution state for EL0 is determined by
        ///   the current value of PSTATE.nRW when executing at EL0.
        ///
        /// If all lower Exception levels cannot use AArch32 then this bit is RAO/WI.
        ///
        /// In an implementation that includes EL3, when SCR_EL3.NS==0, the PE behaves as if this
        /// bit has the same value as the SCR_EL3.RW bit for all purposes other than a direct read
        /// or write access of HCR_EL2.
        ///
        /// 
        
        /// The RW bit is permitted to be cached in a TLB.
        ///
        /// When ARMv8.1-VHE is implemented, and the value of HCR_EL2.{E2H, TGE} is {1, 1}, this
        /// field behaves as 1 for all purposes other than a direct read of the value of this bit.7
        RW   OFFSET(31) NUMBITS(1) [
            AllLowerELsAreAarch32 = 0,
            EL1IsAarch64 = 1
        ],

        TWI OFFSET(13) NUMBITS(1) [
            Disable = 0,
            Enable = 1
        ],

        TGE OFFSET(21) NUMBITS(1) [
            Disable = 0,
            Enable = 1
        ],

        E2H OFFSET(34) NUMBITS(1) [
            Disable = 0,
            Enable = 1
        ],

        


        /// Default Cacheability.
        ///
        /// 0 This control has no effect on the Non-secure EL1&0 translation regime.
        ///
        /// 1 In Non-secure state:
        ///   - When EL1 is using AArch64, the PE behaves as if the value of the SCTLR_EL1.M field
        ///     is 0 for all purposes other than returning the value of a direct read of SCTLR_EL1.
        ///
        ///   - When EL1 is using AArch32, the PE behaves as if the value of the SCTLR.M field is 0
        ///     for all purposes other than returning the value of a direct read of SCTLR.
        ///
        ///   - The PE behaves as if the value of the HCR_EL2.VM field is 1 for all purposes other
        ///     than returning the value of a direct read of HCR_EL2.
        ///
        ///   - The memory type produced by stage 1 of the EL1&0 translation regime is Normal
        ///     Non-Shareable, Inner Write-Back Read-Allocate Write-Allocate, Outer Write-Back
        ///     Read-Allocate Write-Allocate.
        ///
        /// This field has no effect on the EL2, EL2&0, and EL3 translation regimes.
        ///
        /// This field is permitted to be cached in a TLB.
        ///
        /// In an implementation that includes EL3, when the value of SCR_EL3.NS is 0 the PE behaves
        /// as if this field is 0 for all purposes other than a direct read or write access of
        /// HCR_EL2.
        ///
        /// When ARMv8.1-VHE is implemented, and the value of HCR_EL2.{E2H, TGE} is {1, 1}, this
        /// field behaves as 0 for all purposes other than a direct read of the value of this field.
        DC   OFFSET(12) NUMBITS(1) [],

        /// Set/Way Invalidation Override. Causes Non-secure EL1 execution of the data cache
        /// invalidate by set/way instructions to perform a data cache clean and invalidate by
        /// set/way:
        ///
        /// 0 This control has no effect on the operation of data cache invalidate by set/way
        ///   instructions.
        ///
        /// 1 Data cache invalidate by set/way instructions perform a data cache clean and
        ///   invalidate by set/way.
        ///
        /// When the value of this bit is 1:
        ///
        /// AArch32: DCISW performs the same invalidation as a DCCISW instruction.
        ///
        /// AArch64: DC ISW performs the same invalidation as a DC CISW instruction.
        ///
        /// This bit can be implemented as RES 1.
        ///
        /// In an implementation that includes EL3, when the value of SCR_EL3.NS is 0 the PE behaves
        /// as if this field is 0 for all purposes other than a direct read or write access of
        /// HCR_EL2.
        ///
        /// When HCR_EL2.TGE is 1, the PE ignores the value of this field for all purposes other
        /// than a direct read of this field.
        SWIO OFFSET(1) NUMBITS(1) [],

        /// Virtualization enable. Enables stage 2 address translation for the EL1&0 translation regime,
        /// when EL2 is enabled in the current Security state. The possible values are:
        ///
        /// 0    EL1&0 stage 2 address translation disabled.
        /// 1    EL1&0 stage 2 address translation enabled.
        ///
        /// When the value of this bit is 1, data cache invalidate instructions executed at EL1 perform
        /// a data cache clean and invalidate. For the invalidate by set/way instruction this behavior
        /// applies regardless of the value of the HCR_EL2.SWIO bit.
        ///
        /// This bit is permitted to be cached in a TLB.
        ///
        /// When ARMv8.1-VHE is implemented, and the value of HCR_EL2.{E2H, TGE} is {1, 1}, this
        /// field behaves as 0 for all purposes other than a direct read of the value of this field.
        VM OFFSET(0) NUMBITS(1) [
            Disable = 0,
            Enable = 1
        ]
    ]
}

pub struct Reg;

impl Readable for Reg {
    type T = u64;
    type R = HCR_EL2::Register;

    sys_coproc_read_raw!(u64, "HCR_EL2", "x");
}

impl Writeable for Reg {
    type T = u64;
    type R = HCR_EL2::Register;

    sys_coproc_write_raw!(u64, "HCR_EL2", "x");
}

pub const HCR_EL2: Reg = Reg {};
