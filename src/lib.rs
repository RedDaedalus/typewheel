mod event;
mod style;

pub use self::style::{Style, TextColor};
use crate::event::{ClickEvent, HoverEvent};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[non_exhaustive]
pub struct Component {
    #[serde(flatten)]
    pub style: Style,
    #[serde(flatten)]
    pub body: ComponentBody,
    #[serde(skip_serializing_if = "Vec::is_empty", rename = "extra")]
    pub children: Vec<Component>,
}

impl Component {
    pub fn text(text: impl Into<String>) -> Self {
        Self {
            style: Style::default(),
            children: Vec::new(),
            body: ComponentBody::Text { text: text.into() },
        }
    }

    pub fn with_bold(mut self, state: bool) -> Self {
        self.style.bold = Some(state);
        self
    }

    pub fn with_italic(mut self, state: bool) -> Self {
        self.style.italic = Some(state);
        self
    }

    pub fn with_underlined(mut self, state: bool) -> Self {
        self.style.underlined = Some(state);
        self
    }

    pub fn with_strikethrough(mut self, state: bool) -> Self {
        self.style.strikethrough = Some(state);
        self
    }

    pub fn with_obfuscated(mut self, state: bool) -> Self {
        self.style.obfuscated = Some(state);
        self
    }

    pub fn with_color(mut self, state: impl Into<TextColor>) -> Self {
        self.style.color = Some(state.into());
        self
    }

    pub fn with_click_event(mut self, state: impl Into<ClickEvent>) -> Self {
        self.style.click_event = Some(state.into());
        self
    }

    pub fn with_hover_event(mut self, state: impl Into<HoverEvent>) -> Self {
        self.style.hover_event = Some(state.into());
        self
    }

    pub fn with(mut self, child: impl Into<Component>) -> Self {
        self.children.push(child.into());
        self
    }
}

#[derive(Serialize, Deserialize)]
#[non_exhaustive]
#[serde(untagged)]
pub enum ComponentBody {
    Text { text: String },
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

#[cfg(test)]
mod tests {
    use super::{Component, TextColor};

    #[test]
    fn serial() {
        let test = Component::text("Daedalus")
            .with_bold(true)
            .with_color(0xff0049)
            .with_hover_event(Component::text("Player ID 427").with_color(TextColor::Gray))
            .with(Component::text(" has joined for the first time.").with_color(TextColor::Gray));

        println!("{}", serde_json::to_string_pretty(&test).unwrap());
    }
}
