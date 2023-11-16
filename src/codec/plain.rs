use crate::codec::ComponentCodec;
use crate::Component;

/// A component codec for encoding components as plain text without any formatting.
///
/// Components that are not [text components][Content::Text] will be encoded with a
/// best-effort approach:
/// * [Content::Keybind] components are encoded as `[{key}]`.
/// * [Content::Translation] components are encoded as `<{key}:{...args}>`.
/// * [Content::Score] components are encoded as their value.
///
/// These representations are **one-way**. [ComponentCodec::deserialize()] will only ever emit text.
///
/// # Examples
/// ```
/// # use typewheel::codec::{ComponentCodec, PlainTextCodec};
/// # use typewheel::Component;
/// #
/// let component = Component::text("hello ")
///     .with_bold(true)
///     .with_extra([Component::text("world").with_italic(true)]);
///
/// let codec = PlainTextCodec;
/// // Serializing a component
/// assert_eq!(codec.serialize(&component), "hello world");
///
/// // Deserializing a component
/// assert_eq!(codec.deserialize("hello world"), Component::text("hello world"));
/// ```
#[derive(Clone, Copy)]
pub struct PlainTextCodec;

impl ComponentCodec for PlainTextCodec {
	type DecodeInput = String;
	type EncodeOutput = String;
	type DecodeOutput = Component;

	fn serialize(self, component: &Component) -> Self::EncodeOutput {
		let mut out = String::new();

		for node in component {
			if let Some(content) = node.shallow_text() {
				out += content;
			}
		}

		out
	}

	#[inline(always)]
	fn deserialize(self, value: impl Into<Self::DecodeInput>) -> Self::DecodeOutput {
		Component::text(value)
	}
}
