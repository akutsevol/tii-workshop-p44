use p44::big_uint::*;

#[test]
fn test_from_hex_str_and_to_hex_string() {
    let hex_str = "123456789abcdef0";
    let big_uint = BigUint4096::from_hex_str(hex_str).unwrap();
    assert_eq!(big_uint.to_hex_string(), hex_str);
}

#[test]
fn test_display_trait() {
    let hex_str = "123456789abcdef0";
    let big_uint = BigUint4096::from_hex_str(hex_str).unwrap();
    assert_eq!(format!("{}", big_uint), hex_str);
}

#[test]
fn test_addition() {
    let num1 = BigUint4096::from_hex_str("1234").unwrap();
    let num2 = BigUint4096::from_hex_str("5678").unwrap();
    let result = num1.add(&num2);
    assert_eq!(result.to_hex_string(), "68ac");
}

#[test]
fn test_subtraction() {
    let num1 = BigUint4096::from_hex_str("5678").unwrap();
    let num2 = BigUint4096::from_hex_str("1234").unwrap();
    let result = num1.sub(&num2);
    assert_eq!(result.to_hex_string(), "4444");
}

#[test]
fn test_biguint_macro_generated_types() {
    let big_uint_1024 = BigUint1024::from_hex_str("123456").unwrap();
    let big_uint_2048 = BigUint2048::from_hex_str("abcdef").unwrap();
    let big_uint_4096 = BigUint4096::from_hex_str("7890ab").unwrap();
    let big_uint_8192 = BigUint8192::from_hex_str("1234abcd").unwrap();

    assert_eq!(big_uint_1024.to_hex_string(), "123456");
    assert_eq!(big_uint_2048.to_hex_string(), "abcdef");
    assert_eq!(big_uint_4096.to_hex_string(), "7890ab");
    assert_eq!(big_uint_8192.to_hex_string(), "1234abcd");
}
