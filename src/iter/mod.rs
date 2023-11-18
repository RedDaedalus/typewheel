//! A module containing two iterator types: [FlatIterator] and [VisitingIterator]. These types
//! provide support for iterating over components and their children (extra).
//!
//! Component iterators can be accessed in 3 ways:
//! * [Component::iter()] – Provides an implementation of [`Iterator<Item = &Component>`][Iterator].
//! * [Component::into_iter()] – Same as above but useful for use in `for` loops.
//! * [Component::visit()] – Provides a [VisitingIterator] instance. See the struct docs for more
//! information.

mod tree;
mod visit;

pub use self::{
	tree::FlatIterator,
	visit::{Visit, VisitingIterator},
};
use crate::Component;

/// Determines the order over which a component iterator runs through child components.
///
/// Most use cases will want [depth-first ordering][IterOrder::DepthFirst], so that is the
/// [default][Default::default()].
///
// # Examples
/// ```rust,no_run
/// # // Proper unit tests for iterators are in crate-level integration tests.
/// # use typewheel::Component;
/// #
/// let component = Component::text("a").with_extra(["b", "c"]);
///
/// for child in component.iter() {
///     println!("{}", child.shallow_text().unwrap_or(""));
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

impl Component {
	/// Creates an iterator of [component references][Component]. Components are traversed in the
	/// provided order.
	///
	/// # Usage
	/// Normally, iterating over components can be done with a for-in loop. This uses the default
	/// [IterOrder::DepthFirst] ordering:
	/// ```rust,no_run
	/// # use typewheel::Component;
	/// use typewheel::iter::IterOrder;
	/// #
	/// let component = Component::text("a").with_extra(["b", "c"]);
	/// for node in component.iter().with_translate_args() {
	///     println!("{component:?}");
	/// }
	/// ```
	#[inline(always)]
	pub fn iter(&self) -> FlatIterator {
		FlatIterator::new(self)
	}

	/// Creates an iterator that traverses through the component tree while preserving depth context.
	/// This is accomplished via the [Visit] enum; when a node is *entered*, a [Visit::Push] enum
	/// variant is emitted. After all of its children have been iterated over, a [Visit::Pop] value
	/// is emitted. This allows the caller to keep track of state across the entire component.
	///
	/// This method returns an [Iterator] of [Visit] items.
	#[inline(always)]
	pub fn visit(&self) -> VisitingIterator {
		VisitingIterator::new(self)
	}
}
