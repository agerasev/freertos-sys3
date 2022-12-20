use crate::*;
use core::ffi::c_void;

pub type TaskHandle_t = *mut c_void;

extern "C" {
    pub fn xTaskGetTickCountFromISR() -> TickType_t;
    pub fn xTaskGetTickCount() -> TickType_t;

    pub fn xTaskGetCurrentTaskHandle() -> TaskHandle_t;

    pub fn vTaskResume(xTaskToResume: TaskHandle_t);
    pub fn vTaskSuspend(xTaskToSuspend: TaskHandle_t);
    pub fn vTaskDelayUntil(pxPreviousWakeTime: *mut TickType_t, xTimeIncrement: TickType_t);
    pub fn xTaskDelayUntil(
        pxPreviousWakeTime: *mut TickType_t,
        xTimeIncrement: TickType_t,
    ) -> BaseType_t;
    pub fn vTaskDelay(xTicksToDelay: TickType_t);

    pub fn pvTaskGetThreadLocalStoragePointer(
        xTaskToQuery: TaskHandle_t,
        xIndex: BaseType_t,
    ) -> *mut c_void;

    pub fn vTaskSetThreadLocalStoragePointer(
        xTaskToSet: TaskHandle_t,
        xIndex: BaseType_t,
        pvValue: *mut c_void,
    );

}
