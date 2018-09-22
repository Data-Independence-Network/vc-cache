//! A contiguous growable array type with heap-allocated contents, written
//! `Vec<T>`.
//!
//! Vectors have `O(1)` indexing, amortized `O(1)` push (to the end) and
//! `O(1)` pop (from the end).
//!
//! # Examples
//!
//! You can explicitly create a [`Vec<T>`] with [`new`]:
//!
//! ```
//! let v: Vec<i32> = Vec::new();
//! ```
//!
//! ...or by using the [`vec!`] macro:
//!
//! ```
//! let v: Vec<i32> = vec![];
//!
//! let v = vec![1, 2, 3, 4, 5];
//!
//! let v = vec![0; 10]; // ten zeroes
//! ```
//!
//! You can [`push`] values onto the end of a vector (which will grow the vector
//! as needed):
//!
//! ```
//! let mut v = vec![1, 2];
//!
//! v.push(3);
//! ```
//!
//! Vectors also support indexing (through the [`Index`] and [`IndexMut`] traits):
//!
//! ```
//! let mut v = vec![1, 2, 3];
//! let three = v[2];
//! v[1] = v[1] + 5;
//! ```


use core::cmp::{self, Ordering};
use core::fmt;
use core::hash::{self, Hash};
use core::intrinsics::{arith_offset, assume};
use core::iter::{FromIterator, FusedIterator, TrustedLen};
use core::marker::PhantomData;
use core::mem;
use core::ops::Bound::{Excluded, Included, Unbounded};
use core::ops::{Index, IndexMut, RangeBounds};
use core::ops;
use core::ptr;
use core::ptr::NonNull;
use core::slice;

use collections::CollectionAllocErr;
use borrow::ToOwned;
use borrow::Cow;
use boxed::Box;
use raw_vec::RawVec;


pub struct Vec<T> {
    buf: RawVec<T>,
    len: usize,
}

////////////////////////////////////////////////////////////////////////////////
// Inherent methods
////////////////////////////////////////////////////////////////////////////////

impl<T> Vec<T> {


    #[inline]
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn with_8(capacity: usize) -> Vec<T> {
        Vec {
            buf: RawVec::with_capacity(capacity),
            len: 0,
        }
    }


    #[inline]
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn capacity(&self) -> usize {
        self.buf.cap()
    }

    pub fn reserve(&mut self) {
        self.buf.reserve(self.len, 8);
    }


    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn reserve_exact(&mut self, additional: usize) {
        self.buf.reserve_exact(self.len, additional);
    }



    /// Converts the vector into [`Box<[T]>`][owned slice].
    ///
    /// Note that this will drop any excess capacity.
    ///
    /// [owned slice]: ../../std/boxed/struct.Box.html
    ///
    /// # Examples
    ///
    /// ```
    /// let v = vec![1, 2, 3];
    ///
    /// let slice = v.into_boxed_slice();
    /// ```
    ///
    /// Any excess capacity is removed:
    ///
    /// ```
    /// let mut vec = Vec::with_capacity(10);
    /// vec.extend([1, 2, 3].iter().cloned());
    ///
    /// assert_eq!(vec.capacity(), 10);
    /// let slice = vec.into_boxed_slice();
    /// assert_eq!(slice.into_vec().capacity(), 3);
    /// ```
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn into_boxed_slice(mut self) -> Box<[T]> {
        unsafe {
            self.shrink_to_fit();
            let buf = ptr::read(&self.buf);
            mem::forget(self);
            buf.into_box()
        }
    }


    /// Extracts a slice containing the entire vector.
    ///
    /// Equivalent to `&s[..]`.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::io::{self, Write};
    /// let buffer = vec![1, 2, 3, 5, 8];
    /// io::sink().write(buffer.as_slice()).unwrap();
    /// ```
    #[inline]
    #[stable(feature = "vec_as_slice", since = "1.7.0")]
    pub fn as_slice(&self) -> &[T] {
        self
    }


