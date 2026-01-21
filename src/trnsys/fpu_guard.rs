#[cfg(target_os = "windows")]
pub(crate) mod fpu_guard {
    use std::os::raw::c_uint;

    unsafe extern "C" {
        // https://learn.microsoft.com/en-us/cpp/c-runtime-library/reference/control87-controlfp-control87-2
        fn _control87(new: c_uint, mask: c_uint) -> c_uint;
    }

    // Define constants for exception masks
    const _MCW_EM: c_uint = 0x0008001F; // Mask for all exception bits
    const _EM_INVALID: c_uint = 0x00000010; // Mask for invalid operation exception
    const _EM_ZERODIVIDE: c_uint = 0x00000008; // Mask for division by zero exception
    const _EM_OVERFLOW: c_uint = 0x00000004; // Mask for overflow exception

    // We mask all exceptions
    const MASK_ALL_EXCEPTIONS: c_uint = _MCW_EM;

    /// This struct is responsible for saving the state before entering and restoring it upon destruction.
    pub struct FpuGuard {
        original_control_word: c_uint,
    }

    impl FpuGuard {
        pub fn new() -> Self {
            unsafe {
                // Get the current control word by calling _control87 with 0 as the first argument
                let current = _control87(0, 0);

                // Set the control word to mask all exceptions
                _control87(MASK_ALL_EXCEPTIONS, _MCW_EM);

                FpuGuard {
                    original_control_word: current,
                }
            }
        }
    }

    impl Drop for FpuGuard {
        fn drop(&mut self) {
            unsafe {
                // Restore the original control word when the FpuGuard is dropped
                _control87(self.original_control_word, _MCW_EM);
            }
        }
    }
}

// Empty implementation for non-Windows platforms (just in case)
#[cfg(not(target_os = "windows"))]
mod fpu_guard {
    pub struct FpuGuard;
    impl FpuGuard {
        pub fn new() -> Self {
            Self
        }
    }
}
