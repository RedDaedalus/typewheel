use crate::Component;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "action", content = "value", rename_all = "snake_case")]
#[non_exhaustive]
pub enum ClickEvent {
    RunCommand(String),
    SuggestCommand(String),
    ChangePage(i32),
    #[serde(rename = "copy_to_clipboard")]
    Copy(String),
}

impl ClickEvent {
    #[inline]
    pub fn run_command(command: impl Into<String>) -> Self {
        Self::RunCommand(command.into())
    }

    #[inline]
    pub fn suggest_command(command: impl Into<String>) -> Self {
        Self::SuggestCommand(command.into())
    }

    #[inline]
    pub fn change_page(page: u16) -> Self {
        // We constrict the range here because while incoming components may have negative pages,
        // sending an outgoing negative page is invalid.
        Self::ChangePage(page.into())
    }

    #[inline]
    pub fn copy(text: impl Into<String>) -> Self {
        Self::Copy(text.into())
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "action", content = "contents", rename_all = "snake_case")]
pub enum HoverEvent {
    ShowText(Box<Component>),
    // TODO: item & entity
}

impl HoverEvent {
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
