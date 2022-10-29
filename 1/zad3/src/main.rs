fn square_area_to_circle(size:f64) -> f64 {
    let r = size.sqrt() / 2.0;
    std::f64::consts::PI * r * r
}

fn assert_close(a:f64, b:f64, epsilon:f64) {
    assert!( (a-b).abs() < epsilon, "Expected: {}, got: {}",b,a);
}

mod tests {
    use super::square_area_to_circle;
    use super::assert_close;

    #[test]
    fn test() {
        assert_close(square_area_to_circle(9.0), 7.0685834705770345, 1e-8);
        assert_close(square_area_to_circle(20.0), 15.70796326794897, 1e-8);
        assert_close(square_area_to_circle(64.0), 50.26548245743669, 1e-8);
        assert_close(square_area_to_circle(4.0), 3.141592653589793, 1e-8);
        assert_close(square_area_to_circle(100.0), 78.53981633974483096156, 1e-8);
    }
}