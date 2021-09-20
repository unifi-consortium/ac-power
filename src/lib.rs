pub mod trig;
pub mod transforms;
pub mod common_tables;


#[cfg(test)]
mod tests {

    use crate::trig::SinCos;
    use crate::transforms::Abc;

    // define a simple macro for comparing floating point numbers
    macro_rules! assert_delta {
        ($x:expr, $y:expr, $d:expr) => {
            if !($x - $y < $d || $y - $x < $d) { panic!(); }
        }
    }

    #[test]
    fn clark_transform() {
        // test fixed-point
        let abc: Abc<i32> = Abc{a:3827, b:6088, c:-9914};
        let alpha_beta = abc.to_alpha_beta();
        assert_eq!(alpha_beta.alpha, 3826);
        assert_eq!(alpha_beta.beta, 13065);
        assert_eq!(alpha_beta.gamma, 0);

        // test floating-point
        let abc: Abc<f32> = Abc{a: 3827.0, b:6088.0, c:-9914.0};
        let alpha_beta = abc.to_alpha_beta();
        assert_delta!(alpha_beta.alpha, 3826.667, 1e-10);
        assert_delta!(alpha_beta.beta, 13065.579, 1e-10);
        assert_delta!(alpha_beta.gamma, 0.33325195, 1e-10);
    }

    #[test]
    fn dq0_transform() {
 
        let abc: Abc<i32> = Abc {a:0, b:-86603, c:86603};
        let sin_cos = SinCos::<i32>::from_theta(0);
        let dq0 = abc.to_dq0(sin_cos);
        assert_eq!(dq0.d, 100000);
        assert_eq!(dq0.q, 0);
        assert_eq!(dq0.z, 0);
    }

    #[test]
    fn sin_cos_calc() {
        let mut sin_cos: SinCos::<i32>;

        /* test at theta = -pi */
        sin_cos = SinCos::<i32>::from_theta(-2147483648);
        assert_eq!(sin_cos.sin, 0);
        assert_eq!(sin_cos.cos, -2147483648);

        /* test at theta = -3*pi/4 */
        sin_cos = SinCos::<i32>::from_theta(-1610612736);
        assert_eq!(sin_cos.sin, -1518500250);
        assert_eq!(sin_cos.cos, -1518500250);

        /* test at theta = -pi/2 */
        sin_cos = SinCos::<i32>::from_theta(-1073741824);
        assert_eq!(sin_cos.sin, -2147483648);
        assert_eq!(sin_cos.cos, 0);

        /* test at theta = -pi/4 */
        sin_cos = SinCos::<i32>::from_theta(-536870912);
        assert_eq!(sin_cos.sin, -1518500250);
        assert_eq!(sin_cos.cos, 1518500250);

        /* test at theta = 0 */
        sin_cos = SinCos::<i32>::from_theta(0);
        assert_eq!(sin_cos.sin, 0);
        assert_eq!(sin_cos.cos, 2147483647);

        /* test at theta = pi/4 */
        sin_cos = SinCos::<i32>::from_theta(536870912);
        assert_eq!(sin_cos.sin, 1518500250);
        assert_eq!(sin_cos.cos, 1518500250);

        /* test at theta = pi/2 */
        sin_cos = SinCos::<i32>::from_theta(1073741824);
        assert_eq!(sin_cos.sin, 2147483647);
        assert_eq!(sin_cos.cos, 0);

        /* test at theta = 3*pi/4 */
        sin_cos = SinCos::<i32>::from_theta(1610612736);
        assert_eq!(sin_cos.sin, 1518500250);
        assert_eq!(sin_cos.cos, -1518500250);

        /* test at theta = pi */
        sin_cos = SinCos::<i32>::from_theta(2147483647);
        assert_eq!(sin_cos.sin, 2);
        assert_eq!(sin_cos.cos, -2147483648);

        let sin_cos: SinCos::<f32>;
        sin_cos = SinCos::<f32>::from_theta(2.356194490192345);
        assert_delta!(sin_cos.sin, 0.70710677, 1e-5);
        assert_delta!(sin_cos.cos, -0.70710677, 1e-5);
    }

    #[test]
    fn sin_cos_shift_120() {
        let sin_cos = SinCos::<i32>::from_theta(0);

        let sin_cos_shift_right = sin_cos.shift_right_120();
        let sin_cos_shift_left = sin_cos.shift_left_120();
        assert_eq!(sin_cos_shift_right.sin, 1859775392);
        assert_eq!(sin_cos_shift_right.cos, -1073741824);
        assert_eq!(sin_cos_shift_left.sin, -1859775393);
        assert_eq!(sin_cos_shift_left.cos, -1073741824);
        // println!("right -> {:?}", sin_cos_shift_right);
        // println!("right -> {:?}", sin_cos_shift_left);
    }
}
