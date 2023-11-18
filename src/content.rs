use crate::Component;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

/// Represents the contents of a component. Different component types have different content types,
/// with each variant containing its own specialized fields.
///
/// This enum should not be manually constructed. Use the factory functions in [Component] instead.
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
#[serde(rename_all = "snake_case")]
pub enum Content {
	/// Plain text content. Text components have a raw content string and no other special state.
	///
	/// # Usage
	/// To create a new text component, use [Component::text()].
	Text(String),

	/// Content representing a keybind. Keybind components hold a action ID (`key.jump`, `key.drop`,
	///etc.) that is used to render the user's chosen keybind for that action.
	///
	/// # Usage
	/// To create a new keybind component, use [Component::keybind(key)].
	Keybind(String),

	/// Component content holding a scoreboard score. When sent to the client, the value of the
	/// score is displayed.
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

	/// Localized component content. Translation components have a translation key and an array of
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

	/// An empty component with no content fields. Empty components can still set style and have
	/// extra children.
	#[serde(untagged)]
	#[default]
	Empty,
}

impl Display for Content {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Text(contents) => write!(f, "{contents}")?,
			Self::Keybind(key) => write!(f, "[{key}]")?,
			Self::Score { value, .. } => write!(f, "{value}")?,
			Self::Translation { key, with: args } => {
				write!(f, "<{key}")?;
				for arg in args {
					write!(f, ":{}", arg.content)?;
				}
				write!(f, ">")?;
			}
			Self::Empty => {}
		}

		Ok(())
	}
}
