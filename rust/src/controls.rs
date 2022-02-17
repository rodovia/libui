use std::os::raw::*;

#[repr(C)]
#[allow(non_snake_case)]
pub struct uiControl
{
    pub Signature: i32,
    pub OSSignature: i32,
    pub TypeSignature: i32,
    pub Destroy: extern "C" fn(c: *mut uiControl),
    pub Handle: extern "C" fn(c: *mut uiControl) -> c_ulonglong,
    pub Parent: extern "C" fn(c: *mut uiControl) -> *mut uiControl,
    pub SetParent: extern "C" fn(c: *mut uiControl, c1: *mut uiControl),
    pub Toplevel: extern "C" fn(c: *mut uiControl) -> i32,
    pub Visible: extern "C" fn(c: *mut uiControl) -> i32,
    pub Show: extern "C" fn(c: *mut uiControl),
    pub Hide: extern "C" fn(c: *mut uiControl),
    pub Enabled: extern "C" fn(c: *mut uiControl) -> i32,
    pub Enable: extern "C" fn(c: *mut uiControl),
    pub Disable: extern "C" fn(c: *mut uiControl),
}

extern "C"
{
    pub fn uiControlEnable(c: *mut uiControl);
    pub fn uiControlDisable(c: *mut uiControl);
    pub fn uiControlShow(c: *mut uiControl);
    pub fn uiControlHide(c: *mut uiControl);
    pub fn uiControlVisible(c: *mut uiControl) -> i32;
    pub fn uiControlToplevel(c: *mut uiControl) -> i32;
    pub fn uiControlEnabled(c: *mut uiControl) -> i32;
    pub fn uiControlParent(c: *mut uiControl, c1: *mut uiControl) -> *mut uiControl;
    pub fn uiControlDestroy(c: *mut uiControl);
    pub fn uiControlHandle(c: *mut uiControl) -> c_ulonglong;
    pub fn uiControlVerifySetParent(c: *mut uiControl, c1: *mut uiControl);
    pub fn uiControlEnabledToUser(c: *mut uiControl) -> i32;   

    pub fn uiAllocControl(n: c_ulonglong, OSSig: u32, typesig: u32, typenamestr: *const c_char) -> *mut uiControl;
    pub fn uiFreeControl(c: *mut uiControl);
}


