fn main() {
    println!("{}", encode("S".into()));
    println!("{}", encode("Su".into()));
    println!("{}", encode("Sun".into()));
}

const NBR_BITS_TO_READ: u8 = 6;
const SIZE_OF_CHAR: u8 = 8;
const PADDING_CHAR: char = '=';
const BASE_64_ALPHABET: &[char] = &[
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l',
    'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4',
    '5', '6', '7', '8', '9', '+', '/',
];

fn encode(s: String) -> String {
    let bytes = s.as_bytes();
    println!("{:?}", s);
    println!("{:?}", bytes);

    let nbr_of_encoded_parts = (s.len() as f32 * 8.0 / NBR_BITS_TO_READ as f32).ceil() as usize;
    let mut c = 0;
    let mut encoded_string: String = (0..nbr_of_encoded_parts)
        .map(|n| {
            let current_char = n * NBR_BITS_TO_READ as usize / SIZE_OF_CHAR as usize;
            // index of the bit from the current character where we start to read from
            // is always a multiple of two, since we read 6 bits during each iteration
            let nth_bit = c % SIZE_OF_CHAR;

            let encoded = match nth_bit {
                0 => get_n_first_bit(bytes[current_char], NBR_BITS_TO_READ),
                2 => get_n_last_bits(bytes[current_char], NBR_BITS_TO_READ),
                _ => {
                    // in this situation, nth_bit could either be 4 or 6
                    // so we need to concatanate two groups of bits

                    // 8 - nth_bit: number of bits read from the previous character
                    let nbr_of_bits_to_read_from_next_char =
                        NBR_BITS_TO_READ - (SIZE_OF_CHAR - nth_bit);
                    let bits_from_current_char =
                        get_n_last_bits(bytes[current_char], SIZE_OF_CHAR - nth_bit);
                    let bits_from_next_char = get_n_first_bit(
                        *bytes.get(current_char + 1).unwrap_or(&0u8),
                        nbr_of_bits_to_read_from_next_char,
                    );
                    bits_from_current_char << nbr_of_bits_to_read_from_next_char
                        | bits_from_next_char
                }
            };

            c += NBR_BITS_TO_READ;
            BASE_64_ALPHABET[encoded as usize]
        })
        .collect();

    match encoded_string.len() % 4 {
        3 => {
            encoded_string.push(PADDING_CHAR);
        }
        2 => {
            encoded_string.push(PADDING_CHAR);
            encoded_string.push(PADDING_CHAR);
        }
        _ => {}
    }

    encoded_string
}

fn get_n_first_bit(number: u8, n: u8) -> u8 {
    if n == 0 {
        0
    } else {
        number >> 8 - n
    }
}

fn get_n_last_bits(number: u8, n: u8) -> u8 {
    if n == 8 {
        number
    } else {
        number & ((1 << n) - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn b64_encode() {
        test_cases()
            .into_iter()
            .for_each(|(original, encoded)| assert_eq!(encoded, encode(original.into())))
    }

    fn test_cases<'a>() -> HashMap<&'a str, &'a str> {
        HashMap::from([("S", "Uw=="), ("Su", "U3U="), ("Sun", "U3Vu")])
    }
}