    /// Appends an element to the back of a collection.
    ///
    /// # Panics
    ///
    /// Panics if the number of elements in the vector overflows a `usize`.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut vec = vec![1, 2];
    /// vec.push(3);
    /// assert_eq!(vec, [1, 2, 3]);
    /// ```
    #[inline]
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn push(&mut self, value: T) {
        // This will panic or abort if we would allocate > isize::MAX bytes
        // or if the length increment would overflow for zero-sized types.
        if self.len == self.buf.cap() {
            self.reserve(1);
        }
        unsafe {
            let end = self.as_mut_ptr().offset(self.len as isize);
            ptr::write(end, value);
            self.len += 1;
        }
    }

    /// Appends elements to `Self` from other buffer.
    #[inline]
    unsafe fn append_elements(&mut self, other: *const [T]) {
        let count = (*other).len();
        self.reserve(count);
        let len = self.len();
        ptr::copy_nonoverlapping(other as *const T, self.get_unchecked_mut(len), count);
        self.len += count;
    }

    // TODO: check modified
    pub fn truncate(&mut self) {
        unsafe {
            // drop any extra elements
            while 8 < self.len {
                // decrement len before the drop_in_place(), so a panic on Drop
                // doesn't re-drop the just-failed value.
                self.len -= 1;
                let len = self.len;
                ptr::drop_in_place(self.get_unchecked_mut(len));
            }
        }
    }

    /// Returns the number of elements in the vector, also referred to
    /// as its 'length'.
    ///
    /// # Examples
    ///
    /// ```
    /// let a = vec![1, 2, 3];
    /// assert_eq!(a.len(), 3);
    /// ```
    #[inline]
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn len(&self) -> usize {
        self.len
    }

}



// This code generalises `extend_with_{element,default}`.
trait ExtendWith<T> {
    fn next(&mut self) -> T;
    fn last(self) -> T;
}

struct ExtendElement<T>(T);
impl<T: Clone> ExtendWith<T> for ExtendElement<T> {
    fn next(&mut self) -> T { self.0.clone() }
    fn last(self) -> T { self.0 }
}

struct ExtendDefault;
impl<T: Default> ExtendWith<T> for ExtendDefault {
    fn next(&mut self) -> T { Default::default() }
    fn last(self) -> T { Default::default() }
}

struct ExtendFunc<F>(F);
impl<T, F: FnMut() -> T> ExtendWith<T> for ExtendFunc<F> {
    fn next(&mut self) -> T { (self.0)() }
    fn last(mut self) -> T { (self.0)() }
}


////////////////////////////////////////////////////////////////////////////////
// Internal methods and functions
////////////////////////////////////////////////////////////////////////////////

#[doc(hidden)]
#[stable(feature = "rust1", since = "1.0.0")]
pub fn from_elem<T: Clone>(elem: T, n: usize) -> Vec<T> {
    <T as SpecFromElem>::from_elem(elem, n)
}

// Specialization trait used for Vec::from_elem
trait SpecFromElem: Sized {
    fn from_elem(elem: Self, n: usize) -> Vec<Self>;
}

impl<T: Clone> SpecFromElem for T {
    default fn from_elem(elem: Self, n: usize) -> Vec<Self> {
        let mut v = Vec::with_capacity(n);
        v.extend_with(n, ExtendElement(elem));
        v
    }
}

impl SpecFromElem for u8 {
    #[inline]
    fn from_elem(elem: u8, n: usize) -> Vec<u8> {
        if elem == 0 {
            return Vec {
                buf: RawVec::with_capacity_zeroed(n),
                len: n,
            }
        }
        unsafe {
            let mut v = Vec::with_capacity(n);
            ptr::write_bytes(v.as_mut_ptr(), elem, n);
            v.set_len(n);
            v
        }
    }
}

impl<T: Clone + IsZero> SpecFromElem for T {
    #[inline]
    fn from_elem(elem: T, n: usize) -> Vec<T> {
        if elem.is_zero() {
            return Vec {
                buf: RawVec::with_capacity_zeroed(n),
                len: n,
            }
        }
        let mut v = Vec::with_capacity(n);
        v.extend_with(n, ExtendElement(elem));
        v
    }
}

unsafe trait IsZero {
    /// Whether this value is zero
    fn is_zero(&self) -> bool;
}


