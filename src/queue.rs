use crate::{task::TaskHandle_t, *};
use core::ffi::c_void;

pub(crate) const queueSEND_TO_BACK: BaseType_t = 0;
pub(crate) const queueSEND_TO_FRONT: BaseType_t = 1;
pub(crate) const queueOVERWRITE: BaseType_t = 2;

pub(crate) const queueQUEUE_TYPE_BASE: u8 = 0;
pub(crate) const queueQUEUE_TYPE_SET: u8 = 0;
pub(crate) const queueQUEUE_TYPE_MUTEX: u8 = 1;
pub(crate) const queueQUEUE_TYPE_COUNTING_SEMAPHORE: u8 = 2;
pub(crate) const queueQUEUE_TYPE_BINARY_SEMAPHORE: u8 = 3;
pub(crate) const queueQUEUE_TYPE_RECURSIVE_MUTEX: u8 = 4;

#[repr(C)]
pub struct QueueDefinition {
    _unused: [u8; 0],
}

pub type QueueHandle_t = *mut QueueDefinition;
pub type QueueSetHandle_t = QueueHandle_t;
pub type QueueSetMemberHandle_t = QueueHandle_t;

extern "C" {

    pub fn xQueueGenericSend(
        xQueue: QueueHandle_t,
        pvItemToQueue: *const c_void,
        xTicksToWait: TickType_t,
        xCopyPosition: BaseType_t,
    ) -> BaseType_t;

    pub fn xQueuePeek(
        xQueue: QueueHandle_t,
        pvBuffer: *mut c_void,
        xTicksToWait: TickType_t,
    ) -> BaseType_t;

    pub fn xQueuePeekFromISR(xQueue: QueueHandle_t, pvBuffer: *const c_void) -> BaseType_t;

    pub fn xQueueReceive(
        xQueue: QueueHandle_t,
        pvBuffer: *mut c_void,
        xTicksToWait: TickType_t,
    ) -> BaseType_t;

    pub fn uxQueueMessagesWaiting(xQueue: QueueHandle_t) -> UBaseType_t;

    pub fn uxQueueSpacesAvailable(xQueue: QueueHandle_t) -> UBaseType_t;

    pub fn vQueueDelete(xQueue: QueueHandle_t);

    pub fn xQueueGenericReceive(
        xQueue: QueueHandle_t,
        pvBuffer: *const c_void,
        xTicksToWait: TickType_t,
        xJustPeek: BaseType_t,
    ) -> BaseType_t;

    pub fn xQueueGenericSendFromISR(
        xQueue: QueueHandle_t,
        pvItemToQueue: *const c_void,
        pxHigherPriorityTaskWoken: *mut BaseType_t,
        xCopyPosition: BaseType_t,
    ) -> BaseType_t;

    pub fn xQueueGiveFromISR(
        xQueue: QueueHandle_t,
        pxHigherPriorityTaskWoken: *mut BaseType_t,
    ) -> BaseType_t;

    pub fn xQueueReceiveFromISR(
        xQueue: QueueHandle_t,
        pvBuffer: *mut c_void,
        pxHigherPriorityTaskWoken: *mut BaseType_t,
    ) -> BaseType_t;

    pub fn xQueueIsQueueEmptyFromISR(xQueue: QueueHandle_t) -> BaseType_t;
    pub fn xQueueIsQueueFullFromISR(xQueue: QueueHandle_t) -> BaseType_t;
    pub fn uxQueueMessagesWaitingFromISR(xQueue: QueueHandle_t) -> UBaseType_t;

    pub(crate) fn xQueueCreateMutex(ucQueueType: u8) -> QueueHandle_t;
    pub(crate) fn xQueueCreateMutexStatic(
        ucQueueType: u8,
        pxStaticQueue: *mut StaticQueue_t,
    ) -> QueueHandle_t;
    pub(crate) fn xQueueCreateCountingSemaphore(
        uxMaxCount: UBaseType_t,
        uxInitialCount: UBaseType_t,
    ) -> QueueHandle_t;
    pub(crate) fn xQueueCreateCountingSemaphoreStatic(
        uxMaxCount: UBaseType_t,
        uxInitialCount: UBaseType_t,
        pxStaticQueue: *mut StaticQueue_t,
    ) -> QueueHandle_t;
    pub(crate) fn xQueueSemaphoreTake(
        xQueue: QueueHandle_t,
        xTicksToWait: TickType_t,
    ) -> BaseType_t;
    pub(crate) fn xQueueGetMutexHolder(xSemaphore: QueueHandle_t) -> TaskHandle_t;
    pub(crate) fn xQueueGetMutexHolderFromISR(xSemaphore: QueueHandle_t) -> TaskHandle_t;

    pub(crate) fn xQueueTakeMutexRecursive(
        xMutex: QueueHandle_t,
        xTicksToWait: TickType_t,
    ) -> BaseType_t;
    pub(crate) fn xQueueGiveMutexRecursive(xMutex: QueueHandle_t) -> BaseType_t;

    #[cfg(feature = "support_dynamic_allocation")]
    pub fn xQueueGenericCreate(
        uxQueueLength: UBaseType_t,
        uxItemSize: UBaseType_t,
        ucQueueType: u8,
    ) -> QueueHandle_t;
    #[cfg(feature = "support_static_allocation")]
    pub fn xQueueGenericCreateStatic(
        uxQueueLength: UBaseType_t,
        uxItemSize: UBaseType_t,
        pucQueueStorage: *mut u8,
        pxStaticQueue: *mut StaticQueue_t,
        ucQueueType: u8,
    ) -> QueueHandle_t;

    pub fn xQueueCreateSet(uxEventQueueLength: UBaseType_t) -> QueueSetHandle_t;
    pub fn xQueueAddToSet(
        xQueueOrSemaphore: QueueSetMemberHandle_t,
        xQueueSet: QueueSetHandle_t,
    ) -> BaseType_t;
    pub fn xQueueRemoveFromSet(
        xQueueOrSemaphore: QueueSetMemberHandle_t,
        xQueueSet: QueueSetHandle_t,
    ) -> BaseType_t;
    pub fn xQueueSelectFromSet(
        xQueueSet: QueueSetHandle_t,
        xTicksToWait: TickType_t,
    ) -> QueueSetMemberHandle_t;
    pub fn xQueueSelectFromSetFromISR(xQueueSet: QueueSetHandle_t) -> QueueSetMemberHandle_t;

    fn vQueueWaitForMessageRestricted(
        xQueue: QueueHandle_t,
        xTicksToWait: TickType_t,
        xWaitIndefinitely: BaseType_t,
    );
    fn xQueueGenericReset(xQueue: QueueHandle_t, xNewQueue: BaseType_t) -> BaseType_t;
    fn vQueueSetQueueNumber(xQueue: QueueHandle_t, uxQueueNumber: UBaseType_t);
    fn uxQueueGetQueueNumber(xQueue: QueueHandle_t) -> UBaseType_t;
    fn ucQueueGetQueueType(xQueue: QueueHandle_t) -> u8;

}

