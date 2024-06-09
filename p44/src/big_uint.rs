use std::fmt;
use std::str::FromStr;

#[macro_export]
macro_rules! new_biguints {
    ( $( $name:ident, $words:expr );* $(;)? ) => {
        $(
            #[derive(Clone, Debug)]
            pub struct $name {
                data: [u64; $words],
            }

            impl $name {
                pub fn new() -> Self {
                    $name { data: [0; $words] }
                }

                pub fn from_hex_str(hex_str: &str) -> Result<Self, String> {
                    let hex_str = if hex_str.len() % 2 == 0 {
                        hex_str.to_string()
                    } else {
                        format!("0{}", hex_str) // Prepend a '0' to make it even length
                    };

                    let mut bytes = hex::decode(hex_str).map_err(|e| format!("Hex decode error: {}", e))?;
                    let mut data = [0; $words];

                    let data_len = data.len();
                    // add leading '0' for further fast processing
                    let padding = 8 - (bytes.len() % 8);
                    if padding < 8 {
                        bytes = [vec![0u8; padding], bytes].concat();
                    }
                    if bytes.len() * 8 > data_len * 64 {
                        return Err("Input string too long".to_string());
                    }

                    for (i, chunk) in bytes.chunks(8).enumerate() {
                        let mut word = [0u8; 8];
                        word.copy_from_slice(chunk);
                        data[data_len - 1 - i] = u64::from_be_bytes(word);
                    }

                    Ok($name { data })
                }

                pub fn to_hex_string(&self) -> String {
                    let mut hex_string = String::new();

                    for &word in &self.data {
                        if word == 0 { continue; }
                        let bytes = word.to_be_bytes();

                        hex_string.push_str(&hex::encode(bytes));
                    }
                    // Remove leading zeros
                    hex_string.trim_start_matches('0').to_string()
                }

                // Addition
                pub fn add(&self, other: &Self) -> Self {
                    let mut result = Self::new();
                    let mut carry = 0;
                
                    for i in (0..$words).rev() {
                        let sum = self.data[i].wrapping_add(other.data[i]).wrapping_add(carry);
                        result.data[i] = sum & u64::MAX; // Mask to handle overflow
                        carry = if sum > u64::MAX { 1 } else { 0 }; // Calculate carry
                    }
                
                    result
                }

                // Subtraction
                pub fn sub(&self, other: &Self) -> Self {
                    let mut result = Self::new();
                    let mut borrow = 0;

                    for i in (0..$words).rev() {
                        let diff = self.data[i].wrapping_sub(other.data[i]).wrapping_sub(borrow);
                        result.data[i] = diff;
                        borrow = (diff >> 63) & 1; // Check if the subtraction caused underflow
                    }

                    result
                }
            }

            impl FromStr for $name {
                type Err = String;

                fn from_str(s: &str) -> Result<Self, Self::Err> {
                    Self::from_hex_str(s)
                }
            }

            impl fmt::Display for $name {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    write!(f, "{}", self.to_hex_string())
                }
            }
        )*
    };
}

// Generate BigUint types
new_biguints!(
    BigUint1024, 16;
    BigUint2048, 32;
    BigUint4096, 64;
    BigUint8192, 128;
);
