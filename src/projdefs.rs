use crate::*;
use core::ffi::c_void;

pub type TaskFunction_t = Option<unsafe extern "C" fn(*mut c_void)>;

pub const pdFALSE: BaseType_t = 0;
pub const pdTRUE: BaseType_t = 1;

pub const pdPASS: BaseType_t = pdTRUE;
pub const pdFAIL: BaseType_t = pdFALSE;
pub const errQUEUE_EMPTY: BaseType_t = 0;
pub const errQUEUE_FULL: BaseType_t = 0;

pub const errCOULD_NOT_ALLOCATE_REQUIRED_MEMORY: BaseType_t = -1;
pub const errQUEUE_BLOCKED: BaseType_t = -4;
pub const errQUEUE_YIELD: BaseType_t = -5;

#[cfg(feature = "use_16_bit_ticks")]
pub const pdINTEGRITY_CHECK_VALUE: TickType_t = 0x5a5a;
#[cfg(not(feature = "use_16_bit_ticks"))]
pub const pdINTEGRITY_CHECK_VALUE: TickType_t = 0x5a5a5a5a;

pub const pdLITTLE_ENDIAN: BaseType_t = 0;
pub const pdBIG_ENDIAN: BaseType_t = 1;
