use crate::{Component, Key};
#[cfg(feature = "nbt")]
use quartz_nbt::NbtCompound;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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
	/// `/msg`), it cannot be triggered by this action.
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
	ShowText(Box<Component>),

	/// Displays an item in chat when hovered. The inner string is an sNBT tag containing 3 fields:
	/// * `id`: The item's material as a namespaced key (i.e `minecraft:stone`).
	/// * `count`: The number of items in the item stack.
	/// * `tag`: Additional NBT properties such as display name and enchantments.
	ShowItem(Box<ItemHover>),

	/// Displays an entity in chat when hovered. The inner string is an sNBT tag containing 3 fields:
	/// * `id`: The entity's String UUID.
	/// * `name`: The entity's name.
	/// * `type` (optional): The entity's type as a namespaced key (i.e `minecraft:creeper`).
	ShowEntity(Box<EntityHover>),
}

impl HoverEvent {
	/// Creates a [HoverEvent] that displays another component in chat when hovered.
	#[inline]
	pub fn show_text(text: impl Into<Component>) -> Self {
		Self::ShowText(Box::new(text.into()))
	}

	/// Creates a [HoverEvent] that displays an entity in chat when hovered. The entity is only
	/// visible to users with advanced tooltips enabled.
	#[inline]
	pub fn show_entity(hover: impl Into<EntityHover>) -> Self {
		Self::ShowEntity(Box::new(hover.into()))
	}

	/// Creates a [HoverEvent] that displays an item in chat when hovered.
	#[inline]
	#[cfg(feature = "nbt")]
	pub fn show_item(hover: impl Into<ItemHover>) -> Self {
		Self::ShowItem(Box::new(hover.into()))
	}
}

/// A type modeling the data for showing [entity hovers][HoverEvent::show_entity()]. Entities do not
/// need to exist for the client to render them.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct EntityHover {
	/// The entity's [id][Uuid].
	pub id: Uuid,

	/// The entity's display name.
	pub name: Component,

	/// The entity's type.
	#[serde(rename = "type")]
	pub entity_type: Key,
}

impl EntityHover {
	/// Creates a new [EntityHover] with the provided parameters.
	pub fn new(id: Uuid, name: impl Into<Component>, ty: impl Into<Key>) -> Self {
		Self {
			id,
			name: name.into(),
			entity_type: ty.into(),
		}
	}
}

/// A type modeling the data for showing [item hovers][HoverEvent::show_item()]. In order to make
/// full use of this type, the `nbt` create feature is required. Without it, the [ItemHover::tag]
/// field cannot be set.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct ItemHover {
	/// The item's material. This is a namespaced key holding the material name.
	pub id: Key,

	/// The number of items in the displayed item stack. If not provided for deserialization,
	/// defaults to 1.
	#[serde(default = "one")]
	pub count: i32,

	/// Additional NBT on the item. Using this field requires the `nbt` crate feature.
	#[cfg(any(feature = "nbt", doc))]
	#[serde(with = "nbt")]
	pub tag: Option<NbtCompound>,
}

impl ItemHover {
	/// Creates a new [ItemHover] with the provided material ID and stack count.
	pub fn new(id: impl Into<Key>, count: i32) -> Self {
		Self::_new(id.into(), count)
	}

	/// Creates a new [ItemHover] with an NBT tag as well as a material ID and stack count.
	#[cfg(feature = "nbt")]
	pub fn with_tag(id: impl Into<Key>, count: i32, tag: impl Into<NbtCompound>) -> Self {
		Self {
			id: id.into(),
			count,
			tag: Some(tag.into()),
		}
	}

	#[cfg(feature = "nbt")]
	fn _new(id: Key, count: i32) -> Self {
		Self {
			id,
			count,
			tag: None,
		}
	}

	#[cfg(not(feature = "nbt"))]
	fn _new(id: Key, count: i32) -> Self {
		Self { id, count }
	}
}

// This is logically sound because all of the base types in NbtCompound implement Eq. Since this may
// change in the future, the version for `quartz_nbt` has been pinned.
impl Eq for ItemHover {}

#[cfg(feature = "nbt")]
mod nbt {
	use quartz_nbt::{snbt, NbtCompound};
	use serde::de::Error as _;
	use serde::{Deserialize, Deserializer, Serialize, Serializer};

	#[inline(always)] // This has one call site so we can inline it without bloat
	pub(super) fn serialize<S: Serializer>(
		value: &Option<NbtCompound>,
		serializer: S,
	) -> Result<S::Ok, S::Error> {
		value
			.as_ref()
			.map(|compound| compound.to_snbt())
			.serialize(serializer)
	}

	#[inline(always)]
	pub(super) fn deserialize<'de, D: Deserializer<'de>>(
		deserializer: D,
	) -> Result<Option<NbtCompound>, D::Error> {
		let tag = Option::<String>::deserialize(deserializer)?;
		tag.map(|s| snbt::parse(&s).map_err(D::Error::custom))
			.transpose()
	}
}

/// Used for the [HoverEvent] Serde model as a default for the item count.
#[doc(hidden)]
#[inline(always)]
#[allow(unused)] // unused when `experimental_hover_events` is unused
fn one() -> i32 {
	1
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
