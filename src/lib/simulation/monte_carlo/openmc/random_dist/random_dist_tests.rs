







///```python
/// def test_maxwell_spectrum():
///    prn_seed = 1
///    T = 0.5
///    ref_val = 0.27767406743161277
///    test_val = openmc.lib.math.maxwell_spectrum(T, prn_seed)
///
///    assert ref_val == test_val
///```
///
///
#[test]
pub fn test_maxwell_spectrum(){
    use super::maxwell_spectrum;
    let mut prn_seed: u64 = 1;
    #[allow(non_snake_case)]
    let T = 0.5;
    let ref_val = 0.27767406743161277;

    let test_val = maxwell_spectrum(T, &mut prn_seed).unwrap();

    assert_eq!(ref_val,test_val);
    

}

/// translation of test:
///```python 
///def test_watt_spectrum():
///    prn_seed = 1
///    a = 0.5
///    b = 0.75
///    ref_val = 0.30957476387766697
///    test_val = openmc.lib.math.watt_spectrum(a, b, prn_seed)
///
///    assert ref_val == test_val
///```
#[test]
pub fn test_watt_spectrum(){

    use super::watt_spectrum;
    let mut prn_seed: u64 = 1;
    let a = 0.5;
    let b = 0.75;
    let ref_val = 0.30957476387766697;

    let test_val = watt_spectrum(a, b, &mut prn_seed).unwrap();

    assert_eq!(ref_val,test_val);
}

// First part of 
// Translation of test:
// ```python
// def test_normal_dist():
//     # When standard deviation is zero, sampled value should be mean
//     prn_seed = 1
//     mean = 14.08
//     stdev = 0.0
//     ref_val = 14.08
//     test_val = openmc.lib.math.normal_variate(mean, stdev, prn_seed)
//     assert ref_val == pytest.approx(test_val)
// 
//     # Use Shapiro-Wilk test to ensure normality of sampled vairates
//     stdev = 1.0
//     samples = []
//     num_samples = 10000
//     for _ in range(num_samples):
//         # sample the normal distribution from openmc
//         samples.append(openmc.lib.math.normal_variate(mean, stdev, prn_seed))
//         prn_seed += 1
//     stat, p = shapiro(samples)
//     assert p > 0.05
// ```
#[test]
pub fn test_normal_dist_part_one(){
    use crate::simulation::monte_carlo::openmc::random_dist::normal_variate;
    let mut prn_seed: u64 = 1;
    let mean = 14.08;
    let stdev = 0.0; 
    let ref_val = 14.08;

    let test_val = normal_variate(mean, stdev, &mut prn_seed).unwrap();

    assert_eq!(ref_val,test_val);
}


// Second part of 
// Translation of test:
// ```python
// def test_normal_dist():
//     # When standard deviation is zero, sampled value should be mean
//     prn_seed = 1
//     mean = 14.08
//     stdev = 0.0
//     ref_val = 14.08
//     test_val = openmc.lib.math.normal_variate(mean, stdev, prn_seed)
//     assert ref_val == pytest.approx(test_val)
// 
//     # Use Shapiro-Wilk test to ensure normality of sampled vairates
//     stdev = 1.0
//     samples = []
//     num_samples = 10000
//     for _ in range(num_samples):
//         # sample the normal distribution from openmc
//         samples.append(openmc.lib.math.normal_variate(mean, stdev, prn_seed))
//         prn_seed += 1
//     stat, p = shapiro(samples)
//     assert p > 0.05
// ```
#[test]
pub fn test_normal_dist_kolmogorov_smirnov() -> Result<(),
crate::teh_o_error::TehOError>{

    use crate::simulation::monte_carlo::openmc::random_dist::normal_variate;
    use std::num::Wrapping;
    let mut prn_seed: u64 = 1;
    let mean = 14.08;
    let stdev = 1.0; 
    let num_samples = 10000;

    let mut test_samples: Vec<f64> = vec![];

    // perform 10,000 samples

    for _i in 0..num_samples {
        let sample = normal_variate(mean, stdev, &mut prn_seed)?;
        test_samples.push(sample);
        

        // perform wrapping arithmetic on prn 
        let mut wrapped_seed = Wrapping(prn_seed);
        wrapped_seed += Wrapping(1);
        prn_seed = wrapped_seed.0;
    }

    // tbd
    //// Now, the shapiro test makes use of scipy code.
    //// So we sort of need to use scipy here
    //// from scipy.stats import shapiro
    //let test_val = normal_variate(mean, stdev, &mut prn_seed)?;

    // The Shapiro-Wilk test is used to check if a distribution is normal
    // shapiro test is harder to find in Rust libraries,
    //
    // However, it is available in github repos:
    // https://github.com/larsgw/stattest/blob/main/src/test/shapiro_wilk.rs
    // stattest
    //
    // Nevertheless, I prefer to have it on crates.io 
    // the closest thing is the Kolmogorov–Smirnov test 
    // https://github.com/doraneko94/statest
    // in the statest crate
    //
    // It tests whether a dataset matches a certain distribution 
    // so, we can also use it to test if a distribution is normal
    //
    use statrs::distribution::Normal;
    use rand::distributions::Distribution;
    
    let mut r = rand::thread_rng();
    let n = Normal::new(mean, stdev).unwrap();

    let mut reference_samples: Vec<f64> = vec![];

    for _i in 0..num_samples {
        let sample = n.sample(&mut r);
        reference_samples.push(sample);
    }

    // now the reference samples are normal dist 
    // and the test samples are also normal dist
    //
    // let's use the Kolmogorov Smirnov test rather than the
    // Shapiro-Wilk test to check if both come from the same 
    // normal distribution

    // we use a 99% confidence level, 
    // normally, 95% will do
    let confidence = 0.99;

    use kolmogorov_smirnov;

    let result = kolmogorov_smirnov::test_f64(
        &reference_samples, &test_samples, confidence);

    if !result.is_rejected {
        // in this case, we are 99% sure that both are from 
        // the same distribution (ie normal)
        return Ok(());
    } else {
        panic!("test failed, not normal distribution")
    }



}

