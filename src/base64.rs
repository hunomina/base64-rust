const NBR_BITS_TO_READ: u8 = 6;
const PADDING_CHAR: char = '=';
const BASE_64_ALPHABET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

pub fn encode(s: String) -> String {
    let bytes = s.as_bytes();
    let number_of_byte_groups = bytes.len() / 3 + if bytes.len() % 3 == 0 { 0 } else { 1 };

    let mut encoded_string = (0..number_of_byte_groups)
        .flat_map(|n| {
            let (first_char, second_char, third_char) =
                (*bytes.get(n).unwrap(), bytes.get(n + 1), bytes.get(n + 2));

            let mut encoded_chars = vec![];

            let first_encoded_char = get_n_first_bit(first_char, NBR_BITS_TO_READ);
            encoded_chars.push(first_encoded_char);

            let second_encoded_char = get_n_last_bits(first_char, 2) << 4
                | get_n_first_bit(*second_char.unwrap_or(&0), 4);
            encoded_chars.push(second_encoded_char);

            if second_char.is_none() {
                return encoded_chars;
            }

            let third_encoded_char = get_n_last_bits(*second_char.unwrap(), 4) << 2
                | get_n_first_bit(*third_char.unwrap_or(&0), 2);
            encoded_chars.push(third_encoded_char);

            if third_char.is_none() {
                return encoded_chars;
            }

            let fourth_encoded_char = get_n_last_bits(*third_char.unwrap(), NBR_BITS_TO_READ);
            encoded_chars.push(fourth_encoded_char);

            encoded_chars
        })
        .map(|c| BASE_64_ALPHABET.chars().nth(c as usize).unwrap())
        .collect::<String>();

    match encoded_string.len() % 4 {
        2 => {
            encoded_string.push(PADDING_CHAR);
            encoded_string.push(PADDING_CHAR);
        }
        3 => {
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
            bytes.next().unwrap(),
            bytes.next().unwrap(),
            bytes.next().unwrap(),
            bytes.next().unwrap(),
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
        number >> (8 - n)
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
