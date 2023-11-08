Typewheel is a library for creating and serializing Minecraft [text components][wiki]. This
project is loosely inspired by the [Adventure project][adventure].

Components can be modified using both setters and the builder pattern, where each field has a
method for setting its value directly, and for producing a new component with the new value.

# Creating Components
```rust
use typewheel::{Component, TextColor};

fn main() {
    // Setter pattern: setters pass back a reference for chaining.
    let mut inner = Component::text("world");
    inner.bold(true).color(0xAA7FFF);

    assert_eq!(inner.style.bold, Some(true));
    assert_eq!(inner.style.color, Some(TextColor::Hex(0xAA7FFF)));

    // Builder pattern: builders accept ownership of `self` and return an owned value back.
    let component = Component::text("Hello, ")
        .with_color(TextColor::Gray)
        .with_extra([inner, "!".into()]);

    assert_eq!(component.style.color, Some(TextColor::Gray));
    assert!(!component.extra.is_empty());
}
```

# Serializing Components
Components can be serialized and deserialized using codecs. The most commonly used format for
encoding components is JSON, which is implemented with the `codec::JsonComponentCodec` struct.
Other serializers exist for a variety of formats and implementations.

# Create Features
* `json`: Enables the use of `codec::JsonComponentCodec` via `serde_json`.
* `experimental_hover_events`: Enables the use of the unfinished `show_item` and `show_entity` hover events.

[wiki]: https://wiki.vg/Chat
[adventure]: https://docs.advntr.dev
