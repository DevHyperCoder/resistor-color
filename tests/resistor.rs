use std::str::FromStr;

use resistor_color::{band_color::BandColor, resistor::Resistor};

#[test]
fn test_resistor_from_str() {
    let c = "brown brown red";
    let resistor = Resistor::from_str(c).unwrap();
    let bands = resistor.get_bands();

    assert_eq!(bands[0], BandColor::Brown);
    assert_eq!(bands[1], BandColor::Brown);
    assert_eq!(bands[2], BandColor::Red);
}

#[test]
fn test_resistor_value() {
    str_to_val("brown brown red", 1100);

    str_to_val("green yellow orange black", 54000);

    str_to_val("brown black orange", 10000);

    str_to_val("yellow blue green", 4600000);

    str_to_val("red green brown", 250);

    str_to_val("brown red green red white", 12500);

    str_to_val("black black black black black black", 0);
}

fn str_to_val(c: &str, v: u32) {
    let resistor = Resistor::from_str(c).unwrap();
    assert_eq!(resistor.get_value(), v);
}
