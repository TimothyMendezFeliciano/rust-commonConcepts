pub fn scalar_main() {
    let _true_or_false: bool = false;
    let _signed_number_8_bit: u8 = 255;
    let _signed_number_16_bit: u16 = 2255;
    let _signed_number_32_bit: u32 = 2222555;
    let _signed_number_64_bit: u64 = 2222555;
    let _signed_number_128_bit: u128 = 22222255555;
    let _number_8_bit: i8 = -128;
    let _number_16_bit: i16 = -1128;
    let _number_32_bit: i32 = -11228;
    let _number_64_bit: i64 = -112288;
    let _number_128_bit: i128 = -11228800;

    let _floatin_point_example: f32 = 128.256;
    let _character_example: char = 'x';

    let result = integer_division(_number_8_bit, -25);
    println!("The result of division {_number_8_bit} and -25 is: {result}")
}

fn integer_division(unsigned_number: i8, floater: i8) -> i8 {
//     division
    let quotient = unsigned_number / floater;

    return quotient;
}