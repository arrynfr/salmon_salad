use crate::{arch, print};
use core::{ascii, ptr::write_volatile};

const HELP_TEXT: &str = "arsh shell
copyright 2023 arrynfr

Available builtin commands:
cd          help
exit";

// Custom error type for the shell
#[derive(Debug)]
enum ShellError {
    UnknownCommand,
    ParsingError,
    UserExit,
}

// Function to write a value to a memory address
fn write_mem(offset: usize, value: usize) -> Result<(), ShellError> {
    println!("Writing {} to {:?}", value, offset);
    unsafe { write_volatile(offset as *mut usize, value) }
    Ok(())
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
        "wb" => {
            // Write to memory command
            let mut args = [0; 2];
            for (i, x) in input_it.take(2).enumerate() {
                args[i] = match x.starts_with("0x") {
                    true => usize::from_str_radix(&x[2..], 16).map_err(|_| ShellError::ParsingError)?,
                    false => x.parse().map_err(|_| ShellError::ParsingError)?,
                };
            }

            write_mem(args[0], args[1])?;
            Ok(())
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
        if let Some(c) = arch::host::serial::serial_getchar() {
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
                        input_cmd[string_size] = ascii::Char::from_u8('\n' as u8).unwrap();
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
                        if let Some(d) = arch::host::serial::serial_getchar() {
                            if d == b'[' {
                                if let Some(f) = arch::host::serial::serial_getchar() {
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
                    _ => {}
                }
            }
        }
    }
}
