/// Generate the boilerplate for a newtyped index struct, for use with
/// `IndexVec`.
///
/// In the future, if the compile-time overhead of doing so is reduced, this may
/// be replaced with a proc macro.
///
/// ## Usage
///
/// ### Standard
///
/// The rough usage pattern of this macro is:
///
/// ```rust,no_run
/// oxc_index::define_index_type! {
///     // Note that isn't actually a type alias, `MyIndex` is
///     // actually defined as a struct. XXX is this too confusing?
///     pub struct MyIndex = u32;
///     // optional extra configuration here of the form:
///     // `OPTION_NAME = stuff;`
///     // See below for details.
/// }
/// ```
///
/// Note that you can use other index types than `u32`, and you can set it to be
/// `MyIndex(pub u32)` as well. Currently, the wrapped item be a tuple struct,
/// however (patches welcome).
///
/// ### Customization
///
/// After the struct declaration, there are a number of configuration options
/// the macro uses to customize how the type it generates behaves. For example:
///
/// ```rust,no_run
/// oxc_index::define_index_type! {
///     pub struct Span = u32;
///
///     // Don't allow any spans with values higher this.
///     MAX_INDEX = 0x7fff_ff00;
///
///     // But I also am not too worried about it, so only
///     // perform the asserts in debug builds.
///     DISABLE_MAX_INDEX_CHECK = cfg!(not(debug_assertions));
/// }
/// ```
///
/// ## Configuration options
///
/// This macro has a few ways you can customize it's output behavior. There's
/// not really any great syntax I can think of for them, but, well.
///
/// #### `MAX_INDEX = <expr producing usize>`
///
/// Assert if anything tries to construct an index above that value.
///
/// By default, this is `$raw_type::max_value() as usize`, e.g. we check that
/// our cast from `usize` to our wrapper is lossless, but we assume any all
/// instance of `$raw_type` is valid in this index domain.
///
/// Note that these tests can be disabled entirely, or conditionally, with
/// `DISABLE_MAX_INDEX_CHECK`. Additionally, the generated type has
/// `from_usize_unchecked` and `from_raw_unchecked` functions which can be used
/// to ignore these checks.
///
/// #### `DISABLE_MAX_INDEX_CHECK = <expr>;`
///
/// Set to true to disable the assertions mentioned above. False by default.
///
/// To be clear, if this is set to false, we blindly assume all casts between
/// `usize` and `$raw_type` succeed.
///
/// A common use is setting `DISABLE_MAX_INDEX_CHECK = !cfg!(debug_assertions)`
/// to avoid the tests at compile time
///
/// For the sake of clarity, disabling this cannot lead to memory unsafety -- we
/// still go through bounds checks when accessing slices, and no unsafe code
/// should rely on on these checks (unless you write some, and don't! only use
/// this for correctness!).
///
/// #### `DEFAULT = <expr>;`
/// If provided, we'll implement `Default` for the index type using this
/// expression.
///
/// Example:
///
/// ```rust,no_run
/// oxc_index::define_index_type! {
///     pub struct MyIdx = u16;
///     MAX_INDEX = (u16::max_value() - 1) as usize;
///     // Set the default index to be an invalid index, as
///     // a hacky way of having this type behave somewhat
///     // like it were an Option<MyIdx> without consuming
///     // extra space.
///     DEFAULT = (MyIdx::from_raw_unchecked(u16::max_value()));
/// }
/// ```
///
/// #### `DEBUG_FORMAT = <expr>;`
///
/// By default we write the underlying integer out in a Debug implementation
/// with `{:?}`. Sometimes you'd like more info though. For example, the type of
/// the index. This can be done via `DEBUG_FORMAT`:
///
/// ```rust
/// oxc_index::define_index_type! {
///     struct FooIdx = usize;
///     DEBUG_FORMAT = "Foo({})";
/// }
/// // Then ...
/// # fn main() {
/// let v = FooIdx::new(10);
/// assert_eq!("Foo(10)", format!("{:?}", v));
/// # }
/// ```
///
/// #### `DISPLAY_FORMAT = <expr>;`
///
/// Similarly to `DEBUG_FORMAT`, we can implement Display for you. Unlike
/// `DEBUG_FORMAT`, if you do not set this, we will not implement `Display` for
/// the index type.
///
/// ```rust
/// oxc_index::define_index_type! {
///     struct FooIdx = usize;
///     DISPLAY_FORMAT = "{}";
///     // Note that you can use both DEBUG_FORMAT and DISPLAY_FORMAT.
///     DEBUG_FORMAT = "#<foo {}>";
/// }
/// // Then ...
/// # fn main() {
/// let v = FooIdx::new(10);
/// assert_eq!("10", format!("{}", v));
/// assert_eq!("#<foo 10>", format!("{:?}", v));
/// # }
/// ```
///
/// #### `IMPL_RAW_CONVERSIONS = true;`
///
/// We always automatically implement `From<usize> for YourIndex` and
/// `From<YourIndex> for usize`. We don't do this for the "raw" type (e.g. `u32`
/// if your type is declared as `struct FooIdx = u32;`), unless you request it
/// via this option. It's an error to use this if your raw type is usize.
///
/// ```rust
/// oxc_index::define_index_type! {
///     struct FooIdx = u32;
///     IMPL_RAW_CONVERSIONS = true;
/// }
///
/// # fn main() {
/// let as_index = FooIdx::from(5u32);
/// let as_u32 = u32::from(as_index);
/// assert_eq!(as_u32, 5);
/// # }
/// ```
#[macro_export]
macro_rules! define_index_type {
    // public api for primitive types (u8, u16, u32, usize, etc.)
    (
        $(#[$attrs:meta])*
        $v:vis struct $type:ident = $raw:ident;
        $($CONFIG_NAME:ident = $value:expr_2021;)* $(;)?
    ) => {
        $crate::__define_index_type_inner!{
            @configs [$(($CONFIG_NAME; $value))*]
            @attrs [$(#[$attrs])*]
            @derives [#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]]
            @decl [$v struct $type ($raw)]
            @debug_fmt ["{}"]
            @max [(<$raw>::max_value() as usize)]
            @no_check_max [false]
        }
    };
    // public api for complex types (NonMaxU32, etc.) - requires explicit MAX_INDEX
    (
        $(#[$attrs:meta])*
        $v:vis struct $type:ident = $raw:ty;
        $($CONFIG_NAME:ident = $value:expr_2021;)+ $(;)?
    ) => {
        $crate::__define_index_type_inner!{
            @configs [$(($CONFIG_NAME; $value))*]
            @attrs [$(#[$attrs])*]
            @derives [#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]]
            @decl [$v struct $type ($raw)]
            @debug_fmt ["{}"]
            @max [(usize::MAX)]
            @no_check_max [false]
        }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! unknown_define_index_type_option {
    () => {};
}

/// Generate the boilerplate for a newtyped index struct using NonMaxU32.
/// This is a specialized version of `define_index_type!` for use with `NonMaxU32` from the `nonmax` crate.
///
/// ## Usage
///
/// ```rust,ignore
/// oxc_index::define_nonmax_index_type! {
///     pub struct MyIndex;
/// }
/// ```
///
/// This creates a tuple struct `pub struct MyIndex(NonMaxU32)` with all the necessary trait
/// implementations. The type is backed by `NonMaxU32`, which has the same size as `u32` but
/// can represent values from `0` to `u32::MAX - 1`.
///
/// ## Custom Attributes and Proc Macros
///
/// You can add custom attributes, including proc macros, to the generated struct:
///
/// ```rust,ignore
/// oxc_index::define_nonmax_index_type! {
///     /// Documentation for MyIndex
///     #[ast]  // Proc macros work correctly
///     #[builder(default)]
///     #[allow(dead_code)]
///     pub struct MyIndex;
/// }
/// ```
///
/// **Attribute Ordering:** The macro applies attributes in this order:
/// 1. Built-in derives: `#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]`
/// 2. Your custom attributes (including proc macros)
///
/// This ordering ensures proc macros can properly process the struct with all standard derives already applied.
///
/// **Note:** The macro automatically provides:
/// - Derives: `Copy`, `Clone`, `PartialEq`, `Eq`, `Hash`, `PartialOrd`, `Ord`
/// - Manual implementations: `Debug`, `From<usize>`, `From<MyIndex> for usize`, and arithmetic ops
///
/// Do not add `#[derive(Debug)]` or other conflicting derives/impls as they are already provided.
#[cfg(feature = "nonmax")]
#[macro_export]
macro_rules! define_nonmax_index_type {
    (
        $(#[$attrs:meta])*
        $v:vis struct $type:ident;
        $($CONFIG_NAME:ident = $value:expr_2021;)* $(;)?
    ) => {
        #[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
        $(#[$attrs])*
        $v struct $type(nonmax::NonMaxU32);

        impl $type {
            $v const MAX_INDEX: usize = (u32::MAX - 1) as usize;
            $v const CHECKS_MAX_INDEX: bool = true;

            #[inline(always)]
            $v const fn new(value: usize) -> Self {
                Self::from_usize(value)
            }

            #[inline(always)]
            $v const fn from_raw(value: nonmax::NonMaxU32) -> Self {
                Self(value)
            }

            #[inline(always)]
            $v fn from_foreign<F: $crate::Idx>(value: F) -> Self {
                Self::from_usize(value.index())
            }

            #[inline(always)]
            $v const fn from_usize_unchecked(value: usize) -> Self {
                // SAFETY: Caller must ensure value is valid
                Self(unsafe { nonmax::NonMaxU32::new_unchecked(value as u32) })
            }

            #[inline(always)]
            $v const fn from_raw_unchecked(raw: u32) -> Self {
                // SAFETY: Caller must ensure value is valid
                Self(unsafe { nonmax::NonMaxU32::new_unchecked(raw) })
            }

            #[inline]
            $v const fn from_usize(value: usize) -> Self {
                Self::check_index(value);
                match nonmax::NonMaxU32::new(value as u32) {
                    Some(raw) => Self(raw),
                    None => panic!("index_vec index overflow"),
                }
            }

            #[inline(always)]
            $v const fn index(self) -> usize {
                self.0.get() as usize
            }

            #[inline(always)]
            $v const fn raw(self) -> nonmax::NonMaxU32 {
                self.0
            }

            #[inline]
            $v const fn check_index(v: usize) {
                if Self::CHECKS_MAX_INDEX && (v > Self::MAX_INDEX) {
                    panic!("index_vec index overflow");
                }
            }
        }

        impl core::fmt::Debug for $type {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, "{}", self.index())
            }
        }

        impl core::cmp::PartialOrd<usize> for $type {
            #[inline]
            fn partial_cmp(&self, other: &usize) -> Option<core::cmp::Ordering> {
                self.index().partial_cmp(other)
            }
        }

        impl core::cmp::PartialOrd<$type> for usize {
            #[inline]
            fn partial_cmp(&self, other: &$type) -> Option<core::cmp::Ordering> {
                self.partial_cmp(&other.index())
            }
        }

        impl PartialEq<usize> for $type {
            #[inline]
            fn eq(&self, other: &usize) -> bool {
                self.index() == *other
            }
        }

        impl PartialEq<$type> for usize {
            #[inline]
            fn eq(&self, other: &$type) -> bool {
                *self == other.index()
            }
        }

        impl core::ops::Add<usize> for $type {
            type Output = Self;
            #[inline]
            fn add(self, other: usize) -> Self {
                Self::new(self.index().wrapping_add(other))
            }
        }

        impl core::ops::Sub<usize> for $type {
            type Output = Self;
            #[inline]
            fn sub(self, other: usize) -> Self {
                Self::new(self.index().wrapping_sub(other))
            }
        }

        impl core::ops::AddAssign<usize> for $type {
            #[inline]
            fn add_assign(&mut self, other: usize) {
                *self = *self + other
            }
        }

        impl core::ops::SubAssign<usize> for $type {
            #[inline]
            fn sub_assign(&mut self, other: usize) {
                *self = *self - other;
            }
        }

        impl core::ops::Rem<usize> for $type {
            type Output = Self;
            #[inline]
            fn rem(self, other: usize) -> Self {
                Self::new(self.index() % other)
            }
        }

        impl core::ops::Add<$type> for usize {
            type Output = $type;
            #[inline]
            fn add(self, other: $type) -> $type {
                other + self
            }
        }

        impl core::ops::Sub<$type> for usize {
            type Output = $type;
            #[inline]
            fn sub(self, other: $type) -> $type {
                $type::new(self.wrapping_sub(other.index()))
            }
        }

        impl core::ops::Add for $type {
            type Output = $type;
            #[inline]
            fn add(self, other: $type) -> $type {
                $type::new(other.index() + self.index())
            }
        }

        impl core::ops::Sub for $type {
            type Output = $type;
            #[inline]
            fn sub(self, other: $type) -> $type {
                $type::new(self.index().wrapping_sub(other.index()))
            }
        }

        impl core::ops::AddAssign for $type {
            #[inline]
            fn add_assign(&mut self, other: $type) {
                *self = *self + other
            }
        }

        impl core::ops::SubAssign for $type {
            #[inline]
            fn sub_assign(&mut self, other: $type) {
                *self = *self - other;
            }
        }

        impl $crate::Idx for $type {
            const MAX: usize = Self::MAX_INDEX;

            #[inline]
            unsafe fn from_usize_unchecked(idx: usize) -> Self {
                Self::from_usize_unchecked(idx)
            }

            #[inline]
            fn index(self) -> usize {
                usize::from(self)
            }
        }

        impl From<$type> for usize {
            #[inline]
            fn from(v: $type) -> usize {
                v.index()
            }
        }

        impl From<usize> for $type {
            #[inline]
            fn from(value: usize) -> Self {
                $type::from_usize(value)
            }
        }

        $crate::__internal_maybe_index_impl_serde!($type);
    };
}

#[cfg(feature = "serde")]
#[macro_export]
#[doc(hidden)]
macro_rules! __internal_maybe_index_impl_serde {
    ($type:ident) => {
        impl serde::ser::Serialize for $type {
            fn serialize<S: serde::ser::Serializer>(
                &self,
                serializer: S,
            ) -> Result<S::Ok, S::Error> {
                self.index().serialize(serializer)
            }
        }

        impl<'de> serde::de::Deserialize<'de> for $type {
            fn deserialize<D: serde::de::Deserializer<'de>>(
                deserializer: D,
            ) -> Result<Self, D::Error> {
                usize::deserialize(deserializer).map(Self::from_usize)
            }
        }
    };
}

#[cfg(not(feature = "serde"))]
#[macro_export]
#[doc(hidden)]
macro_rules! __internal_maybe_index_impl_serde {
    ($type:ident) => {};
}

#[macro_export]
#[doc(hidden)]
macro_rules! __define_index_type_inner {
    // DISABLE_MAX_INDEX_CHECK
    (
        @configs [(DISABLE_MAX_INDEX_CHECK; $no_check_max:expr_2021) $(($CONFIG_NAME:ident; $value:expr_2021))*]
        @attrs [$(#[$attrs:meta])*]
        @derives [$(#[$derive:meta])*]
        @decl [$v:vis struct $type:ident ($raw:ty)]
        @debug_fmt [$dbg:expr_2021]
        @max [$max:expr_2021]
        @no_check_max [$_old_no_check_max:expr_2021]
    ) => {
        $crate::__define_index_type_inner!{
            @configs [$(($CONFIG_NAME; $value))*]
            @attrs [$(#[$attrs])*]
            @derives [$(#[$derive])*]
            @decl [$v struct $type ($raw)]
            @debug_fmt [$dbg]
            @max [$max]
            @no_check_max [$no_check_max]
        }
    };

    // MAX_INDEX
    (
        @configs [(MAX_INDEX; $new_max:expr_2021) $(($CONFIG_NAME:ident; $value:expr_2021))*]
        @attrs [$(#[$attrs:meta])*]
        @derives [$(#[$derive:meta])*]
        @decl [$v:vis struct $type:ident ($raw:ty)]
        @debug_fmt [$dbg:expr_2021]
        @max [$max:expr_2021]
        @no_check_max [$cm:expr_2021]
    ) => {
        $crate::__define_index_type_inner!{
            @configs [$(($CONFIG_NAME; $value))*]
            @attrs [$(#[$attrs])*]
            @derives [$(#[$derive])*]
            @decl [$v struct $type ($raw)]
            @debug_fmt [$dbg]
            @max [$new_max]
            @no_check_max [$cm]
        }
    };

    // DEFAULT
    (
        @configs [(DEFAULT; $default_expr:expr_2021) $(($CONFIG_NAME:ident; $value:expr_2021))*]
        @attrs [$(#[$attrs:meta])*]
        @derives [$(#[$derive:meta])*]
        @decl [$v:vis struct $type:ident ($raw:ty)]
        @debug_fmt [$dbg:expr_2021]
        @max [$max:expr_2021]
        @no_check_max [$no_check_max:expr_2021]
    ) => {
        $crate::__define_index_type_inner!{
            @configs [$(($CONFIG_NAME; $value))*]
            @attrs [$(#[$attrs])*]
            @derives [$(#[$derive])*]
            @decl [$v struct $type ($raw)]
            @debug_fmt [$dbg]
            @max [$max]
            @no_check_max [$no_check_max]
        }
        impl Default for $type {
            #[inline]
            fn default() -> Self {
                $default_expr
            }
        }
    };

    // DEBUG_FORMAT
    (
        @configs [(DEBUG_FORMAT; $dbg:expr_2021) $(($CONFIG_NAME:ident; $value:expr_2021))*]
        @attrs [$(#[$attrs:meta])*]
        @derives [$(#[$derive:meta])*]
        @decl [$v:vis struct $type:ident ($raw:ty)]
        @debug_fmt [$old_dbg:expr_2021]
        @max [$max:expr_2021]
        @no_check_max [$no_check_max:expr_2021]
    ) => {
        $crate::__define_index_type_inner!{
            @configs [$(($CONFIG_NAME; $value))*]
            @attrs [$(#[$attrs])*]
            @derives [$(#[$derive])*]
            @decl [$v struct $type ($raw)]
            @debug_fmt [$dbg]
            @max [$max]
            @no_check_max [$no_check_max]
        }
    };

    // DISPLAY_FORMAT
    (
        @configs [(DISPLAY_FORMAT; $format:expr_2021) $(($CONFIG_NAME:ident; $value:expr_2021))*]
        @attrs [$(#[$attrs:meta])*]
        @derives [$(#[$derive:meta])*]
        @decl [$v:vis struct $type:ident ($raw:ty)]
        @debug_fmt [$dbg:expr_2021]
        @max [$max:expr_2021]
        @no_check_max [$no_check_max:expr_2021]
    ) => {
        $crate::__define_index_type_inner!{
            @configs [$(($CONFIG_NAME; $value))*]
            @attrs [$(#[$attrs])*]
            @derives [$(#[$derive])*]
            @decl [$v struct $type ($raw)]
            @debug_fmt [$dbg]
            @max [$max]
            @no_check_max [$no_check_max]
        }

        impl core::fmt::Display for $type {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, $format, self.index())
            }
        }
    };

    // IMPL_RAW_CONVERSIONS
    (
        @configs [(IMPL_RAW_CONVERSIONS; $val:expr_2021) $(($CONFIG_NAME:ident; $value:expr_2021))*]
        @attrs [$(#[$attrs:meta])*]
        @derives [$(#[$derive:meta])*]
        @decl [$v:vis struct $type:ident ($raw:ty)]
        @debug_fmt [$dbg:expr_2021]
        @max [$max:expr_2021]
        @no_check_max [$no_check_max:expr_2021]
    ) => {
        $crate::__define_index_type_inner!{
            @configs [$(($CONFIG_NAME; $value))*]
            @attrs [$(#[$attrs])*]
            @derives [$(#[$derive])*]
            @decl [$v struct $type ($raw)]
            @debug_fmt [$dbg]
            @max [$max]
            @no_check_max [$no_check_max]
        }
        // Ensure they passed in true. This is... cludgey.
        const _: [(); 1] = [(); $val as usize];

        impl From<$type> for $raw {
            #[inline]
            fn from(v: $type) -> $raw {
                v.raw()
            }
        }

        impl From<$raw> for $type {
            #[inline]
            fn from(value: $raw) -> Self {
                Self::from_raw(value)
            }
        }
    };
    // Try to make rust emit a decent error message...
    (
        @configs [($other:ident; $format:expr_2021) $(($CONFIG_NAME:ident; $value:expr_2021))*]
        @attrs [$(#[$attrs:meta])*]
        @derives [$(#[$derive:meta])*]
        @decl [$v:vis struct $type:ident ($raw:ty)]
        @debug_fmt [$dbg:expr_2021]
        @max [$max:expr_2021]
        @no_check_max [$no_check_max:expr_2021]
    ) => {
        $crate::unknown_define_index_type_option!($other);
    };
    // finish
    (
        @configs []
        @attrs [$(#[$attrs:meta])*]
        @derives [$(#[$derive:meta])*]
        @decl [$v:vis struct $type:ident ($raw:ty)]
        @debug_fmt [$dbg:expr_2021]
        @max [$max:expr_2021]
        @no_check_max [$no_check_max:expr_2021]
    ) => {

        $(#[$derive])*
        $(#[$attrs])*
        #[repr(transparent)]
        $v struct $type { _raw: $raw }

        impl $type {
            /// If `Self::CHECKS_MAX_INDEX` is true, we'll assert if trying to
            /// produce a value larger than this in any of the ctors that don't
            /// have `unchecked` in their name.
            $v const MAX_INDEX: usize = $max;

            /// Does this index type assert if asked to construct an index
            /// larger than MAX_INDEX?
            $v const CHECKS_MAX_INDEX: bool = !$no_check_max;

            /// Construct this index type from a usize. Alias for `from_usize`.
            #[inline(always)]
            $v const fn new(value: usize) -> Self {
                Self::from_usize(value)
            }

            /// Construct this index type from the wrapped integer type.
            #[inline(always)]
            $v const fn from_raw(value: $raw) -> Self {
                Self::from_usize(value as usize)
            }

            /// Construct this index type from one in a different domain
            #[inline(always)]
            $v fn from_foreign<F: $crate::Idx>(value: F) -> Self {
                Self::from_usize(value.index())
            }

            /// Construct from a usize without any checks.
            #[expect(clippy::cast_possible_truncation)]
            #[inline(always)]
            $v const fn from_usize_unchecked(value: usize) -> Self {
                Self { _raw: value as $raw }
            }

            /// Construct from the underlying type without any checks.
            #[inline(always)]
            $v const fn from_raw_unchecked(raw: $raw) -> Self {
                Self { _raw: raw }
            }

            /// Construct this index type from a usize.
            #[expect(clippy::cast_possible_truncation)]
            #[inline]
            $v const fn from_usize(value: usize) -> Self {
                Self::check_index(value as usize);
                Self { _raw: value as $raw }
            }

            /// Get the wrapped index as a usize.
            #[inline(always)]
            $v const fn index(self) -> usize {
                self._raw as usize
            }

            /// Get the wrapped index.
            #[inline(always)]
            $v const fn raw(self) -> $raw {
                self._raw
            }

            /// Asserts `v <= Self::MAX_INDEX` unless Self::CHECKS_MAX_INDEX is false.
            #[inline]
            $v const fn check_index(v: usize) {
                if Self::CHECKS_MAX_INDEX && (v > Self::MAX_INDEX) {
                    panic!("index_vec index overflow");
                }
            }
        }

        impl core::fmt::Debug for $type {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, $dbg, self.index())
            }
        }

        impl core::cmp::PartialOrd<usize> for $type {
            #[inline]
            fn partial_cmp(&self, other: &usize) -> Option<core::cmp::Ordering> {
                self.index().partial_cmp(other)
            }
        }

        impl core::cmp::PartialOrd<$type> for usize {
            #[inline]
            fn partial_cmp(&self, other: &$type) -> Option<core::cmp::Ordering> {
                self.partial_cmp(&other.index())
            }
        }

        impl PartialEq<usize> for $type {
            #[inline]
            fn eq(&self, other: &usize) -> bool {
                self.index() == *other
            }
        }

        impl PartialEq<$type> for usize {
            #[inline]
            fn eq(&self, other: &$type) -> bool {
                *self == other.index()
            }
        }

        impl core::ops::Add<usize> for $type {
            type Output = Self;
            #[inline]
            fn add(self, other: usize) -> Self {
                // use wrapping add so that it's up to the index type whether or
                // not to check -- e.g. if checks are disabled, they're disabled
                // on both debug and release.
                Self::new(self.index().wrapping_add(other))
            }
        }

        impl core::ops::Sub<usize> for $type {
            type Output = Self;
            #[inline]
            fn sub(self, other: usize) -> Self {
                // use wrapping sub so that it's up to the index type whether or
                // not to check -- e.g. if checks are disabled, they're disabled
                // on both debug and release.
                Self::new(self.index().wrapping_sub(other))
            }
        }

        impl core::ops::AddAssign<usize> for $type {
            #[inline]
            fn add_assign(&mut self, other: usize) {
                *self = *self + other
            }
        }

        impl core::ops::SubAssign<usize> for $type {
            #[inline]
            fn sub_assign(&mut self, other: usize) {
                *self = *self - other;
            }
        }

        impl core::ops::Rem<usize> for $type {
            type Output = Self;
            #[inline]
            fn rem(self, other: usize) -> Self {
                Self::new(self.index() % other)
            }
        }

        impl core::ops::Add<$type> for usize {
            type Output = $type;
            #[inline]
            fn add(self, other: $type) -> $type {
                other + self
            }
        }

        impl core::ops::Sub<$type> for usize {
            type Output = $type;
            #[inline]
            fn sub(self, other: $type) -> $type {
                $type::new(self.wrapping_sub(other.index()))
            }
        }

        impl core::ops::Add for $type {
            type Output = $type;
            #[inline]
            fn add(self, other: $type) -> $type {
                $type::new(other.index() + self.index())
            }
        }

        impl core::ops::Sub for $type {
            type Output = $type;
            #[inline]
            fn sub(self, other: $type) -> $type {
                $type::new(self.index().wrapping_sub(other.index()))
            }
        }

        impl core::ops::AddAssign for $type {
            #[inline]
            fn add_assign(&mut self, other: $type) {
                *self = *self + other
            }
        }

        impl core::ops::SubAssign for $type {
            #[inline]
            fn sub_assign(&mut self, other: $type) {
                *self = *self - other;
            }
        }

        impl $crate::Idx for $type {
            const MAX: usize = Self::MAX_INDEX;

            #[inline]
            unsafe fn from_usize_unchecked(idx: usize) -> Self {
                Self::from_usize_unchecked(idx)
            }

            #[inline]
            fn index(self) -> usize {
                usize::from(self)
            }
        }

        impl From<$type> for usize {
            #[inline]
            fn from(v: $type) -> usize {
                v.index()
            }
        }

        impl From<usize> for $type {
            #[inline]
            fn from(value: usize) -> Self {
                $type::from_usize(value)
            }
        }

        $crate::__internal_maybe_index_impl_serde!($type);
    };
}
