#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub const AI_MAX_NUMBER_OF_TEXTURECOORDS: usize = 0x8;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
