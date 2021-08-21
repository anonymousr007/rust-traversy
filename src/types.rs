/*
Primitive Types
Integers: u8, u16, u32, u64, u128, i8, i16, i32, i64, i128 (number of bits they take in memory)
u is unsigned and i is signed
The minimum and maximum values are from -(2ⁿ⁻¹) to 2ⁿ⁻¹-1.
Floats: f32, f64
Boolean (bool)
Characters (char)
Tuples
Arrays


Rust is a statically typed language, which means that it must know the types of all variables at compile time, 
however, the compiler can usually infer what type we want to use based on the value and how we use it.
*/
pub fn run() {
    // Default is "i32"
    let x = 1;

    // Default is "f64"
    let y = 2.5;

    // Add explicit type 
    let z: i64 = 4545445454545;

    // Find MIN and MAX size 

    // Signed integers
    println!("Min i8: {} to Max i8: {}", std::i8::MIN, std::i8::MAX); 
    // Min i8: -128 to Max i8: 127

    println!("Min i16: {} to Max i16: {}", std::i16::MIN, std::i16::MAX); 
    // Max i16: -32768 to 32767  

    println!("Min i32: {} to Max i32: {}", std::i32::MIN, std::i32::MAX); 
    // Min i32: -2147483648 to Max i32: 2147483647

    println!("Min i64: {} to Max i64: {}", std::i64::MIN, std::i64::MAX); 
    // Min i64: -9223372036854775808 to Max i64: 9223372036854775807

    println!("Min i128: {} to Max i128: {}", std::i128::MIN, std::i128::MAX); 
    // Min i128: -170141183460469231731687303715884105728 to Max i128: 170141183460469231731687303715884105727

    // Unsigned Integers

    println!("Min u8: {} to Max u8: {}", std::u8::MIN, std::u8::MAX); 
    // Min u8: 0 to Max u8: 255

    println!("Min u16: {} to Max u16: {}",std::u16::MIN, std::u16::MAX);
    // Min u16: 0 to Max u16: 65535

    println!("Min u32: {} to Max u32: {}",std::u32::MIN, std::u32::MAX); 
    // Min u32: 0 to Max u32: 4294967295

    println!("Min u64: {} to Max u64: {}",std::u64::MIN, std::u64::MAX); 
    // Min u64: 0 to Max u64: 18446744073709551615

    println!("Min u128: {} to Max u128: {}",std::u128::MIN, std::u128::MAX); 
    // Min u128: 0 to Max u128: 340282366920938463463374607431768211455

    println!("Min f32: {} to Max f32: {}",std::f32::MIN, std::f32::MAX); 
    // Min f32: -340282350000000000000000000000000000000 to Max f32: 340282350000000000000000000000000000000

    println!("Min f64: {} to Max f64: {}",std::f64::MIN, std::f64::MAX);
    // Min f64: -1797693134862315700000000000000000000000000000000000000000000000000000000000000000000000000
    // 00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
    // 00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
    // 0000000000000000 to Max f64: 179769313486231570000000000000000000000000000000000000000000000000000000
    // 00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
    // 00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
    // 00000000000000000000000000000000000

    // Boolean 
    let is_active: bool = true;
    println!("{:?}",(x, y, z, is_active));

    // Get Boolean fom expression
    let is_greater: bool = 10 < 5;
    let a1 = 'a'; // char
    let face = '\u{1F600}'; // emoji 
    println!("{:?}",(x, y, z, is_active, is_greater, a1, face));
}