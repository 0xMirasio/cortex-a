// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (c) 2018-2022 by the author(s)
//
// Author(s):
//   - Andre Richter <andre.o.richter@gmail.com>

//! Vector Base Address Register - EL1
//!
//! Holds the vector base address for any exception that is taken to EL1.

use tock_registers::interfaces::{Readable, Writeable};

pub struct Reg;

impl Readable for Reg {
    type T = u64;
    type R = ();

    sys_coproc_read_raw!(u64, "VBAR_EL2", "x");
}

impl Writeable for Reg {
    type T = u64;
    type R = ();

    sys_coproc_write_raw!(u64, "VBAR_EL2", "x");
}

pub const VBAR_EL2: Reg = Reg {};
