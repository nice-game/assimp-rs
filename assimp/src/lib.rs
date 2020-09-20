macro_rules! define_type {
    // Reference type
    ($(#[$type_attr:meta])* struct $name:ident(&$raw:ty)) => (
		$(#[$type_attr])*
		#[repr(C)]
        pub struct $name<'a>(&'a $raw);

        #[doc(hidden)]
        impl<'a> $name<'a> {
            pub fn from_raw(raw: *const $raw) -> $name<'a> {
                unsafe { $name(&*raw) }
            }
            pub fn to_raw(&self) -> *const $raw {
                self.0
            }
        }

        impl<'a> ::std::ops::Deref for $name<'a> {
            type Target = $raw;
            fn deref<'b>(&'b self) -> &'b $raw { &self.0 }
        }
    );
    // Non-reference type = POD
    ($(#[$type_attr:meta])* struct $name:ident($raw:ty)) => (
        $(#[$type_attr])*
        pub struct $name($raw);

        #[doc(hidden)]
        impl $name {
            pub fn from_raw(raw: *const $raw) -> $name {
                unsafe { $name(*raw) }
            }
        }

        impl ::std::ops::Deref for $name {
            type Target = $raw;
            fn deref<'a>(&'a self) -> &'a $raw { &self.0 }
        }
    );
}

mod importer;
mod scene;

pub use importer::*;
pub use scene::*;
