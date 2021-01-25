//! Make any type have some fun.
//!
//! This is convenient when you want to insert
//! some function call in a chain without reordering
//! it or create intermediate variables.
//!
//! ```
//! use have::Fun;
//!
//! let sum = [1, 2, 3]
//!     .fun(|v| println!("v is size {}", v.len()))
//!     .iter()
//!     .sum::<usize>();
//!
//! ```

pub trait Fun {
    fn fun<F, D>(self, f: F) -> Self where F: FnMut(&Self) -> D;
}

impl<T> Fun for T {
    fn fun<F, D>(self, mut f: F) -> Self where F: FnMut(&Self) -> D {
        f(&self);
        self
    }
}
