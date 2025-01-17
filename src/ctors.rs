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

use crate::Map;
use std::mem::MaybeUninit;

impl<K: PartialEq, V, const N: usize> Default for Map<K, V, N> {
    /// Make a default empty [`Map`].
    #[inline]
    #[must_use]
    fn default() -> Self {
        Self::new()
    }
}

impl<K: PartialEq, V, const N: usize> Map<K, V, N> {
    /// Make it.
    ///
    /// The size of the map is defined by the generic argument. For example,
    /// this is how you make a map of four key-values pairs:
    #[inline]
    #[must_use]
    #[allow(clippy::uninit_assumed_init)]
    pub const fn new() -> Self {
        unsafe {
            Self {
                next: 0,
                pairs: MaybeUninit::<[MaybeUninit<Option<(K, V)>>; N]>::uninit().assume_init(),
            }
        }
    }
}

impl<K: PartialEq, V, const N: usize> Drop for Map<K, V, N> {
    fn drop(&mut self) {
        for i in 0..self.next {
            unsafe {
                self.pairs[i].assume_init_drop();
            }
        }
    }
}

#[test]
fn makes_default_map() {
    let m: Map<u8, u8, 8> = Map::default();
    assert_eq!(0, m.len());
}

#[test]
fn makes_new_map() {
    let m: Map<u8, u8, 8> = Map::new();
    assert_eq!(0, m.len());
}

#[test]
fn drops_correctly() {
    let _m: Map<Vec<u8>, u8, 8> = Map::new();
}

#[test]
fn drops_keys() {
    use std::rc::Rc;
    let mut m: Map<Rc<()>, (), 8> = Map::new();
    let k = Rc::new(());
    m.insert(Rc::clone(&k), ());
    drop(m);
    assert_eq!(Rc::strong_count(&k), 1);
}

#[test]
fn drops_values() {
    use std::rc::Rc;
    let mut m: Map<(), Rc<()>, 8> = Map::new();
    let v = Rc::new(());
    m.insert((), Rc::clone(&v));
    drop(m);
    assert_eq!(Rc::strong_count(&v), 1);
}
