//! Typewheel is a library for creating and serializing [text components][wiki].
//!
//! [Component]s can be modified using both setters and the builder pattern, where each field has a
//! method for setting its value directly, and for producing a new [Component] with the new value.
//!
//! # Creating Components
//! ```
//! # use typewheel::{Component, TextColor};
//! #
//! // Setter pattern: setters pass back a reference for chaining.
//! let mut inner = Component::text("world");
//! inner.bold(true).color(0xAA7FFF);
//!
//! assert_eq!(inner.style.bold, Some(true));
//! assert_eq!(inner.style.color, Some(TextColor::Hex(0xAA7FFF)));
//!
//! // Builder pattern: builders accept ownership of `self` and return an owned value back.
//! let component = Component::text("Hello, ")
//!     .with_color(TextColor::Gray)
//!     .with_extra([inner, "!".into()]);
//!
//! assert_eq!(component.style.color, Some(TextColor::Gray));
//! assert!(!component.extra.is_empty());
//! ```
//!
//! [wiki]: https://wiki.vg/Chat

#![cfg_attr(ci, deny(missing_docs))]
#![cfg_attr(not(ci), warn(missing_docs))]

mod codec;
mod event;
mod iter;
mod style;

#[cfg(feature = "json")]
pub use self::codec::json::JsonComponentCodec;
pub use self::{
	codec::PlainTextComponentCodec,
	event::{ClickEvent, HoverEvent},
	iter::IterOrder,
	style::{Style, TextColor},
};
use std::collections::VecDeque;

use crate::iter::ComponentIterator;
use serde::{Deserialize, Serialize};
use std::mem;

/// A struct modeling a text component. Components are all styled, and hold children. When displayed,
/// components are written to the output depth-first.
///
/// # Component Types
/// There are several basic component types: text, translation, and keybind. Text components contain
/// a raw string which is rendered for their contents. Translation components contains a resource
/// key that is used to display a localized message to any recipients. Keybind components also hold
/// a resource key which is used to display the key a player has bound for a specific action.
///
/// TODO: other component types
/// -- why aren't these implemented? -> they require some sort of rendering system
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(from = "String")]
#[non_exhaustive]
pub struct Component {
	/// This component's style. The style contains common properties to how a component is rendered,
	/// including font, color, and event behavior.
	#[serde(flatten)]
	pub style: Style,

	/// The component's body. The body holds the actual content of this component that varies based
	/// on its type.
	#[serde(flatten)]
	pub body: ComponentBody,

	/// The component's children. When being written out, components are read depth-first.
	#[serde(skip_serializing_if = "Vec::is_empty")]
	pub extra: Vec<Component>,
}

