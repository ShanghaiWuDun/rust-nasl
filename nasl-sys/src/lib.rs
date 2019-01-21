#![allow(unused_imports, non_camel_case_types, non_snake_case)]

extern crate libc;
extern crate glib;
extern crate glib_sys;

pub mod kb;
pub mod nvti;
pub mod ast;

pub use self::kb::*;
pub use self::nvti::*;
pub use self::ast::*;


// https://developer.gnome.org/glib/stable/glib-Basic-Types.html#D
pub type gchar = std::os::raw::c_char;
pub type gint = std::os::raw::c_int;
pub type guint = std::os::raw::c_uint;


#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct __BindgenBitfieldUnit<Storage, Align>
where
    Storage: AsRef<[u8]> + AsMut<[u8]>,
{
    storage: Storage,
    align: [Align; 0],
}

impl<Storage, Align> __BindgenBitfieldUnit<Storage, Align>
where
    Storage: AsRef<[u8]> + AsMut<[u8]>,
{
    #[inline]
    pub fn new(storage: Storage) -> Self {
        Self { storage, align: [] }
    }

    #[inline]
    pub fn get_bit(&self, index: usize) -> bool {
        debug_assert!(index / 8 < self.storage.as_ref().len());

        let byte_index = index / 8;
        let byte = self.storage.as_ref()[byte_index];

        let bit_index = index % 8;
        let mask = 1 << bit_index;

        byte & mask == mask
    }

    #[inline]
    pub fn set_bit(&mut self, index: usize, val: bool) {
        debug_assert!(index / 8 < self.storage.as_ref().len());

        let byte_index = index / 8;
        let byte = &mut self.storage.as_mut()[byte_index];

        let bit_index = index % 8;
        let mask = 1 << bit_index;

        if val {
            *byte |= mask;
        } else {
            *byte &= !mask;
        }
    }

    #[inline]
    pub fn get(&self, bit_offset: usize, bit_width: u8) -> u64 {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len());

        let mut val = 0;

        for i in 0..(bit_width as usize) {
            if self.get_bit(i + bit_offset) {
                val |= 1 << i;
            }
        }

        val
    }

    #[inline]
    pub fn set(&mut self, bit_offset: usize, bit_width: u8, val: u64) {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len());

        for i in 0..(bit_width as usize) {
            let mask = 1 << i;
            let val_bit_is_set = val & mask == mask;
            self.set_bit(i + bit_offset, val_bit_is_set);
        }
    }
}

#[repr(C)]
#[derive(Default)]
pub struct __IncompleteArrayField<T>(::std::marker::PhantomData<T>);
impl<T> __IncompleteArrayField<T> {
    #[inline]
    pub fn new() -> Self {
        __IncompleteArrayField(::std::marker::PhantomData)
    }
    #[inline]
    pub unsafe fn as_ptr(&self) -> *const T {
        ::std::mem::transmute(self)
    }
    #[inline]
    pub unsafe fn as_mut_ptr(&mut self) -> *mut T {
        ::std::mem::transmute(self)
    }
    #[inline]
    pub unsafe fn as_slice(&self, len: usize) -> &[T] {
        ::std::slice::from_raw_parts(self.as_ptr(), len)
    }
    #[inline]
    pub unsafe fn as_mut_slice(&mut self, len: usize) -> &mut [T] {
        ::std::slice::from_raw_parts_mut(self.as_mut_ptr(), len)
    }
}

impl<T> ::std::fmt::Debug for __IncompleteArrayField<T> {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        fmt.write_str("__IncompleteArrayField")
    }
}

impl<T> ::std::clone::Clone for __IncompleteArrayField<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self::new()
    }
}

impl<T> ::std::marker::Copy for __IncompleteArrayField<T> {}


