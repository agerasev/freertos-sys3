//! freertos-sys2 provides low-level bindings to FreeRTOS functions
//!
//! NOTE: this is currently a very incomplete selection of function signatures, that match the
//! selection we needed. Additionally, the presence of some of these functions vary based on
//! FreeRTOS configuration, as does the types used.
#![no_std]
// The types, values, functions, and other items should match the naming used in FreeRTOS
#![allow(non_camel_case_types, non_upper_case_globals, non_snake_case)]
#![allow(clippy::missing_safety_doc)]
#![forbid(unsafe_op_in_unsafe_fn)]

mod config;
mod projdefs;

pub mod queue;
pub mod semphr;
pub mod task;

pub use config::*;
pub use projdefs::*;

use core::ffi::c_void;

#[cfg(not(any(
    feature = "support_static_allocation",
    feature = "support_dynamic_allocation"
)))]
compile_error!(
    "'support_static_allocation' and 'support_dynamic_allocation' cannot both be disabled."
);

pub type BaseType_t = i32;
pub type UBaseType_t = u32;

#[repr(C)]
pub struct StaticListItem_t {
    #[cfg(feature = "use_list_data_integrity_check_bytes")]
    xDummy1: TickType_t,
    xDummy2: TickType_t,
    pvDummy3: [*mut c_void; 4],
    #[cfg(feature = "use_list_data_integrity_check_bytes")]
    xDummy4: TickType_t,
}

#[cfg(feature = "use_mini_list_item")]
struct StaticMiniListItem_t {
    #[cfg(feature = "use_list_data_integrity_check_bytes")]
    xDummy1: TickType_t,
    xDummy2: TickType_t,
    pvDummy3: [*mut c_void; 2],
}

#[cfg(not(features = "use_mini_list_item"))]
pub type StaticMiniListItem_t = StaticListItem_t;

#[repr(C)]
pub struct StaticList_t {
    #[cfg(feature = "use_list_data_integrity_check_bytes")]
    xDummy1: TickType_t,
    uxDummy2: UBaseType_t,
    pvDummy3: *mut c_void,
    xDummy4: StaticMiniListItem_t,
    #[cfg(feature = "use_list_data_integrity_check_bytes")]
    xDummy5: TickType_t,
}

#[repr(C)]
union StaticQueue__U_t {
    pvDummy2: *mut c_void,
    uxDummy2: UBaseType_t,
}

#[repr(C)]
pub struct StaticQueue_t {
    pvDummy1: [*mut c_void; 3],
    u: StaticQueue__U_t,
    xDummy3: [StaticList_t; 2],
    uxDummy4: [UBaseType_t; 3],
    ucDummy5: [u8; 2],

    #[cfg(all(
        feature = "support_static_allocation",
        feature = "support_dynamic_allocation"
    ))]
    ucDummy6: u8,

    #[cfg(feature = "use_queue_sets")]
    pvDummy7: *mut c_void,

    #[cfg(feature = "use_trace_facility")]
    uxDummy8: UBaseType_t,
    #[cfg(feature = "use_trace_facility")]
    ucDummy9: u8,
}

pub const portMAX_DELAY: TickType_t = TickType_t::MAX;
