use std::os::raw::*;
use paste::paste;

#[macro_export]
/// Creates a opaque struct. Used only for readability
macro_rules! opaque
{
    ($id:ident) => {
        #[repr(C)]
        pub struct $id { _opaque: [u8; 0] }
    }
}

macro_rules! callback
{
    ($id:ident) => {
        extern "C" fn(*mut $id, *mut c_void)
    }
}

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

// ** button **

opaque!(uiButton);

extern "C"
{
    pub fn uiNewButton(text: *const c_char) -> *mut uiButton;
    pub fn uiButtonSetText(b: *mut uiButton, text: *const c_char);
    pub fn uiButtonOnClicked(b: *mut uiButton, f: callback!(uiButton), data: *mut c_void);
    pub fn uiButtonText(b: *mut uiButton) -> *mut c_char;
}

// ** Box **

opaque!(uiBox);

extern "C"
{
    pub fn uiNewHorizontalBox() -> *mut uiBox;
    pub fn uiNewVerticalBox() -> *mut uiBox;
    pub fn uiBoxSetPadded(b: *mut uiBox, padded: i32);
    pub fn uiBoxPadded(b: *mut uiBox) -> i32;
    pub fn uiBoxAppend(b: *mut uiBox, child: *mut uiControl, stretchy: i32);
    pub fn uiBoxDelete(b: *mut uiBox, index: i32);
}

opaque!(uiCheckbox);

extern "C"
{
    pub fn uiNewCheckbox(text: *const c_char) -> *mut uiCheckbox;
    pub fn uiCheckboxSetChecked(c: *mut uiCheckbox, checked: i32);
    pub fn uiCheckboxChecked(c: *mut uiCheckbox) -> i32;
    pub fn uiCheckboxOnToggled(c: *mut uiCheckbox, f: extern "C" fn(c: *mut uiCheckbox, data: *mut c_void), data: *mut c_void);
    pub fn uiCheckBoxSetText(c: *mut uiCheckbox, text: *const c_char);
    pub fn uiCheckboxText(c: *mut uiCheckbox) -> *const c_char;
}

opaque!(uiEntry);

extern "C"
{
    pub fn uiNewEntry() -> *mut uiEntry;
    pub fn uiNewPasswordEntry() -> *mut uiEntry;
    pub fn uiNewSearchEntry() -> *mut uiEntry;

    pub fn uiEntrySetReadOnly(e: *mut uiEntry, readonly: i32);
    pub fn uiEntryReadOnly(e: *mut uiEntry) -> i32;
    pub fn uiEntryText(e: *mut uiEntry) -> *mut c_char;
    pub fn uiEntrySetText(e: *mut uiEntry, text: *const c_char);
    pub fn uiEntryOnChanged(e: *mut uiEntry, f: extern "C" fn(e: *mut uiEntry, data: *mut c_void), data: *mut c_void);
}

opaque!(uiLabel);

extern "C"
{
    pub fn uiNewLabel(text: *const c_char) -> *mut uiLabel;
    pub fn uiLabelSetText(l: *mut uiLabel, text: *const c_char);
    pub fn uiLabelText(l: *mut uiLabel) -> *mut c_char;
}

opaque!(uiTab);

extern "C"
{
    pub fn uiNewTab() -> *mut uiTab;
    pub fn uiTabSetMargined(t: *mut uiTab, page: i32, margined: i32);
    pub fn uiTabMargined(t: *mut uiTab, page: i32);
    pub fn uiTabNumPages(t: *mut uiTab) -> i32;
    pub fn uiTabDelete(t: *mut uiTab, index: i32);
    pub fn uiTabInsertAt(t: *mut uiTab, name: *const c_char, before: i32, c: *mut uiControl);
    pub fn uiTabAppend(t: *mut uiTab, name: *const c_char, c: *mut uiControl);
}

opaque!(uiGroup);

extern "C"
{
    pub fn uiNewGroup() -> *mut uiGroup;
    pub fn uiGroupMargined(g: *mut uiGroup) -> i32;
    pub fn uiGroupSetMargined(g: *mut uiGroup, margined: i32);
    pub fn uiGroupTitle(g: *mut uiGroup) -> *mut c_char;
    pub fn uiGroupSetTitle(g: *mut uiGroup, title: *const c_char);
    pub fn uiGroupSetChild(g: *mut uiGroup, c: *mut uiControl);
}

opaque!(uiSpinbox);

extern "C"
{
    pub fn uiNewSpinbox(min: i32, max: i32) -> *mut uiSpinbox;
    pub fn uiSpinboxSetValue(s: *mut uiSpinbox, value: i32);
    pub fn uiSpinboxValue(s: *mut uiSpinbox) -> i32;
    pub fn uiSpinboxOnChanged(s: *mut uiSpinbox, f: callback!(uiSpinbox), data: *mut c_void);
}

opaque!(uiSlider);

extern "C"
{
    pub fn uiNewSlider(min: i32, max: i32) -> *mut uiSlider;
    pub fn uiSliderValue(s: *mut uiSlider) -> i32;
    pub fn uiSliderSetValue(s: *mut uiSlider, value: i32);
    pub fn uiSliderOnChanged(s: *mut uiSlider, f: callback!(uiSlider), data: *mut c_void);
}

