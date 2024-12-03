use crate::{arch::{self, aarch64::platform::delay_s}, kernel::uname::Utsname, print, util::util::hex_print32, KERNEL_STRUCT};
use core::{arch::asm, ascii, ptr::{self, write_volatile}, sync::atomic::Ordering};

use super::graphics::{console::GfxConsole, gfx::Color};

const HELP_TEXT: &str = "arsh shell
copyright 2023 arrynfr

Available builtin commands:
bg          cd
clear       dbg
drop        exit
fsdown      fsup
hd          help
mmu         shutdown
svc         wb";

// Custom error type for the shell
#[derive(Debug)]
enum ShellError {
    UnknownCommand,
    ParsingError,
    UserExit,
    ArgumentError,
    _InvalidPtr
}

// Function to write a value to a memory address
fn write_mem64(offset: usize, value: u8) -> Result<(), ShellError> {
    println!("Writing {} to {:?}", value, offset);
    unsafe { write_volatile(offset as *mut u8, value) }
    Ok(())
}

extern "C" {
    static _user_start: u8;
    static _user_stack: u8;
}

// Function to process user commands
fn process_command(input_cmd: &mut [ascii::Char; 128]) -> Result<(), ShellError> {
    let cmd_line = input_cmd.as_str().lines().next().unwrap_or("");
    let mut input_it = cmd_line.split_whitespace();
    let cmd = input_it.next().unwrap_or("");

    match cmd {
        "" => Ok(()),
        "help" => {
            println!("{}", HELP_TEXT);
            Ok(())
        }
        "cd" => Ok(()),
        "bg" => {
            let mut args = [0; 3];
            let mut it = input_it.take(3).enumerate().peekable();
            if it.peek().is_none() {
                return Err(ShellError::ArgumentError);
            }

            while let Some((i, x)) = it.next() {
                args[i] = match x.starts_with("0x") {
                    true => usize::from_str_radix(&x[2..], 16).map_err(|_| ShellError::ParsingError)?,
                    false => x.parse().map_err(|_| ShellError::ParsingError)?,
                };
                
                if it.peek().is_none() && i != 2 {
                    return Err(ShellError::ArgumentError);
                }
            }
            
            let ks = KERNEL_STRUCT.load(Ordering::SeqCst);
            if ks != ptr::null_mut() {
                GfxConsole::_aquire();
                unsafe {
                    match &mut (*ks).console {
                        Some(c) => {
                                    c.set_background_color(Color { b: args[0] as u16, g: args[1] as u16, r: args[2] as u16});
                                    c.clear();
                                }
                        None => {}
                    }
                }
                GfxConsole::_release();
            }

            Ok(())
        }
        "fg" => {
            let mut args = [0; 3];
            let mut it = input_it.take(3).enumerate().peekable();
            if it.peek().is_none() {
                return Err(ShellError::ArgumentError);
            }

            while let Some((i, x)) = it.next() {
                args[i] = match x.starts_with("0x") {
                    true => usize::from_str_radix(&x[2..], 16).map_err(|_| ShellError::ParsingError)?,
                    false => x.parse().map_err(|_| ShellError::ParsingError)?,
                };
                
                if it.peek().is_none() && i != 2 {
                    return Err(ShellError::ArgumentError);
                }
            }
            
            let ks = KERNEL_STRUCT.load(Ordering::SeqCst);
            if ks != ptr::null_mut() {
                GfxConsole::_aquire();
                unsafe {
                    match &mut (*ks).console {
                        Some(c) => {
                                    c.set_font_color(Color { b: args[0] as u16, g: args[1] as u16, r: args[2] as u16});
                                    c.clear();
                                }
                        None => {}
                    }
                }
                GfxConsole::_release();
            }

            Ok(())
        }
        "wb" => {
            // Write to memory command
            let mut args = [0; 2];
            for (i, x) in input_it.take(2).enumerate() {
                args[i] = match x.starts_with("0x") {
                    true => usize::from_str_radix(&x[2..], 16).map_err(|_| ShellError::ParsingError)?,
                    false => x.parse().map_err(|_| ShellError::ParsingError)?,
                };
            }

            write_mem64(args[0], args[1].try_into().unwrap())?;
            Ok(())
        }
        "clear" => {
            let ks = KERNEL_STRUCT.load(Ordering::SeqCst);
            if ks != ptr::null_mut() {
                //GfxConsole::_aquire();
                unsafe {
                    match &mut (*ks).console {
                        Some(c) => {c.clear()}
                        None => {}
                    }
                }
                //GfxConsole::_release();
                Ok(())
            } else { panic!("Kernel struct is null!"); }
        }
        "mmu" => {
            println!("{}", arch::host::platform::get_mmu_state());
            Ok(())
        }
        "fsup" => {
            let ks = KERNEL_STRUCT.load(Ordering::SeqCst);
            if ks != ptr::null_mut() {
                GfxConsole::_aquire();
                unsafe {
                    match &mut (*ks).console {
                        Some(c) => {c.set_font_scale(c._get_font_scale()+1)}
                        None => {}
                    }
                }
                GfxConsole::_release();
            }
                Ok(())
        }
        "fsdown" => {
            let ks = KERNEL_STRUCT.load(Ordering::SeqCst);
            if ks != ptr::null_mut() {
                GfxConsole::_aquire();
                unsafe {
                    match &mut (*ks).console {
                        Some(c) => {c.set_font_scale(c._get_font_scale()-1)}
                        None => {}
                    }
                }
                GfxConsole::_release();
            }
                Ok(())
        }
        "uname" => {
            let mut ustr = Utsname::default();
            unsafe {
                asm!("svc 0x0",
                    in("x0") &mut ustr)
            }
            print!("{} ", ustr.sysname.as_str().trim_matches('\x00'));
            print!("{} ", ustr.nodename.as_str().trim_matches('\x00'));
            print!("{} ", ustr.release.as_str().trim_matches('\x00'));
            print!("{} ", ustr.version.as_str().trim_matches('\x00'));
            println!("{}", ustr.machine.as_str().trim_matches('\x00'));
            Ok(())
        }
        "svc" => {
            #[cfg(target_arch = "aarch64")]
            unsafe {asm!("svc 0x5") }
            Ok(()) 
        }
        "dbg" => {
            #[cfg(target_arch = "aarch64")]
            unsafe {asm!("svc 0xdb9") }
            Ok(()) 
        }
        "drop" => {
            #[cfg(target_arch = "aarch64")]      
            arch::host::platform::drop_to_el0(
                unsafe {&_user_start} as *const u8 as *mut u8,
                unsafe {&_user_stack} as *const u8 as *mut u8);
            Ok(())
        }
        "hd" => {
            let mut args = [0; 2];
            let mut it = input_it.take(2).enumerate().peekable();
            if it.peek().is_none() {
                return Err(ShellError::ArgumentError);
            }

            while let Some((i, x)) = it.next() {
                args[i] = match x.starts_with("0x") {
                    true => usize::from_str_radix(&x[2..], 16).map_err(|_| ShellError::ParsingError)?,
                    false => x.parse().map_err(|_| ShellError::ParsingError)?,
                };
                
                if it.peek().is_none() && i != 1 {
                    return Err(ShellError::ArgumentError);
                }
            }

                hex_print32(args[0] as *mut u8, args[1]);
                Ok(())
        }

        "shutdown" =>  {
            print!("Shutting down the system");
            for _ in 0..3 {
                print!(".");
                delay_s(1);
            }
            println!();
            const SYSTEM_OFF: u32 = 0x8400_0008;
            unsafe {
                asm!("hvc 0",
                in("x0") SYSTEM_OFF,
                options(nostack, nomem, noreturn));
            }
        }
        "exit" => Err(ShellError::UserExit),
        _ => Err(ShellError::UnknownCommand),
    }
}

