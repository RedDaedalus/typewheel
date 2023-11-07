mod event;
mod style;

pub use self::style::{Style, TextColor};
use crate::event::{ClickEvent, HoverEvent};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(from = "String")]
#[non_exhaustive]
pub struct Component {
    #[serde(flatten)]
    pub style: Style,
    #[serde(flatten)]
    pub body: ComponentBody,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub extra: Vec<Component>,
}

macro_rules! declare_style_methods {
    ($($field: ident: $ty: ty { set: $setter: ident, build: $builder: ident, clear: $clearer: ident } ),+ $(,)?) => {
        $(
            pub fn $setter(&mut self, state: impl Into<$ty>) {
                self.style.$field = Some(state.into());
            }

            pub fn $builder(mut self, state: impl Into<$ty>) -> Self {
                self.$setter(state);
                self
            }

            pub fn $clearer(&mut self) {
                self.style.$field = None;
            }
        )+
    };
}

impl Component {
    // <editor-fold desc="> Factory functions">
    pub fn text(text: impl Into<String>) -> Self {
        Self {
            style: Style::default(),
            extra: Vec::new(),
            body: ComponentBody::Text { text: text.into() },
        }
    }

    pub fn translate(
        key: impl Into<String>,
        with: impl IntoIterator<Item = impl Into<Component>>,
    ) -> Self {
        Self {
            style: Style::default(),
            extra: Vec::new(),
            body: ComponentBody::Translation {
                key: key.into(),
                with: with.into_iter().map(Into::into).collect(),
            },
        }
    }

    pub fn keybind(key: impl Into<String>) -> Self {
        Self {
            style: Style::default(),
            extra: Vec::new(),
            body: ComponentBody::Keybind { key: key.into() },
        }
    }
    // </editor-fold>

    declare_style_methods! {
        bold: bool { set: bold, build: with_bold, clear: clear_bold },
        italic: bool { set: italic, build: with_italic, clear: clear_italic },
        underlined: bool { set: underlined, build: with_underlined, clear: clear_underliend },
        strikethrough: bool { set: strikethrough, build: with_strikethrough, clear: clear_strikethrough },
        obfuscated: bool { set: obfuscated, build: with_obfuscated, clear: clear_obfuscated },
        font: String { set: font, build: with_font, clear: clear_font },
        color: TextColor { set: color, build: with_color, clear: clear_color },
        insertion: String { set: insertion, build: with_insertion, clear: clear_insertion },

        click_event: ClickEvent { set: click_event, build: with_click_event, clear: clear_click_event },
        hover_event: HoverEvent { set: hover_event, build: with_hover_event, clear: clear_hover_event },
    }

    fn append(&mut self, extra: impl IntoIterator<Item = impl Into<Component>>) {
        self.extra.extend(extra.into_iter().map(Into::into))
    }

    fn with_extra(mut self, extra: impl IntoIterator<Item = impl Into<Component>>) -> Self {
        self.append(extra);
        self
    }

    fn clear_extra(&mut self) {
        self.extra.clear();
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[non_exhaustive]
#[serde(untagged)]
pub enum ComponentBody {
    Text {
        text: String,
    },
    Translation {
        #[serde(rename = "translate")]
        key: String,
        #[serde(skip_serializing_if = "Vec::is_empty")]
        with: Vec<Component>,
    },
    Keybind {
        #[serde(rename = "keybind")]
        key: String,
    },
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
            .with_extra([
                Component::text(" has joined for the first time.").with_color(TextColor::Gray)
            ])
            .with_extra(Some("h"));

        println!("{}", serde_json::to_string_pretty(&test).unwrap());
    }
}
