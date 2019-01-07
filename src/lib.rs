#![no_std]

extern crate libc;

use libc::isalnum;

fn plus_one(x: i32) -> i32 {
    x + 1
}

#[no_mangle]
pub extern "C" fn _DllMainCRTStartup() -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn hello() -> libc::c_int {
    unsafe {
        let r = isalnum(0);
        r
    }
}

use core::panic::PanicInfo;

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
