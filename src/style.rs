use crate::event::{ClickEvent, HoverEvent};
use serde::{Deserialize, Serialize};
use std::mem;

/// Represents a component's style. The style contains all aspects of a component that define how it
/// is rendered and are not part of its content. Every component type supports every style property.
///
/// Every field in this struct is optional. If a field is set to [None], its value is inherited from
/// its parent.
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq, Eq)]
pub struct Style {
	/// Controls whether this component and its children render in bold. Defaults to `false` if
	/// unspecified.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bold: Option<bool>,

	/// Controls whether this component and its children render as italic. Defaults to `false` if
	/// unspecified.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub italic: Option<bool>,

	/// Controls whether this component and its children are underlined. Defaults to `false` if
	/// unspecified.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub underlined: Option<bool>,

	/// Controls whether this component and its children render with strikethrough. Defaults to
	/// `false` if unspecified.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub strikethrough: Option<bool>,

	/// Controls whether this component and its children render as obfuscated. Defaults to `false`
	/// if unspecified.
	///
	/// Note that obfuscated text renders as random characters to the user, but the actual contents
	/// of the component are visible to anyone who checks their client logs.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub obfuscated: Option<bool>,

	/// Controls the font of this component and its children. Defaults to `minecraft:default` if
	/// unspecified.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub font: Option<String>,

	/// Controls the color of this component and its children. If unspecified, the text renders as
	/// white, although resource packs can change the base text color (note that [TextColor::White]
	/// will always render as pure white, regardless of resource packs).
	#[serde(skip_serializing_if = "Option::is_none")]
	pub color: Option<TextColor>,

	/// Controls the insertion for this component that is inserted into chat when shift-clicked.
	/// Defaults to [None] if unspecified.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub insertion: Option<String>,

	/// Controls the action that occurs when the user clicks this component or its children. See the
	/// docs on the [ClickEvent] enum for details on specific events. Defaults to [None] if
	/// unspecified.
	#[serde(skip_serializing_if = "Option::is_none", rename = "clickEvent")]
	pub click_event: Option<ClickEvent>,

	/// Controls the value that is displayed when the user hovers over this component or its
	/// children. See the docs on the [HoverEvent] enum for details on specific events. Defaults to
	/// [None] if unspecified.
	#[serde(skip_serializing_if = "Option::is_none", rename = "hoverEvent")]
	pub hover_event: Option<HoverEvent>,
}

impl Style {
	/// Clears this style, resetting all of its fields. If this component is the child of another,
	/// all of its values will be inherited from its parent.
	pub fn clear(&mut self) {
		mem::swap(self, &mut Self::default())
	}

	/// Merges two styles together, with the style parameter taking precedence over `self`. This is
	/// designed to mimic the way styles are inherited when rendering a component; a node's effective
	/// style is the result of merging the styles of all of its parent components and its own.
	///
	/// # Examples
	/// ```
	/// # use typewheel::Style;
	/// #
	/// let mut style = Style {
	///     bold: Some(true),
	///     underlined: Some(true),
	///     ..Default::default()
	/// };
	///
	/// style.merge(&Style {
	///     bold: Some(false),
	///     italic: Some(true),
	///     ..Default::default()
	/// });
	///
	/// assert_eq!(style.bold, Some(false));
	/// assert_eq!(style.italic, Some(true));
	/// assert_eq!(style.underlined, Some(true));
	/// ```
	pub fn merge(&mut self, other: &Self) {
		self.bold = other.bold.or(self.bold);
		self.italic = other.italic.or(self.italic);
		self.underlined = other.underlined.or(self.underlined);
		self.strikethrough = other.strikethrough.or(self.strikethrough);
		self.obfuscated = other.obfuscated.or(self.obfuscated);
		self.font = other.font.as_ref().or(self.font.as_ref()).cloned();
		self.color = other.color.or(self.color);
		self.insertion = other
			.insertion
			.as_ref()
			.or(self.insertion.as_ref())
			.cloned();
		self.click_event = other
			.click_event
			.as_ref()
			.or(self.click_event.as_ref())
			.cloned();
		self.hover_event = other
			.hover_event
			.as_ref()
			.or(self.hover_event.as_ref())
			.cloned();
	}
}

