use serde_test::{assert_tokens, Token};
use typewheel::{Component, ItemHover, TextColor};

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
			// content.text
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
			// extra[0].content.text
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
fn compacting_tokens() {
	assert_tokens(
		&Component::text("hello world"),
		&[Token::String("hello world")],
	);

	// Ensure that a styled component isn't compacted
	assert_tokens(
		&Component::text("hello bold").with_bold(true),
		&[
			Token::Map { len: None },
			Token::String("bold"),
			Token::Some,
			Token::Bool(true),
			Token::String("text"),
			Token::String("hello bold"),
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

#[test]
#[cfg(feature = "nbt")]
fn event_tokens() {
	use quartz_nbt::compound;
	let item_hover = ItemHover::with_tag(
		"minecraft:stone",
		1,
		compound! {
			"display": {
				"name": "hello world"
			}
		},
	);

	assert_tokens(
		&item_hover,
		&[
			Token::Struct {
				name: "ItemHover",
				len: 3,
			},
			Token::String("id"),
			Token::String("minecraft:stone"),
			Token::String("count"),
			Token::I32(1),
			Token::String("tag"),
			Token::Some,
			Token::String(r#"{display:{name:hello world}}"#),
			Token::StructEnd,
		],
	);
}
