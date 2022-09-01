extern crate dynamic_midi;
extern crate pancurses;

use dual_shock4_controller::joystick::{DeviceInfo, Joystick};
use dynamic_midi::main::MidiDevice;
use pancurses::{endwin, initscr, noecho, Input};
use std::thread;

fn main() {
    let window = initscr();
    window.printw("Type things, press delete to quit\n");
    window.refresh();
    window.keypad(true);
    noecho();

    let mut device: Option<MidiDevice> = None;
    loop {
        match window.getch() {
            Some(Input::Character(c)) => match c {
                '1' => {
                    window.addstr("You press 1");
                }
                '2' => {
                    window.addstr("You press 2");
                }
                'o' => {
                    let name = "test";
                    device = Some(dynamic_midi::main::open_port(name));
                    window.addstr(format!("Port {} is open\n", name));
                }
                'c' => match device {
                    Some(ref port) => {
                        port.close();
                        window.addstr(format!("Port is closed\n"));
                        device = None;
                    }
                    None => {
                        window.addstr(format!("Port was not open\n"));
                    }
                },
                'q' => {
                    return;
                }
                _ => (),
            },
            Some(Input::KeyDC) => break,
            Some(input) => {
                window.addstr(&format!("{:?}", input));
            }
            None => (),
        }
    }
    endwin();
}

/*

'v' => {
                    let ver = midi_api::midi_api::get_client_version();
                    window.addstr(format!("Client version: {}\n", ver));
                }
                'd' => {
                    let ver = midi_api::midi_api::get_driver_version();
                    window.addstr(format!("Driver version: {}\n", ver));
                }

                'p' => {
                #https://rendered-obsolete.github.io/2018/09/17/gamepad-rust.html
                    thread::spawn(|| {
                        let joystick = Joystick::new();
                        let device_info = DeviceInfo {
                            vid: 0x054c,
                            pid: 0x09CC,
                        }; //HID\VID_054C&PID_0x09CC\7&3869AC07&0&0000
                        let device = joystick.connect(device_info).expect("can't find device!"); //
                        loop {
                            let mut buf = [0u8; 64];
                            device.read_timeout(&mut buf[..], 1000).unwrap();
                            let gamepad = joystick.get_gamepad().get_state(&buf);

                            if gamepad.square_button.pressed {
                                midi_api::midi_api::play_note(port, 153, 35, 64);
                            }
                            if gamepad.x_button.pressed {
                                midi_api::midi_api::play_note(port, 137, 35, 0);
                            }
                            if gamepad.share_button.pressed {
                                break;
                            }
                        }
                    });
                    window.addstr("Lets play!");
                }
                's' => {
                    midi_api::midi_api::play_note(port, 137, 35, 0);
                }
                'c' => {
                    midi_api::midi_api::close_port(port);
                    window.addstr(format!("Port is closed\n"));
                }


*/
