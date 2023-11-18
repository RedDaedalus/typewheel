use serde::{Deserialize, Serialize};
use std::mem;

use crate::{serial, Content, Style};

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
/// rendered by the client, and have to instead be replaced with [text][Content::Text] components.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(from = "serial::SerialVessel", into = "serial::SerialVessel")]
#[non_exhaustive]
pub struct Component {
	/// The component's content, containing the values that are actually outputted to the user.
	pub content: Content,

	/// This component's style. The style contains common properties to how a component is rendered,
	/// including font, color, and event behavior.
	pub style: Style,

	/// This component's children (or extra). When being rendered out, children are read from left
	/// to right.
	///
	/// This field should generally not be modified directly. Instead, use the [Self::append()] and
	/// [Self::clear_extra()] methods.
	pub extra: Vec<Component>,
}

impl Component {
	// <editor-fold desc="> Factory functions">

	/// Creates a new [Component] with the provided content.
	#[inline]
	pub const fn new(content: Content) -> Self {
		Self {
			content,
			style: Style::BLANK,
			extra: Vec::new(),
		}
	}

	/// Creates a new fully described [Component], computing its size from its children.
	pub(crate) fn create(content: Content, style: Style, extra: Vec<Component>) -> Self {
		Self {
			content,
			style,
			extra,
		}
	}

	pub(crate) fn create_flat(content: Content, style: Style) -> Self {
		Self {
			content,
			style,
			extra: Vec::new(),
		}
	}

	/// Creates a new text component with no styling and no children.
	#[inline]
	pub fn text(text: impl Into<String>) -> Self {
		Self::new(Content::Text(text.into()))
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
	/// ```rust,no_run
	/// # use typewheel::Component;
	/// let component = Component::translate("key", None::<Component>);
	/// ```
	#[inline]
	pub fn translate(
		key: impl Into<String>,
		with: impl IntoIterator<Item = impl Into<Component>>,
	) -> Self {
		Self::new(Content::Translation {
			key: key.into(),
			with: with.into_iter().map(Into::into).collect(),
		})
	}

	/// Creates a new keybind component. The keybind is an identifier to a button binding, and the
	/// client will show the button the action is bound to when rendered.
	///
	/// A list of keybind keys can be found in the client's `options.txt` file in the `.minecraft`
	/// directory.
	#[inline]
	pub fn keybind(key: impl Into<String>) -> Self {
		Self::new(Content::Keybind(key.into()))
	}

	/// Creates a new score component. The name field is a player name or UUID, the objective is an
	/// arbitrary scoreboard objective name, and the value is the resolved value from the server.
	///
	/// In the Vanilla server, when deserializing a component, the score value is automatically
	/// populated. In this library, it has to be set at creation time. If it is desired to populate
	/// scores automatically, an iterator can be used to find all score components and fill them in.
	/// To accomplish this, mutate the [Content::Score] `value` field.
	#[inline]
	pub fn score(
		name: impl Into<String>,
		objective: impl Into<String>,
		value: impl Into<String>,
	) -> Self {
		Self::new(Content::Score {
			name: name.into(),
			objective: objective.into(),
			value: value.into(),
		})
	}

	/// Creates an empty component with no content. Empty components can still contain styles and
	/// extras, but do not render anything themselves.
	///
	/// # Examples
	/// ```
	/// # use typewheel::Component;
	/// #
	/// let component = Component::empty();
	/// assert!(component.shallow_text().is_none());
	/// ```
	#[inline(always)]
	pub const fn empty() -> Self {
		Self::new(Content::Empty)
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
	/// assert_eq!(component.extra().first(), Some(&extra));
	/// ```
	pub fn append(&mut self, extra: impl IntoIterator<Item = impl Into<Component>>) {
		self.extra.extend(extra.into_iter().map(Into::into));
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
	/// assert_eq!(component.extra().first(), Some(&extra));
	/// ```
	#[must_use]
	#[inline]
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
	/// assert!(component.extra().is_empty());
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

	/// Gets the number of content nodes in this component. `content_size() >= tree_size()` is
	/// invariant.
	///
	/// The difference between the content and the tree size is that the content size includes the
	/// arguments of [translation][Content::Translation] components.
	pub fn content_size(&self) -> usize {
		0
	}

	/// Gets the text of this component, excluding that of its children. If its [content]
	/// [Self::content] is not [Content::Text], this method will return [None].
	///
	/// To get the full contents of a component, use the [plain text codec][plain]. To render the
	/// component content regardless of its type, use [Content::to_string].
	///
	/// # Examples
	/// ```
	/// use typewheel::Component;
	/// assert_eq!(Component::text("!").shallow_text(), Some("!"));
	/// assert_eq!(Component::keybind("key.jump").shallow_text(), None);
	/// ```
	///
	/// [plain]: codec::PlainTextComponentCodec
	pub fn shallow_text(&self) -> Option<&str> {
		match &self.content {
			Content::Text(text) => Some(text),
			_ => None,
		}
	}

	/// Flattens a component if its body is [Content::Empty] by taking its first child and merging
	/// it.
	///
	/// This might eventually get added as a public API, but it needs to be refined first.
	#[doc(hidden)]
	pub(crate) fn flattened(mut self) -> Self {
		let Content::Empty = self.content else {
			return self;
		};

		if self.extra.is_empty() {
			return self;
		}

		let first = self.extra.swap_remove(0);
		let mut style = self.style;
		style.merge(&first.style);

		self.extra.extend(first.extra);

		Self {
			content: first.content,
			style,
			extra: self.extra,
		}
	}
}

impl PartialEq<String> for Component {
	fn eq(&self, other: &String) -> bool {
		if let Some(text) = self.shallow_text() {
			text == other
		} else {
			false
		}
	}
}

impl PartialEq<&str> for Component {
	fn eq(&self, other: &&str) -> bool {
		if let Some(text) = self.shallow_text() {
			text == *other
		} else {
			false
		}
	}
}

impl Extend<Component> for Component {
	#[inline(always)]
	fn extend<T: IntoIterator<Item = Component>>(&mut self, iter: T) {
		self.append(iter);
	}
}

impl Default for Component {
	#[inline]
	fn default() -> Self {
		Self::empty()
	}
}

impl From<String> for Component {
	fn from(value: String) -> Self {
		Self::text(value)
	}
}

impl From<&str> for Component {
	fn from(value: &str) -> Self {
		Self::text(value.to_owned())
	}
}

impl TryInto<String> for Component {
	type Error = Self;

	fn try_into(self) -> Result<String, Self::Error> {
		match self.content {
			Content::Text(text) => Ok(text),
			_ => Err(self),
		}
	}
}
