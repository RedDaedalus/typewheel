use serde_test::{assert_tokens, Token};
use typewheel::TextColor;

mod common;

#[test]
fn general_serial() {
	// JSON rep: {"text":"hello ","bold":true,"color":"green","extra":[{"text":"world","color":\
	// "blue","italic":true}]}
	assert_tokens(
		&common::styled_hello(),
		&[
			// root
			Token::Map { len: None },
			// style.bold
			Token::String("bold"),
			Token::Some,
			Token::Bool(true),
			// style.color
			Token::Str("color"),
			Token::Some,
			Token::UnitVariant {
				name: "TextColor",
				variant: "green",
			},
			// body.text
			Token::String("text"),
			Token::String("hello "),
			// extra
			Token::String("extra"),
			Token::Seq { len: Some(1) },
			// extra[0]
			Token::Map { len: None },
			// extra[0].style.italic
			Token::String("italic"),
			Token::Some,
			Token::Bool(true),
			// extra[0].style.color
			Token::String("color"),
			Token::Some,
			Token::UnitVariant {
				name: "TextColor",
				variant: "blue",
			},
			// extra[0].body.text
			Token::String("text"),
			Token::String("world"),
			// ..end
			Token::MapEnd,
			Token::SeqEnd,
			Token::MapEnd,
		],
	);
}

#[test]
fn color_tokens() {
	assert_tokens(
		&TextColor::LightPurple,
		&[Token::UnitVariant {
			name: "TextColor",
			variant: "light_purple",
		}],
	);
	assert_tokens(&TextColor::Hex(0x000123), &[Token::String("#000123")]);
	assert!(serde_json::to_string(&TextColor::Hex(u32::MAX)).is_err());
}
