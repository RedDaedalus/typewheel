#![allow(unused)] // Rust doesn't consider items used if only referenced in tests

use typewheel::{Component, TextColor};

/// A common component reusable by multiple tests. Returns a deeply-nested component with the
/// following content:
///
/// ```monospace
/// a
/// |- b
/// |  |- c, d
/// |- e
///    |- f
///       |- g, h
/// ```
pub fn deeply_nested() -> Component {
	Component::text("a").with_extra([
		Component::text("b").with_extra(["c", "d"]),
		Component::text("e").with_extra([Component::text("f").with_extra(["g", "h"])]),
	])
}

pub fn styled_hello() -> Component {
	Component::text("hello ")
		.with_bold(true)
		.with_color(TextColor::Green)
		.with_extra([Component::text("world")
			.with_italic(true)
			.with_color(TextColor::Blue)])
}
