// SPDX-FileCopyrightText: 2026 Elio Severo Junior <elioseverojunior@gmail.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Parsing for mise tool specifiers such as `cargo:cargo-nextest@0.9.140`.

/// A tool specifier split into its three parts.
#[derive(Debug, PartialEq, Eq)]
pub struct ToolSpec<'a> {
    /// Backend prefix ahead of `:`, e.g. `cargo`. `None` for a registry tool.
    pub backend: Option<&'a str>,
    /// The tool name.
    pub name: &'a str,
    /// Version after `@`, e.g. `0.9.140`. `None` when the spec is unpinned.
    pub version: Option<&'a str>,
}

/// Split a mise tool specifier into backend, name and version.
///
/// The version is taken from the LAST `@`, so a scoped npm name keeps the `@`
/// that starts it: `npm:@scope/pkg@1.2.3` is `@scope/pkg` at `1.2.3`.
///
/// ```
/// # use mise_install::{parse_tool_spec, ToolSpec};
/// let spec = parse_tool_spec("cargo:cargo-nextest@0.9.140");
/// assert_eq!(spec.backend, Some("cargo"));
/// assert_eq!(spec.name, "cargo-nextest");
/// assert_eq!(spec.version, Some("0.9.140"));
/// ```
#[must_use]
pub fn parse_tool_spec(spec: &str) -> ToolSpec<'_> {
    let (rest, version) = match spec.rfind('@') {
        // `i > 0` keeps a leading `@` with the name rather than reading it as
        // an empty version.
        Some(i) if i > 0 => (&spec[..i], Some(&spec[i + 1..])),
        _ => (spec, None),
    };
    let (backend, name) = match rest.find(':') {
        Some(i) => (Some(&rest[..i]), &rest[i + 1..]),
        None => (None, rest),
    };
    ToolSpec {
        backend,
        name,
        version,
    }
}

#[cfg(test)]
mod tests {
    use super::{ToolSpec, parse_tool_spec};

    #[test]
    fn splits_backend_name_and_version() {
        assert_eq!(
            parse_tool_spec("cargo:cargo-nextest@0.9.140"),
            ToolSpec {
                backend: Some("cargo"),
                name: "cargo-nextest",
                version: Some("0.9.140")
            }
        );
    }

    #[test]
    fn registry_tool_has_no_backend() {
        assert_eq!(
            parse_tool_spec("actionlint@1.7.12"),
            ToolSpec {
                backend: None,
                name: "actionlint",
                version: Some("1.7.12")
            }
        );
    }

    #[test]
    fn unpinned_spec_has_no_version() {
        assert_eq!(
            parse_tool_spec("aqua:hk"),
            ToolSpec {
                backend: Some("aqua"),
                name: "hk",
                version: None
            }
        );
    }

    #[test]
    fn scoped_name_keeps_its_leading_at() {
        assert_eq!(
            parse_tool_spec("npm:@scope/pkg@1.2.3"),
            ToolSpec {
                backend: Some("npm"),
                name: "@scope/pkg",
                version: Some("1.2.3")
            }
        );
    }
}
