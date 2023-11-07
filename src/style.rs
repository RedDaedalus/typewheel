use crate::event::{ClickEvent, HoverEvent};
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
pub struct Style {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bold: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub italic: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underlined: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strikethrough: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub obfuscated: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<TextColor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insertion: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "clickEvent")]
    pub click_event: Option<ClickEvent>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "hoverEvent")]
    pub hover_event: Option<HoverEvent>,
}

/// Models a text component's color. There are 16 named colors, and any hex color can be created via
/// the [TextColor::Hex] variant.
///
/// Other crates that provide color-like types should implement [`Into<TextColor>`](Into), as
/// Typewheel will accept any type that can be converted in its APIs.
#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "snake_case")]
pub enum TextColor {
    /// Named color `black` (hex `#000`; code `§0`).
    Black,
    /// Named color `dark_blue` (hex `#00A`; code `§1`).
    DarkBlue,
    /// Named color `dark_green` (hex `#0A0`; code `§2`).
    DarkGreen,
    /// Named color `dark_aqua` (hex `#0AA`; code `§3`).
    DarkAqua,
    /// Named color `dark_red` (hex `#A00`; code `§4`).
    DarkRed,
    /// Named color `dark_purple` (hex `#A0A`; code `§5`).
    DarkPurple,
    /// Named color `gold` (hex `#FA0`; code `§6`).
    Gold,
    /// Named color `gray` (hex `#AAA`; code `§7`).
    Gray,
    /// Named color `dark_gray` (hex `#555`; code `§8`).
    DarkGray,

    /// Named color `blue` (hex `#55F`; code `§9`).
    Blue,
    /// Named color `green` (hex `#5F5`; code `§a`).
    Green,
    /// Named color `aqua` (hex `#5FF`; code `§b`).
    Aqua,
    /// Named color `red` (hex `#F55`; code `§c`).
    Red,
    /// Named color `light_purple` (hex `#F5F`; code `§d`).
    LightPurple,
    /// Named color `yellow` (hex `#FF5`; code `§e`).
    Yellow,
    /// Named color `white` (hex `#FFF`; code `§f`).
    White,

    /// An arbitrary RGB hex color. The most significant byte represents the red channel, the next
    /// represents the green, and the final represents the blue channel.
    ///
    /// When serialized, hex colors are represented as an uppercase hex string:
    /// ```rust
    /// # use typewheel::TextColor;
    ///
    /// let color = TextColor::Hex(0xAABBCC);
    /// assert_eq!(serde_json::to_string(&color).unwrap(), r##""#AABBCC""##);
    /// ```
    ///
    /// The inner value must be within range `(0, 0xFFFFFF)` (inclusive). If not, serialization will
    /// yield an error.
    #[serde(untagged, with = "hex_serde")]
    Hex(u32),
}

impl From<u32> for TextColor {
    fn from(value: u32) -> Self {
        Self::Hex(value)
    }
}

mod hex_serde {
    use serde::{
        de::Error as _, ser::Error as _, Deserialize, Deserializer, Serialize, Serializer,
    };

    pub(crate) fn serialize<S: Serializer>(value: &u32, serializer: S) -> Result<S::Ok, S::Error> {
        if *value > 0xFFFFFF {
            return Err(S::Error::custom("hex value cannot exceed 0xFFFFFF"));
        }

        format!("#{value:06X}").serialize(serializer)
    }

    pub(crate) fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<u32, D::Error> {
        const ERROR_MESSAGE: &str = "expected hex literal";

        let hex_string = String::deserialize(deserializer)?;
        if !hex_string.starts_with('#') {
            return Err(D::Error::custom(ERROR_MESSAGE));
        }

        u32::from_str_radix(&hex_string[1..], 16).map_err(|_| D::Error::custom(ERROR_MESSAGE))
    }

    #[cfg(test)]
    mod tests {
        use crate::TextColor;
        use serde_test::{assert_tokens, Token};

        #[test]
        fn color_codec() {
            assert_tokens(
                &TextColor::LightPurple,
                &[Token::UnitVariant {
                    name: "TextColor",
                    variant: "light_purple",
                }],
            );

            assert_tokens(&TextColor::Hex(0xAABBCC), &[Token::String("#AABBCC")])
        }
    }
}
