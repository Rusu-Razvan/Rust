fn convert_temperature(temp: i32, from_scale: char, to_scale: char) -> Result<f64, &'static str> {
    match (from_scale, to_scale) {
        ('C', 'F') => {
            if temp < -273 {
                Err("Invalid temperature. Below absolute zero!")
            } else {
                Ok(celsius_to_fahrenheit(temp))
            }
        }
        ('C', 'K') => {
            if temp < -273 {
                Err("Invalid temperature. Below absolute zero!")
            } else {
                Ok(celsius_to_kelvin(temp))
            }
        }
        ('F', 'C') => {
            if temp < -459 {
                Err("Invalid temperature. Below absolute zero!")
            } else {
                Ok(fahrenheit_to_celsius(temp))
            }
        }
        ('F', 'K') => {
            if temp < -459 {
                Err("Invalid temperature. Below absolute zero!")
            } else {
                Ok(fahrenheit_to_kelvin(temp))
            }
        }
        ('K', 'C') => {
            if temp < 0 {
                Err("Invalid temperature. Below absolute zero!")
            } else {
                Ok(kelvin_to_celsius(temp))
            }
        }
        ('K', 'F') => {
            if temp < 0 {
                Err("Invalid temperature. Below absolute zero!")
            } else {
                Ok(kelvin_to_fahrenheit(temp))
            }
        }
        _ => Err("Invalid scale. Use 'C' for Celsius, 'F' for Fahrenheit, or 'K' for Kelvin."),
    }
}

fn celsius_to_fahrenheit(celsius: i32) -> f64 {
    (celsius as f64 * 9.0 / 5.0) + 32.0
}

fn fahrenheit_to_celsius(fahrenheit: i32) -> f64 {
    (fahrenheit as f64 - 32.0) * 5.0 / 9.0
}

fn celsius_to_kelvin(celsius: i32) -> f64 {
    (celsius as f64) + 273.15
}

fn kelvin_to_celsius(kelvin: i32) -> f64 {
    (kelvin as f64) - 273.15
}

fn fahrenheit_to_kelvin(fahrenheit: i32) -> f64 {
    celsius_to_kelvin(fahrenheit_to_celsius(fahrenheit) as i32)
}

fn kelvin_to_fahrenheit(kelvin: i32) -> f64 {
    celsius_to_fahrenheit(kelvin_to_celsius(kelvin) as i32)
}

fn display_conversion(
    result: Result<f64, &'static str>,
    temp: i32,
    from_scale: char,
    to_scale: char,
) {
    match result {
        Ok(converted_temp) => println!("{}{} = {:.2}{}", temp, from_scale, converted_temp, to_scale),
        Err(err) => println!("Error: {}", err),
    }
}

fn main() {
    let temp1 = 20;
    let temp2 = 70;
    let kelvin_temp = 290;
    let invalid_temp = -300;

    let c_to_f_result = convert_temperature(temp1, 'C', 'F');
    display_conversion(c_to_f_result, temp1, 'C', 'F');

    let f_to_c_result = convert_temperature(temp2, 'F', 'C');
    display_conversion(f_to_c_result, temp2, 'F', 'C');

    let c_to_k_result = convert_temperature(temp1, 'C', 'K');
    display_conversion(c_to_k_result, temp1, 'C', 'K');

    let k_to_c_result = convert_temperature(kelvin_temp, 'K', 'C');
    display_conversion(k_to_c_result, kelvin_temp, 'K', 'C');

    let f_to_k_result = convert_temperature(temp2, 'F', 'K');
    display_conversion(f_to_k_result, temp2, 'F', 'K');

    let k_to_f_result = convert_temperature(kelvin_temp, 'K', 'F');
    display_conversion(k_to_f_result, kelvin_temp, 'K', 'F');

    let invalid_result = convert_temperature(invalid_temp, 'C', 'F');
    display_conversion(invalid_result, invalid_temp, 'C', 'F');
}
