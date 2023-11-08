use crate::Component;
use serde::{Deserialize, Serialize};

/// Represents a click event. Click events are actions that are triggered when a user clicks on a
/// component in chat, on a sign, or in a book. Components with click events in other areas do not
/// have any effect.
///
/// Since this enum and its variants are non-exhaustive, they must be constructed through the factory
/// functions.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(tag = "action", content = "value", rename_all = "snake_case")]
#[non_exhaustive]
pub enum ClickEvent {
	/// Runs a command when clicked. This command is executed by the client as if the user typed it
	/// out in chat themselves.
	///
	/// The command must start with a leading `/`. If the command has any signed arguments (such as
	/// `/msg`), it cannot be ran by a component.
	#[non_exhaustive]
	RunCommand(String),

	/// Inserts text into the chat bar when clicked.
	#[non_exhaustive]
	SuggestCommand(String),

	/// Changes the page on a book. This is a signed type because the format allows for negative
	/// pages to be set, but they do not have any effect when sent.
	#[non_exhaustive]
	ChangePage(i32),

	/// Copies text to the user's system clipboard.
	#[serde(rename = "copy_to_clipboard")]
	#[non_exhaustive]
	Copy(String),
}

impl ClickEvent {
	/// Creates a [ClickEvent] that runs a command when clicked. This command is executed by the
	/// client as if the user typed it out in chat themselves.
	///
	/// The command must start with a leading `/`. If the command has any signed arguments (such as
	/// `/msg`), it cannot be ran by a component.
	#[inline]
	pub fn run_command(command: impl Into<String>) -> Self {
		Self::RunCommand(command.into())
	}

	/// Creates a [ClickEvent] that inserts text into the chat bar when clicked.
	#[inline]
	pub fn suggest_command(command: impl Into<String>) -> Self {
		Self::SuggestCommand(command.into())
	}

	/// Creates a [ClickEvent] that changes the page on a book. This is a signed type because the
	/// format allows for negative pages to be set, but they do not have any effect when sent.
	#[inline]
	pub fn change_page(page: u16) -> Self {
		// We constrict the range here because while incoming components may have negative pages,
		// sending an outgoing negative page is invalid.
		Self::ChangePage(page.into())
	}

	/// Creates a [ClickEvent] that copies text to the user's system clipboard.
	#[inline]
	pub fn copy(text: impl Into<String>) -> Self {
		Self::Copy(text.into())
	}
}

/// Represents a hover event. Hover events control content that is displayed when a user hovers over
/// a component in chat or in a book. Hover events do not have any effect in any other location.
///
/// Since this enum and its variants are non-exhaustive, they must be constructed through the factory
/// functions.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(tag = "action", content = "contents", rename_all = "snake_case")]
#[non_exhaustive]
pub enum HoverEvent {
	/// Displays another component in chat when hovered.
	#[non_exhaustive]
	ShowText(Box<Component>),

	// TODO: Proper sNBT serialization
	/// Displays an item in chat when hovered. The inner string is an sNBT tag containing 3 fields:
	/// * `id`: The item's material as a namespaced key (i.e `minecraft:stone`).
	/// * `count`: The number of items in the item stack.
	/// * `tag`: Additional NBT properties such as display name and enchantments.
	#[non_exhaustive]
	ShowItem(String),

	/// Displays an entity in chat when hovered. The inner string is an sNBT tag containing 3 fields:
	/// * `id`: The entity's String UUID.
	/// * `name`: The entity's name.
	/// * `type` (optional): The entity's type as a namespaced key (i.e `minecraft:creeper`).
	#[non_exhaustive]
	ShowEntity(String),
}

/// Used for the [HoverEvent] Serde model as a default for the item count.
#[doc(hidden)]
#[inline(always)]
#[allow(unused)] // unused when `experimental_hover_events` is unused
fn one() -> i32 {
	1
}

impl HoverEvent {
	/// Creates a [HoverEvent] that displays another component in chat when hovered.
	#[inline]
	pub fn show_text(text: impl Into<Component>) -> Self {
		Self::ShowText(Box::new(text.into()))
	}
}

#[cfg(feature = "experimental_hover_events")]
impl HoverEvent {
	/// Creates a [HoverEvent] that displays an item when hovered. See [HoverEvent::ShowItem] for
	/// more details.
	///
	/// # Experimental
	/// This method is experimental and *will* change. It exists to provide support if needed, but
	/// the API for creating item meta will be improved at a later date.
	#[inline]
	pub fn show_item(item_meta: impl Into<String>) -> Self {
		Self::ShowItem(item_meta.into())
	}
	/// Creates a [HoverEvent] that displays an entity when hovered. See [HoverEvent::ShowEntity]
	/// for more details.
	///
	/// # Experimental
	/// This method is experimental and *will* change. It exists to provide support if needed, but
	/// the API for creating entity meta will be improved at a later date.
	#[inline]
	pub fn show_entity(entity_meta: impl Into<String>) -> Self {
		Self::ShowEntity(entity_meta.into())
	}
}

impl From<Component> for HoverEvent {
	#[inline(always)]
	fn from(value: Component) -> Self {
		Self::show_text(value)
	}
}

impl From<String> for HoverEvent {
	#[inline(always)]
	fn from(value: String) -> Self {
		Self::show_text(value)
	}
}

impl From<&str> for HoverEvent {
	#[inline(always)]
	fn from(value: &str) -> Self {
		Self::show_text(value)
	}
}
