use serde::{Deserialize, Serialize, Serializer};
use std::fmt::{Debug, Display, Formatter};

/// Represents a namespaced key pointing to a Minecraft resource such as a material, command, entity
/// type, or other value.
///
/// The namespace used for all Vanilla values is [`minecraft`](Self::MINECRAFT_NAMESPACE). Mods and
/// plugins generally create their own namespaces.
///
/// # Creating a Key
/// The most direct way to create a key is with the [Key::new()] and [Key::minecraft()] methods:
/// ```
/// # use typewheel::Key;
/// #
/// let key_1 = Key::new("minecraft", "stone");
/// let key_2 = Key::minecraft("stone");
///
/// assert_eq!(key_1, key_2);
/// ```
/// [Key] also implements [From] for both [`String`] and `(String, String)`, where the former parses
/// a key in the format `"namespace:value"` (when the namespace is left out, `minecraft` is used.)
///
/// # Serial Representation
/// This type implements [serde]'s [Serialize] and [Deserialize], where it is encoded as a string.
/// The serial representation is equivalent to the output of [ToString::to_string].
#[derive(Deserialize, Clone, PartialEq, Eq)]
#[serde(from = "String")]
pub struct Key(Option<String>, String);

// Serialize is manually implemented to avoid the clone that #[serde(into)] requires.
impl Serialize for Key {
	#[inline(always)]
	fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
		self.to_string().serialize(serializer)
	}
}

impl Key {
	/// The namespace used by Minecraft for its built-in items and other values.
	pub const MINECRAFT_NAMESPACE: &'static str = "minecraft";

	/// Creates a new key in the provided namespace.
	///
	/// # Examples
	/// ```
	/// # use typewheel::Key;
	/// #
	/// let key = Key::new("typewheel", "value");
	/// assert_eq!(key.to_string(), "typewheel:value");
	/// ```
	pub fn new(namespace: impl Into<String>, value: impl Into<String>) -> Self {
		let namespace = namespace.into();
		if namespace == Self::MINECRAFT_NAMESPACE {
			Self(None, value.into())
		} else {
			Self(namespace.into(), value.into())
		}
	}

	/// Creates a new key in the [Minecraft namespace][Self::MINECRAFT_NAMESPACE].
	///
	/// # Examples
	/// ```
	/// # use typewheel::Key;
	/// #
	/// let key = Key::minecraft("stone");
	/// assert_eq!(key.to_string(), "minecraft:stone");
	/// ```
	pub fn minecraft(value: impl Into<String>) -> Self {
		Self(None, value.into())
	}

	/// Gets this key's namespace as a reference.
	///
	/// ```
	/// # use typewheel::Key;
	/// #
	/// assert_eq!(Key::minecraft("stone").namespace(), Key::MINECRAFT_NAMESPACE);
	/// assert_eq!(Key::new("ns", "val").namespace(), "ns");
	/// ```
	pub fn namespace(&self) -> &str {
		if let Some(namespace) = &self.0 {
			namespace
		} else {
			Self::MINECRAFT_NAMESPACE
		}
	}

	/// Gets this key's value as a string reference.
	///
	/// # Examples
	/// ```
	/// # use typewheel::Key;
	/// #
	/// assert_eq!(Key::minecraft("stone").value(), "stone");
	/// ```
	///
	pub fn value(&self) -> &str {
		&self.1
	}

	/// Gets this key's string representation, eliding the namespace if it is in the [default]
	/// [Self::MINECRAFT_NAMESPACE]. For the full string representation, use [ToString::to_string].
	///
	/// # Examples
	/// ```
	/// # use typewheel::Key;
	/// #
	/// assert_eq!(Key::minecraft("stone").to_compact_string(), "stone");
	/// assert_eq!(Key::new("bukkit", "help").to_compact_string(), "bukkit:help");
	/// ```
	pub fn to_compact_string(&self) -> String {
		let value = &self.1;

		if let Some(namespace) = &self.0 {
			format!("{namespace}:{value}")
		} else {
			value.clone()
		}
	}

	/// Test fixture to get the inner namespace representation.
	#[cfg(test)]
	#[doc(hidden)]
	pub(crate) fn stored_namespace(&self) -> Option<&str> {
		self.0.as_deref()
	}

	/// Gets the parts of this key as a tuple, with the namespace in the first slot and the value in
	/// the second.
	///
	/// # Examples
	/// ```
	/// # use typewheel::Key;
	/// #
	/// assert_eq!(Key::minecraft("stone").parts(), ("minecraft", "stone"));
	/// ```
	#[inline(always)]
	pub fn parts(&self) -> (&str, &str) {
		(self.namespace(), self.value())
	}
}

impl Display for Key {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		let (namespace, value) = self.parts();
		write!(f, "{namespace}:{value}")
	}
}

impl Debug for Key {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		f.debug_tuple("Key").field(&self.to_string()).finish()
	}
}

impl From<(String, String)> for Key {
	fn from(value: (String, String)) -> Self {
		Self::new(value.0, value.1)
	}
}

impl From<Key> for (String, String) {
	fn from(value: Key) -> Self {
		(
			value
				.0
				.unwrap_or_else(|| Key::MINECRAFT_NAMESPACE.to_owned()),
			value.1,
		)
	}
}

impl From<String> for Key {
	fn from(value: String) -> Self {
		if let Some(sep_index) = value.find(':') {
			let (namespace, value) = value.split_at(sep_index);
			Self::new(namespace, &value[1..])
		} else {
			Self::minecraft(value)
		}
	}
}

impl From<&str> for Key {
	fn from(value: &str) -> Self {
		value.to_string().into()
	}
}

impl From<Key> for String {
	#[inline(always)]
	fn from(value: Key) -> Self {
		value.to_string()
	}
}

#[cfg(test)]
mod tests {
	use super::Key;
	use serde_test::{assert_tokens, Token};

	#[test]
	fn namespace_elision() {
		assert_eq!(Key::minecraft("stone").stored_namespace(), None);
		assert_eq!(Key::new("minecraft", "air").stored_namespace(), None);
		assert_eq!(
			Key::new("bukkit", "help").stored_namespace(),
			Some("bukkit")
		);
	}

	#[test]
	fn serde_rep() {
		assert_tokens(
			&Key::minecraft("stone"),
			&[Token::String("minecraft:stone")],
		);
		assert_tokens(&Key::new("bukkit", "help"), &[Token::String("bukkit:help")]);
	}
}
