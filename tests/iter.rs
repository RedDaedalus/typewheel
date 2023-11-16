use typewheel::{Component, IterOrder};
mod common;

#[test]
fn depth_first_iter() {
	let component = common::deeply_nested();
	let mut iter = component.iter(IterOrder::DepthFirst);

	assert_eq!(iter.next().and_then(Component::shallow_text), Some("a"));
	assert_eq!(iter.next().and_then(Component::shallow_text), Some("b"));
	assert_eq!(iter.next().and_then(Component::shallow_text), Some("c"));
	assert_eq!(iter.next().and_then(Component::shallow_text), Some("d"));
	assert_eq!(iter.next().and_then(Component::shallow_text), Some("e"));
	assert_eq!(iter.next().and_then(Component::shallow_text), Some("f"));
	assert_eq!(iter.next().and_then(Component::shallow_text), Some("g"));
	assert_eq!(iter.next().and_then(Component::shallow_text), Some("h"));

	assert!(iter.next().is_none());
}

#[test]
fn breadth_first_iter() {
	let component = common::deeply_nested();
	let mut iter = component.iter(IterOrder::BreadthFirst);

	assert_eq!(iter.next().and_then(Component::shallow_text), Some("a"));
	assert_eq!(iter.next().and_then(Component::shallow_text), Some("b"));
	assert_eq!(iter.next().and_then(Component::shallow_text), Some("e"));
	assert_eq!(iter.next().and_then(Component::shallow_text), Some("c"));
	assert_eq!(iter.next().and_then(Component::shallow_text), Some("d"));
	assert_eq!(iter.next().and_then(Component::shallow_text), Some("f"));
	assert_eq!(iter.next().and_then(Component::shallow_text), Some("g"));
	assert_eq!(iter.next().and_then(Component::shallow_text), Some("h"));

	assert!(iter.next().is_none());
}
