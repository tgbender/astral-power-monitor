//! Custom caption; native child buttons retain keyboard and accessibility support.
use std::ptr::{null, null_mut};
use windows_sys::Win32::Foundation::*;
use windows_sys::Win32::Graphics::Dwm::*;
use windows_sys::Win32::Graphics::Gdi::*;
use windows_sys::Win32::UI::Controls::*;
use windows_sys::Win32::UI::WindowsAndMessaging::*;

pub const HEIGHT: i32 = 46;
pub const MINIMIZE: usize = 111;
pub const HIDE: usize = 112;
const BUTTON_WIDTH: i32 = 48;

fn wide(value: &str) -> Vec<u16> {
    value.encode_utf16().chain(Some(0)).collect()
}

pub fn caption_hit(width: i32, x: i32, y: i32) -> i32 {
    if x >= 0 && x < width - BUTTON_WIDTH * 2 && (0..HEIGHT).contains(&y) {
        HTCAPTION as i32
    } else {
        HTCLIENT as i32
    }
}

pub unsafe fn create(hwnd: HWND, instance: HINSTANCE) -> Result<(), String> {
    unsafe {
        for (id, name) in [(MINIMIZE, "Minimize window"), (HIDE, "Hide to tray")] {
            let button = CreateWindowExW(
                0,
                wide("BUTTON").as_ptr(),
                wide(name).as_ptr(),
                WS_CHILD | WS_VISIBLE | WS_TABSTOP | BS_OWNERDRAW as u32,
                0,
                0,
                BUTTON_WIDTH,
                HEIGHT,
                hwnd,
                id as HMENU,
                instance,
                null(),
            );
            if button.is_null() {
                return Err("Cannot create caption controls".into());
            }
        }
        layout(hwnd);
        // Preserve the compositor's shadow and rounded corners where supported.
        let margins = MARGINS {
            cxLeftWidth: 1,
            cxRightWidth: 1,
            cyTopHeight: 1,
            cyBottomHeight: 1,
        };
        DwmExtendFrameIntoClientArea(hwnd, &margins);
        let dark: i32 = 1;
        DwmSetWindowAttribute(hwnd, 20, &dark as *const _ as _, size_of::<i32>() as u32);
        let rounded: i32 = 2;
        DwmSetWindowAttribute(hwnd, 33, &rounded as *const _ as _, size_of::<i32>() as u32);
        SetWindowPos(
            hwnd,
            null_mut(),
            0,
            0,
            0,
            0,
            SWP_NOMOVE | SWP_NOSIZE | SWP_NOZORDER | SWP_NOACTIVATE | SWP_FRAMECHANGED,
        );
    }
    Ok(())
}

pub unsafe fn layout(hwnd: HWND) {
    unsafe {
        let mut client: RECT = std::mem::zeroed();
        GetClientRect(hwnd, &mut client);
        for (id, offset) in [(MINIMIZE, 2), (HIDE, 1)] {
            let button = GetDlgItem(hwnd, id as i32);
            if !button.is_null() {
                MoveWindow(
                    button,
                    client.right - BUTTON_WIDTH * offset,
                    0,
                    BUTTON_WIDTH,
                    HEIGHT,
                    1,
                );
            }
        }
    }
}

pub unsafe fn draw_button(item: &DRAWITEMSTRUCT) -> bool {
    if ![MINIMIZE as u32, HIDE as u32].contains(&item.CtlID) {
        return false;
    }
    unsafe {
        let pressed = item.itemState & ODS_SELECTED != 0;
        let background = if pressed { 0x003b3029 } else { 0x00221914 };
        let brush = CreateSolidBrush(background);
        FillRect(item.hDC, &item.rcItem, brush);
        DeleteObject(brush);
        let pen = CreatePen(PS_SOLID, 2, 0x00f6efe8);
        let old = SelectObject(item.hDC, pen);
        let cx = (item.rcItem.left + item.rcItem.right) / 2;
        let cy = (item.rcItem.top + item.rcItem.bottom) / 2;
        if item.CtlID == MINIMIZE as u32 {
            MoveToEx(item.hDC, cx - 6, cy + 1, null_mut());
            LineTo(item.hDC, cx + 6, cy + 1);
        } else {
            MoveToEx(item.hDC, cx - 5, cy - 5, null_mut());
            LineTo(item.hDC, cx + 5, cy + 5);
            MoveToEx(item.hDC, cx + 5, cy - 5, null_mut());
            LineTo(item.hDC, cx - 5, cy + 5);
        }
        SelectObject(item.hDC, old);
        DeleteObject(pen);
        if item.itemState & ODS_FOCUS != 0 {
            let focus = RECT {
                left: item.rcItem.left + 6,
                top: item.rcItem.top + 6,
                right: item.rcItem.right - 6,
                bottom: item.rcItem.bottom - 6,
            };
            DrawFocusRect(item.hDC, &focus);
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn caption_drag_excludes_buttons_and_body() {
        assert_eq!(caption_hit(730, 40, 20), HTCAPTION as i32);
        for (x, y) in [(634, 20), (700, 20), (40, 46), (-1, 20), (40, -1)] {
            assert_eq!(caption_hit(730, x, y), HTCLIENT as i32);
        }
    }
}
