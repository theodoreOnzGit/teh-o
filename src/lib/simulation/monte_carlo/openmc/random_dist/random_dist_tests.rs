

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

//def test_watt_spectrum():
//    prn_seed = 1
//    a = 0.5
//    b = 0.75
//    ref_val = 0.30957476387766697
//    test_val = openmc.lib.math.watt_spectrum(a, b, prn_seed)
//
//    assert ref_val == test_val