/// Models a text component's color. There are 16 named colors, and any hex color can be created via
/// the [TextColor::Hex] variant.
///
/// Other crates that provide color-like types should implement [`Into<TextColor>`](Into), as
/// Typewheel will accept any type that can be converted in its APIs.
///
/// # Serial Representation
/// Named text colors are serialized in `snake_case`. Each variant documents its own string
/// representation for clarity. Hex colors are serialized as hex strings with a leading `#`.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TextColor {
	/// Named color `black` (hex `#000`; code `§0`).
	Black,
	/// Named color `dark_blue` (hex `#00A`; code `§1`).
	DarkBlue,
	/// Named color `dark_green` (hex `#0A0`; code `§2`).
	DarkGreen,
	/// Named color `dark_aqua` (hex `#0AA`; code `§3`).
	DarkAqua,
	/// Named color `dark_red` (hex `#A00`; code `§4`).
	DarkRed,
	/// Named color `dark_purple` (hex `#A0A`; code `§5`).
	DarkPurple,
	/// Named color `gold` (hex `#FA0`; code `§6`).
	Gold,
	/// Named color `gray` (hex `#AAA`; code `§7`).
	Gray,
	/// Named color `dark_gray` (hex `#555`; code `§8`).
	DarkGray,

	/// Named color `blue` (hex `#55F`; code `§9`).
	Blue,
	/// Named color `green` (hex `#5F5`; code `§a`).
	Green,
	/// Named color `aqua` (hex `#5FF`; code `§b`).
	Aqua,
	/// Named color `red` (hex `#F55`; code `§c`).
	Red,
	/// Named color `light_purple` (hex `#F5F`; code `§d`).
	LightPurple,
	/// Named color `yellow` (hex `#FF5`; code `§e`).
	Yellow,
	/// Named color `white` (hex `#FFF`; code `§f`).
	White,

	/// An arbitrary RGB hex color. The most significant byte represents the red channel, the next
	/// represents the green, and the final represents the blue channel.
	///
	/// When serialized, hex colors are represented as an uppercase hex string:
	/// ```rust
	/// # use typewheel::TextColor;
	///
	/// let color = TextColor::Hex(0xAABBCC);
	/// assert_eq!(serde_json::to_string(&color).unwrap(), r##""#AABBCC""##);
	/// ```
	///
	/// The inner value must be within range `(0, 0xFFFFFF)` (inclusive). If not, serialization will
	/// yield an error.
	#[serde(untagged, with = "hex_serde")]
	Hex(u32),
}

impl From<u32> for TextColor {
	fn from(value: u32) -> Self {
		Self::Hex(value)
	}
}

mod hex_serde {
	use serde::{
		de::Error as _, ser::Error as _, Deserialize, Deserializer, Serialize, Serializer,
	};

	pub(crate) fn serialize<S: Serializer>(value: &u32, serializer: S) -> Result<S::Ok, S::Error> {
		if *value > 0xFFFFFF {
			return Err(S::Error::custom("hex value cannot exceed 0xFFFFFF"));
		}

		format!("#{value:06X}").serialize(serializer)
	}

	pub(crate) fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<u32, D::Error> {
		const ERROR_MESSAGE: &str = "expected hex literal";

		let hex_string = String::deserialize(deserializer)?;
		if !hex_string.starts_with('#') {
			return Err(D::Error::custom(ERROR_MESSAGE));
		}

		u32::from_str_radix(&hex_string[1..], 16).map_err(|_| D::Error::custom(ERROR_MESSAGE))
	}
}
