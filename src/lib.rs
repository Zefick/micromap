// Copyright (c) 2023 Yegor Bugayenko
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included
// in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NON-INFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

//! This is a simpler and faster alternative implementation of the standard `HashMap`.
//! It doesn't use heap and doesn't use hashing at all. It simply keeps all key-value
//! pairs in an array and when it's necessary to retrieve by key, it scrolls through
//! the entire array. This implementation works much faster for small maps of
//! less than 50 keys, but definitely is not suitable for larger maps.
//!
//! Check [this page](https://github.com/yegor256/micromap#benchmark)
//! for the recent benchmarking results.
//!
//! For example, here is how a map with a few keys can be created:
//!
//! ```
//! use micromap::Map;
//! let mut m : Map<u64, &str, 10> = Map::new();
//! m.insert(1, "Hello, world!");
//! m.insert(2, "Good bye!");
//! assert_eq!(2, m.len());
//! ```
//!
//! Creating a [`Map`] requires knowing the maximum size of it, upfront. This is
//! what the third type argument `10` is for, in the example above. The array
//! will have exactly ten elements. An attempt to add an 11th element will lead
//! to a panic.

#![doc(html_root_url = "https://docs.rs/micromap/0.0.0")]
#![deny(warnings)]
#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]
#![allow(clippy::multiple_inherent_impl)]
#![allow(clippy::multiple_crate_versions)]
mod clone;
mod ctors;
mod debug;
mod eq;
mod from;
mod index;
mod iterators;
mod keys;
mod map;
#[cfg(feature = "serde")]
mod serialization;
mod values;

use std::mem::MaybeUninit;

/// A faster alternative of [`std::collections::HashMap`].
///
/// For example, this is how you make a map, which is allocated on stack and is capable of storing
/// up to eight key-values pairs:
///
/// ```
/// let mut m : micromap::Map<u64, &str, 8> = micromap::Map::new();
/// m.insert(1, "Jeff Lebowski");
/// m.insert(2, "Walter Sobchak");
/// assert_eq!(2, m.len());
/// ```
///
/// It is faster because it doesn't use a hash function at all. It simply keeps
/// all pairs in an array and when it's necessary to find a value, it goes through
/// all pairs comparing the needle with each pair available. Also it is faster
/// because it doesn't use heap. When a [`Map`] is being created, it allocates the necessary
/// space on stack. That's why the maximum size of the map must be provided in
/// compile time.
///
/// It is also faster because it doesn't grow in size. When a [`Map`] is created,
/// its size is fixed on stack. If an attempt is made to insert too many keys
/// into it, it simply panics. Moreover, in the "release" mode it doesn't panic,
/// but its behaviour is undefined. In the "release" mode all boundary checks
/// are disabled, for the sake of higher performance.
pub struct Map<K: PartialEq, V, const N: usize> {
    /// The next available pair in the array.
    next: usize,
    /// The fixed-size array of key-value pairs.
    pairs: [MaybeUninit<Option<(K, V)>>; N],
}

/// Iterator over the [`Map`].
pub struct Iter<'a, K, V, const N: usize> {
    /// The next available pair in the array.
    next: usize,
    /// The next position in the iterator to read.
    pos: usize,
    /// The fixed-size array of key-value pairs.
    pairs: &'a [MaybeUninit<Option<(K, V)>>; N],
}

/// Mutable Iterator over the [`Map`].
pub struct IterMut<'a, K, V> {
    next: usize,
    pos: usize,
    iter: core::slice::IterMut<'a, MaybeUninit<Option<(K, V)>>>,
}

/// Into-iterator over the [`Map`].
pub struct IntoIter<K: PartialEq, V, const N: usize> {
    pos: usize,
    map: Map<K, V, N>,
}

/// An iterator over the values of the [`Map`].
pub struct Values<'a, K: PartialEq, V, const N: usize> {
    iter: Iter<'a, K, V, N>,
}

/// Mutable iterator over the values of the [`Map`].
pub struct ValuesMut<'a, K: PartialEq, V> {
    iter: IterMut<'a, K, V>,
}

/// Consuming iterator over the values of the [`Map`].
pub struct IntoValues<K: PartialEq, V, const N: usize> {
    iter: IntoIter<K, V, N>,
}

/// A read-only iterator over the keys of the [`Map`].
pub struct Keys<'a, K: PartialEq, V, const N: usize> {
    iter: Iter<'a, K, V, N>,
}

/// Consuming iterator over the keys of the [`Map`].
pub struct IntoKeys<K: PartialEq, V, const N: usize> {
    iter: IntoIter<K, V, N>,
}
