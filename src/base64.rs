const NBR_BITS_TO_READ: u8 = 6;
const SIZE_OF_CHAR: u8 = 8;
const PADDING_CHAR: char = '=';
const BASE_64_ALPHABET: &str = &"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

pub fn encode(s: String) -> String {
    let bytes = s.as_bytes();

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
            BASE_64_ALPHABET.chars().nth(encoded as usize).unwrap()
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

pub fn decode(s: String) -> String {
    let mut decoded_s = String::new();
    let mut bytes = s.bytes();
    while bytes.len() > 0 {
        // The number of characters of a Base64 encoded string is always divisable by four.
        // Thus we can process four characters of the string in one step to retrieve three decoded bytes
        let (first_char, second_char, third_char, fourth_char) = (
            bytes.nth(0).unwrap(),
            bytes.nth(0).unwrap(),
            bytes.nth(0).unwrap(),
            bytes.nth(0).unwrap(),
        );

        // look up for the corresponding chars in the alphabet / reverse lookup
        // unwrap_or here allows to replace the padding char, that can't be found in the alphabet, by the NUL char
        let (first_char, second_char, third_char, fourth_char) = (
            BASE_64_ALPHABET.find(first_char as char).unwrap() as u8,
            BASE_64_ALPHABET.find(second_char as char).unwrap() as u8,
            BASE_64_ALPHABET.find(third_char as char).unwrap_or(0) as u8,
            BASE_64_ALPHABET.find(fourth_char as char).unwrap_or(0) as u8,
        );

        // merge the four encoded chars into 3 decoded chars
        // reminder: base64 chars are encoded on 6 bits
        let (first_char, second_char, third_char) = (
            first_char << 2 | get_n_first_bit(second_char << 2, 2),
            get_n_last_bits(second_char, 4) << 4 | get_n_first_bit(third_char << 2, 4),
            get_n_last_bits(third_char, 2) << 6 | fourth_char,
        );

        // add them to the decoded string
        decoded_s.push_str(&format!(
            "{}{}{}",
            first_char as char, second_char as char, third_char as char
        ));
    }

    decoded_s.trim_end_matches(0 as char).into()
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

    #[test]
    fn b64_decode() {
        test_cases()
            .into_iter()
            .for_each(|(original, encoded)| assert_eq!(original, decode(encoded.into())))
    }

    fn test_cases<'a>() -> HashMap<&'a str, &'a str> {
        HashMap::from([("S", "Uw=="), ("Su", "U3U="), ("Sun", "U3Vu")])
    }
}
