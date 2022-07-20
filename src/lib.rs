#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]
#![allow(
    // Allowed to avoid breaking changes.
    clippy::module_name_repetitions,
    clippy::struct_excessive_bools,
    clippy::unused_self,
    // Allowed as they are too pedantic.
    clippy::cast_possible_truncation,
    clippy::unreadable_literal,
    clippy::cast_possible_wrap,
    clippy::wildcard_imports,
    clippy::cast_sign_loss,
    clippy::too_many_lines,
    clippy::doc_markdown,
    clippy::cast_lossless,
    clippy::must_use_candidate
)]
#![deny(
    rust_2018_idioms,
    clippy::unwrap_used,
    clippy::non_ascii_literal,
    clippy::unused_async,
    unused
)]

#[macro_use]
extern crate napi_derive;

/// Returns "world".
#[napi]
pub fn hello() -> String {
    String::from("world")
}

#[cfg(test)]
mod test {
    #[test]
    fn hello() {
        let result = super::hello();

        assert_eq!("world", &result);
    }
}
