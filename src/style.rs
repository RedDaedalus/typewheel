use crate::event::{ClickEvent, HoverEvent};
use serde::{Deserialize, Serialize};
use std::mem;

/// This is quite a hefty macro, so let's break down what it does:
///
/// This macro is responsible for generating the [Style] struct, its fields, and accessors to its
/// values in [crate::Component]. This exists because there is a ton of repeated skeleton for each
/// style field that can be generated automatically. While the details may be a bit obscured, the
/// actual implementation becomes easier to read.
///
/// The first part of the macro accepts a general struct definition:
/// ```no_run
/// /// ... docs/attributes
/// pub struct Name;
/// ```
/// This is the skeleton that receives all of the fields and methods. It uses this definition when
/// generating a proper struct.
///
/// > Note that some attributes are applied by default -- notably [Serialize], [Deserialize], and a
///few other traits are automatically derived.
///
/// The rest of the body of this macro is field definitions. Fields are defined like so:
///```not_rust
/// name<T> {
///    set: <setter_name>,
///    build: <builder_name>,
///    clear: <clearer_name>
///}
///```
/// Note that the actual type of the field is `Option<T>`, not `T`.
///
/// The `set`, `build`, and `clear` fields define the names for their respective methods that are
/// implemented on [crate::Component]:
/// * `<setter_name>(&mut self, state: impl Into<T>) -> &mut Self`
/// * `<builder_name>(mut self, state: impl Into<T>) -> Self`
/// * `<clearer_name>(&mut self) -> &mut Self`
///  
/// Each of these fields should have accompanying docs, but there is a doc skeleton that is attached
/// to each method as well containing generic information. The provided docs should only have tests
/// and any extra information that is not added by default.
#[doc(hidden)]
macro_rules! style_fields {
	(
		$(#[$struct_meta: meta])*
		$vis: vis struct $name: ident;

		$(
			$(#[$field_meta: meta])*
			$field: ident< $ty: ty > {
				$(#[$set_meta: meta])*
				set: $setter: ident,
				$(#[$build_meta: meta])*
				build: $builder: ident,
				$(#[$clear_meta: meta])*
				clear: $clearer: ident
			}
		),+ $(,)?
	) => {
		$(#[$struct_meta])*
		// "intrinsic" traits -- expected to exist by this macro.
		#[derive(serde::Serialize, serde::Deserialize, Clone)]
		$vis struct $name {
			$(
				#[serde(skip_serializing_if = "Option::is_none")]
				$(#[$field_meta])*
				$vis $field: Option<$ty>
			),+
		}

		impl $name {
			/// A blank style. All of this style's fields are set to [None]. This is the [default]
			/// [Default::default()] style.
			pub const BLANK: Self = Self {
				$($field: None),+
			};

			$(
			#[doc = concat!("Creates a new style with only the `", stringify!($field), "` field set.")]
			/// All other fields are initialized to [None].
			pub fn $field(state: impl Into<$ty>) -> Self {
				Self {
					$field: Some(state.into()),
					..Default::default()
				}
			}
			)+

			/// Checks if this style is blank, or in other words, that all of its fields are [None].
			///
			/// # Examples
			/// ```
			/// # use typewheel::Style;
			/// assert!(Style::default().is_blank());
			/// assert!(!Style {
			///     bold: Some(true),
			///     ..Default::default()
			/// }.is_blank());
			/// ```
			pub const fn is_blank(&self) -> bool {
				$(self.$field.is_none() )&&+
			}

			/// Merges two styles together, with the style parameter taking precedence over `self`.
			/// This is designed to mimic the way styles are inherited when rendering a component;
			/// a node's effective style is the result of merging the styles of all of its parent
			/// components and its own.
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
			pub fn merge(&mut self, src: &Self) {
				$(
					if let Some(v) = &src.$field {
						self.$field = Some(v.clone());
					}
				)+
			}
		}

		impl std::default::Default for $name {
			#[inline]
			fn default() -> Self {
				Self::BLANK
			}
		}

		impl std::fmt::Debug for $name {
			fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
				if self.is_blank() {
					return f.write_str("Style{<blank>}");
				}

				let mut readout = f.debug_struct(stringify!($name));

				$(
					if let Some(v) = &self.$field {
						readout.field(stringify!($field), v);
					}
				)+

				readout.finish()
    		}
		}

		impl crate::Component {
			$(
				#[doc = concat!(
					"Sets this component's `", stringify!($field), "` property to the provided value."
				)]
				/// This setter can be chained with other setter calls, as it returns a mutable
				/// reference to `self`.
				///
				/// # Examples
				$(#[$set_meta])*
				pub fn $setter(&mut self, state: impl Into<$ty>) -> &mut Self {
					self.style.$field = Some(state.into());
					self
				}

				#[doc = concat!(
					"A builder method for setting this component's `", stringify!($field), "` property.",
					" This method assumes ownership of `self`, and passes it back when it returns."
				)]
				///
				/// # Examples
				///
				$(#[$build_meta])*
				pub fn $builder(mut self, state: impl Into<$ty>) -> Self {
					self.$setter(state);
					self
				}

				$(#[$clear_meta])*
				pub fn $clearer(&mut self) -> &mut Self {
					self.style.$field = None;
					self
				}
			)+
		}
	};
}

style_fields! {
	/// Represents a component's style. The style contains all aspects of a component that define
	/// how it is rendered and are not part of its content. Every component type supports every
	/// style property.
	///
	/// Every field in this struct is optional. If a field is set to [None], its value is inherited
	/// from its parent.
	///
	/// # Exhaustiveness
	/// It is important to note that, while not marked as such, this struct is **non exhaustive**.
	/// New style fields my be added if Mojang decides to extend components at a later date. This
	/// struct unfortunately cannot be marked as such because the struct update syntax is forbidden
	/// on non-exhaustive structs. To respect the semver implications of this, new fields will incur
	/// a minor version bump.
	///
	#[derive(PartialEq, Eq)]
	pub struct Style;

	/// Controls whether this component and its children render in bold. Defaults to `false` if
	/// unspecified.
	bold<bool> {
		/// ```
		/// # use typewheel::Component;
		/// let mut component = Component::text("hello world");
		/// component.bold(true);
		///
		/// assert_eq!(component.style.bold, Some(true));
		/// ```
		set: bold,

		/// ```
		/// # use typewheel::Component;
		/// let mut component = Component::text("hello world")
		///     .with_bold(true);
		///
		/// assert_eq!(component.style.bold, Some(true));
		build: with_bold,

		/// ```
		/// # use typewheel::Component;
		/// let mut component = Component::text("hello world")
		///     .with_bold(true);
		///
		/// assert_eq!(component.style.bold, Some(true));
		/// component.clear_bold();
		/// assert_eq!(component.style.bold, None);
		clear: clear_bold
	},

	/// Controls whether this component and its children render as italic. Defaults to `false` if
	/// unspecified.
	italic<bool> {
		/// ```
		/// # use typewheel::Component;
		/// let mut component = Component::text("hello world");
		/// component.italic(true);
		///
		/// assert_eq!(component.style.italic, Some(true));
		/// ```
		set: italic,

		/// ```
		/// # use typewheel::Component;
		/// let component = Component::text("hello world")
		///     .with_italic(true);
		///
		/// assert_eq!(component.style.italic, Some(true));
		/// ```
		build: with_italic,

		/// ```
		/// # use typewheel::Component;
		/// let mut component = Component::text("hello world")
		///     .with_italic(true);
		///
		/// assert_eq!(component.style.italic, Some(true));
		/// component.clear_italic();
		/// assert_eq!(component.style.italic, None);
		/// ```
		clear: clear_italic
	},

	/// Controls whether this component and its children are underlined. Defaults to `false` if
	/// unspecified.
	underlined<bool> {
		/// ```
		/// # use typewheel::Component;
		/// let mut component = Component::text("hello world");
		/// component.underlined(true);
		///
		/// assert_eq!(component.style.underlined, Some(true));
		/// ```
		set: underlined,

		/// ```
		/// # use typewheel::Component;
		/// let component = Component::text("hello world")
		///     .with_underlined(true);
		///
		/// assert_eq!(component.style.underlined, Some(true));
		/// ```
		build: with_underlined,

		/// ```
		/// # use typewheel::Component;
		/// let mut component = Component::text("hello world")
		///     .with_underlined(true);
		///
		/// assert_eq!(component.style.underlined, Some(true));
		/// component.clear_underlined();
		/// assert_eq!(component.style.underlined, None);
		/// ```
		clear: clear_underlined
	},

	/// Controls whether this component and its children render with strikethrough. Defaults to
	/// `false` if unspecified.
	strikethrough<bool> {
		/// ```
		/// # use typewheel::Component;
		/// let mut component = Component::text("hello world");
		/// component.strikethrough(true);
		///
		/// assert_eq!(component.style.strikethrough, Some(true));
		/// ```
			set: strikethrough,

		/// ```
		/// # use typewheel::Component;
		/// let component = Component::text("hello world")
		///     .with_strikethrough(true);
		///
		/// assert_eq!(component.style.strikethrough, Some(true));
		/// ```
		build: with_strikethrough,

		/// ```
		/// # use typewheel::Component;
		/// let mut component = Component::text("hello world")
		///     .with_strikethrough(true);
		///
		/// assert_eq!(component.style.strikethrough, Some(true));
		/// component.clear_strikethrough();
		/// assert_eq!(component.style.strikethrough, None);
		/// ```
		clear: clear_strikethrough
	},

	/// Controls whether this component and its children render as obfuscated. Defaults to `false`
	/// if unspecified.
	///
	/// Note that obfuscated text renders as random characters to the user, but the actual contents
	/// of the component are visible to anyone who checks their client logs.
	obfuscated<bool> {
		/// ```
		/// # use typewheel::Component;
		/// let mut component = Component::text("hello world");
		/// component.obfuscated(true);
		///
		/// assert_eq!(component.style.obfuscated, Some(true));
		/// ```
		set: obfuscated,

		/// ```
		/// # use typewheel::Component;
		/// let component = Component::text("hello world")
		///     .with_obfuscated(true);
		///
		/// assert_eq!(component.style.obfuscated, Some(true));
		/// ```
		build: with_obfuscated,

		/// ```
		/// # use typewheel::Component;
		/// let mut component = Component::text("hello world")
		///     .with_obfuscated(true);
		///
		/// assert_eq!(component.style.obfuscated, Some(true));
		/// component.clear_obfuscated();
		/// assert_eq!(component.style.obfuscated, None);
		/// ```
		clear: clear_obfuscated
	},

	/// Controls the font of this component and its children. Defaults to `minecraft:default` if
	/// unspecified.
	font<String> {
		/// ```
		/// # use typewheel::Component;
		/// let mut component = Component::text("hello world");
		/// component.font("minecraft:uniform");
		///
		/// assert_eq!(component.style.font, Some("minecraft:uniform".to_string()));
		/// ```
		set: font,

		/// ```
		/// # use typewheel::Component;
		/// let component = Component::text("hello world")
		///     .with_font("minecraft:uniform");
		///
		/// assert_eq!(component.style.font, Some("minecraft:uniform".to_string()));
		/// ```
		build: with_font,

		/// ```
		/// # use typewheel::Component;
		/// let mut component = Component::text("hello world")
		///     .with_font("minecraft:uniform");
		///
		/// assert_eq!(component.style.font, Some("minecraft:uniform".to_string()));
		/// component.clear_font();
		/// assert_eq!(component.style.font, None);
		/// ```
		clear: clear_font

	},

	/// Controls the color of this component and its children. If unspecified, the text renders as
	/// white, although resource packs can change the base text color (note that [TextColor::White]
	/// will always render as pure white, regardless of resource packs).
	color<TextColor> {
		/// ```
		/// # use typewheel::{Component, TextColor};
		/// let mut component = Component::text("hello world");
		/// component.color(TextColor::Blue);
		///
		/// assert_eq!(component.style.color, Some(TextColor::Blue));
		/// ```
		set: color,

		/// ```
		/// # use typewheel::{Component, TextColor};
		/// let component = Component::text("hello world")
		///     .with_color(TextColor::Blue);
		///
		/// assert_eq!(component.style.color, Some(TextColor::Blue));
		/// ```
		build: with_color,

		/// ```
		/// # use typewheel::{Component, TextColor};
		/// let mut component = Component::text("hello world")
		///     .with_color(TextColor::Blue);
		///
		/// assert_eq!(component.style.color, Some(TextColor::Blue));
		/// component.clear_color();
		/// assert_eq!(component.style.color, None);
		/// ```
		clear: clear_color
	},

	/// Controls the insertion for this component that is inserted into chat when shift-clicked.
	/// Defaults to [None] if unspecified.
	insertion<String> {
		/// ```
		/// # use typewheel::Component;
		/// let mut component = Component::text("hello world");
		/// component.insertion("insertion");
		///
		/// assert_eq!(component.style.insertion, Some("insertion".to_string()));
		/// ```
		set: insertion,

		/// ```
		/// # use typewheel::Component;
		/// let component = Component::text("hello world")
		///     .with_insertion("insertion");
		///
		/// assert_eq!(component.style.insertion, Some("insertion".to_string()));
		/// ```
		build: with_insertion,

		/// ```
		/// # use typewheel::Component;
		/// let mut component = Component::text("hello world")
		///     .with_insertion("insertion");
		///
		/// assert_eq!(component.style.insertion, Some("insertion".to_string()));
		/// component.clear_insertion();
		/// assert_eq!(component.style.insertion, None);
		/// ```
		clear: clear_insertion
	},

	/// Controls the action that occurs when the user clicks this component or its children. See the
	/// docs on the [ClickEvent] enum for details on specific events. Defaults to [None] if
	/// unspecified.
	click_event<ClickEvent> {
		/// ```
		/// # use typewheel::{Component, ClickEvent};
		/// const COMMAND: &str = "/help";
		///
		/// let mut component = Component::text("click me");
		/// component.click_event(ClickEvent::run_command(COMMAND));
		///
		/// assert_eq!(
		///     component.style.click_event,
		///	 	Some(ClickEvent::run_command(COMMAND))
		/// );
		/// ```
		set: click_event,

		/// ```
		/// # use typewheel::{Component, ClickEvent};
		/// const COMMAND: &str = "/help";
		///
		/// let component = Component::text("click me")
		///     .with_click_event(ClickEvent::run_command(COMMAND));
		///
		/// assert_eq!(
		///     component.style.click_event,
		///	 	Some(ClickEvent::run_command(COMMAND))
		/// );
		/// ```
		build: with_click_event,

		/// ```
		/// # use typewheel::{Component, ClickEvent};
		/// const COMMAND: &str = "/help";
		///
		/// let mut component = Component::text("click me")
		///     .with_click_event(ClickEvent::run_command(COMMAND));
		///
		/// assert!(matches!(component.style.click_event, Some(_)));
		/// component.clear_click_event();
		/// assert_eq!(component.style.click_event, None);
		/// ```
		clear: clear_click_event
	},

	/// Controls the value that is displayed when the user hovers over this component or its
	/// children. See the docs on the [HoverEvent] enum for details on specific events. Defaults to
	/// [None] if unspecified.
	hover_event<HoverEvent> {
		/// ```
		/// # use typewheel::{Component, HoverEvent};
		/// const HOVER_TEXT: &str = "hidden message";
		///
		/// let mut component = Component::text("hover me");
		/// component.hover_event(HoverEvent::show_text(HOVER_TEXT));
		///
		/// assert_eq!(
		///     component.style.hover_event,
		///	 	Some(HoverEvent::show_text(HOVER_TEXT))
		/// );
		/// ```
		set: hover_event,

		/// ```
		/// # use typewheel::{Component, HoverEvent};
		/// const HOVER_TEXT: &str = "hidden message";
		///
		/// let component = Component::text("hover me")
		///     .with_hover_event(HoverEvent::show_text(HOVER_TEXT));
		///
		/// assert_eq!(
		///     component.style.hover_event,
		///	 	Some(HoverEvent::show_text(HOVER_TEXT))
		/// );
		/// ```
		build: with_hover_event,

		/// ```
		/// # use typewheel::{Component, HoverEvent};
		/// const HOVER_TEXT: &str = "hidden message";
		///
		/// let mut component = Component::text("hover me")
		///     .with_hover_event(HoverEvent::show_text(HOVER_TEXT));
		///
		/// assert!(matches!(component.style.hover_event, Some(_)));
		/// component.clear_hover_event();
		/// assert_eq!(component.style.hover_event, None);
		/// ```
		clear: clear_hover_event
	},
}

impl Style {
	/// Clears this style, resetting all of its fields to [None]. When in the context of a tree,
	/// this indicates that a node will inherit all of its values from its parent.
	///
	/// # Examples
	/// ```
	/// # use typewheel::Style;
	/// #
	/// let mut style = Style {
	///     bold: Some(true),
	///     italic: Some(false),
	///     ..Default::default()
	/// };
	///
	/// style.clear();
	/// assert_eq!(style.bold, None);
	/// assert_eq!(style.italic, None);
	/// ```
	pub fn clear(&mut self) {
		mem::swap(self, &mut Default::default());
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

impl TextColor {
	pub(crate) const fn color_code(&self) -> char {
		match self {
			TextColor::Black => '0',
			TextColor::DarkBlue => '1',
			TextColor::DarkGreen => '2',
			TextColor::DarkAqua => '3',
			TextColor::DarkRed => '4',
			TextColor::DarkPurple => '5',
			TextColor::Gold => '6',
			TextColor::Gray => '7',
			TextColor::DarkGray => '8',
			TextColor::Blue => '9',
			TextColor::Green => 'a',
			TextColor::Aqua => 'b',
			TextColor::Red => 'c',
			TextColor::LightPurple => 'd',
			TextColor::Yellow => 'e',
			TextColor::White => 'f',
			TextColor::Hex(_) => '\0',
		}
	}

	pub(crate) fn from_color_code(code: char) -> Option<Self> {
		let color = match code {
			'0' => TextColor::Black,
			'1' => TextColor::DarkBlue,
			'2' => TextColor::DarkGreen,
			'3' => TextColor::DarkAqua,
			'4' => TextColor::DarkRed,
			'5' => TextColor::DarkPurple,
			'6' => TextColor::Gold,
			'7' => TextColor::Gray,
			'8' => TextColor::DarkGray,
			'9' => TextColor::Blue,
			'a' => TextColor::Green,
			'b' => TextColor::Aqua,
			'c' => TextColor::Red,
			'd' => TextColor::LightPurple,
			'e' => TextColor::Yellow,
			'f' => TextColor::White,
			_ => {
				return None;
			}
		};

		Some(color)
	}
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
