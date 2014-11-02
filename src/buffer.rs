
#![experimental]
#![doc(hidden)]

// This is a strange, wonderful, disgusting, and useful object.

use std::mem;
use std::raw::Slice;
use std::slice::AsSlice;
use std::ops::{Index, IndexMut};

pub struct Buffer
{
    bytes: Vec<u8>,
    stride: uint,
}

impl<T> Index<uint, T> for Buffer where T:'static
{
    #[inline(always)]
    fn index<'a>(&'a self, &index: &uint) -> &T {
        &self.as_slice()[index]
    }
}

impl<T> IndexMut<uint, T> for Buffer where T:'static
{
    #[inline(always)]
    fn index_mut<'a>(&'a mut self, &index: &uint) -> &mut T {
        let offset = self.stride * index;
        let length = self.bytes.len();
        if offset >= length {
            self.bytes.grow(offset - length + self.stride, 0);
        }
        &mut self.as_mut_slice()[index]
    }
}

impl<T> AsSlice<T> for Buffer where T:'static
{
    fn as_slice(&self) -> &[T]
    {
        debug_assert_eq!(mem::size_of::<T>(), self.stride);
        unsafe
        {
            mem::transmute(Slice
            {
                data: self.bytes.as_ptr(),
                len: self.len()
            })
        }
    }
}

impl Buffer
{
    #[inline(always)]
    pub fn new<T:'static>() -> Buffer
    {
        Buffer
        {
            bytes: Vec::new(),
            stride: mem::size_of::<T>(),
        }
    }

    pub fn as_mut_slice<T:'static>(&mut self) -> &mut [T]
    {
        debug_assert_eq!(mem::size_of::<T>(), self.stride);
        unsafe
        {
            mem::transmute(Slice
            {
                data: self.bytes.as_ptr(),
                len: self.len()
            })
        }
    }

    #[inline(always)]
    pub fn len(&self) -> uint
    {
        self.bytes.len() / self.stride
    }

    #[inline(always)]
    pub fn bytes_len(&self) -> uint
    {
        self.bytes.len()
    }

    #[inline(always)]
    pub fn stride(&self) -> uint
    {
        self.stride
    }

    #[inline(always)]
    pub fn as_bytes(&self) -> &Vec<u8>
    {
        &self.bytes
    }
}

pub trait IntoBuffer
{
    fn into_buffer(self) -> Buffer;
}

impl<T> IntoBuffer for Vec<T> where T:'static
{
    fn into_buffer(mut self) -> Buffer
    {
        let stride = mem::size_of::<T>();
        unsafe
        {
            let pointer: *mut u8 = mem::transmute(self.as_mut_ptr());
            Buffer
            {
                bytes: Vec::from_raw_parts(pointer, self.len() * stride, self.capacity() * stride),
                stride: stride
            }
        }
    }
}