////////////////////////////////////////////////////////////////////////////////
// Common trait implementations for Vec
////////////////////////////////////////////////////////////////////////////////


#[stable(feature = "rust1", since = "1.0.0")]
#[rustc_on_unimplemented(
message="vector indices are of type `usize` or ranges of `usize`",
label="vector indices are of type `usize` or ranges of `usize`",
)]
impl<T, I> Index<I> for Vec<T>
    where
        I: ::core::slice::SliceIndex<[T]>,
{
    type Output = I::Output;

    #[inline]
    fn index(&self, index: I) -> &Self::Output {
        Index::index(&**self, index)
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
#[rustc_on_unimplemented(
message="vector indices are of type `usize` or ranges of `usize`",
label="vector indices are of type `usize` or ranges of `usize`",
)]
impl<T, I> IndexMut<I> for Vec<T>
    where
        I: ::core::slice::SliceIndex<[T]>,
{
    #[inline]
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        IndexMut::index_mut(&mut **self, index)
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<T> ops::Deref for Vec<T> {
    type Target = [T];

    fn deref(&self) -> &[T] {
        unsafe {
            let p = self.buf.ptr();
            assume(!p.is_null());
            slice::from_raw_parts(p, self.len)
        }
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<T> ops::DerefMut for Vec<T> {
    fn deref_mut(&mut self) -> &mut [T] {
        unsafe {
            let ptr = self.buf.ptr();
            assume(!ptr.is_null());
            slice::from_raw_parts_mut(ptr, self.len)
        }
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<T> FromIterator<T> for Vec<T> {
    #[inline]
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Vec<T> {
        <Self as SpecExtend<T, I::IntoIter>>::from_iter(iter.into_iter())
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<T> IntoIterator for Vec<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    /// Creates a consuming iterator, that is, one that moves each value out of
    /// the vector (from start to end). The vector cannot be used after calling
    /// this.
    ///
    /// # Examples
    ///
    /// ```
    /// let v = vec!["a".to_string(), "b".to_string()];
    /// for s in v.into_iter() {
    ///     // s has type String, not &String
    ///     println!("{}", s);
    /// }
    /// ```
    #[inline]
    fn into_iter(mut self) -> IntoIter<T> {
        unsafe {
            let begin = self.as_mut_ptr();
            assume(!begin.is_null());
            let end = if mem::size_of::<T>() == 0 {
                arith_offset(begin as *const i8, self.len() as isize) as *const T
            } else {
                begin.offset(self.len() as isize) as *const T
            };
            let cap = self.buf.cap();
            mem::forget(self);
            IntoIter {
                buf: NonNull::new_unchecked(begin),
                phantom: PhantomData,
                cap,
                ptr: begin,
                end,
            }
        }
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<'a, T> IntoIterator for &'a Vec<T> {
    type Item = &'a T;
    type IntoIter = slice::Iter<'a, T>;

    fn into_iter(self) -> slice::Iter<'a, T> {
        self.iter()
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<'a, T> IntoIterator for &'a mut Vec<T> {
    type Item = &'a mut T;
    type IntoIter = slice::IterMut<'a, T>;

    fn into_iter(self) -> slice::IterMut<'a, T> {
        self.iter_mut()
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<T> Extend<T> for Vec<T> {
    #[inline]
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        <Self as SpecExtend<T, I::IntoIter>>::spec_extend(self, iter.into_iter())
    }
}

// Specialization trait used for Vec::from_iter and Vec::extend
trait SpecExtend<T, I> {
    fn from_iter(iter: I) -> Self;
    fn spec_extend(&mut self, iter: I);
}

impl<T, I> SpecExtend<T, I> for Vec<T>
    where I: Iterator<Item=T>,
{
    default fn from_iter(mut iterator: I) -> Self {
        // Unroll the first iteration, as the vector is going to be
        // expanded on this iteration in every case when the iterable is not
        // empty, but the loop in extend_desugared() is not going to see the
        // vector being full in the few subsequent loop iterations.
        // So we get better branch prediction.
        let mut vector = match iterator.next() {
            None => return Vec::new(),
            Some(element) => {
                let (lower, _) = iterator.size_hint();
                let mut vector = Vec::with_capacity(lower.saturating_add(1));
                unsafe {
                    ptr::write(vector.get_unchecked_mut(0), element);
                    vector.set_len(1);
                }
                vector
            }
        };
        <Vec<T> as SpecExtend<T, I>>::spec_extend(&mut vector, iterator);
        vector
    }

    default fn spec_extend(&mut self, iter: I) {
        self.extend_desugared(iter)
    }
}

impl<T, I> SpecExtend<T, I> for Vec<T>
    where I: TrustedLen<Item=T>,
{
    default fn from_iter(iterator: I) -> Self {
        let mut vector = Vec::new();
        vector.spec_extend(iterator);
        vector
    }

    default fn spec_extend(&mut self, iterator: I) {
        // This is the case for a TrustedLen iterator.
        let (low, high) = iterator.size_hint();
        if let Some(high_value) = high {
            debug_assert_eq!(low, high_value,
                             "TrustedLen iterator's size hint is not exact: {:?}",
                             (low, high));
        }
        if let Some(additional) = high {
            self.reserve(additional);
            unsafe {
                let mut ptr = self.as_mut_ptr().offset(self.len() as isize);
                let mut local_len = SetLenOnDrop::new(&mut self.len);
                for element in iterator {
                    ptr::write(ptr, element);
                    ptr = ptr.offset(1);
                    // NB can't overflow since we would have had to alloc the address space
                    local_len.increment_len(1);
                }
            }
        } else {
            self.extend_desugared(iterator)
        }
    }
}

impl<T> SpecExtend<T, IntoIter<T>> for Vec<T> {
    fn from_iter(iterator: IntoIter<T>) -> Self {
        // A common case is passing a vector into a function which immediately
        // re-collects into a vector. We can short circuit this if the IntoIter
        // has not been advanced at all.
        if iterator.buf.as_ptr() as *const _ == iterator.ptr {
            unsafe {
                let vec = Vec::from_raw_parts(iterator.buf.as_ptr(),
                                              iterator.len(),
                                              iterator.cap);
                mem::forget(iterator);
                vec
            }
        } else {
            let mut vector = Vec::new();
            vector.spec_extend(iterator);
            vector
        }
    }

    fn spec_extend(&mut self, mut iterator: IntoIter<T>) {
        unsafe {
            self.append_elements(iterator.as_slice() as _);
        }
        iterator.ptr = iterator.end;
    }
}

impl<'a, T: 'a, I> SpecExtend<&'a T, I> for Vec<T>
    where I: Iterator<Item=&'a T>,
          T: Clone,
{
    default fn from_iter(iterator: I) -> Self {
        SpecExtend::from_iter(iterator.cloned())
    }

    default fn spec_extend(&mut self, iterator: I) {
        self.spec_extend(iterator.cloned())
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
unsafe impl<#[may_dangle] T> Drop for Vec<T> {
    fn drop(&mut self) {
        unsafe {
            // use drop for [T]
            ptr::drop_in_place(&mut self[..]);
        }
        // RawVec handles deallocation
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<T> Default for Vec<T> {
    /// Creates an empty `Vec<T>`.
    fn default() -> Vec<T> {
        Vec::new()
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<T: fmt::Debug> fmt::Debug for Vec<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&**self, f)
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<T> AsRef<Vec<T>> for Vec<T> {
    fn as_ref(&self) -> &Vec<T> {
        self
    }
}

#[stable(feature = "vec_as_mut", since = "1.5.0")]
impl<T> AsMut<Vec<T>> for Vec<T> {
    fn as_mut(&mut self) -> &mut Vec<T> {
        self
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<T> AsRef<[T]> for Vec<T> {
    fn as_ref(&self) -> &[T] {
        self
    }
}

#[stable(feature = "vec_as_mut", since = "1.5.0")]
impl<T> AsMut<[T]> for Vec<T> {
    fn as_mut(&mut self) -> &mut [T] {
        self
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<'a, T: Clone> From<&'a [T]> for Vec<T> {
    #[cfg(not(test))]
    fn from(s: &'a [T]) -> Vec<T> {
        s.to_vec()
    }
    #[cfg(test)]
    fn from(s: &'a [T]) -> Vec<T> {
        ::slice::to_vec(s)
    }
}

#[stable(feature = "vec_from_mut", since = "1.19.0")]
impl<'a, T: Clone> From<&'a mut [T]> for Vec<T> {
    #[cfg(not(test))]
    fn from(s: &'a mut [T]) -> Vec<T> {
        s.to_vec()
    }
    #[cfg(test)]
    fn from(s: &'a mut [T]) -> Vec<T> {
        ::slice::to_vec(s)
    }
}

#[stable(feature = "vec_from_cow_slice", since = "1.14.0")]
impl<'a, T> From<Cow<'a, [T]>> for Vec<T> where [T]: ToOwned<Owned=Vec<T>> {
    fn from(s: Cow<'a, [T]>) -> Vec<T> {
        s.into_owned()
    }
}

// note: test pulls in libstd, which causes errors here
#[cfg(not(test))]
#[stable(feature = "vec_from_box", since = "1.18.0")]
impl<T> From<Box<[T]>> for Vec<T> {
    fn from(s: Box<[T]>) -> Vec<T> {
        s.into_vec()
    }
}

// note: test pulls in libstd, which causes errors here
#[cfg(not(test))]
#[stable(feature = "box_from_vec", since = "1.20.0")]
impl<T> From<Vec<T>> for Box<[T]> {
    fn from(v: Vec<T>) -> Box<[T]> {
        v.into_boxed_slice()
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<'a> From<&'a str> for Vec<u8> {
    fn from(s: &'a str) -> Vec<u8> {
        From::from(s.as_bytes())
    }
}

////////////////////////////////////////////////////////////////////////////////
// Clone-on-write
////////////////////////////////////////////////////////////////////////////////

#[stable(feature = "cow_from_vec", since = "1.8.0")]
impl<'a, T: Clone> From<&'a [T]> for Cow<'a, [T]> {
    fn from(s: &'a [T]) -> Cow<'a, [T]> {
        Cow::Borrowed(s)
    }
}

#[stable(feature = "cow_from_vec", since = "1.8.0")]
impl<'a, T: Clone> From<Vec<T>> for Cow<'a, [T]> {
    fn from(v: Vec<T>) -> Cow<'a, [T]> {
        Cow::Owned(v)
    }
}

#[stable(feature = "cow_from_vec_ref", since = "1.28.0")]
impl<'a, T: Clone> From<&'a Vec<T>> for Cow<'a, [T]> {
    fn from(v: &'a Vec<T>) -> Cow<'a, [T]> {
        Cow::Borrowed(v.as_slice())
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<'a, T> FromIterator<T> for Cow<'a, [T]> where T: Clone {
    fn from_iter<I: IntoIterator<Item = T>>(it: I) -> Cow<'a, [T]> {
        Cow::Owned(FromIterator::from_iter(it))
    }
}

////////////////////////////////////////////////////////////////////////////////
// Iterators
////////////////////////////////////////////////////////////////////////////////

/// An iterator that moves out of a vector.
///
/// This `struct` is created by the `into_iter` method on [`Vec`][`Vec`] (provided
/// by the [`IntoIterator`] trait).
///
/// [`Vec`]: struct.Vec.html
/// [`IntoIterator`]: ../../std/iter/trait.IntoIterator.html
#[stable(feature = "rust1", since = "1.0.0")]
pub struct IntoIter<T> {
    buf: NonNull<T>,
    phantom: PhantomData<T>,
    cap: usize,
    ptr: *const T,
    end: *const T,
}

#[stable(feature = "vec_intoiter_debug", since = "1.13.0")]
impl<T: fmt::Debug> fmt::Debug for IntoIter<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("IntoIter")
            .field(&self.as_slice())
            .finish()
    }
}

impl<T> IntoIter<T> {
    /// Returns the remaining items of this iterator as a slice.
    ///
    /// # Examples
    ///
    /// ```
    /// let vec = vec!['a', 'b', 'c'];
    /// let mut into_iter = vec.into_iter();
    /// assert_eq!(into_iter.as_slice(), &['a', 'b', 'c']);
    /// let _ = into_iter.next().unwrap();
    /// assert_eq!(into_iter.as_slice(), &['b', 'c']);
    /// ```
    #[stable(feature = "vec_into_iter_as_slice", since = "1.15.0")]
    pub fn as_slice(&self) -> &[T] {
        unsafe {
            slice::from_raw_parts(self.ptr, self.len())
        }
    }

    /// Returns the remaining items of this iterator as a mutable slice.
    ///
    /// # Examples
    ///
    /// ```
    /// let vec = vec!['a', 'b', 'c'];
    /// let mut into_iter = vec.into_iter();
    /// assert_eq!(into_iter.as_slice(), &['a', 'b', 'c']);
    /// into_iter.as_mut_slice()[2] = 'z';
    /// assert_eq!(into_iter.next().unwrap(), 'a');
    /// assert_eq!(into_iter.next().unwrap(), 'b');
    /// assert_eq!(into_iter.next().unwrap(), 'z');
    /// ```
    #[stable(feature = "vec_into_iter_as_slice", since = "1.15.0")]
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        unsafe {
            slice::from_raw_parts_mut(self.ptr as *mut T, self.len())
        }
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
unsafe impl<T: Send> Send for IntoIter<T> {}
#[stable(feature = "rust1", since = "1.0.0")]
unsafe impl<T: Sync> Sync for IntoIter<T> {}

#[stable(feature = "rust1", since = "1.0.0")]
impl<T> Iterator for IntoIter<T> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<T> {
        unsafe {
            if self.ptr as *const _ == self.end {
                None
            } else {
                if mem::size_of::<T>() == 0 {
                    // purposefully don't use 'ptr.offset' because for
                    // vectors with 0-size elements this would return the
                    // same pointer.
                    self.ptr = arith_offset(self.ptr as *const i8, 1) as *mut T;

                    // Use a non-null pointer value
                    // (self.ptr might be null because of wrapping)
                    Some(ptr::read(1 as *mut T))
                } else {
                    let old = self.ptr;
                    self.ptr = self.ptr.offset(1);

                    Some(ptr::read(old))
                }
            }
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let exact = if mem::size_of::<T>() == 0 {
            (self.end as usize).wrapping_sub(self.ptr as usize)
        } else {
            unsafe { self.end.offset_from(self.ptr) as usize }
        };
        (exact, Some(exact))
    }

    #[inline]
    fn count(self) -> usize {
        self.len()
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<T> DoubleEndedIterator for IntoIter<T> {
    #[inline]
    fn next_back(&mut self) -> Option<T> {
        unsafe {
            if self.end == self.ptr {
                None
            } else {
                if mem::size_of::<T>() == 0 {
                    // See above for why 'ptr.offset' isn't used
                    self.end = arith_offset(self.end as *const i8, -1) as *mut T;

                    // Use a non-null pointer value
                    // (self.end might be null because of wrapping)
                    Some(ptr::read(1 as *mut T))
                } else {
                    self.end = self.end.offset(-1);

                    Some(ptr::read(self.end))
                }
            }
        }
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<T> ExactSizeIterator for IntoIter<T> {
    fn is_empty(&self) -> bool {
        self.ptr == self.end
    }
}

#[stable(feature = "fused", since = "1.26.0")]
impl<T> FusedIterator for IntoIter<T> {}

#[unstable(feature = "trusted_len", issue = "37572")]
unsafe impl<T> TrustedLen for IntoIter<T> {}

#[stable(feature = "vec_into_iter_clone", since = "1.8.0")]
impl<T: Clone> Clone for IntoIter<T> {
    fn clone(&self) -> IntoIter<T> {
        self.as_slice().to_owned().into_iter()
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
unsafe impl<#[may_dangle] T> Drop for IntoIter<T> {
    fn drop(&mut self) {
        // destroy the remaining elements
        for _x in self.by_ref() {}

        // RawVec handles deallocation
        let _ = unsafe { RawVec::from_raw_parts(self.buf.as_ptr(), self.cap) };
    }
}

/// A draining iterator for `Vec<T>`.
///
/// This `struct` is created by the [`drain`] method on [`Vec`].
///
/// [`drain`]: struct.Vec.html#method.drain
/// [`Vec`]: struct.Vec.html
#[stable(feature = "drain", since = "1.6.0")]
pub struct Drain<'a, T: 'a> {
    /// Index of tail to preserve
    tail_start: usize,
    /// Length of tail
    tail_len: usize,
    /// Current remaining range to remove
    iter: slice::Iter<'a, T>,
    vec: NonNull<Vec<T>>,
}

#[stable(feature = "collection_debug", since = "1.17.0")]
impl<'a, T: 'a + fmt::Debug> fmt::Debug for Drain<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("Drain")
            .field(&self.iter.as_slice())
            .finish()
    }
}

#[stable(feature = "drain", since = "1.6.0")]
unsafe impl<'a, T: Sync> Sync for Drain<'a, T> {}
#[stable(feature = "drain", since = "1.6.0")]
unsafe impl<'a, T: Send> Send for Drain<'a, T> {}

#[stable(feature = "drain", since = "1.6.0")]
impl<'a, T> Iterator for Drain<'a, T> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<T> {
        self.iter.next().map(|elt| unsafe { ptr::read(elt as *const _) })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

#[stable(feature = "drain", since = "1.6.0")]
impl<'a, T> DoubleEndedIterator for Drain<'a, T> {
    #[inline]
    fn next_back(&mut self) -> Option<T> {
        self.iter.next_back().map(|elt| unsafe { ptr::read(elt as *const _) })
    }
}

#[stable(feature = "drain", since = "1.6.0")]
impl<'a, T> Drop for Drain<'a, T> {
    fn drop(&mut self) {
        // exhaust self first
        self.for_each(drop);

        if self.tail_len > 0 {
            unsafe {
                let source_vec = self.vec.as_mut();
                // memmove back untouched tail, update to new length
                let start = source_vec.len();
                let tail = self.tail_start;
                if tail != start {
                    let src = source_vec.as_ptr().offset(tail as isize);
                    let dst = source_vec.as_mut_ptr().offset(start as isize);
                    ptr::copy(src, dst, self.tail_len);
                }
                source_vec.set_len(start + self.tail_len);
            }
        }
    }
}


#[stable(feature = "drain", since = "1.6.0")]
impl<'a, T> ExactSizeIterator for Drain<'a, T> {
    fn is_empty(&self) -> bool {
        self.iter.is_empty()
    }
}

#[stable(feature = "fused", since = "1.26.0")]
impl<'a, T> FusedIterator for Drain<'a, T> {}

/// A splicing iterator for `Vec`.
///
/// This struct is created by the [`splice()`] method on [`Vec`]. See its
/// documentation for more.
///
/// [`splice()`]: struct.Vec.html#method.splice
/// [`Vec`]: struct.Vec.html
#[derive(Debug)]
#[stable(feature = "vec_splice", since = "1.21.0")]
pub struct Splice<'a, I: Iterator + 'a> {
    drain: Drain<'a, I::Item>,
    replace_with: I,
}

#[stable(feature = "vec_splice", since = "1.21.0")]
impl<'a, I: Iterator> Iterator for Splice<'a, I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.drain.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.drain.size_hint()
    }
}

#[stable(feature = "vec_splice", since = "1.21.0")]
impl<'a, I: Iterator> DoubleEndedIterator for Splice<'a, I> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.drain.next_back()
    }
}

#[stable(feature = "vec_splice", since = "1.21.0")]
impl<'a, I: Iterator> ExactSizeIterator for Splice<'a, I> {}


#[stable(feature = "vec_splice", since = "1.21.0")]
impl<'a, I: Iterator> Drop for Splice<'a, I> {
    fn drop(&mut self) {
        self.drain.by_ref().for_each(drop);

        unsafe {
            if self.drain.tail_len == 0 {
                self.drain.vec.as_mut().extend(self.replace_with.by_ref());
                return
            }

            // First fill the range left by drain().
            if !self.drain.fill(&mut self.replace_with) {
                return
            }

            // There may be more elements. Use the lower bound as an estimate.
            // FIXME: Is the upper bound a better guess? Or something else?
            let (lower_bound, _upper_bound) = self.replace_with.size_hint();
            if lower_bound > 0  {
                self.drain.move_tail(lower_bound);
                if !self.drain.fill(&mut self.replace_with) {
                    return
                }
            }

            // Collect any remaining elements.
            // This is a zero-length vector which does not allocate if `lower_bound` was exact.
            let mut collected = self.replace_with.by_ref().collect::<Vec<I::Item>>().into_iter();
            // Now we have an exact count.
            if collected.len() > 0 {
                self.drain.move_tail(collected.len());
                let filled = self.drain.fill(&mut collected);
                debug_assert!(filled);
                debug_assert_eq!(collected.len(), 0);
            }
        }
        // Let `Drain::drop` move the tail back if necessary and restore `vec.len`.
    }
}

/// Private helper methods for `Splice::drop`
impl<'a, T> Drain<'a, T> {
    /// The range from `self.vec.len` to `self.tail_start` contains elements
    /// that have been moved out.
    /// Fill that range as much as possible with new elements from the `replace_with` iterator.
    /// Return whether we filled the entire range. (`replace_with.next()` didn’t return `None`.)
    unsafe fn fill<I: Iterator<Item=T>>(&mut self, replace_with: &mut I) -> bool {
        let vec = self.vec.as_mut();
        let range_start = vec.len;
        let range_end = self.tail_start;
        let range_slice = slice::from_raw_parts_mut(
            vec.as_mut_ptr().offset(range_start as isize),
            range_end - range_start);

        for place in range_slice {
            if let Some(new_item) = replace_with.next() {
                ptr::write(place, new_item);
                vec.len += 1;
            } else {
                return false
            }
        }
        true
    }

    /// Make room for inserting more elements before the tail.
    unsafe fn move_tail(&mut self, extra_capacity: usize) {
        let vec = self.vec.as_mut();
        let used_capacity = self.tail_start + self.tail_len;
        vec.buf.reserve(used_capacity, extra_capacity);

        let new_tail_start = self.tail_start + extra_capacity;
        let src = vec.as_ptr().offset(self.tail_start as isize);
        let dst = vec.as_mut_ptr().offset(new_tail_start as isize);
        ptr::copy(src, dst, self.tail_len);
        self.tail_start = new_tail_start;
    }
}

/// An iterator produced by calling `drain_filter` on Vec.
#[unstable(feature = "drain_filter", reason = "recently added", issue = "43244")]
#[derive(Debug)]
pub struct DrainFilter<'a, T: 'a, F>
    where F: FnMut(&mut T) -> bool,
{
    vec: &'a mut Vec<T>,
    idx: usize,
    del: usize,
    old_len: usize,
    pred: F,
}

#[unstable(feature = "drain_filter", reason = "recently added", issue = "43244")]
impl<'a, T, F> Iterator for DrainFilter<'a, T, F>
    where F: FnMut(&mut T) -> bool,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        unsafe {
            while self.idx != self.old_len {
                let i = self.idx;
                self.idx += 1;
                let v = slice::from_raw_parts_mut(self.vec.as_mut_ptr(), self.old_len);
                if (self.pred)(&mut v[i]) {
                    self.del += 1;
                    return Some(ptr::read(&v[i]));
                } else if self.del > 0 {
                    let del = self.del;
                    let src: *const T = &v[i];
                    let dst: *mut T = &mut v[i - del];
                    // This is safe because self.vec has length 0
                    // thus its elements will not have Drop::drop
                    // called on them in the event of a panic.
                    ptr::copy_nonoverlapping(src, dst, 1);
                }
            }
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.old_len - self.idx))
    }
}

#[unstable(feature = "drain_filter", reason = "recently added", issue = "43244")]
impl<'a, T, F> Drop for DrainFilter<'a, T, F>
    where F: FnMut(&mut T) -> bool,
{
    fn drop(&mut self) {
        self.for_each(drop);
        unsafe {
            self.vec.set_len(self.old_len - self.del);
        }
    }
}
