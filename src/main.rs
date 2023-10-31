mod run_length_encoding {
    pub fn encode(text: &str) -> String {
        let mut res: String = "".to_string();
        let mut num_occs: u8 = 0;
        
        let mut text_iter = text.chars();
        let mut next_opt = text_iter.next();
        let mut prev_char: char = next_opt.unwrap_or_default();

        while  next_opt.is_some() {
            let curr_char = next_opt.unwrap();    
            next_opt = text_iter.next();

            if  curr_char == prev_char && num_occs != 9 {
                num_occs += 1;
                prev_char =  curr_char;
            } else {
                res = res + &num_occs.to_string() + &prev_char.to_string();
                num_occs = 1;
                prev_char = curr_char; 
            }

            if next_opt.is_none() {
                res = res + &num_occs.to_string() + &prev_char.to_string();
            }
        }
        res
    }

    pub fn decode(text: &str) -> String {
        let mut res: String = "".to_string();
        let mut text_iter = text.chars();
        let mut num_occs_iter = text_iter.next();
        let mut char_iter = text_iter.next();

        while  char_iter.is_some() {
            let num_occs = num_occs_iter.unwrap();
            let char = char_iter.unwrap();
            let size = num_occs.to_digit(10).unwrap() as usize;

            res += &char.to_string().repeat(size);

            num_occs_iter = text_iter.next();
            char_iter = text_iter.next();
        }
        res
    }
}

fn main() {
    use run_length_encoding::*;
    //println!("{}", encode("abc"));

    let input = encode("AAAAA AAAAAAAAAA AAAAAAAAAAAAAAAAAAAA");
    println!("{}", decode(&input));

    let input = encode("LinkedIn");
    println!("{}", decode(&input));


}

#[test]
fn abc() {
    use run_length_encoding::*;

    assert_eq!(encode("abc"), "1a1b1c");
}

#[test]
fn round_trip() {
    use run_length_encoding::*;

    let input = "LinkedIn";
    println!("{}", encode(input));
    assert_eq!(decode(&encode(input)), input);
}

#[test]
fn long_run() {
    use run_length_encoding::*;

    let input = "AAAAA AAAAAAAAAA AAAAAAAAAAAAAAAAAAAA";
    assert_eq!(encode(input), "5A1 9A1A1 9A9A2A");
}
