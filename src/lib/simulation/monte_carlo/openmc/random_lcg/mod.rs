use std::{ops::Deref, num::Wrapping};

use crate::teh_o_error::TehOError;

/// pseudo random number generator
///
///
/// https://github.com/openmc-dev/openmc/blob/develop/docs/source/methods/random_numbers.rst
///
///
/// OpenMC uses a future seeding algorithm using the following literature
/// to skip N random numbers ahead:
/// @article{brown1994random,
///  title={Random number generation with arbitrary strides},
///  author={Brown, Forrest B},
///  journal={Transactions of the American Nuclear Society},
///  volume={71},
///  number={CONF-941102-},
///  year={1994}
///}

static PRN_MULT: u64 = 6364136223846793005;
static PRN_ADD: u64 = 1442695040888963407;
static PRN_STRIDE: u64 = 152917;

pub fn prn(seed: &mut u64) -> Result<f64,TehOError> {

    // advance LCG
    //
    // OpenMC apparently uses wrapped arithmetic on purpose to permutate 
    // the random numbers
    let mut wrapped_seed: Wrapping<u64> = 
        Wrapping(*seed);

    wrapped_seed = Wrapping(PRN_MULT) * wrapped_seed + Wrapping(PRN_ADD);


    // permutate the output,
    // OpenMC usses some bit shifting magic here, so i don't know what's 
    // really going on, I'll just use it.
    //
    // Apparently the hat operator ^ is a bitwise exclusive or 
    // https://learn.microsoft.com/en-us/cpp/cpp/bitwise-exclusive-or-operator-hat?view=msvc-170
    //
    // It also means a bitwise exclusive or in Rust, so thankfully, it's 
    // mostly a copy/paste
    // https://doc.rust-lang.org/book/appendix-02-operators.html
    //
    // this part is buggy:
    //
    //let word: u64 = 
    //    ((*seed  >> ((*seed >> 59 as u64) + 5 as u64)) ^ *seed) * 12605985483714917081 as u64;

    //let result: u64 = (word >> 43 as u64) ^ word ;
    //
    // note that this is page 27/28 onwards of 
    // @techreport{oneill:pcg2014,
    //    title = "PCG: A Family of Simple Fast Space-Efficient Statistically Good
    //    Algorithms for Random Number Generation", author = "Melissa E. O'Neill",
    //    institution = "Harvey Mudd College",
    //    address = "Claremont, CA",
    //    number = "HMC-CS-2014-0905",
    //    year = "2014",
    //    month = Sep,
    //    xurl = "https://www.cs.hmc.edu/tr/hmc-cs-2014-0905.pdf",
    //}
    //
    // In that section, permutation functions are defined
    //
    // on page 32 specifically, it talks about dropping bits
    // and in page 44, it talks about taking some dropped bits to the 
    // power of other dropped bits similar to what we see in openmc

    let bit_dropped_stuff: Wrapping<u64> = 
        Wrapping(
            (wrapped_seed.0  >> ((wrapped_seed.0 >> 59 ) + 5 )) ^ wrapped_seed.0
            );


    // had a prior error with 'attempt to multiply with overflow'
    // probably means there are too many bits to fit in a u64,
    // so u64 is a huge integer but is like 2^64 
    // 18446744073709551615
    //
    // To remedy this, I used the std::num::Wrapping 
    // struct, which explicitly uses number wrapping for integer overflows 
    // of u64 types similar to how openmc functions, except that 
    // this is done explicitly.
    //
    // For example, u8 has a max value of 255,
    // if we add 1, then we have 256. But u8 cannot contain the number 256
    // as there are not enough bits. This is called integer overflow.
    //
    // For unsigned integers,
    // the solution is that 255_u8 + 1_u8 = 0_u8 
    //
    // This means that we go all the way back from 255 to start from 0 
    // again. This is called wrapping.
    // 
    // in C++, this happens automatically. For Rust, we consider integer 
    // overflows undefined behaviour and we want to avoid it.
    //
    // However, we can still use it. But we must do so explicitly
    // to do so, we need to use the Wrapping Struct:
    //
    //
    let word: Wrapping<u64> = 
         bit_dropped_stuff * Wrapping(12605985483714917081);

    //let bit_shifted_word = word >> 43;

    let result: Wrapping<u64> = Wrapping((word.0 >> 43 ) ^ word.0);

    let result_float: f64 = result.0 as f64;

    // the ldexp (load exponent, is called)
    // it multiplies a floating point value arg by the number 2 raised to the exp power.
    // https://en.cppreference.com/w/c/numeric/math/ldexp

    //panic!("{}",result_float);
    // set the seed to be the wrapping seed amount
    *seed = wrapped_seed.0;


    return ldexp(result_float, -64);
}

/// it multiplies a floating point value arg by the number 2 raised to the exp power.
pub fn ldexp(arg: f64, int: i64) -> Result<f64, TehOError>{

    let two_exp: f64 = (2.0 as f64).powf(int as f64);

    let value: f64 = arg * two_exp;

    return Ok(value);
}

/// The algorithm here to determine the parameters used to skip ahead is
/// described in F. Brown, "Random Number Generation with Arbitrary Stride,"
/// Trans. Am. Nucl. Soc. (Nov. 1994). This algorithm is able to skip ahead in
/// O(log2(N)) operations instead of O(N). Basically, it computes parameters G
/// and C which can then be used to find x_N = G*x_0 + C mod 2^M.
///
/// Still need to test
pub fn future_seed(n: u64, seed: u64) -> Result<u64, TehOError>{

    // initialise constants
    let mut g = PRN_MULT;
    let mut c = PRN_ADD;
    let mut g_new: u64 = 1;
    let mut c_new: u64 = 0;

    // assign a local mutable n 
    let mut local_n = n;
    
    while local_n > 0 {
        //Check if least significant bit is 1 

        if (local_n & 1) > 0 {
            g_new *= g;
            c_new = c_new * g + c;
        }

        c *= g + 1;
        g *= g;

        // Move bits right, dropping least significant bit 

        local_n >>= 1;

    };

    return Ok(g_new * seed + c_new);

}
