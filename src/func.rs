#[cfg(windows)]
extern crate winapi;

//left = 0 -> left
//left = 1 -> right
#[cfg(windows)]
pub mod func {
    pub fn swap_mouse_button(fswap: bool) -> Result<i32, std::io::Error> {
        use winapi::shared::minwindef::BOOL;

        use winapi::um::winuser::SwapMouseButton;
        let state = fswap as BOOL;
        let ret = unsafe { SwapMouseButton(state) };
        if ret == 0 {
            Err(std::io::Error::last_os_error())
        } else {
            Ok(ret)
        }
    }
}
