fn main() {
    use p44::BigUint4096;

    let hex_str = "0505";
    let big_uint = BigUint4096::from_hex_str(hex_str).unwrap();
    println!("{:?}", big_uint.to_hex_string());
    println!("{:?}", big_uint);

    let num1 = BigUint4096::from_hex_str("1234").unwrap();
    let num2 = BigUint4096::from_hex_str("5678").unwrap();
    let result1 = num1.add(&num2);
    println!("{:?}", result1.to_hex_string());

    let num1 = BigUint4096::from_hex_str("5678").unwrap();
    let num2 = BigUint4096::from_hex_str("1234").unwrap();
    let result2 = num1.sub(&num2);
    println!("{:?}", result2.to_hex_string());
}
