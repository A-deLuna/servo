// Copyright (c) 2014 Marshall A. Greenblatt. All rights reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are
// met:
//
//    * Redistributions of source code must retain the above copyright
// notice, this list of conditions and the following disclaimer.
//    * Redistributions in binary form must reproduce the above
// copyright notice, this list of conditions and the following disclaimer
// in the documentation and/or other materials provided with the
// distribution.
//    * Neither the name of Google Inc. nor the name Chromium Embedded
// Framework nor the names of its contributors may be used to endorse
// or promote products derived from this software without specific prior
// written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
// OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
// LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
// DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
// THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
// (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
//
// ---------------------------------------------------------------------------
//
// This file was generated by the CEF translator tool and should not be edited
// by hand. See the translator.README.txt file in the tools directory for
// more information.
//

#![allow(non_snake_case, unused_imports)]

use eutil;
use interfaces;
use types;
use wrappers::CefWrap;

use libc;
use std::collections::HashMap;
use std::ptr;

//
// Implement this structure to receive string values asynchronously.
//
#[repr(C)]
pub struct _cef_string_visitor_t {
  //
  // Base structure.
  //
  pub base: types::cef_base_t,

  //
  // Method that will be executed.
  //
  pub visit: Option<extern "C" fn(this: *mut cef_string_visitor_t,
      string: *const types::cef_string_t) -> ()>,

  //
  // The reference count. This will only be present for Rust instances!
  //
  pub ref_count: uint,

  //
  // Extra data. This will only be present for Rust instances!
  //
  pub extra: u8,
} 

pub type cef_string_visitor_t = _cef_string_visitor_t;


//
// Implement this structure to receive string values asynchronously.
//
pub struct CefStringVisitor {
  c_object: *mut cef_string_visitor_t,
}

impl Clone for CefStringVisitor {
  fn clone(&self) -> CefStringVisitor{
    unsafe {
      if !self.c_object.is_null() {
        ((*self.c_object).base.add_ref.unwrap())(&mut (*self.c_object).base);
      }
      CefStringVisitor {
        c_object: self.c_object,
      }
    }
  }
}

impl Drop for CefStringVisitor {
  fn drop(&mut self) {
    unsafe {
      if !self.c_object.is_null() {
        ((*self.c_object).base.release.unwrap())(&mut (*self.c_object).base);
      }
    }
  }
}

impl CefStringVisitor {
  pub unsafe fn from_c_object(c_object: *mut cef_string_visitor_t) -> CefStringVisitor {
    CefStringVisitor {
      c_object: c_object,
    }
  }

  pub unsafe fn from_c_object_addref(c_object: *mut cef_string_visitor_t) -> CefStringVisitor {
    if !c_object.is_null() {
      ((*c_object).base.add_ref.unwrap())(&mut (*c_object).base);
    }
    CefStringVisitor {
      c_object: c_object,
    }
  }

  pub fn c_object(&self) -> *mut cef_string_visitor_t {
    self.c_object
  }

  pub fn c_object_addrefed(&self) -> *mut cef_string_visitor_t {
    unsafe {
      if !self.c_object.is_null() {
        eutil::add_ref(self.c_object as *mut types::cef_base_t);
      }
      self.c_object
    }
  }

  pub fn is_null_cef_object(&self) -> bool {
    self.c_object.is_null()
  }
  pub fn is_not_null_cef_object(&self) -> bool {
    !self.c_object.is_null()
  }

  //
  // Method that will be executed.
  //
  pub fn visit(&self, string: &[u16]) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).visit.unwrap())(
          self.c_object,
          CefWrap::to_c(string)))
    }
  }
} 

impl CefWrap<*mut cef_string_visitor_t> for CefStringVisitor {
  fn to_c(rust_object: CefStringVisitor) -> *mut cef_string_visitor_t {
    rust_object.c_object_addrefed()
  }
  unsafe fn to_rust(c_object: *mut cef_string_visitor_t) -> CefStringVisitor {
    CefStringVisitor::from_c_object_addref(c_object)
  }
}
impl CefWrap<*mut cef_string_visitor_t> for Option<CefStringVisitor> {
  fn to_c(rust_object: Option<CefStringVisitor>) -> *mut cef_string_visitor_t {
    match rust_object {
      None => ptr::null_mut(),
      Some(rust_object) => rust_object.c_object_addrefed(),
    }
  }
  unsafe fn to_rust(c_object: *mut cef_string_visitor_t) -> Option<CefStringVisitor> {
    if c_object.is_null() {
      None
    } else {
      Some(CefStringVisitor::from_c_object_addref(c_object))
    }
  }
}
