#![cfg_attr(test, allow(dead_code))]

#[cfg(not(target_os = "uwp"))]
use crate::sys_common::util::report_overflow;
#[cfg(not(target_os = "uwp"))]
use crate::sys::c;

pub struct Handler;

impl Handler {
    #[cfg(not(target_os = "uwp"))]
    pub unsafe fn new() -> Handler {
        // This API isn't available on XP, so don't panic in that case and just
        // pray it works out ok.
        if c::SetThreadStackGuarantee(&mut 0x5000) == 0 {
            if c::GetLastError() as u32 != c::ERROR_CALL_NOT_IMPLEMENTED as u32 {
                panic!("failed to reserve stack space for exception handling");
            }
        }
        Handler
    }

    #[cfg(target_os = "uwp")]
    pub fn new() -> Handler {
        Handler
    }
}

#[cfg(not(target_os = "uwp"))]
extern "system" fn vectored_handler(ExceptionInfo: *mut c::EXCEPTION_POINTERS)
                                    -> c::LONG {
    unsafe {
        let rec = &(*(*ExceptionInfo).ExceptionRecord);
        let code = rec.ExceptionCode;

        if code == c::EXCEPTION_STACK_OVERFLOW {
            report_overflow();
        }
        c::EXCEPTION_CONTINUE_SEARCH
    }
}

#[cfg(not(target_os = "uwp"))]
pub unsafe fn init() {
    if c::AddVectoredExceptionHandler(0, vectored_handler).is_null() {
        panic!("failed to install exception handler");
    }
    // Set the thread stack guarantee for the main thread.
    let _h = Handler::new();
}

#[cfg(target_os = "uwp")]
pub unsafe fn init() {}

pub unsafe fn cleanup() {}
