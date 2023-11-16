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
//! # use typewheel::codec::{ComponentCodec, JsonCodec};
//! # use typewheel::Component;
//! #
//! fn serialize(component: &Component) -> String {
//!     JsonCodec.serialize(component).unwrap()
//! }
//! ```

#[cfg(any(feature = "json", doc))]
mod json;
// mod legacy;
mod plain;

#[cfg(any(feature = "json", doc))]
pub use self::{json::JsonCodec, plain::PlainTextCodec};
// pub use self::{legacy::LegacyCodec, plain::PlainTextCodec};

use crate::Component;

/// A trait for encoding and decoding components to arbitrary formats.
///
/// # Implementation
/// Implementors of this trait are generally, but not always, zero-sized types. Codecs are required
/// to implement [Clone] and [Copy]. This allows them to be used repeatedly. Generally, codecs
/// should not contain any significant state.
pub trait ComponentCodec
where
	Self: Clone + Copy,
{
	/// The input type for deserialization. Generally a [String] or some collection of bytes.
	/// [Self::deserialize()] actually accepts [`impl Into<&Self::Input>`][Into], so that should
	///be taken into account when choosing an input type.
	type DecodeInput;

	/// The output type for serialization. If this codec is fallible, this should be a [Result]
	/// type.
	type EncodeOutput;

	/// The output type for deserialization. This should be one of [Component],
	/// [`Result<Component, _>`][Result], or [`Option<Component>`][Option].
	type DecodeOutput;

	/// Serializes a component, returning its encoded representation as [Self::EncodeOutput].
	fn serialize(self, component: &Component) -> Self::EncodeOutput;

	/// Deserializes a component from an arbitrary [input representation][Self::Input].
	fn deserialize(self, value: impl Into<Self::DecodeInput>) -> Self::DecodeOutput;
}
