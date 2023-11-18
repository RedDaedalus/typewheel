use crate::codec::ComponentCodec;
use crate::{iter::Visit, Component, Content, Style};
use std::collections::VecDeque;

#[allow(unused_imports)]
// only used in rustdoc -- https://github.com/rust-lang/rust/issues/79542
use crate::TextColor;

/// A [ComponentCodec] for serializing components in Minecraft's legacy format.
///
/// # Legacy Format
/// The legacy style format is a string format based on control codes. There are codes for the 16
/// non-hex colors and the format attributes (bold, italic, underlined, strikethrough, obfuscated).
///
/// ## Color Codes
/// | Code | Value                              |
/// | ---- | ---------------------------------- |
/// | `§0` | [TextColor::Black]                 |
/// | `§1` | [TextColor::DarkBlue]              |
/// | `§2` | [TextColor::DarkGreen]             |
/// | `§3` | [TextColor::DarkAqua]              |
/// | `§4` | [TextColor::DarkRed]               |
/// | `§5` | [TextColor::DarkPurple]            |
/// | `§6` | [TextColor::Gold]                  |
/// | `§7` | [TextColor::Gray]                  |
/// | `§8` | [TextColor::DarkGray]              |
/// | `§9` | [TextColor::Blue]                  |
/// | `§a` | [TextColor::Green]                 |
/// | `§b` | [TextColor::Aqua]                  |
/// | `§c` | [TextColor::Red]                   |
/// | `§d` | [TextColor::LightPurple]           |
/// | `§e` | [TextColor::Yellow]                |
/// | `§f` | [TextColor::White]                 |
///
/// ## Style Codes
/// | Code | Style Field                        |
/// | ---- | ---------------------------------- |
/// | `§k` | [Style::obfuscated]                |
/// | `§l` | [Style::Bold]                      |
/// | `§m` | [Style::Strikethrough]             |
/// | `§n` | [Style::Underlined]                |
/// | `§o` | [Style::Italic]                    |
/// | `§r` | [Style::clear] (resets all styles) |
///
/// This table uses the section symbol (`§`) as this is the character used by Minecraft. This codec
/// supports both the section symbol and ampersand (`&`) for the color code indicator. The different
/// variations can be accessed via the [LegacyCodec::SECTION] and [LegacyCodec::AMPERSAND] constants.
/// Additionally, to use a custom character, the `S` generic parameter may be specified:
///
/// ```
/// # use typewheel::{Component, codec::{ComponentCodec, LegacyCodec}};
/// #
/// let component = Component::text("hello").with_bold(true);
///
/// let ampersand_codec = LegacyCodec::AMPERSAND;
/// assert_eq!(ampersand_codec.serialize(&component), "&lhello");
///
/// let dollar_codec = LegacyCodec::<'$'>;
/// assert_eq!(dollar_codec.serialize(&component), "$lhello");
/// ```
#[derive(Clone, Copy)]
pub struct LegacyCodec<const S: char>;

impl LegacyCodec<'§'> {
	/// A [LegacyCodec] instance that uses the section symbol (`§`) as the control character.
	pub const SECTION: Self = Self;
}

impl LegacyCodec<'&'> {
	/// A [LegacyCodec] instance that uses the ampersand symbol (`&`) as the control character.
	pub const AMPERSAND: Self = Self;
}

impl<const S: char> ComponentCodec for LegacyCodec<S> {
	type DecodeInput = String;
	type EncodeOutput = String;
	type DecodeOutput = Component;

	fn serialize(self, component: &Component) -> Self::EncodeOutput {
		let mut out = String::new();
		let mut styles = VecDeque::<&Style>::new();

		for op in component.visit() {
			match op {
				Visit::Push(node) => {
					styles.push_back(&node.style);

					// Collect every active style into one value.
					let mut active_style = Style::default();
					for style in &styles {
						active_style.merge(style);
					}

					let mut to_apply = node.style.clone();

					// Color wipes all prior formatting, so we need to reapply it.
					if let Some(color) = node.style.color {
						out.extend([S, color.color_code()]);
						to_apply.merge(&active_style);
					}

					apply_format_styles::<S>(&mut out, &to_apply);

					if let Some(content) = node.shallow_text() {
						out += content;
					}
				}

				Visit::Pop(_) => {
					styles.pop_back();
				}
			}
		}

		out
	}

	fn deserialize(self, value: impl Into<Self::DecodeInput>) -> Self::DecodeOutput {
		// Push a reset code to the start of the string. Having a code at the beginning lets us
		// assume that each segment starts with a code.
		let value = format!("{}r{}", S, value.into());

		let extra = value
			.split(S)
			.filter_map(|segment| {
				let code = segment.chars().next()?;

				if let Some(color) = TextColor::from_color_code(code) {
					// Color codes reset all other fields.
					let style = Style {
						color: Some(color),
						..RESETTING
					};
					return Some((style, &segment[1..]));
				}

				let style = match code {
					BOLD => Style::bold(true),
					ITALIC => Style::italic(true),
					UNDERLINED => Style::underlined(true),
					STRIKETHROUGH => Style::strikethrough(true),
					OBFUSCATED => Style::obfuscated(true),
					RESET => RESETTING,

					_ => {
						return Some((Style::default(), segment));
					}
				};

				Some((style, &segment[1..]))
			})
			.map(|(style, content)| {
				Component::create_flat(
					if content.is_empty() {
						Content::Empty
					} else {
						Content::Text(content.to_owned())
					},
					style,
				)
			});

		Component::empty().with_extra(extra).flattened()
	}
}

const RESETTING: Style = Style {
	bold: Some(false),
	italic: Some(false),
	underlined: Some(false),
	strikethrough: Some(false),
	obfuscated: Some(false),
	font: None,
	color: Some(TextColor::White),
	insertion: None,
	click_event: None,
	hover_event: None,
};

const BOLD: char = 'l';
const ITALIC: char = 'o';
const UNDERLINED: char = 'n';
const STRIKETHROUGH: char = 'm';
const OBFUSCATED: char = 'k';
const RESET: char = 'r';

fn apply_format_styles<const S: char>(target: &mut String, style: &Style) {
	if let Some(true) = style.bold {
		target.extend([S, BOLD]);
	}

	if let Some(true) = style.italic {
		target.extend([S, ITALIC]);
	}

	if let Some(true) = style.underlined {
		target.extend([S, UNDERLINED]);
	}

	if let Some(true) = style.strikethrough {
		target.extend([S, STRIKETHROUGH]);
	}

	if let Some(true) = style.obfuscated {
		target.extend([S, OBFUSCATED]);
	}
}

#[cfg(test)]
mod tests {
	use crate::codec::{ComponentCodec, LegacyCodec};
	use crate::{Component, TextColor};

	#[test]
	fn encode_color_codes() {
		let component = Component::text("hello ")
			.with_bold(true)
			.with_color(TextColor::Green)
			.with_extra([Component::text("world").with_color(TextColor::Blue)]);

		let codec = LegacyCodec::SECTION;
		assert_eq!(codec.serialize(&component), "§a§lhello §9§lworld");
	}
}
