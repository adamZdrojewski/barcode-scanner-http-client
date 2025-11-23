use std::{path::Path, process};

use evdev::{Device, EventType, KeyCode};

const SCANNER_DEVICE_PATH: &str = "/dev/input/by-id/usb-ﾩSymbol_Technologies__Inc__2002_Symbol_Bar_Code_Scanner_S_N:E341D2CD621FA445B3463D034B66084B_Rev:NBRMIAAU3-event-kbd";
const MAX_BARCODE_LENGTH: usize = 64;

fn main() {
    // Open scanner device
    println!("ℹ️ Attempting to open scanner device: {}", SCANNER_DEVICE_PATH);
    let mut scanner_device = match Device::open(Path::new(SCANNER_DEVICE_PATH)) {
        Ok(scanner_device) => scanner_device,
        Err(err) => {
            eprint!("❌ Error opening the scanner: {}", err);
            process::exit(1);
        }
    };
    println!("✅ Scanner opened successfully");

    // Grab scanner device (prevents it from being used with other programs)
    println!("ℹ️ Attempting grab scanner device: {}", SCANNER_DEVICE_PATH);
    match scanner_device.grab() {
        Ok(_) => (),
        Err(err) => {
            eprint!("❌ Error grabbing the scanner: {}", err);
            process::exit(1);
        }
    }
    println!("✅ Scanner grabbed successfully.");

    // Working barcode variable to store keys as they are entered by the scanner
    let mut current_barcode = String::with_capacity(MAX_BARCODE_LENGTH);

    // Infinite loop listening for input events from the scanner
    loop {
        // Read events from scanner device.
        let scanner_device_events = match scanner_device.fetch_events() {
            Ok(events) => events,
            Err(err) => {
                eprint!("❌ Error reading events from scanner device: {}", err);
                process::exit(1);
            }
        };

        // Iterate through scanner events
        for event in scanner_device_events {
            // Check if event type isn't a key event
            if event.event_type() != EventType::KEY {
                // Not a key event - end iteration
                continue;
            }

            // Check if event is a keydown event (type 1) (0 is keyup and 2 is auto-repeat)
            if event.value() != 1 {
                // Not a keydown event - end iteration
                continue;
            }

            // Check if enter key was pressed
            if event.code() == KeyCode::KEY_ENTER.0 {
                // Enter key was pressed - handle scan event and reset current_barcode variable
                handle_scan(current_barcode.clone());
                current_barcode.clear();
            } else {
                // Not enter key - get char from keycode and add it to the current_barcode variable
                match keycode_to_char(event.code()) {
                    Some(char) => current_barcode.push(char),
                    None => println!("❌ Could not find char for code: {}", event.code())
                }
            }
        }
    }
}

fn keycode_to_char(code: u16) -> Option<char> {
    if code == KeyCode::KEY_0.0 {
        Some('0')
    } else if code == KeyCode::KEY_1.0 {
        Some('1')
    } else if code == KeyCode::KEY_2.0 {
        Some('2')
    } else if code == KeyCode::KEY_3.0 {
        Some('3')
    } else if code == KeyCode::KEY_4.0 {
        Some('4')
    } else if code == KeyCode::KEY_5.0 {
        Some('5')
    } else if code == KeyCode::KEY_6.0 {
        Some('6')
    } else if code == KeyCode::KEY_7.0 {
        Some('7')
    } else if code == KeyCode::KEY_8.0 {
        Some('8')
    } else if code == KeyCode::KEY_9.0 {
        Some('9')
    } else {
        return None
    }
}

fn handle_scan(barcode_value: String) {
    println!("Handeling barcode... {}", barcode_value);
}
