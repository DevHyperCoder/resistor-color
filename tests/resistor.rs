use std::str::FromStr;

use resistor_color::{BandColor, Resistor};

#[test]
fn test_band_color_from_str() {
    assert_eq!(BandColor::from_str("black").unwrap(), BandColor::Black);
    assert_eq!(BandColor::from_str("brown").unwrap(), BandColor::Brown);
    assert_eq!(BandColor::from_str("red").unwrap(), BandColor::Red);
    assert_eq!(BandColor::from_str("orange").unwrap(), BandColor::Orange);
    assert_eq!(BandColor::from_str("yellow").unwrap(), BandColor::Yellow);
    assert_eq!(BandColor::from_str("green").unwrap(), BandColor::Green);
    assert_eq!(BandColor::from_str("blue").unwrap(), BandColor::Blue);
    assert_eq!(BandColor::from_str("violet").unwrap(), BandColor::Violet);
    assert_eq!(BandColor::from_str("grey").unwrap(), BandColor::Grey);
    assert_eq!(BandColor::from_str("white").unwrap(), BandColor::White);
}

#[test]
fn test_invalid_band_color() {
    assert!(BandColor::from_str("asdf").is_err());
    assert!(BandColor::from_str("bluee").is_err());
    assert!(BandColor::from_str("redder").is_err());
}

#[test]
fn test_brand_color_from_str_all_case() {
    assert_eq!(BandColor::from_str("BLACK").unwrap(), BandColor::Black);
    assert_eq!(BandColor::from_str("BlacK").unwrap(), BandColor::Black);
    assert_eq!(BandColor::from_str("black").unwrap(), BandColor::Black);
    assert_eq!(BandColor::from_str("BlAcK").unwrap(), BandColor::Black);
}

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
