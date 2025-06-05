//! Defines types and macros for Objective-C interoperability.

#![cfg(any(target_vendor = "apple", doc))]
#![doc(cfg(target_vendor = "apple"))]

use crate::fmt;

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

/// Equivalent to Objective-C’s `SEL` type.
pub type SEL = *mut objc_selector;

/// Equivalent to Objective-C’s `@selector` directive.
///
/// This macro will yield an expression of type [`SEL`] for the given method name.
/// Valid Objective-C method names consist of either:
/// - a single identifier or
/// - a series of `:`, each optionally preceded by an identifier.
///
/// # Examples
///
/// ```no_run
/// let alloc_sel = selector!(alloc);
/// let init_sel = selector!(initWithCString:encoding:);
/// ```
#[allow_internal_unstable(rustc_attrs)]
pub macro selector($($methname:tt)+) {{
    unsafe extern "C" {
        #[rustc_objc_selector($($methname)+)]
        safe static VAL: $crate::ffi::objc::SEL;
    }
    VAL
}}
