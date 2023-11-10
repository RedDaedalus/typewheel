use crate::{Component, ComponentBody, Style};
use serde::{Deserialize, Serialize};

/// An enum used as the actual serial representation for a component. If a component has only text
/// and no styling, it is represented as a plain string. Else, it's serialized as a JSON object.
///
/// [Component] uses this type via `#[serde(from = "SerialVessel", into = "SerialVessel")]` to
/// handle serialization.  
/// 
/// Unit tests for serialization are in `tests/serial.rs`.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub(crate) enum SerialVessel {
	PlainText(String),
	Component {
		#[serde(flatten)]
		style: Style,
		#[serde(flatten)]
		body: ComponentBody,
		#[serde(default, skip_serializing_if = "Vec::is_empty")]
		extra: Vec<Component>,
	},
}

impl From<SerialVessel> for Component {
	#[inline(always)]
	fn from(bowl: SerialVessel) -> Component {
		match bowl {
			SerialVessel::PlainText(text) => Component::text(text),
			SerialVessel::Component { style, body, extra } => Component { style, body, extra },
		}
	}
}

impl From<Component> for SerialVessel {
	#[inline(always)]
	fn from(value: Component) -> Self {
		// Compacting: encode this component as a raw string if possible.
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
