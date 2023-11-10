use crate::{Component, ComponentBody, Style};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::borrow::Cow;

// <editor-fold desc="> Serialize">

/// A container type that holds all of the fields of a component. We cannot derive Serialize for the
/// actual [Component] type because we want to compact the component if possible. After we do this
/// check, though, we want to just fall back on Serde's normal rep. The easiest way to do this is a
/// duplicate type (`#[serde(flatten)]` uses a private serializer type so we can't just copy the
///derive output unfortunately.)
#[derive(Serialize)]
struct SerialVessel<'a> {
	#[serde(flatten)]
	style: Cow<'a, Style>,
	#[serde(flatten)]
	body: Cow<'a, ComponentBody>,
	#[serde(skip_serializing_if = "is_empty")]
	extra: &'a [Component],
}

/// Wrapper because `&[Component]` is not a nameable type.
#[inline(always)]
fn is_empty(target: &[Component]) -> bool {
	target.is_empty()
}

impl Serialize for Component {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		// Compacting: encode this component as a raw string if possible.
		if self.extra.is_empty() && self.style.is_blank() {
			if let ComponentBody::Text(text) = &self.body {
				return serializer.serialize_str(text);
			}
		}

		let bowl = SerialVessel {
			style: Cow::Borrowed(&self.style),
			body: Cow::Borrowed(&self.body),
			extra: &self.extra[..],
		};

		bowl.serialize(serializer)
	}
}
// </editor-fold>

// <editor-fold desc="> Deserialize">

#[derive(Deserialize)]
#[serde(untagged)]
pub(crate) enum DeserializeVessel {
	PlainText(String),
	Component {
		#[serde(flatten)]
		style: Style,
		#[serde(flatten)]
		body: ComponentBody,
		#[serde(default)]
		extra: Vec<Component>,
	},
}

impl From<DeserializeVessel> for Component {
	fn from(bowl: DeserializeVessel) -> Component {
		match bowl {
			DeserializeVessel::PlainText(text) => Component::text(text),
			DeserializeVessel::Component { style, body, extra } => Component { style, body, extra },
		}
	}
}

#[cfg(no)]
impl From<Component> for DeserializeVessel {
	fn from(value: Component) -> Self {
		if value.extra.is_empty() && value.style.is_blank() {
			if let ComponentBody::Text(text) = value.body {
				return Self::PlainText(text);
			}
		}

		Self::Component {
			style: value.style,
			body: value.body,
			extra: value.extra,
		}
	}
}

impl<'de> Deserialize<'de> for Component {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		let bowl = DeserializeVessel::deserialize(deserializer)?;
		Ok(bowl.into())
	}
}

// </editor-fold>
