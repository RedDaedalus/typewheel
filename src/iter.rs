use crate::Component;
use std::collections::VecDeque;

/// Determines the order over which a component iterator runs through child components.
///
/// Most use cases will want [depth-first ordering][IterOrder::DepthFirst], so that is the
/// [default][Default::default()].
///
/// # Examples
/// ```rust,no_run
/// # // Proper unit tests for iterators are in crate-level integration tests.
/// # use typewheel::Component;
/// #
/// let component = Component::text("a").with_extra(["b", "c"]);
///
/// for child in component.iter(Default::default()) {
///     println!("{}", child.shallow_content().unwrap_or(""));
/// }
/// ```
#[derive(Default)]
#[non_exhaustive]
pub enum IterOrder {
	/// Depth-first iteration order. When used in an iterator, the tree is traversed by visiting
	/// a component's children first, then advancing to the next sibling. This is the iteration
	/// order that should be used to traverse components in the order that they would be displayed
	/// on-screen.
	///
	/// This is the default order (returned by [Default::default()]), as it is the most commonly
	/// used.
	///
	/// # Examples
	/// Take the following component tree, where each letter denotes a component:
	/// ```monospace
	/// a
	/// ╠═ b
	/// ║  ╚═ c, d
	/// ╠═ e
	/// ╠═ f
	/// ╚═ g, h
	/// ```
	///
	/// The components in this graph will be traversed in this order:
	/// 1. a
	/// 2. b
	/// 3. c
	/// 4. d
	/// 5. e
	/// 6. f
	/// 7. g
	/// 8. h
	#[default]
	DepthFirst,

	/// Breadth-first iteration order. When used in an iterator, components are visited first by
	/// iterating over all children, then moving on to all of their children, etc. until all nodes
	/// have been visited.
	///
	///
	/// # Examples
	/// Take the following component tree, where each letter denotes a component:
	/// ```monospace
	/// a
	/// ╠═ b
	/// ║  ╚═ c, d
	/// ╠═ e
	/// ╠═ f
	/// ╚═ g, h
	/// ```
	///
	/// The components in this graph will be traversed in this order:
	/// 1. a
	/// 2. b
	/// 3. e
	/// 4. c
	/// 5. d
	/// 6. f
	/// 7. g
	/// 8. h
	BreadthFirst,
}

/// An internal iterator type. Eventually, this will not be exposed to the public API at all, and
/// should not be used directly. (waiting on `impl_trait_in_assoc_type` to actually implement this)
#[doc(hidden)]
pub struct ComponentIterator<'a> {
	queue: VecDeque<&'a Component>,
	order: IterOrder,
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

impl Component {
	/// Creates an iterator of [component references][Component].
	pub fn iter(&self, order: IterOrder) -> impl Iterator<Item = &Self> {
		ComponentIterator {
			queue: VecDeque::from([self]),
			order,
		}
	}
}

impl<'a> IntoIterator for &'a Component {
	type Item = Self;
	type IntoIter = ComponentIterator<'a>; // TODO: impl Iterator<Item = Self::Item>

	fn into_iter(self) -> Self::IntoIter {
		ComponentIterator {
			queue: VecDeque::from([self]),
			order: IterOrder::default(),
		}
	}
}
