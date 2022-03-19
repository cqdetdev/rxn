use std::mem::size_of as size;
use std::thread::sleep as std_sleep;
use std::time::Duration;
use winapi::{
    um::{
        winuser::{
            GetDC,
            HWND_DESKTOP,
            INPUT,
            INPUT_u,
            SendInput,
            MOUSEEVENTF_LEFTDOWN,
            MOUSEEVENTF_LEFTUP,
            INPUT_MOUSE,
            GetCursorPos,
        },
        wingdi::GetPixel
    },
    shared::windef::POINT
};

pub fn main() {
    unsafe {
        loop {
            let mut cursor: POINT = POINT { x: 0, y: 0 };
            GetCursorPos(&mut cursor);
            let rgb = get_rgb(cursor.x, cursor.y);
            if *rgb.get(0).unwrap() == 106 && *rgb.get(1).unwrap() == 219 && *rgb.get(2).unwrap() == 75 {
                click();
            }
            sleep();
        }
    }
}

pub unsafe fn get_rgb(x: i32, y: i32) -> [u32; 3] {
    let h = GetDC(HWND_DESKTOP);
    let rgb = GetPixel(h, x, y);
    let r = (rgb >> 16) & 0xFF;
    let g = (rgb >> 8) & 0xFF;
    let b = rgb & 0xFF;
    [r, g, b]
}

pub unsafe fn click() {
    let mut i1 = INPUT {
        type_: INPUT_MOUSE,
        u: INPUT_u::default()
    };
    let mut mi = i1.u.mi_mut();
    mi.dwFlags = MOUSEEVENTF_LEFTDOWN;
    SendInput(1, &mut i1, size::<INPUT>() as i32);
    sleep();
    i1 = INPUT {
        type_: INPUT_MOUSE,
        u: INPUT_u::default()
    };
    let mut mi = i1.u.mi_mut();
    mi.dwFlags = MOUSEEVENTF_LEFTUP;
    SendInput(1, &mut i1, size::<INPUT>() as i32);
}

pub fn sleep() { std_sleep(Duration::from_millis(1))}