use super::IterOrder;
use crate::{Component, Content};
use std::cmp::max;
use std::collections::VecDeque;
use std::iter::FusedIterator;

/// A variable-order iterator over [Component] trees. This iterator does *not* preserve information
/// about the "shape" of the component tree. For a more structured iterator, use a [visiting
/// iterator][visit].
///
/// [visit]: crate::iter::VisitingIterator;
#[derive(Clone, Debug)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct FlatIterator<'a> {
	queue: VecDeque<&'a Component>,
	order: IterOrder,
	include_translate_args: bool,

	/// A lower bound for the iterator size hint. This is a conservative estimate and does not
	/// represent the true size of the iterator. Default is extra.len().
	size_hint: usize,
}

impl<'a> FlatIterator<'a> {
	#[inline]
	pub(super) fn new(root: &'a Component) -> Self {
		Self {
			queue: VecDeque::from([root]),
			order: IterOrder::default(),
			include_translate_args: false,
			size_hint: root.extra.len(),
		}
	}
}

impl FlatIterator<'_> {
	/// Sets a flag to include [translation arguments][crate::Content::Translation] in the output
	/// of this iterator.
	///
	/// # Examples
	/// ```
	/// use typewheel::Component;
	///
	/// let user_arg = "user";
	/// let task_arg = "task";
	/// let component = Component::translate("chat.type.advancement.task", [user_arg, task_arg]);
	///
	/// let mut iter = component.iter().with_translate_args();
	/// assert_eq!(iter.next(), Some(&component));
	/// assert_eq!(iter.next(), Some(&Component::text(user_arg)));
	/// assert_eq!(iter.next(), Some(&Component::text(task_arg)));
	/// assert_eq!(iter.next(), None);
	/// ```
	pub fn with_translate_args(mut self) -> Self {
		self.include_translate_args = true;
		self
	}

	/// Sets the [iteration order][IterOrder] for this iterator. See the [IterOrder] docs for more
	/// information.
	pub fn with_order(mut self, order: IterOrder) -> Self {
		self.order = order;
		self
	}
}

impl<'a> Iterator for FlatIterator<'a> {
	type Item = &'a Component;

	fn next(&mut self) -> Option<Self::Item> {
		let next = self.queue.pop_front();
		if let Some(item) = next {
			let extra = &item.extra;

			match self.order {
				IterOrder::BreadthFirst => {
					if let (true, Content::Translation { with: args, .. }) =
						(self.include_translate_args, &item.content)
					{
						// Reserve room for args and extra all at once.
						self.queue.reserve(args.len() + extra.len());
						self.queue.extend(args);
					}

					self.queue.extend(extra);
				}

				IterOrder::DepthFirst => {
					if let (true, Content::Translation { with: args, .. }) =
						(self.include_translate_args, &item.content)
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
		}

		next
	}

	fn size_hint(&self) -> (usize, Option<usize>) {
		// It /is/ possible to compute the full tree size of a component, but because components are
		// deserialized much more often than they are iterated, so the cost of traveling the tree
		// again after serialization doesn't make sense.

		// Consider using a custom JSON deserializer to compute the proper size at parse time
		(max(self.size_hint, self.queue.len()), None)
	}
}

impl FusedIterator for FlatIterator<'_> {}

impl<'a> IntoIterator for &'a Component {
	type Item = &'a Component;
	type IntoIter = FlatIterator<'a>;

	#[inline(always)]
	fn into_iter(self) -> Self::IntoIter {
		self.iter()
	}
}
