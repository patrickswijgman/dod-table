use core::array;
use std::slice::{Iter, IterMut};

/// A contiguous memory block.
///
/// # Example:
/// ```
/// // Make sure that the type used in the table implements the Default trait.
/// #[derive(Default)]
/// struct Entity {
///     x: f32,
///     y: f32,   
/// }
///
/// struct Data {
///     // Memory reserved for (in this specific example) 512 entities.
///     entities: Table<Entity, 512>
/// }
/// ```
pub struct Table<T, const N: usize> {
    elements: [T; N],
}

impl<T, const N: usize> Table<T, N>
where
    T: Default,
{
    /// Get a reference to an element at the given index.
    pub fn get(&self, i: usize) -> &T {
        &self.elements[i]
    }

    /// Get a mutable reference to an element at the given index.
    pub fn get_mut(&mut self, i: usize) -> &mut T {
        &mut self.elements[i]
    }

    /// Set an element at the given index.
    pub fn set(&mut self, i: usize, e: T) {
        self.elements[i] = e;
    }

    /// Iterate over references of the elements.
    pub fn iter<P>(&self) -> Iter<T> {
        self.elements.iter()
    }

    /// Iterate over mutable references of the elements.
    pub fn iter_mut<P>(&mut self) -> IterMut<T> {
        self.elements.iter_mut()
    }

    /// Reset the element at the given index back to its default values (AKA zeroing).
    pub fn zero(&mut self, i: usize) {
        self.elements[i] = T::default()
    }
}

impl<T, const N: usize> Default for Table<T, N>
where
    T: Default,
{
    fn default() -> Self {
        Self {
            elements: array::from_fn(|_i| T::default()),
        }
    }
}
