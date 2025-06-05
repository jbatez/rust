//! Defines types and macros for Objective-C interoperability.

#![cfg(any(target_vendor = "apple", doc))]
#![doc(cfg(target_vendor = "apple"))]

pub use core::ffi::objc::{Class, SEL, class, objc_class, objc_selector, selector};
