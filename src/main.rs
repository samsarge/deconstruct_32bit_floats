// Deconstruct floating point values (32 bit data)
// src: Rust in Action


// can also get the same ones from std::f32 but this is a demonstration
const BIAS: i32 = 127;
const RADIX: f32 = 2.0;

fn main() {
    let n: f32 = 42.42;

    let (sign, exp, frac) = to_parts(n);
    let (sign_num, exp_num, mant_num) = decode(sign, exp, frac);
    let n_ = from_parts(sign_num, exp_num, mant_num);

    println!("{} -> {}", n, n_);
    println!("field    |  as bits     | as real number");
    println!("sign     |  {:01b}      | {}",  sign, sign_num);
    println!("exponent |  {:08b}      | {}", exp, exp_num);
    println!("mantissa |  {:023b}     | {}", frac, mant_num);
}

/// Isolate the fields that make up a floating point number.
fn to_parts(n: f32) -> (u32, u32, u32) {
    // note the type we're working with its 32 bits for all of this
    // positions start at 0 with right being least significant and left being most signficant
    // 32 bit positions in total (cause starts at 0) (aka 4 bytes)
    // pos 31 = sign
    // pos 23-30 = exponent
    // pos 0-22 = mantissa (or fraction, its a mantissa once its decoded)
    let bits = n.to_bits();

    // strips 31 unwanted bits away to get us our sign bit from significant position
    let sign = (bits >> 31) & 1;
    // filters out the top bit with a logical AND mask, then strips out the 23 unwanted bits
    let exponent = (bits >> 23) & 0xff;
    // retain only the 23 least significant bits via AND mask, called fraction because its a mantissa once its decoded
    let fraction = bits & 0x7fffff;

    (sign, exponent, fraction)
}

/// Decode the components of the floats raw bit pattern into a single number
/// Uses the following calculation:
/// (-1^sign_bit) x mantissa x Radix^(exponent-Bias)
/// where Radix and Bias are defined constants from the encoding standard (IEEE 754-2019)
fn decode(sign: u32, exponent: u32, fraction: u32) -> (f32, f32, f32) {
    // Convert the sign bit to 1.0 or -1.0. Use parens to clarify calculation.
    let signed_i = (-1.0_f32).powf(sign as f32);

    // Exponent must become an i32 in case subtracting the BIAS results in a negative number
    // then it needs to be cast as f32 so that it can be used for exponentiation
    let exponent = (exponent as i32) - BIAS;
    let exponent = RADIX.powf(exponent as f32);

    // decode the mantissa
    let mut mantissa: f32 = 0.0;

    for i in 0..23 {
        let mask = 1 << i;
        let one_at_bit_i = fraction & mask;
        if one_at_bit_i != 0 {
            let i_ = i as f32;
            let weight = 2_f32.powf(i_ - 23.0);
            mantissa += weight;
        }
    }

    (signed_i, exponent, mantissa)
}

// combines the deconstruct_f32() and decode_f32_parts() to create a single decimal number
fn from_parts(sign: f32, exponent: f32, mantissa: f32) -> f32 {
    sign * exponent * mantissa
}