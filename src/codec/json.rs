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
/// use typewheel::{Component, TextColor, codec::{ComponentCodec, JsonCodec}};
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
/// let codec = JsonCodec;
/// assert_eq!(codec.serialize(&component).unwrap(), JSON);
/// assert_eq!(codec.deserialize(JSON).unwrap(), component);
/// assert_eq!(codec.deserialize("\"hello world\"").unwrap(), Component::text("hello world"));
/// assert_eq!(codec.serialize(&Component::text("hello world")).unwrap(), "\"hello world\"");
/// ```
#[derive(Clone, Copy)] // Clone has no meaning here but Copy does
pub struct JsonCodec;

impl ComponentCodec for JsonCodec {
	type DecodeInput = String;
	type EncodeOutput = Result<String, serde_json::Error>;
	type DecodeOutput = Result<Component, serde_json::Error>;

	#[inline(always)]
	fn serialize(self, component: &Component) -> Self::EncodeOutput {
		serde_json::to_string(component)
	}

	#[inline(always)]
	fn deserialize<'a>(self, value: impl Into<Self::DecodeInput>) -> Self::DecodeOutput {
		serde_json::from_str(&value.into())
	}
}
