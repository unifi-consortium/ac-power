pub mod trig;
pub mod common_tables;


#[cfg(test)]
mod tests {

    use crate::trig::SinCos;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn sin_cos_calc() {
        let mut sin_cos: SinCos;

        /* test at theta = -pi */
        sin_cos = SinCos::from_theta(-2147483648);
        assert_eq!(sin_cos.sin, 0);
        assert_eq!(sin_cos.cos, -2147483648);

        /* test at theta = -3*pi/4 */
        sin_cos = SinCos::from_theta(-1610612736);
        assert_eq!(sin_cos.sin, -1518500250);
        assert_eq!(sin_cos.cos, -1518500250);

        /* test at theta = -pi/2 */
        sin_cos = SinCos::from_theta(-1073741824);
        assert_eq!(sin_cos.sin, -2147483648);
        assert_eq!(sin_cos.cos, 0);

        /* test at theta = -pi/4 */
        sin_cos = SinCos::from_theta(-536870912);
        assert_eq!(sin_cos.sin, -1518500250);
        assert_eq!(sin_cos.cos, 1518500250);

        /* test at theta = 0 */
        sin_cos = SinCos::from_theta(0);
        assert_eq!(sin_cos.sin, 0);
        assert_eq!(sin_cos.cos, 2147483647);

        /* test at theta = pi/4 */
        sin_cos = SinCos::from_theta(536870912);
        assert_eq!(sin_cos.sin, 1518500250);
        assert_eq!(sin_cos.cos, 1518500250);

        /* test at theta = pi/2 */
        sin_cos = SinCos::from_theta(1073741824);
        assert_eq!(sin_cos.sin, 2147483647);
        assert_eq!(sin_cos.cos, 0);

        /* test at theta = 3*pi/4 */
        sin_cos = SinCos::from_theta(1610612736);
        assert_eq!(sin_cos.sin, 1518500250);
        assert_eq!(sin_cos.cos, -1518500250);

        /* test at theta = pi */
        sin_cos = SinCos::from_theta(2147483647);
        assert_eq!(sin_cos.sin, 1);
        assert_eq!(sin_cos.cos, -2147483648);
    }
}