#[cfg(feature = "support_dynamic_allocation")]
pub unsafe fn xQueueCreate(uxQueueLength: UBaseType_t, uxItemSize: UBaseType_t) -> QueueHandle_t {
    unsafe { xQueueGenericCreate(uxQueueLength, uxItemSize, queueQUEUE_TYPE_BASE) }
}

#[cfg(feature = "support_static_allocation")]
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

pub unsafe fn xQueueSendToFront(
    xQueue: QueueHandle_t,
    pvItemToQueue: *const c_void,
    xTicksToWait: TickType_t,
) -> BaseType_t {
    unsafe { xQueueGenericSend(xQueue, pvItemToQueue, xTicksToWait, queueSEND_TO_FRONT) }
}

pub unsafe fn xQueueSendToBack(
    xQueue: QueueHandle_t,
    pvItemToQueue: *const c_void,
    xTicksToWait: TickType_t,
) -> BaseType_t {
    unsafe { xQueueGenericSend(xQueue, pvItemToQueue, xTicksToWait, queueSEND_TO_BACK) }
}

#[deprecated(note = "Use 'xQueueSendToBack' instead")]
pub unsafe fn xQueueSend(
    xQueue: QueueHandle_t,
    pvItemToQueue: *const c_void,
    xTicksToWait: TickType_t,
) -> BaseType_t {
    unsafe { xQueueGenericSend(xQueue, pvItemToQueue, xTicksToWait, queueSEND_TO_BACK) }
}

pub unsafe fn xQueueOverwrite(xQueue: QueueHandle_t, pvItemToQueue: *const c_void) -> BaseType_t {
    unsafe { xQueueGenericSend(xQueue, pvItemToQueue, 0, queueOVERWRITE) }
}

pub unsafe fn xQueueSendToFrontFromISR(
    xQueue: QueueHandle_t,
    pvItemToQueue: *const c_void,
    pxHigherPriorityTaskWoken: *mut BaseType_t,
) -> BaseType_t {
    unsafe {
        xQueueGenericSendFromISR(
            xQueue,
            pvItemToQueue,
            pxHigherPriorityTaskWoken,
            queueSEND_TO_FRONT,
        )
    }
}

pub unsafe fn xQueueSendToBackFromISR(
    xQueue: QueueHandle_t,
    pvItemToQueue: *const c_void,
    pxHigherPriorityTaskWoken: *mut BaseType_t,
) -> BaseType_t {
    unsafe {
        xQueueGenericSendFromISR(
            xQueue,
            pvItemToQueue,
            pxHigherPriorityTaskWoken,
            queueSEND_TO_BACK,
        )
    }
}
pub unsafe fn xQueueOverwriteFromISR(
    xQueue: QueueHandle_t,
    pvItemToQueue: *const c_void,
    pxHigherPriorityTaskWoken: *mut BaseType_t,
) -> BaseType_t {
    unsafe {
        xQueueGenericSendFromISR(
            xQueue,
            pvItemToQueue,
            pxHigherPriorityTaskWoken,
            queueOVERWRITE,
        )
    }
}
#[deprecated(note = "Use 'xQueueSendToBackFromISR' instead")]
pub unsafe fn xQueueSendFromISR(
    xQueue: QueueHandle_t,
    pvItemToQueue: *const c_void,
    pxHigherPriorityTaskWoken: *mut BaseType_t,
) -> BaseType_t {
    unsafe {
        xQueueGenericSendFromISR(
            xQueue,
            pvItemToQueue,
            pxHigherPriorityTaskWoken,
            queueSEND_TO_BACK,
        )
    }
}

pub unsafe fn xQueueReset(xQueue: QueueHandle_t) -> BaseType_t {
    unsafe { xQueueGenericReset(xQueue, pdFALSE) }
}
