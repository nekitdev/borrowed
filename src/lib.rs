//! Postfix borrowing.
//!
//! This crate provides the [`Borrowed`] trait, implemented for every type,
//! including unsized ones such as [`str`] and slices.
//!
//! The methods are intentionally _not_ called `by_ref` and `by_mut`, since `by_ref`
//! would shadow [`Iterator::by_ref`] and similar methods during method resolution.
//!
//! # Examples
//!
//! Here is a contrived example:
//!
//! ```
//! use borrowed::Borrowed;
//!
//! let mut values = vec![13, 42];
//!
//! values.borrowed_mut().push(69);
//!
//! assert_eq!(values.borrowed().len(), 3);
//! ```

#![no_std]

/// Postfix borrowing.
///
/// This trait provides [`borrowed`] and [`borrowed_mut`] methods, which take `&self`
/// and `&mut self` as their receivers respectively, and simply return `self`.
///
/// These methods can be used in method chains and other postfix positions.
///
/// Note that due to method resolution, calling [`borrowed`] on `&T` returns `&T` rather
/// than `&&T`, and the same applies to [`borrowed_mut`] on `&mut T`.
///
/// [`borrowed`]: Self::borrowed
/// [`borrowed_mut`]: Self::borrowed_mut
pub trait Borrowed {
    /// Borrows `self` immutably, equivalent to using `&self`
    /// as the receiver and simply returning `self`.
    ///
    /// # Examples
    ///
    /// ```
    /// use borrowed::Borrowed;
    ///
    /// let nekit = "nekit";
    ///
    /// let string = nekit.to_owned();
    ///
    /// let borrowed = string.borrowed();
    ///
    /// assert_eq!(borrowed, nekit);
    /// ```
    #[must_use]
    fn borrowed(&self) -> &Self {
        self
    }

    /// Borrows `self` mutably, equivalent to using `&mut self`
    /// as the receiver and simply returning `self`.
    ///
    /// # Examples
    ///
    /// ```
    /// use borrowed::Borrowed;
    ///
    /// let mut count = 12;
    ///
    /// *count.borrowed_mut() += 1;
    ///
    /// assert_eq!(count, 13);
    /// ```
    #[must_use]
    fn borrowed_mut(&mut self) -> &mut Self {
        self
    }
}

/// [`Borrowed`] is implemented for every `T`, [`Sized`] or not.
impl<T: ?Sized> Borrowed for T {}
