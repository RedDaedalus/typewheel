use crate::Component;
use std::cmp::max;
use std::collections::VecDeque;
use std::iter::FusedIterator;

/// An [Iterator] type that iterates over [Component]s while keeping awareness of their tree-like
/// structure.
///
/// This is accomplished by offering a [Visit] enum with two states -- push and pop. A [Visit::Push]
/// value is emitted when a node is first encountered by the iterator. When it enters the iterator,
/// all of its children are then visited next. After all of its children are complete, a [Visit::Pop]
/// value is emitted.
///
/// This allows for more contextual iteration. For example, here is a quick sample snippet that
/// keeps track of the depth of a node:
/// ```
/// # use typewheel::{Component, iter::Visit};
/// let component = Component::text("A")
///     .with_extra([Component::text("B")
///         .with_extra([Component::text("C")])]);
///
/// let mut depth = 0;
/// for op in component.visit() {
///     match op {
///         Visit::Push(..) => depth += 1,
///         Visit::Pop(..) => depth -= 1,
///     }
///     assert!(depth <= 3);
/// }
/// ```
#[derive(Clone, Debug)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct VisitingIterator<'a> {
	queue: VecDeque<Visit<'a>>,
	size_hint: usize,
}

impl<'a> VisitingIterator<'a> {
	#[inline]
	pub(super) fn new(root: &'a Component) -> Self {
		Self {
			queue: VecDeque::from([Visit::Push(root)]),
			size_hint: root.extra.len(),
		}
	}
}

impl<'a> Iterator for VisitingIterator<'a> {
	type Item = Visit<'a>;

	fn next(&mut self) -> Option<Self::Item> {
		let op = self.queue.pop_front();
		if let Some(Visit::Push(item)) = op {
			let extra = &item.extra;

			self.queue.reserve(extra.len() + 1);
			self.queue.push_front(Visit::Pop(item));

			for child in extra.iter().rev() {
				self.queue.push_front(Visit::Push(child));
			}
		}

		op
	}

	fn size_hint(&self) -> (usize, Option<usize>) {
		(max(self.queue.len(), self.size_hint), None)
	}
}

impl FusedIterator for VisitingIterator<'_> {}

/// Represents an operation in a [VisitingIterator]. See the iterator docs for more information.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Visit<'a> {
	/// Indicates that a component has entered the context of a [VisitingIterator]. After all of its
	/// children have been consumed, a [Self::Pop] value is emitted.
	Push(&'a Component),

	/// Indicates that a component and all of its children have been consumed by a [VisitingIterator].
	Pop(&'a Component),
}