// Main shell function
pub fn sh_main() {
    // Buffer to store user input
    let mut input_cmd: [ascii::Char; 128] = [ascii::Char::Null; 128];
    let mut input_index = 0;
    let mut string_size = 0;

    print!("> ");

    loop {
        if let Some(c) = arch::host::driver::serial::serial_getchar() {
            if let Some(ca) = ascii::Char::from_u8(c) {
                if !c.is_ascii_control() {
                    print!("{}", ca);
                    input_cmd[input_index] = ca;
                    if input_index == string_size {
                        string_size += 1;
                    }
                    input_index += 1;
                }

                match c {
                    b'\r' => {
                        println!();
                        // Process the user command
                        input_cmd[string_size] = ascii::Char::from_u8(b'\n').unwrap();
                        match process_command(&mut input_cmd) {
                            Ok(()) => {},
                            Err(ShellError::UserExit) => { panic!("User exited the shell!"); },
                            Err(err) => { println!("{:?}: {:?}", input_cmd.as_str().lines().next().unwrap_or(""), err); }
                        };

                        // Reset the input buffer
                        input_cmd.iter_mut().for_each(|x| *x = ascii::Char::Null);
                        input_index = 0;
                        string_size = 0;

                        print!("> ");
                    }
                    0x7f => {
                        // Handle backspace
                        if input_index != 0 {
                            if input_index == string_size {
                                input_index -= 1;
                                string_size -= 1;
                                input_cmd[input_index] = ascii::Char::Null;
                            } else {
                                input_index -= 1;
                                input_cmd[input_index] = ascii::Char::Space;
                            }
                            print!("\x08\x20\x08");
                        }
                    }
                    0x1b => {
                        // Handle escape sequences
                        if let Some(d) = arch::host::driver::serial::serial_getchar() {
                            if d == b'[' {
                                if let Some(f) = arch::host::driver::serial::serial_getchar() {
                                    match f {
                                        b'A' => {} // Up arrow
                                        b'B' => {} // Down arrow
                                        b'C' => {
                                            // Right arrow
                                            if input_index < string_size {
                                                print!("\x1b[C");
                                                input_index += 1;
                                            }
                                        }
                                        b'D' => {
                                            // Left arrow
                                            if input_index != 0 {
                                                print!("\x1b[D");
                                                input_index -= 1;
                                            }
                                        }
                                        _ => {
                                            println!("Unknown escape sequence");
                                        }
                                    }
                                }
                            }
                        }
                    }
                   0x03 => {
                        println!("^C");
                        
                        // Reset the input buffer
                        input_cmd.iter_mut().for_each(|x| *x = ascii::Char::Null);
                        input_index = 0;
                        string_size = 0;
                        print!("> ");
                   }

                   0x0C => {
                    print!("\r\x0c");
                    let ks = KERNEL_STRUCT.load(Ordering::SeqCst);
                    if ks != ptr::null_mut() {
                        //GfxConsole::_aquire();
                        unsafe {
                            match &mut (*ks).console {
                                Some(c) => {c.clear()}
                                None => {}
                            }
                        }
                        //GfxConsole::_release();
                    } else { panic!("Kernel struct is null!"); }
                    
                    // Reset the input buffer
                    input_cmd.iter_mut().for_each(|x| *x = ascii::Char::Null);
                    input_index = 0;
                    string_size = 0;
                    print!("> ");
               } 
                    _ => {}
                }
            }
        }
    }
}
