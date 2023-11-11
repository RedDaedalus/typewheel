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

use crate::Component;

#[cfg(doc)]
use crate::ComponentBody;

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

	/// The output type for serialization. If this codec is fallible, this should be a [Result]
	/// type.
	type EncodeOutput;
	/// The output type for deserialization. This should be one of [Component],
	/// [`Result<Component, _>`][Result], or [`Option<Component>`][Option].
	type DecodeOutput;

	/// Serializes a component, returning its encoded representation as [Self::EncodeOutput].
	fn serialize(self, component: &Component) -> Self::EncodeOutput;

	/// Deserializes a component from an arbitrary [input representation][Self::Input].
	fn deserialize(self, value: impl Into<Self::Input>) -> Self::DecodeOutput;
}

#[cfg(any(feature = "json", doc))]
mod json {
	use super::ComponentCodec;
	use crate::Component;

	/// A component codec for serializing components to and from their JSON representation. This is
	/// locked behind the `json` crate feature.
	///
	/// Text components with no styling can be represented as strings. For example, `"hello world"`
	/// is deserialized into the equivalent of `Component::text("hello world")`.  
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
	/// # // Not a raw string so we can use a line break
	/// const JSON: &str = "{\"color\":\"blue\",\"text\":\"hello \",\"extra\":[\
	/// {\"bold\":true,\"color\":\"green\",\"text\":\"world\"}\
	/// ]}";
	///
	/// let codec = JsonComponentCodec;
	/// assert_eq!(codec.serialize(&component).unwrap(), JSON);
	/// assert_eq!(codec.deserialize(JSON).unwrap(), component);
	/// assert_eq!(codec.deserialize("\"hello world\"").unwrap(), Component::text("hello world"));
	/// assert_eq!(codec.serialize(&Component::text("hello world")).unwrap(), "\"hello world\"");
	/// ```
	#[derive(Clone, Copy)] // Clone has no meaning here but Copy does
	pub struct JsonComponentCodec;

	impl ComponentCodec for JsonComponentCodec {
		type Input = String;
		type EncodeOutput = Result<String, serde_json::Error>;
		type DecodeOutput = Result<Component, serde_json::Error>;

		#[inline(always)]
		fn serialize(self, component: &Component) -> Self::EncodeOutput {
			serde_json::to_string(component)
		}

		#[inline]
		fn deserialize<'a>(self, value: impl Into<Self::Input>) -> Self::DecodeOutput {
			serde_json::from_str(&value.into())
		}
	}
}

#[cfg(feature = "json")]
pub use json::JsonComponentCodec;

/// A component codec for encoding components as plain text without any formatting.
///
/// Components that are not [text components][ComponentBody::Text] will be encoded with a
/// best-effort approach:
/// * [ComponentBody::Keybind] components are encoded as `[{key}]`.
/// * [ComponentBody::Translation] components are encoded as `<{key}:{...args}>`.
/// * [ComponentBody::Score] components are encoded as their value.
///
/// These representations are **one-way**. [ComponentCodec::deserialize()] will only ever emit text.
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
/// assert_eq!(codec.serialize(&component), "hello world");
///
/// // Deserializing a component
/// assert_eq!(codec.deserialize("hello world"), Component::text("hello world"));
/// ```
#[derive(Clone, Copy)] // Clone has no meaning here but Copy does
pub struct PlainTextComponentCodec;

impl ComponentCodec for PlainTextComponentCodec {
	type Input = String;
	type EncodeOutput = String;
	type DecodeOutput = Component;

	fn serialize(self, component: &Component) -> Self::EncodeOutput {
		let mut output = String::new();

		for node in component {
			output += &node.body.to_string();
		}

		output
	}

	#[inline(always)]
	fn deserialize(self, value: impl Into<Self::Input>) -> Self::DecodeOutput {
		Component::text(value.into())
	}
}