// An internal macro for declaring builder, setter, and clear methods for each style flag.
// This macro uses the format `$field: $field_type { set: $set_ident, build: $build_ident,
// clear: $clear_ident}`, where each method accepts attributes for docs.
macro_rules! declare_style_methods {
	($(
		$field: ident: $ty: ty {
			$(#[$set_attr: meta])*
			set: $setter: ident,

			$(#[$build_attr: meta])*
			build: $builder: ident,

			$(#[$clear_attr: meta])*
			clear: $clearer: ident $(,)*
		}
	),+ $(,)?) => {
		$(
			#[doc = concat!(
				"Sets this component's `", stringify!($field), "` property to the provided value."
			)]
			/// This setter can be chained with other setter calls, as it returns a mutable
			/// reference to `self`.
			///
			/// # Examples
			$(#[$set_attr])*
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
			$(#[$build_attr])*
			#[must_use]
			pub fn $builder(mut self, state: impl Into<$ty>) -> Self {
				self.$setter(state);
				self
			}

			#[doc = concat!("Clears this component's `", stringify!($field), "` property.")]
			///
			/// # Examples
			$(#[$clear_attr])*
			pub fn $clearer(&mut self) {
				self.style.$field = None;
			}
		)+
	};
}

impl Component {
	// <editor-fold desc="> Factory functions">

	/// Creates a new text component with no styling and no children.
	pub fn text(text: impl Into<String>) -> Self {
		Self {
			style: Style::default(),
			extra: Vec::new(),
			body: ComponentBody::Text(text.into()),
		}
	}

	/// Creates a new translation component. Translation components contain a key which is used to
	/// look up the string to template into, and an array of components that are interpolated into
	/// the key's argument slots.
	pub fn translate(
		key: impl Into<String>,
		with: impl IntoIterator<Item = impl Into<Component>>,
	) -> Self {
		Self {
			style: Style::default(),
			extra: Vec::new(),
			body: ComponentBody::Translation {
				key: key.into(),
				with: with.into_iter().map(Into::into).collect(),
			},
		}
	}

	/// Creates a new keybind component. The keybind is an identifier to a button binding, and the
	/// client will show the button the action is bound to when rendered.
	pub fn keybind(key: impl Into<String>) -> Self {
		Self {
			style: Style::default(),
			extra: Vec::new(),
			body: ComponentBody::Keybind(key.into()),
		}
	}
	// </editor-fold>

	declare_style_methods! {
		bold: bool {
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

		italic: bool {
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

		underlined: bool {
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

		strikethrough: bool {
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

		obfuscated: bool {
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

		font: String {
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

		color: TextColor {
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

		insertion: String {
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

		click_event: ClickEvent {
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

		hover_event: HoverEvent {
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

	/// Appends additional children to this component. This method expects any type that can provide
	/// an iterator (via [IntoIterator]) with its items implementing [`Into<Component>`](Into).
	///
	/// Ownership of the extra components is claimed to prevent circular references.
	///
	/// # Examples
	/// ```
	/// # use typewheel::{Component, TextColor};
	/// let mut component = Component::text("hello ")
	///     .with_color(TextColor::Gray);
	///
	/// let extra = Component::text("world")
	///     .with_color(TextColor::Blue);
	///
	/// component.append([extra.clone()]);
	///
	/// assert_eq!(component.extra.first(), Some(extra));
	/// ```
	pub fn append(&mut self, extra: impl IntoIterator<Item = impl Into<Component>>) {
		self.extra.extend(extra.into_iter().map(Into::into))
	}

	/// Similar to [Self::append], this method appends additional children to this component. This
	/// is the builder equivalent of the previously mentioned method, taking ownership of `self`
	/// and then passing it back in the return value.
	///
	/// # Examples
	/// ```
	/// # use typewheel::{Component, TextColor};
	/// let extra = Component::text("world")
	///     .with_color(TextColor::Blue);
	///
	/// let component = Component::text("hello ")
	///     .with_color(TextColor::Gray)
	///     .with_extra([extra.clone()]);
	///
	/// assert_eq!(component.extra.first(), Some(extra));
	/// ```
	#[must_use]
	pub fn with_extra(mut self, extra: impl IntoIterator<Item = impl Into<Component>>) -> Self {
		self.append(extra);
		self
	}

	/// Clears this component's children.
	///
	/// # Examples
	/// ```
	/// # use typewheel::Component;
	/// let mut component = Component::text("hello ")
	///     .with_extra(["world"]);
	///
	/// component.clear_extra();
	/// assert!(component.extra.is_empty());
	/// ```
	pub fn clear_extra(&mut self) {
		self.extra.clear();
	}

	/// Clears this component's children, returning them in the process.
	///
	/// # Examples
	/// ```
	/// # use typewheel::Component;
	/// let mut component = Component::text("hello ")
	///    .with_extra(["world"]);
	///
	/// let removed = component.clear_extra();
	/// assert_eq!(removed.first(), Some(Component::text("world")));
	/// ```
	pub fn take_extra(&mut self) -> Vec<Self> {
		let mut target = Vec::new();
		mem::swap(&mut self.extra, &mut target);
		target
	}

	pub fn iter(&self, order: IterOrder) -> impl Iterator<Item = &Component> {
		ComponentIterator {
			queue: VecDeque::from([self]),
			order,
		}
	}

	// Getters

	/// Gets the string contents of this component, excluding that of its children. If its body is
	/// not [ComponentBody::Text], this method will return [None]. To get the full contents of a
	/// component, use the [plain text codec][PlainTextComponentCodec].
	///
	/// # Examples
	/// ```
	/// use typewheel::Component;
	/// assert_eq!(Component::text("!").content(), Some("!"));
	/// assert_eq!(Component::keybind("jump").content(), None);
	/// ```
	pub fn shallow_content(&self) -> Option<&str> {
		match self.body {
			ComponentBody::Text(ref text) => Some(text),
			_ => None,
		}
	}
}

/// Represents the inner contents of a component. Different component types have different bodies,
/// with each variant containing its own specialized fields.
///
/// This enum should not be manually constructed. Use the factory functions in [Component] instead.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
#[serde(rename_all = "snake_case")]
pub enum ComponentBody {
	/// A plain text component body. Text components have a raw content string and no other special
	/// state.
	///
	/// # Usage
	/// To create a new text component, use [Component::text(text)].
	Text(String),

	/// A keybind component body. Keybind components hold a keybind identifier that is rendered as
	/// their bound key by he client.
	///
	/// # Usage
	/// To create a new keybind component, use [Component::keybind(key)].
	Keybind(String),

	/// A translation component body. Translation components have a translation key, and an array of
	/// items that are interpolated into the translated message. Expected parameters vary based on
	/// the translation key. The inner components can also be translation components.
	///
	/// # Usage
	/// To create a new translation component, use [Component::translate()].
	#[serde(untagged)]
	Translation {
		/// The translation key identifier.
		#[serde(rename = "translate")]
		key: String,

		/// The interpolated fragments. Translations define arguments using the `%s` syntax used by
		/// Java string formatters, and these fragments are inserted wherever there is a format tag.
		#[serde(skip_serializing_if = "Vec::is_empty")]
		with: Vec<Component>,
	},
}

impl From<String> for Component {
	fn from(value: String) -> Self {
		Self::text(value)
	}
}

impl From<&str> for Component {
	fn from(value: &str) -> Self {
		Self::text(value.to_string())
	}
}

// TODO: proper tests
#[cfg(test)]
mod tests {
	use super::{Component, TextColor};

	#[test]
	fn serial() {
		let test = Component::text("Daedalus")
			.with_bold(true)
			.with_color(0xff0049)
			.with_hover_event(Component::text("Player ID 427").with_color(TextColor::Gray))
			.with_extra([
				Component::text(" has joined for the first time.").with_color(TextColor::Gray)
			])
			.with_extra(Some("h"));

		println!("{}", serde_json::to_string_pretty(&test).unwrap());
	}
}
