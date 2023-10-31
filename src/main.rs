use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Rgb {
    red : u8,
    green : u8,
    blue : u8,
} // TODO: design data structure

trait RgbChannels {
    fn r(&self) -> u8;
    fn g(&self) -> u8;
    fn b(&self) -> u8;
}

impl RgbChannels for Rgb {
    fn r(&self) -> u8 { self.red }
    fn g(&self) -> u8 { self.green }
    fn b(&self) -> u8 { self.blue } 
}

impl FromStr for Rgb {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 7 {
            Err("Error: Invalid Hex Color".to_string())
        } else {
            match hex::decode(s.replace('#', "")) {
                Ok(rgb) => Ok(Rgb { red: rgb[0], green: rgb[1], blue: rgb[2] }),
                Err(_) => Err("Error: Error Decoding Hex Values".to_string()),
            }
        }
    }
}

impl Display for Rgb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{:02x}{:02x}{:02x}", self.r(), self.g(), self.b())
    }
}

fn main() {
    println!("{:?}", hex::decode("aabbcc"));
}

#[test]
fn every_color() {
    let colors = (0_u8..255).zip(0_u8..255).zip(0_u8..255);

    for ((r, g), b) in colors {
        let hex = format!("#{:02x}{:02x}{:02x}", r, g, b);
        println!("{}", hex);
        let color: Rgb = hex.parse().unwrap();
        assert_eq!(hex, format!("{}", color));
    }
}

#[test]
#[should_panic]
fn too_short () {
    let _: Rgb = "1234".parse().unwrap();
}

#[test]
#[should_panic]
fn not_a_hex_code () {
    let _: Rgb = "?".parse().unwrap();
}

#[test]
#[should_panic]
fn invalid_literals () {
    let _: Rgb = "?".parse().unwrap();
}

#[test]
#[should_panic]
fn no_leading_hash() {
    let _: Rgb = "aabbcc".parse().unwrap();
}

#[test]
#[should_panic]
fn out_of_bounds() {
    let _: Rgb = "00gg00".parse().unwrap();
}

