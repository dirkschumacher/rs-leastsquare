extern crate nalgebra as na;

pub fn least_squares<T>(x: na::DMatrix<T>, y: na::DMatrix<T>) -> std::option::Option<na::DMatrix<T>>
where
    T: 'static,
    T: na::ComplexField,
    T: std::marker::Copy,
    T: std::fmt::Debug,
    T: std::cmp::PartialEq,
{
    let qr_result = x.qr();
    let qty = qr_result.q().transpose() * y;
    let beta_hat = qr_result.r().solve_upper_triangular(&qty);
    beta_hat
}

#[cfg(test)]
mod tests {
    use super::least_squares;

    #[test]
    fn it_works() {
        let x = na::DMatrix::from_row_slice(2, 2, &[1.0, 3.0, 2.0, 13.0]);
        let y = na::DMatrix::from_column_slice(2, 1, &[3.0, 5.0]);

        let res = least_squares(x, y).unwrap();
        let coef0_diff: f64 = res[0] - 3.428571428571428;
        let coef1_diff: f64 = res[1] - -0.142857142857143;
        let eps = 1e-10;
        assert!(coef0_diff.abs() <= eps);
        assert!(coef1_diff.abs() <= eps);
    }

    #[test]
    fn problematic_matrix() {
        let x = na::DMatrix::from_row_slice(2, 2, &[1.0, 1.0, 2.0, 2.0]);
        let y = na::DMatrix::from_column_slice(2, 1, &[3.0, 5.0]);

        let res = least_squares(x, y);
        match res {
            None => assert!(true),
            Some(_x) => panic!("Should not happen"),
        }
    }
}
