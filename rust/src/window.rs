use std::os::raw::*;

#[repr(C)]
pub struct uiWindow { _unused : [ u8 ; 0 ] }

extern "C"
{
    pub fn uiNewWindow(title: *const c_char, width: i32, height: i32, hasMenubar: i32) -> *mut uiWindow;
    pub fn uiWindowSetMargined(w: *mut uiWindow, margined: i32);
    pub fn uiWindowMargined(w: *mut uiWindow) -> i32;
    pub fn uiWindowBorderless(w: *mut uiWindow) -> i32;
    pub fn uiWindowSetBorderless(w: *mut uiWindow, borderless: i32);
    pub fn uiWindowSetFullscreen(w: *mut uiWindow, fullscreen: i32);
    pub fn uiWindowFullscreen(w: *mut uiWindow) -> i32;
    pub fn uiWindowContentSize(w: *mut uiWindow, width: *mut i32, height: *mut i32);
    pub fn uiWindowSetContentSize(w: *mut uiWindow, width: i32, height: i32);

    /* *** Callbacks *** */
    pub fn uiWindowOnClosing(w: *mut uiWindow, f: extern "C" fn(w: *mut uiWindow, data: *mut c_void));
    pub fn uiWindowOnContentSizeChanged(w: *mut uiWindow, f: extern "C" fn(w: *mut uiWindow, data: *mut c_void));
}

