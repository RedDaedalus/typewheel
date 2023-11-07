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
	// TODO: item & entity
}

impl HoverEvent {
	/// Creates a [HoverEvent] that displays another component in chat when hovered.
	#[inline]
	pub fn show_text(text: impl Into<Component>) -> Self {
		Self::ShowText(Box::new(text.into()))
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
