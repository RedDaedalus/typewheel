#![doc = include_str!("../README.md")]
#![cfg_attr(ci, deny(missing_docs))]
#![cfg_attr(not(ci), warn(missing_docs))]
#![cfg_attr(ci, warn(clippy::todo))]
// #![feature(impl_trait_in_assoc_type)] // TODO: use this feature for iterators to seal the type

pub mod codec;
mod event;
mod flatten;
mod iter;
mod serial;
mod style;

#[doc(hidden)]
pub use self::iter::ComponentIterator;
pub use self::{
	event::{ClickEvent, HoverEvent},
	iter::IterOrder,
	style::{Style, TextColor},
};
use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};
use std::mem;

/// A struct modeling a text component. Components are all styled, and hold children. When displayed,
/// components are written to the output depth-first.
///
/// # Component Types
/// | Type                                | Description                                                   |
/// | ----------------------------------- | ------------------------------------------------------------- |
/// | [Text][Component::text()]           | The simplest component type -- contains a raw string of text. |
/// | [Translate][Component::translate()] | Displays a piece of text in the client's language.            |
/// | [Keybind][Component::keybind()]     | Displays the bound button for a client action.                |
/// | [Score][Component::score()]         | Displays a scoreboard score.                                  |
/// For more information on each component type, visit the factory function documentation linked in
/// the table above.
///
/// ## Unsupported Types
/// The `selector` and `nbt` component types are both unsupported. This is because they cannot be
/// rendered by the client, and have to instead be replaced with [text][ComponentBody::Text]
/// components.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(from = "serial::SerialVessel", into = "serial::SerialVessel")]
#[non_exhaustive]
pub struct Component {
	/// This component's style. The style contains common properties to how a component is rendered,
	/// including font, color, and event behavior.
	pub style: Style,

	/// The component's body. The body holds the actual content of this component that varies based
	/// on its type.
	// NOTE: Ideally this would be before `style`, but the order matters for serde
	pub body: ComponentBody,

	/// The component's children. When being written out, components are read depth-first.
	pub extra: Vec<Component>,
}

impl Component {
	// <editor-fold desc="> Factory functions">

	/// Creates a new text component with no styling and no children.
	pub fn text(text: impl Into<String>) -> Self {
		Self {
			style: Style::default(),
			body: ComponentBody::Text(text.into()),
			extra: Vec::new(),
		}
	}

	/// Creates a new translation component. Translation components contain a key which is used to
	/// look up the string to template into, and an array of components that are interpolated into
	/// the key's argument slots.
	///
	/// A list of translation keys can be found in the lang files of the Vanilla resource pack.
	///
	/// # Usage
	/// The `with` parameter accepts any type that implements [IntoIterator] with any item that
	/// can implements [`Into<Component>`][Into]. When creating a component without any translation
	/// arguments, you can use the following syntax:
	/// ```no_run
	/// # use typewheel::Component;
	/// let component = Component::translate("key", None::<Component>.into_iter());
	/// ```
	pub fn translate(
		key: impl Into<String>,
		with: impl IntoIterator<Item = impl Into<Component>>,
	) -> Self {
		Self {
			style: Style::default(),
			body: ComponentBody::Translation {
				key: key.into(),
				with: with.into_iter().map(Into::into).collect(),
			},
			extra: Vec::new(),
		}
	}

	/// Creates a new keybind component. The keybind is an identifier to a button binding, and the
	/// client will show the button the action is bound to when rendered.
	///
	/// A list of keybind keys can be found in the client's `options.txt` file in the `.minecraft`
	/// directory.
	pub fn keybind(key: impl Into<String>) -> Self {
		Self {
			style: Style::default(),
			body: ComponentBody::Keybind(key.into()),
			extra: Vec::new(),
		}
	}

	/// Creates a new score component. The name field is a player name or UUID, the objective is an
	/// arbitrary scoreboard objective name, and the value is the resolved value from the server.
	///
	/// In the Vanilla server, when deserializing a component, the score value is automatically
	/// populated. In this library, it has to be set at creation time. If it is desired to populate
	/// scores automatically, an iterator can be used to find all score components and fill them in.
	/// To accomplish this, mutate the [ComponentBody::Score] `value` field.
	pub fn score(
		name: impl Into<String>,
		objective: impl Into<String>,
		value: impl Into<String>,
	) -> Self {
		Self {
			style: Style::default(),
			body: ComponentBody::Score {
				name: name.into(),
				objective: objective.into(),
				value: value.into(),
			},
			extra: Vec::new(),
		}
	}
	// </editor-fold>

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
	/// assert_eq!(component.extra.first(), Some(&extra));
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
	/// assert_eq!(component.extra.first(), Some(&extra));
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
	/// let removed = component.take_extra();
	/// assert_eq!(removed.first(), Some(&Component::text("world")));
	/// ```
	#[must_use = "use clear_extra() if you do not want to use this component's children"]
	pub fn take_extra(&mut self) -> Vec<Self> {
		let mut target = Vec::new();
		mem::swap(&mut self.extra, &mut target);
		target
	}

	// Getters

	/// Gets the string contents of this component, excluding that of its children. If its body is
	/// not [ComponentBody::Text], this method will return [None]. To get the full contents of a
	/// component, use the [plain text codec][codec::PlainTextComponentCodec].
	///
	/// # Examples
	/// ```
	/// use typewheel::Component;
	/// assert_eq!(Component::text("!").shallow_content(), Some("!"));
	/// assert_eq!(Component::keybind("key.jump").shallow_content(), None);
	/// ```
	pub fn shallow_content(&self) -> Option<&str> {
		match &self.body {
			ComponentBody::Text(text) => Some(text),
			_ => None,
		}
	}
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

impl TryInto<String> for Component {
	type Error = Self;

	fn try_into(self) -> Result<String, Self::Error> {
		match self.body {
			ComponentBody::Text(text) => Ok(text),
			_ => Err(self),
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
	/// To create a new text component, use [Component::text()].
	Text(String),

	/// A keybind component body. Keybind components hold a keybind identifier that is rendered as
	/// their bound key by he client.
	///
	/// # Usage
	/// To create a new keybind component, use [Component::keybind(key)].
	Keybind(String),

	/// Represents a scoreboard score component. When sent to the client, the value of the score is
	/// displayed.
	///
	/// # Usage
	/// To create a new score component, use [Component::score()].
	Score {
		/// The entry name -- either a player name or a UUID.
		name: String,

		/// The objective the score is coming from. This is not rendered in the client, but is used
		/// for resolving the score value.
		objective: String,

		/// The resolved score value. This must be sent to the client in order for it to be
		/// displayed properly. When a score is deserialized without a value, it is set to an empty
		/// string.
		#[serde(default)]
		value: String,
	},

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

impl Display for ComponentBody {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Text(contents) => write!(f, "{contents}")?,
			Self::Keybind(key) => write!(f, "[{key}]")?,
			Self::Score { value, .. } => write!(f, "{value}")?,
			Self::Translation { key, with: args } => {
				write!(f, "<{key}")?;
				for arg in args {
					write!(f, ":{}", arg.body)?;
				}
				write!(f, ">")?;
			}
		}

		Ok(())
	}
}
