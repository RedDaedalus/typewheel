use crate::Component;
use std::collections::VecDeque;

pub(crate) struct ComponentIterator<'a> {
	pub queue: VecDeque<&'a Component>,
	pub order: IterOrder,
}

#[derive(Default)]
#[non_exhaustive]
pub enum IterOrder {
	#[default]
	DepthFirst,
	BreadthFirst,
}

impl<'a> Iterator for ComponentIterator<'a> {
	type Item = &'a Component;

	fn next(&mut self) -> Option<Self::Item> {
		if let Some(item) = self.queue.pop_front() {
			match self.order {
				IterOrder::DepthFirst => {
					for child in item.extra.iter().rev() {
						self.queue.push_front(child);
					}
				}

				IterOrder::BreadthFirst => {
					for child in &item.extra {
						self.queue.push_back(child);
					}
				}
			}

			Some(item)
		} else {
			None
		}
	}
}

#[cfg(test)]
mod tests {
	use super::{ComponentIterator, IterOrder};
	use crate::Component;
	use std::collections::VecDeque;

	/*
	a
	|- b
	|  |- c, d
	|- e
	   |- f
		  |- g, h
	 */

	fn test_component() -> Component {
		Component::text("a").with_extra([
			Component::text("b").with_extra(["c", "d"]),
			Component::text("e").with_extra([Component::text("f").with_extra(["g", "h"])]),
		])
	}

	#[test]
	fn depth_first_iter() {
		let component = test_component();

		let mut iter = ComponentIterator {
			queue: VecDeque::from([&component]),
			order: IterOrder::DepthFirst,
		};

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
	fn breadth_first_iter() {
		let component = test_component();

		let mut iter = ComponentIterator {
			queue: VecDeque::from([&component]),
			order: IterOrder::BreadthFirst,
		};

		assert_eq!(iter.next().and_then(Component::shallow_content), Some("a"));
		assert_eq!(iter.next().and_then(Component::shallow_content), Some("b"));
		assert_eq!(iter.next().and_then(Component::shallow_content), Some("e"));
		assert_eq!(iter.next().and_then(Component::shallow_content), Some("c"));
		assert_eq!(iter.next().and_then(Component::shallow_content), Some("d"));
		assert_eq!(iter.next().and_then(Component::shallow_content), Some("f"));
		assert_eq!(iter.next().and_then(Component::shallow_content), Some("g"));
		assert_eq!(iter.next().and_then(Component::shallow_content), Some("h"));
	}
}
