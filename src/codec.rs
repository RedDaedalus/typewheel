use crate::Component;
use std::convert::Infallible;

pub trait ComponentCodec {
	type Input;
	type Output;
	type Error;

	fn serialize(&self, component: &Component) -> Result<Self::Output, Self::Error>;

	fn deserialize(&self, value: &Self::Input) -> Result<Component, Self::Error>;
}

#[cfg(feature = "json")]
pub mod json {
	use super::ComponentCodec;
	use crate::Component;

	pub struct JsonComponentCodec;

	impl ComponentCodec for JsonComponentCodec {
		type Input = String;
		type Output = String;
		type Error = serde_json::Error;

		#[inline(always)]
		fn serialize(&self, component: &Component) -> Result<Self::Output, Self::Error> {
			serde_json::to_string(component)
		}

		#[inline(always)]
		fn deserialize(&self, value: &Self::Input) -> Result<Component, Self::Error> {
			serde_json::from_str(value)
		}
	}
}

pub struct PlainTextComponentCodec;

impl ComponentCodec for PlainTextComponentCodec {
	type Input = String;
	type Output = String;
	type Error = Infallible;

	fn serialize(&self, component: &Component) -> Result<Self::Output, Self::Error> {
		let mut output = String::new();

		Ok(output)
	}

	fn deserialize(&self, value: &Self::Input) -> Result<Component, Self::Error> {
		Ok(Component::text(value))
	}
}
