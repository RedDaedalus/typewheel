use typewheel::{Component, IterOrder};
mod common;

#[test]
fn depth_first_iter() {
	let component = common::deeply_nested();
	let mut iter = component.iter(IterOrder::DepthFirst);

	assert_eq!(iter.next().and_then(Component::shallow_content), Some("a"));
	assert_eq!(iter.next().and_then(Component::shallow_content), Some("b"));
	assert_eq!(iter.next().and_then(Component::shallow_content), Some("c"));
	assert_eq!(iter.next().and_then(Component::shallow_content), Some("d"));
	assert_eq!(iter.next().and_then(Component::shallow_content), Some("e"));
	assert_eq!(iter.next().and_then(Component::shallow_content), Some("f"));
	assert_eq!(iter.next().and_then(Component::shallow_content), Some("g"));
	assert_eq!(iter.next().and_then(Component::shallow_content), Some("h"));

	assert!(iter.next().is_none());
}

#[test]
fn for_each_test() {
	for node in &common::deeply_nested() {
		println!("{node:?}");
	}
}

#[test]
fn breadth_first_iter() {
	let component = common::deeply_nested();
	let mut iter = component.iter(IterOrder::BreadthFirst);

	assert_eq!(iter.next().and_then(Component::shallow_content), Some("a"));
	assert_eq!(iter.next().and_then(Component::shallow_content), Some("b"));
	assert_eq!(iter.next().and_then(Component::shallow_content), Some("e"));
	assert_eq!(iter.next().and_then(Component::shallow_content), Some("c"));
	assert_eq!(iter.next().and_then(Component::shallow_content), Some("d"));
	assert_eq!(iter.next().and_then(Component::shallow_content), Some("f"));
	assert_eq!(iter.next().and_then(Component::shallow_content), Some("g"));
	assert_eq!(iter.next().and_then(Component::shallow_content), Some("h"));

	assert!(iter.next().is_none());
}
