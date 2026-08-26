// SPDX-FileCopyrightText: 2026 Elio Severo Junior <elioseverojunior@gmail.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use mise_install::parse_tool_spec;

fn main() {
    let spec = parse_tool_spec("cargo:cargo-nextest@0.9.140");
    println!("{spec:?}");
}
