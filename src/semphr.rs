use crate::{queue::*, *};

pub const semGIVE_BLOCK_TIME: TickType_t = 0;

pub type SemaphoreHandle_t = *mut c_void;
pub type StaticSemaphore_t = StaticQueue_t;

pub unsafe fn xSemaphoreTake(xSemaphore: SemaphoreHandle_t, xBlockTime: TickType_t) -> BaseType_t {
    unsafe { xQueueGenericReceive(xSemaphore, core::ptr::null(), xBlockTime, pdFALSE) }
}

pub unsafe fn xSemaphoreGive(xSemaphore: SemaphoreHandle_t) -> BaseType_t {
    unsafe {
        xQueueGenericSend(
            xSemaphore,
            core::ptr::null(),
            semGIVE_BLOCK_TIME,
            queueSEND_TO_BACK,
        )
    }
}

pub unsafe fn xSemaphoreCreateMutex() -> SemaphoreHandle_t {
    unsafe { xQueueCreateMutex(queueQUEUE_TYPE_MUTEX) }
}

pub unsafe fn vSemaphoreDelete(xSemaphore: SemaphoreHandle_t) {
    unsafe { vQueueDelete(xSemaphore) }
}
