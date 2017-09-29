use std::u32;

const FIRST_U8_BIT_MASK: u8 = 0x80;
const FIRST_U32_BIT_MASK: u32 = 0x80;
const NULL_U32_BIT_MASK: u32 = 0x0;

pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    let mut result = Vec::new();
    for value in values.iter().rev() {
        let mut remainer = value.clone();
        let mut first_mask = NULL_U32_BIT_MASK;
        while remainer != 0 || first_mask == NULL_U32_BIT_MASK {
            let byte = remainer & !FIRST_U32_BIT_MASK | first_mask;
            first_mask = FIRST_U32_BIT_MASK;
            remainer >>= 7;
            result.insert(0, byte as u8);
        }
    }
    result
}

pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, &'static str> {
    let mut result: Vec<u32> = vec![0];
    let last_byte = bytes.len() - 1;
    for (b, &byte) in bytes.iter().enumerate() {
        if let Some(mut num) = result.last_mut() {
            if *num >= (u32::MAX >> 4) { return Err("Overflow") }
            *num = (*num << 7) | ((byte & 0x7f) as u32);
        }
        if byte & FIRST_U8_BIT_MASK == 0x0 && b != last_byte { result.push(0); }
        else if byte & FIRST_U8_BIT_MASK != 0x0 && b == last_byte { 
            return Err("Incomplete byte sequence"); 
        }
    }
    Ok(result)
}
