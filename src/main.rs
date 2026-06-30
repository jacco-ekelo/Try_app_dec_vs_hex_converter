// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;
use num_traits::pow;
use slint::ToSharedString;

slint::include_modules!();

// Conversion decimal to hexadecimal
fn dec2hex(mut decimal: u32) -> String {
    // base 0
    if decimal == 0 {
        return "#0".to_string();
    }

    let hex_chars = b"0123456789ABCDEF";

    let mut hex = String::new();

    while decimal > 0 {
        let temp = decimal % 16;

        let ch = hex_chars[temp as usize] as char;

        hex.push(ch);

        decimal /= 16;

    }

    // presuffix # (presuffic to add in reversed order, as the hex outcome will be
    //  reversed at the end
    hex.push('#');

    hex.chars().rev().collect()
}

// Conversion hexadecimal to decimal
fn hex2dec(hexadecimal: String) -> String {

    let mut dec_num = 0;
    let hexadecimal = hexadecimal.chars().rev().collect::<String>();

    for (i, c) in hexadecimal.chars().enumerate() {
        let value: u32 = match c {
            '0'..='9' => c as u32 - '0' as u32,      // 0 t/m 9
            'A'..='F' => c as u32 - 'A' as u32 + 10, // 10 t/m 15
            'a'..='f' => c as u32 - 'a' as u32 + 10, // non cap letters
            _ => 0, // Fallback for invalid characters, although form input is tested
        };

        dec_num += value * pow(16, i);
    }

    return dec_num.to_string();
}

// Callback handle for conversion, triggered by clicking the convert button
fn handle_convert(ui: &AppWindow) {

    let dec = ui.get_dec();
    let hexa = ui.get_hexa();

    // error message if both fields are empty
    if dec == "" && hexa == "" {
        ui.set_err_msg("Submit a decimal or hexadecimal value".into());
        return;
    }

    // error message if both fields are not empty
    if dec != "" && hexa != "" {
        ui.set_err_msg("Only submit decimal or hexadecimal".into());
        return;
    }

    // only one field filled, either dec or hexa
    if dec != "" {
        // validate decimal input
        if validate_decimal_input(&*dec) == false {
            ui.set_err_msg("Decimal input invalid".into());
            return;
        }

        // pass decimal string into
        let hex_from_dec = dec2hex(dec.parse::<u32>().unwrap());
        ui.set_hexa(hex_from_dec.into());

        // reset error message
        ui.set_err_msg("".into());

        return;
    }

    if hexa != "" {
        // validate hexadecimal input
        if validate_hexadecimal_input(&*hexa) == false {
            ui.set_err_msg("Hexadecimal input invalid".into());
            return;
        }

        // pass decimal string into
        let dec_from_hex = hex2dec(hexa.parse::<String>().unwrap());
        ui.set_dec(dec_from_hex.into());

        // reset error message
        ui.set_err_msg("".into());

        return;
    }

}

// Clear input
fn handle_clear(ui: &AppWindow) {
    ui.set_hexa("".to_shared_string());
    ui.set_dec("".to_shared_string());
    ui.set_err_msg("".into());
}

// Validate input
fn validate_decimal_input(s: &str)-> bool {
    s.chars().all(|c| matches!(c, '0'..='9'))
}

fn validate_hexadecimal_input(s: &str) -> bool {
    s.chars().all(|c| {
        matches!(c,
            '0'..='9' |
            'a'..='f' |
            'A'..='F'
        )
    })
}

// Start
fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;

    let weak_convert = ui.as_weak();
    let weak_clear = ui.as_weak();

    ui.on_convert(move || {
        let ui = weak_convert.unwrap();
        
        handle_convert(&ui);
    });

    ui.on_clear(move || {
        let ui = weak_clear.unwrap();

        handle_clear(&ui);
    });

    ui.run()?;
    Ok(())
}
