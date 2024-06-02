use core::array;

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

    /// Find the first element that satisfies the given predicate.
    pub fn find<P>(&self, predicate: P) -> Option<&T>
    where
        P: Fn(&T) -> bool,
    {
        self.elements.iter().find(|e| predicate(e))
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
