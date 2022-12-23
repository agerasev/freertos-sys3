use crate::*;
use core::ffi::{c_char, c_void};

pub const tskDEFAULT_INDEX_TO_NOTIFY: usize = 0;

#[repr(C)]
pub struct tskTaskControlBlock {
    _unused: [u8; 0],
}

pub type TaskHandle_t = *mut tskTaskControlBlock;

pub type TaskHookFunction_t = Option<unsafe extern "C" fn(*mut c_void) -> BaseType_t>;

#[repr(u32)]
pub enum eTaskState {
    eRunning = 0,
    eReady,
    eBlocked,
    eSuspended,
    eDeleted,
    eInvalid,
}

#[repr(u32)]
pub enum eNotifyAction {
    eNoAction = 0,
    eSetBits,
    eIncrement,
    eSetValueWithOverwrite,
    eSetValueWithoutOverwrite,
}

#[repr(C)]
struct TimeOut_t {
    xOverflowCount: BaseType_t,
    xTimeOnEntering: TickType_t,
}
/*
#[repr(C)]
pub struct TaskStatus_t {
    xHandle: TaskHandle_t,
    pcTaskName: *const c_char,
    xTaskNumber: UBaseType_t,
    eCurrentState: eTaskState,
    uxCurrentPriority: UBaseType_t,
    uxBasePriority: UBaseType_t,
    ulRunTimeCounter: configRUN_TIME_COUNTER_TYPE,
    pxStackBase: *mut StackType_t,
    #[cfg(all(feature = "stack_growth > 0", feature = "record_stack_high_address"))]
    pxTopOfStack: *mut StackType_t,
    #[cfg(all(feature = "stack_growth > 0", feature = "record_stack_high_address"))]
    pxEndOfStack: *mut StackType_t,
    usStackHighWaterMark: configSTACK_DEPTH_TYPE,
}
*/
#[repr(u32)]
pub enum eSleepModeStatus {
    eAbortSleep = 0,
    eStandardSleep,
    #[cfg(feature = "include__vTaskSuspend")]
    eNoTasksWaitingTimeout,
}

pub const tskIDLE_PRIORITY: UBaseType_t = 0;

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
