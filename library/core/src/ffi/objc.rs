//! Defines types and macros for Objective-C interoperability.

use crate::fmt;

/// Equivalent to Objective-C’s `struct objc_selector` type.
#[cfg_attr(not(bootstrap), lang = "objc_selector")]
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
#[allow_internal_unstable(builtin_syntax)]
pub macro selector($($t:tt)+) {
    builtin # objc_selector ( $($t)+ )
}
