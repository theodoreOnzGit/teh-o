#[test]
pub fn legendre_polynomial_placeholder_tests(){
    use peroxide;

    //def test_calc_pn():
    //    max_order = 10
    //    test_xs = np.linspace(-1., 1., num=5, endpoint=True)
    //
    //    # Reference solutions from scipy
    //    ref_vals = np.array([sp.special.eval_legendre(n, test_xs)
    //                         for n in range(0, max_order + 1)])
    //
    //    test_vals = []
    //    for x in test_xs:
    //        test_vals.append(openmc.lib.math.calc_pn(max_order, x).tolist())
    //
    //    test_vals = np.swapaxes(np.array(test_vals), 0, 1)
    //
    //    assert np.allclose(ref_vals, test_vals)
    //
    //
    //def test_evaluate_legendre():
    //    max_order = 10
    //    # Coefficients are set to 1, but will incorporate the (2l+1)/2 norm factor
    //    # for the reference solution
    //    test_coeffs = [0.5 * (2. * l + 1.) for l in range(max_order + 1)]
    //    test_xs = np.linspace(-1., 1., num=5, endpoint=True)
    //
    //    ref_vals = np.polynomial.legendre.legval(test_xs, test_coeffs)
    //
    //    # Set the coefficients back to 1s for the test values since
    //    # evaluate legendre incorporates the (2l+1)/2 term on its own
    //    test_coeffs = [1. for l in range(max_order + 1)]
    //
    //    test_vals = np.array([openmc.lib.math.evaluate_legendre(test_coeffs, x)
    //                          for x in test_xs])
    //
    //    assert np.allclose(ref_vals, test_vals)
    //
    //
    //def test_calc_rn():
    //    max_order = 10
    //    test_ns = np.array([i for i in range(0, max_order + 1)])
    //    azi = 0.1  # Longitude
    //    pol = 0.2  # Latitude
    //    test_uvw = np.array([np.sin(pol) * np.cos(azi),
    //                         np.sin(pol) * np.sin(azi),
    //                         np.cos(pol)])
    //
    //    # Reference solutions from the equations
    //    ref_vals = []
    //
    //    def coeff(n, m):
    //        return np.sqrt((2. * n + 1) * sp.special.factorial(n - m) /
    //                       (sp.special.factorial(n + m)))
    //
    //    def pnm_bar(n, m, mu):
    //        val = coeff(n, m)
    //        if m != 0:
    //            val *= np.sqrt(2.)
    //        val *= sp.special.lpmv([m], [n], [mu])
    //        return val[0]
    //
    //    ref_vals = []
    //    for n in test_ns:
    //        for m in range(-n, n + 1):
    //            if m < 0:
    //                ylm = pnm_bar(n, np.abs(m), np.cos(pol)) * \
    //                    np.sin(np.abs(m) * azi)
    //            else:
    //                ylm = pnm_bar(n, m, np.cos(pol)) * np.cos(m * azi)
    //
    //            # Un-normalize for comparison
    //            ylm /= np.sqrt(2. * n + 1.)
    //            ref_vals.append(ylm)
    //
    //    test_vals = []
    //    test_vals = openmc.lib.math.calc_rn(max_order, test_uvw)
    //
    //    assert np.allclose(ref_vals, test_vals)
    //
    //
    //def test_calc_zn():
    //    n = 10
    //    rho = 0.5
    //    phi = 0.5
    //
    //    # Reference solution from running the C++ implementation
    //    ref_vals = np.array([
    //        1.00000000e+00, 2.39712769e-01, 4.38791281e-01,
    //        2.10367746e-01, -5.00000000e-01, 1.35075576e-01,
    //        1.24686873e-01, -2.99640962e-01, -5.48489101e-01,
    //        8.84215021e-03, 5.68310892e-02, -4.20735492e-01,
    //        -1.25000000e-01, -2.70151153e-01, -2.60091773e-02,
    //        1.87022545e-02, -3.42888902e-01, 1.49820481e-01,
    //        2.74244551e-01, -2.43159131e-02, -2.50357380e-02,
    //        2.20500013e-03, -1.98908812e-01, 4.07587508e-01,
    //        4.37500000e-01, 2.61708929e-01, 9.10321205e-02,
    //        -1.54686328e-02, -2.74049397e-03, -7.94845816e-02,
    //        4.75368705e-01, 7.11647284e-02, 1.30266162e-01,
    //        3.37106977e-02, 1.06401886e-01, -7.31606787e-03,
    //        -2.95625975e-03, -1.10250006e-02, 3.55194307e-01,
    //        -1.44627826e-01, -2.89062500e-01, -9.28644588e-02,
    //        -1.62557358e-01, 7.73431638e-02, -2.55329539e-03,
    //        -1.90923851e-03, 1.57578403e-02, 1.72995854e-01,
    //        -3.66267690e-01, -1.81657333e-01, -3.32521518e-01,
    //        -2.59738162e-02, -2.31580576e-01, 4.20673902e-02,
    //        -4.11710546e-04, -9.36449487e-04, 1.92156884e-02,
    //        2.82515641e-02, -3.90713738e-01, -1.69280296e-01,
    //        -8.98437500e-02, -1.08693628e-01, 1.78813094e-01,
    //        -1.98191857e-01, 1.65964201e-02, 2.77013853e-04])
    //
    //    test_vals = openmc.lib.math.calc_zn(n, rho, phi)
    //
    //    assert np.allclose(ref_vals, test_vals)
    //
    //
    //def test_calc_zn_rad():
    //    n = 10
    //    rho = 0.5
    //
    //    # Reference solution from running the C++ implementation
    //    ref_vals = np.array([
    //        1.00000000e+00, -5.00000000e-01, -1.25000000e-01,
    //        4.37500000e-01, -2.89062500e-01,-8.98437500e-02])
    //
    //    test_vals = openmc.lib.math.calc_zn_rad(n, rho)
    //
    //    assert np.allclose(ref_vals, test_vals)
}
