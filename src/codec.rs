//! A module containing structs for serializing and deserializing components into different forms.
//! Some codec types are lossy, such as [plain text][PlainTextComponentCodec], while others
//! preserve all data, such as the [JSON codec][JsonComponentCodec].
//!
//! Even lossless codec implementations may not create the same output when decoded and re-encoded
//! or vice-versa. This is seen, for example, in the legacy codec, where there are multiple valid
//! serial representations that produce the same output.
//!
//! # Examples
//! ```no_run
//! # use typewheel::codec::{ComponentCodec, JsonComponentCodec};
//! # use typewheel::Component;
//! #
//! fn serialize(component: &Component) -> String {
//!     JsonComponentCodec.serialize(component).unwrap()
//! }
//! ```

use crate::{Component, IterOrder};
use std::convert::Infallible;

/// A trait for encoding and decoding components to arbitrary formats.
///
/// # Implementation
/// Implementors of this trait are generally, but not always, zero-sized types. Codecs are allowed
/// to hold state, as seen in [`LegacyComponentCodec`][None], but for simple uses such as the
/// [plain][PlainTextComponentCodec] codec, they should be ZSTs.
pub trait ComponentCodec {
	/// The input type for deserialization. Generally a [String] or some collection of bytes.
	/// [Self::deserialize()] actually accepts [`impl Into<&Self::Input>`][Into], so that should
	///be taken into account when choosing an input type.
	type Input;

	/// The output type for serialization. This is generally the same or similar to [Self::Input].
	type Output;

	/// The error type covering failed de/serialization. This should be [Infallible] if the codec
	/// cannot error.
	type Error;

	/// Serializes a component, returning its encoded representation as [Self::Output].
	fn serialize(self, component: &Component) -> Result<Self::Output, Self::Error>;

	/// Deserializes a component from an arbitrary [input representation][Self::Input].
	fn deserialize(self, value: impl Into<Self::Input>) -> Result<Component, Self::Error>;
}

#[cfg(feature = "json")]
mod json {
	use super::ComponentCodec;
	use crate::Component;

	/// A component codec for serializing components to and from their JSON representation. This is
	/// locked behind the `json` crate feature.
	///
	/// This codec uses [serde_json] to serialize and deserialize components using the derived
	/// component serializer.
	///
	/// # Examples
	/// ```
	/// use typewheel::{Component, TextColor, codec::{ComponentCodec, JsonComponentCodec}};
	/// let component = Component::text("hello ")
	///     .with_color(TextColor::Blue)
	///     .with_extra([
	///         Component::text("world")
	///             .with_color(TextColor::Green)
	///             .with_bold(true)
	///     ]);
	///
	/// const JSON: &str = r#"{"color":"blue","text":"hello ","extra":[{"bold":true,"color":"green","text":"world"}]}"#;
	///
	/// let codec = JsonComponentCodec;
	/// assert_eq!(codec.serialize(&component).unwrap(), JSON);
	/// assert_eq!(codec.deserialize(JSON).unwrap(), component);
	/// ```
	#[derive(Clone, Copy)] // Clone has no meaning here but Copy does
	pub struct JsonComponentCodec;

	impl ComponentCodec for JsonComponentCodec {
		type Input = String;
		type Output = String;
		type Error = serde_json::Error;

		#[inline(always)]
		fn serialize(self, component: &Component) -> Result<Self::Output, Self::Error> {
			serde_json::to_string(component)
		}

		#[inline]
		fn deserialize<'a>(self, value: impl Into<Self::Input>) -> Result<Component, Self::Error> {
			serde_json::from_str(&value.into())
		}
	}
}

#[cfg(feature = "json")]
pub use json::JsonComponentCodec;

/// A component codec for encoding components as plain text without any formatting.
///
/// Components that are not [text components][crate::ComponentBody::Text] will not be included in
/// serialization output. When deserializing a component, the output is [Component::text()].
///
/// # Examples
/// ```
/// # use typewheel::codec::{ComponentCodec, PlainTextComponentCodec};
/// # use typewheel::Component;
/// #
/// let component = Component::text("hello ")
///     .with_bold(true)
///     .with_extra([Component::text("world").with_italic(true)]);
///
/// let codec = PlainTextComponentCodec;
/// // Serializing a component
/// assert_eq!(codec.serialize(&component).unwrap(), "hello world");
///
/// // Deserializing a component
/// assert_eq!(codec.deserialize("hello world").unwrap(), Component::text("hello world"));
/// ```
#[derive(Clone, Copy)] // Clone has no meaning here but Copy does
pub struct PlainTextComponentCodec;

impl ComponentCodec for PlainTextComponentCodec {
	type Input = String;
	type Output = String;
	type Error = Infallible;

	fn serialize(self, component: &Component) -> Result<Self::Output, Self::Error> {
		let mut output = String::new();

		for node in component.iter(IterOrder::DepthFirst) {
			if let Some(content) = node.shallow_content() {
				output += content;
			}
		}

		Ok(output)
	}

	#[inline(always)]
	fn deserialize(self, value: impl Into<Self::Input>) -> Result<Component, Self::Error> {
		Ok(Component::text(value.into()))
	}
}
