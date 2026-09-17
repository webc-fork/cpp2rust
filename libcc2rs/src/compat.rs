// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use std::cell::RefCell;
use std::ffi::c_void;
use std::rc::Rc;

use crate::rc::Ptr;
use crate::{AsPointer, Value};

unsafe extern "C" {
    #[cfg(target_os = "linux")]
    #[link_name = "malloc_usable_size"]
    fn platform_malloc_size(ptr: *mut c_void) -> usize;

    #[cfg(target_os = "macos")]
    #[link_name = "malloc_size"]
    fn platform_malloc_size(ptr: *const c_void) -> usize;

    #[cfg(target_os = "linux")]
    #[link_name = "__errno_location"]
    fn platform_errno_location() -> *mut i32;

    #[cfg(target_os = "macos")]
    #[link_name = "__error"]
    fn platform_errno_location() -> *mut i32;
}

/// # Safety
///
/// The pointer `ptr` must be a pointer to a block of memory allocated by
/// the appropriate allocator (e.g., `malloc`).
// The memory must not have been deallocated.
pub unsafe fn malloc_usable_size(ptr: *mut c_void) -> usize {
    #[cfg(target_os = "linux")]
    {
        unsafe { platform_malloc_size(ptr) }
    }
    #[cfg(target_os = "macos")]
    {
        unsafe { platform_malloc_size(ptr as *const c_void) }
    }
}

/// # Safety
///
/// Invokes the platform specific errno.
pub unsafe fn cpp2rust_errno_unsafe() -> *mut i32 {
    unsafe { platform_errno_location() }
}

thread_local! {
    static ERRNO: Value<i32> = Rc::new(RefCell::new(0));
}

pub fn cpp2rust_errno() -> Ptr<i32> {
    ERRNO.with(AsPointer::as_pointer)
}

use crate::ByteRepr;

#[allow(non_camel_case_types)]
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub struct std_memory_order;

#[allow(non_camel_case_types)]
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub struct std_atomic_flag {
  pub val: bool,
}

impl std_atomic_flag {
  pub fn std_atomic_flag1() -> Self {
    Self { val: false }
  }
  pub fn test_and_set<T>(&self, _order: Option<T>) -> bool {
    true
  }
  pub fn clear<T>(&self, _order: Option<T>) {}
}

impl ByteRepr for std_atomic_flag {
  fn byte_size() -> usize {
    1
  }
  fn to_bytes(&self, buf: &mut [u8]) {
    buf[0] = self.val as u8;
  }
  fn from_bytes(buf: &[u8]) -> Self {
    Self { val: buf[0] != 0 }
  }
}

#[allow(non_camel_case_types)]
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub struct std_atomic_bool_ {
  pub val: bool,
}

impl std_atomic_bool_ {
  pub fn std_atomic_bool_1(val: bool) -> Self {
    Self { val }
  }
  pub fn load<T>(&self, _order: Option<T>) -> bool {
    self.val
  }
  pub fn store<T>(&self, _val: bool, _order: Option<T>) {}
}

impl ByteRepr for std_atomic_bool_ {
  fn byte_size() -> usize {
    1
  }
  fn to_bytes(&self, buf: &mut [u8]) {
    buf[0] = self.val as u8;
  }
  fn from_bytes(buf: &[u8]) -> Self {
    Self { val: buf[0] != 0 }
  }
}

#[allow(non_camel_case_types)]
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub struct std_atomic_unsigned_long_ {
  pub val: u64,
}

impl std_atomic_unsigned_long_ {
  pub fn std_atomic_unsigned_long_1(val: u64) -> Self {
    Self { val }
  }
  pub fn load<T>(&self, _order: Option<T>) -> u64 {
    self.val
  }
  pub fn store<T>(&self, _val: u64, _order: Option<T>) {}
  pub fn fetch_add<T>(&self, _val: u64, _order: Option<T>) -> u64 {
    self.val
  }
}

impl ByteRepr for std_atomic_unsigned_long_ {
  fn byte_size() -> usize {
    8
  }
  fn to_bytes(&self, buf: &mut [u8]) {
    buf.copy_from_slice(&self.val.to_ne_bytes());
  }
  fn from_bytes(buf: &[u8]) -> Self {
    let mut a = [0u8; 8];
    a.copy_from_slice(buf);
    Self { val: u64::from_ne_bytes(a) }
  }
}