opaque!(uiProgressBar);

extern "C"
{
    pub fn uiNewProgressBar() -> *mut uiProgressBar;
    pub fn uiProgressBarValue(p: *mut uiProgressBar) -> i32;
    pub fn uiProgressBarSetValue(p: *mut uiProgressBar, value: i32);
}

opaque!(uiSeparator);

extern "C"
{
    pub fn uiNewHorizontalSeparator() -> *mut uiSeparator;
    pub fn uiNewVerticalSeparator() -> *mut uiSeparator;
}

opaque!(uiCombobox);

extern "C"
{
    pub fn uiNewCombobox() -> *mut uiCombobox;
    pub fn uiComboboxAppend(c: *mut uiCombobox, text: *const c_char);
    pub fn uiComboboxSelected(c: *mut uiCombobox) -> i32;
    pub fn uiComboboxSetSelected(c: *mut uiCombobox, n: i32);
    pub fn uiComboboxOnSelected(c: *mut uiCombobox, f: callback!(uiCombobox), data: *mut c_void);
}

opaque!(uiEditableCombobox);

extern "C"
{
    pub fn uiNewEditableCombobox() -> *mut uiEditableCombobox;
    pub fn uiEditableComboboxAppend(c: *mut uiEditableCombobox, text: *const c_char);
    pub fn uiEditableComboboxText(c: *mut uiEditableCombobox) -> *mut c_char;
    pub fn uiEditableComboboxSetText(c: *mut uiEditableCombobox, text: *const c_char);
    pub fn uiEditableComboboxOnChanged(c: *mut uiEditableCombobox, f: callback!(uiEditableCombobox), data: *mut c_void);
}

opaque!(uiRadioButtons);

extern "C"
{
    pub fn uiNewRadioButtons() -> *mut uiRadioButtons;
    pub fn uiRadioButtonsOnSelected(r: *mut uiRadioButtons, f: callback!(uiRadioButtons), data: *mut c_void);
    pub fn uiRadioButtonsSetSelected(r: *mut uiRadioButtons, n: i32);
    pub fn uiRadioButtonsSelected(r: *mut uiRadioButtons);
    pub fn uiRadioButtonsAppend(r: *mut uiRadioButtons, text: *const c_char);
}

opaque!(uiDateTimePicker);
opaque!(tm);

extern "C"
{
    pub fn uiNewDateTimePicker() -> *mut uiDateTimePicker;
    pub fn uiNewDatePicker() -> *mut uiDateTimePicker;
    pub fn uiNewTimePicker() -> *mut uiDateTimePicker;
    pub fn uiDateTimePickerTime(d: *mut uiDateTimePicker, time: *mut tm);
    pub fn uiDateTimePickerSetTime(d: *mut uiDateTimePicker, time: *const tm);
    pub fn uiDateTimePickerOnChanged(d: *mut uiDateTimePicker, f: callback!(uiDateTimePicker), data: *mut c_void); 
}

opaque!(uiMultilineEntry);
extern
{
    pub fn uiNewMultilineEntry() -> *mut uiMultilineEntry;
    pub fn uiNewNonWrappingMultilineEntry() -> *mut uiMultilineEntry;
    pub fn uiMultilineEntryText(e: *mut uiMultilineEntry) -> *mut c_char;
    pub fn uiMultilineEntrySetText(e: *mut uiMultilineEntry, text: *const c_char);
    pub fn uiMultilineEntryAppend(e: *mut uiMultilineEntry, text: *const c_char);
    pub fn uiMultilineEntryOnChanged(e: *mut uiMultilineEntry, f: callback!(uiMultilineEntry), data: *mut c_void);
    pub fn uiMultilineEntrySetReadOnly(e: *mut uiMultilineEntry, readonly: i32);
    pub fn uiMultilineEntryReadOnly(e: *mut uiMultilineEntry) -> i32;
}

opaque!(uiMenuItem);

extern
{
    pub fn uiMenuItemEnable(m: *mut uiMenuItem);
    pub fn uiMenuItemDisable(m: *mut uiMenuItem);
    pub fn uiMenuItemOnClicked(m: *mut uiMenuItem, f: callback!(uiMenuItem), data: *mut c_void);
    pub fn uiMenuItemChecked(m: *mut uiMenuItem) -> i32;
    pub fn uiMenuItemSetChecked(m: *mut uiMenuItem, checked: i32);
}

opaque!(uiMenu);

extern 
{
    pub fn uiNewMenu(name: *const c_char) -> *mut uiMenu;
    pub fn uiMenuAppendSeparator(m: *mut uiMenu);
    pub fn uiMenuAppendAboutItem(m: *mut uiMenu);
    pub fn uiMenuAppendQuitItem(m: *mut uiMenu);
    pub fn uiMenuAppendPreferencesItem(m: *mut uiMenu);
    pub fn uiMenuAppendCheckItem(m: *mut uiMenu, name: *const c_char);
    pub fn uiMenuAppendItem(m: *mut uiMenu, name: *const c_char);
}
