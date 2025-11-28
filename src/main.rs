use std::{env, path::Path, process};

use dotenv::dotenv;
use evdev::{Device, EventType, KeyCode};
use reqwest::StatusCode;

const MAX_BARCODE_LENGTH: usize = 64;

#[tokio::main]
async fn main() {
    dotenv().ok();

    // Get SCANNER_DEVICE_PATH environment variable
    let scanner_device_path = match env::var("SCANNER_DEVICE_PATH") {
        Ok(scanner_device_path) => scanner_device_path,
        Err(..) => {
            eprintln!("❌ Missing required environment variable 'SCANNER_DEVICE_PATH'");
            process::exit(1);
        }
    };

    // Open scanner device
    println!("ℹ️ Attempting to open scanner device: {}", scanner_device_path);
    let mut scanner_device = match Device::open(Path::new(&scanner_device_path)) {
        Ok(scanner_device) => scanner_device,
        Err(err) => {
            eprintln!("❌ Error opening the scanner: {}", err);
            process::exit(1);
        }
    };
    println!("✅ Scanner opened successfully");

    // Grab scanner device (prevents it from being used with other programs)
    println!("ℹ️ Attempting grab scanner device: {}", scanner_device_path);
    match scanner_device.grab() {
        Ok(_) => (),
        Err(err) => {
            eprintln!("❌ Error grabbing the scanner: {}", err);
            process::exit(1);
        }
    }
    println!("✅ Scanner grabbed successfully.");

    // Everything is ready to roll!
    println!("✅ All ready!  Listening for scanner input...");

    // Working barcode variable to store keys as they are entered by the scanner
    let mut current_barcode = String::with_capacity(MAX_BARCODE_LENGTH);

    // Infinite loop listening for input events from the scanner
    loop {
        // Read events from scanner device.
        let scanner_device_events = match scanner_device.fetch_events() {
            Ok(events) => events,
            Err(err) => {
                eprintln!("❌ Error reading events from scanner device: {}", err);
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
                handle_scan(current_barcode.clone()).await;
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

async fn handle_scan(barcode_value: String) {
    // Log scanned barcode to console
    println!("ℹ️ Scanned barcode: {}", barcode_value);

    // Get HTTP_SERVER_ADDRESS environment variable
    let http_server_address = match env::var("HTTP_SERVER_ADDRESS") {
        Ok(http_server_address) => http_server_address,
        Err(..) => {
            eprintln!("❌ Missing required environment variable 'HTTP_SERVER_ADDRESS'");
            process::exit(1);
        }
    };

    // Send barcode to HTTP server
    let response = reqwest::get(format!("{}?barcode={}", http_server_address, barcode_value)).await;

    // Check if request was successful
    match response {
        Ok(response) => {
            if response.status() == StatusCode::OK {
                println!("✅ Barcode successfully sent to HTTP server");
            } else {
                eprintln!("❌ An error occurred while sending the barcode to the HTTP server: {:?}", response);
                return;
            }
        },
        Err(err) => {
            eprintln!("❌ An error occurred while sending the barcode to the HTTP server: {}", err);
            return;
        }
    }
}
