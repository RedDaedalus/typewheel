use crate::{Component, Content, IterOrder};
use std::collections::VecDeque;

pub struct TreeIterator<'a> {
	queue: VecDeque<&'a Component>,
	order: IterOrder,
	include_translate_args: bool,
}

impl TreeIterator<'_> {
	pub fn with_translate_args(mut self) -> Self {
		self.include_translate_args = true;
		self
	}
}

impl<'a> Iterator for TreeIterator<'a> {
	type Item = &'a Component;

	fn next(&mut self) -> Option<Self::Item> {
		if let Some(item) = self.queue.pop_front() {
			let extra = item.extra();

			match self.order {
				IterOrder::BreadthFirst => {
					if self.include_translate_args
						&& let Content::Translation { with: args, .. } = item.content()
					{
						// Reserve room for args and extra all at once.
						self.queue.reserve(args.len() + extra.len());
						self.queue.extend(args);
					}

					self.queue.extend(extra);
				}

				IterOrder::DepthFirst => {
					if self.include_translate_args
						&& let Content::Translation { with: args, .. } = item.content()
					{
						self.queue.reserve(args.len() + extra.len());

						for arg in args.iter().rev() {
							self.queue.push_front(arg);
						}
					} else {
						self.queue.reserve(extra.len());
					}

					// Reversed because we're pushing elements to the front. They end up in the
					// right order in the actual queue.
					for child in extra.iter().rev() {
						self.queue.push_front(child);
					}
				}
			}

			Some(item)
		} else {
			None
		}
	}

	fn size_hint(&self) -> (usize, Option<usize>) {
		// It /is/ possible to compute the full tree size of a component, but because components are
		// deserialized much more often than they are iterated, so the cost of traveling the tree
		// again after serialization doesn't make sense.

		// TODO: Consider using a custom JSON deserializer to compute the proper size at parse time
		(self.queue.len(), None)
	}
}

impl Component {
	/// Creates an iterator of [component references][Component]. Components are traversed in the
	/// provided order.
	///
	/// # Usage
	/// Normally, iterating over components can be done with a for-in loop. This uses the default
	/// [IterOrder::DepthFirst] ordering:
	/// ```rust,no_run
	/// # use typewheel::Component;
	/// #
	/// let component = Component::text("a").with_extra(["b", "c"]);
	/// for node in &component {
	///     println!("{component:?}");
	/// }
	/// ```
	pub fn iter(&self, order: IterOrder) -> TreeIterator {
		TreeIterator {
			queue: VecDeque::from([self]),
			order,
			include_translate_args: false,
		}
	}
}

impl<'a> IntoIterator for &'a Component {
	type Item = &'a Component;
	type IntoIter = TreeIterator<'a>;

	#[inline(always)]
	fn into_iter(self) -> Self::IntoIter {
		self.iter(IterOrder::default())
	}
}
