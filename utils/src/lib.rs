pub fn decimal_to_binary(x: u32) -> String {
    // let mut binary = String::new();

    // while x != 0 {
    //     if x % 2 == 0 {
    //         binary.push_str("0");
    //     } else {
    //         binary.push_str("1");
    //     }

    //     x /= 2;
    // }

    // println!("{}", binary);
    // binary

    // so apparently
    // format!("{x:b}")
    // exists
    format!("{x:b}")
    // GREAT
}

pub fn decimal_to_hex(x: u32) -> String {
    let hex = format!("{x:x}");
    zero_pad(&hex[..], 8)
}
pub fn zero_pad(text: &str, zeroes: usize) -> String {
    // Pardon my shitty code
    let mut result = String::new();
    if text.len() >= zeroes {
        return String::from(text);
    }
    
    let difference = zeroes - text.len();

    for _ in 0..difference {
        result.push_str("0");
    }
    
    result.push_str(text);

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn decimal_to_binary_works() {
        assert_eq!("100100110", decimal_to_binary(294));
    }

    #[test]
    fn decimal_to_hex_works() {
        assert_eq!("00000123", decimal_to_hex(291));
    }

    #[test]
    fn zero_padding_works() {
        assert_eq!("0a", zero_pad("a", 2));
    }
}
