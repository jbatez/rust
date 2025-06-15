//! Defines types and macros for Objective-C interoperability.

#![cfg(any(target_vendor = "apple", doc))]
#![doc(cfg(target_vendor = "apple"))]

use crate::fmt;

/// Equivalent to Objective-C’s `struct objc_class` type.
#[cfg_attr(not(doc), repr(u8))] // An implementation detail we don't want to show up in rustdoc
pub enum objc_class {
    #[unstable(
        feature = "objc_class_variant",
        reason = "temporary implementation detail",
        issue = "none"
    )]
    #[doc(hidden)]
    __variant1,
    #[unstable(
        feature = "objc_class_variant",
        reason = "temporary implementation detail",
        issue = "none"
    )]
    #[doc(hidden)]
    __variant2,
}

impl fmt::Debug for objc_class {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("objc_class").finish()
    }
}

/// Equivalent to Objective-C’s `struct objc_selector` type.
#[cfg_attr(not(doc), repr(u8))] // An implementation detail we don't want to show up in rustdoc
pub enum objc_selector {
    #[unstable(
        feature = "objc_selector_variant",
        reason = "temporary implementation detail",
        issue = "none"
    )]
    #[doc(hidden)]
    __variant1,
    #[unstable(
        feature = "objc_selector_variant",
        reason = "temporary implementation detail",
        issue = "none"
    )]
    #[doc(hidden)]
    __variant2,
}

impl fmt::Debug for objc_selector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("objc_selector").finish()
    }
}

/// Equivalent to Objective-C’s `Class` type.
pub type Class = *mut objc_class;

/// Equivalent to Objective-C’s `SEL` type.
pub type SEL = *mut objc_selector;

/// Gets a reference to an Objective-C class.
///
/// This macro will yield an expression of type [`Class`] for the given class name string literal.
///
/// # Example
///
/// ```no_run
/// let string_class = class!("NSString");
/// ```
#[allow_internal_unstable(rustc_attrs)]
pub macro class($classname:expr) {{
    unsafe extern "C" {
        #[rustc_objc_class = $classname]
        safe static VAL: $crate::ffi::objc::Class;
    }
    VAL
}}

/// Gets a reference to an Objective-C selector.
///
/// This macro will yield an expression of type [`SEL`] for the given method name string literal.
///
/// It is similar to Objective-C’s `@selector` directive.
///
/// # Examples
///
/// ```no_run
/// let alloc_sel = selector!("alloc");
/// let init_sel = selector!("initWithCString:encoding:");
/// ```
#[allow_internal_unstable(rustc_attrs)]
pub macro selector($methname:expr) {{
    unsafe extern "C" {
        #[rustc_objc_selector = $methname]
        safe static VAL: $crate::ffi::objc::SEL;
    }
    VAL
}}
