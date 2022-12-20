use crate::*;
use core::ffi::c_void;

pub const queueSEND_TO_BACK: BaseType_t = 0;
pub const queueSEND_TO_FRONT: BaseType_t = 1;
pub const queueOVERWRITE: BaseType_t = 2;

pub const queueQUEUE_TYPE_BASE: u8 = 0;
pub const queueQUEUE_TYPE_SET: u8 = 0;
pub const queueQUEUE_TYPE_MUTEX: u8 = 1;
pub const queueQUEUE_TYPE_COUNTING_SEMAPHORE: u8 = 2;
pub const queueQUEUE_TYPE_BINARY_SEMAPHORE: u8 = 3;
pub const queueQUEUE_TYPE_RECURSIVE_MUTEX: u8 = 4;

pub type QueueHandle_t = *mut c_void;

extern "C" {

    pub fn xQueueGenericReceive(
        xQueue: QueueHandle_t,
        pvBuffer: *const c_void,
        xTicksToWait: TickType_t,
        xJustPeek: BaseType_t,
    ) -> BaseType_t;

    pub fn xQueueGenericSend(
        xQueue: QueueHandle_t,
        pvItemToQueue: *const c_void,
        xTicksToWait: TickType_t,
        xCopyPosition: BaseType_t,
    ) -> BaseType_t;

    pub fn xQueueGenericCreateStatic(
        uxQueueLength: UBaseType_t,
        uxItemSize: UBaseType_t,
        pucQueueStorage: *mut u8,
        pxStaticQueue: *mut StaticQueue_t,
        ucQueueType: u8,
    ) -> QueueHandle_t;

    pub fn xQueueCreateMutex(ucQueueType: u8) -> QueueHandle_t;

    pub fn vQueueDelete(xQueue: QueueHandle_t);
}

pub unsafe fn xQueueCreateStatic(
    uxQueueLength: UBaseType_t,
    uxItemSize: UBaseType_t,
    pucQueueStorage: *mut u8,
    pxQueueBuffer: *mut StaticQueue_t,
) -> QueueHandle_t {
    unsafe {
        xQueueGenericCreateStatic(
            uxQueueLength,
            uxItemSize,
            pucQueueStorage,
            pxQueueBuffer,
            queueQUEUE_TYPE_BASE,
        )
    }
}
