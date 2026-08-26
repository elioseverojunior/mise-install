// SPDX-FileCopyrightText: 2026 Elio Severo Junior <elioseverojunior@gmail.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Criterion bench for `parse_tool_spec`.
//!
//! The input is the real shape of a `[tools]` table -- every backend the repo
//! actually pins -- so a regression shows up on the specs that get parsed,
//! rather than on one hand-picked happy path.

use criterion::{Criterion, criterion_group, criterion_main};
use mise_install::parse_tool_spec;
use std::hint::black_box;

const SPECS: &[&str] = &[
    "cargo:cargo-nextest@0.9.140",
    "aqua:rhysd/actionlint@1.7.12",
    "actionlint@1.7.12",
    "aqua:hk",
    "pipx:codecov-cli@11.3.1",
    "npm:@scope/pkg@1.2.3",
];

fn bench_parse_tool_spec(c: &mut Criterion) {
    c.bench_function("parse_tool_spec/table", |b| {
        b.iter(|| {
            for spec in black_box(SPECS) {
                black_box(parse_tool_spec(spec));
            }
        });
    });
}

criterion_group!(benches, bench_parse_tool_spec);
criterion_main!(benches);
