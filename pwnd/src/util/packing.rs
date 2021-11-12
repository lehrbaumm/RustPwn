pub enum ENDIANNESS {
    BIG,
    LITTLE
}

const SUPPORTED_WORD_SIZES: [u16; 4] = [8, 16, 32, 64];

pub fn pack(number: u128, word_size: u16, enidanness: ENDIANNESS) -> Vec<u8> {
    if !SUPPORTED_WORD_SIZES.contains(&word_size) {
        panic!("Unsupported word_size");
    }
    let mut result: Vec<u8> = Vec::new();
    let mut number = number & ((1 << word_size) - 1);
    let byte_size = (word_size + 7) / 8;

    for _ in 0..byte_size {
        result.push((number & 0xFF) as u8);
        number = number >> 8;
    }

    match enidanness {
        ENDIANNESS::LITTLE => result,
        ENDIANNESS::BIG => {
            result.reverse();
            result
        }
    }
}

pub fn unpack(data: Vec<u8>, word_size: u16, enidanness: ENDIANNESS) -> u128 {
    if !SUPPORTED_WORD_SIZES.contains(&word_size) {
        panic!("Unsupported word_size");
    }
    let mut result: u128 = 0;
    let mut data = Vec::from(data);
    let byte_size = (word_size + 7) / 8;

    if byte_size != data.len() as u16 {
        panic!("Data must have length {}, since word size was {}", byte_size, word_size);
    }

    match enidanness {
        ENDIANNESS::LITTLE => data.reverse(),
        ENDIANNESS::BIG => ()
    };

    for c in data {
        result = (result << 8) + c as u128
    }

    result = result & ((1 << word_size) - 1);

    result
}