use crate::math::macros::n_tuple_impl;

n_tuple_impl! {Vector2, x, y}

#[cfg(test)]
mod tests {
    use crate::math::vector2::Vector2;

    #[test]
    fn test_new() {
        let v = Vector2::new(1, 2);
        assert_eq!(v.x, 1);
        assert_eq!(v.y, 2);
    }

    #[test]
    fn test_has_nan() {
        assert!(!Vector2::new(1, 2).has_nan());
        assert!(!Vector2::new(1.0, 2.0).has_nan());
        let mut v = Vector2::new(1.0, 2.0);
        v.x = f64::NAN;
        assert!(v.has_nan());
        let mut v = Vector2::new(1.0, 2.0);
        v.y = f64::NAN;
        assert!(v.has_nan());
    }

    #[test]
    fn test_abs() {
        assert_eq!(Vector2::new(-1, 4).abs(), Vector2::new(1, 4));
        assert_eq!(Vector2::new(-4.6, -0.3).abs(), Vector2::new(4.6, 0.3));
    }

    #[test]
    fn test_floor() {
        assert_eq!(Vector2::new(1.2, 0.4).floor(), Vector2::new(1.0, 0.0));
    }

    #[test]
    fn test_ceil() {
        assert_eq!(Vector2::new(1.2, 0.4).ceil(), Vector2::new(2.0, 1.0));
    }

    #[test]
    fn test_lerp() {
        assert_eq!(
            Vector2::new(1.0, 2.0).lerp(Vector2::new(-2.0, 1.0), 0.2),
            Vector2::new(0.4, 1.8)
        );
    }

    #[test]
    fn test_min() {
        assert_eq!(
            Vector2::new(10, -4).min(Vector2::new(-3, -2)),
            Vector2::new(-3, -4)
        );
        assert_eq!(
            Vector2::new(2.4, -3.1).min(Vector2::new(1.2, -4.5)),
            Vector2::new(1.2, -4.5)
        );
    }

    #[test]
    fn test_max() {
        assert_eq!(
            Vector2::new(10, -4).max(Vector2::new(-3, -2)),
            Vector2::new(10, -2)
        );
        assert_eq!(
            Vector2::new(2.4, -3.1).max(Vector2::new(1.2, -4.5)),
            Vector2::new(2.4, -3.1)
        );
    }

    #[test]
    fn test_min_value() {
        assert_eq!(Vector2::new(10, -4).min_value(), -4);
        assert_eq!(Vector2::new(2.4, -3.1).min_value(), -3.1);
    }

    #[test]
    fn test_max_value() {
        assert_eq!(Vector2::new(10, -4).max_value(), 10);
        assert_eq!(Vector2::new(2.4, -3.1).max_value(), 2.4);
    }

    #[test]
    fn test_min_index() {
        assert_eq!(Vector2::new(10, -4).min_index(), 1);
        assert_eq!(Vector2::new(2.4, -3.1).min_index(), 1);
    }

    #[test]
    fn test_max_index() {
        assert_eq!(Vector2::new(10, -4).max_index(), 0);
        assert_eq!(Vector2::new(2.4, -3.1).max_index(), 0);
    }

    #[test]
    fn test_mul_add() {
        assert_eq!(
            Vector2::new(1, -4).mul_add(Vector2::new(2, 1), Vector2::new(0, 2)),
            Vector2::new(2, -2)
        );
    }

    #[test]
    fn test_permute() {
        assert_eq!(
            Vector2::new(1.2, -0.5).permute(&[1, 0]),
            Vector2::new(-0.5, 1.2)
        );
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn test_permute_out_of_bounds() {
        Vector2::new(1.2, -0.5).permute(&[0, 20]);
    }

    #[test]
    fn test_h_prod() {
        assert_eq!(Vector2::new(0.5, 1.2).h_prod(), 0.6);
    }

    #[test]
    fn test_index() {
        let v = Vector2::new(-10, 4);
        assert_eq!(v[0], -10);
        assert_eq!(v[1], 4);
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn test_index_out_of_bounds() {
        let v = Vector2::new(-10, 4);
        v[42];
    }

    #[test]
    fn test_index_mut() {
        let mut v = Vector2::new(-10, 4);
        v[0] = 19;
        v[1] = 2;
        assert_eq!(v[0], 19);
        assert_eq!(v[1], 2);
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn test_index_mut_out_of_bounds() {
        let mut v = Vector2::new(-10, 4);
        v[10] = 14;
    }

    #[test]
    fn test_add() {
        assert_eq!(
            Vector2::new(1.2, 0.4) + Vector2::new(0.2, 0.3),
            Vector2::new(1.4, 0.7)
        );
    }

    #[test]
    fn test_add_assign() {
        let mut v = Vector2::new(-1, 0);
        v += Vector2::new(2, 1);
        assert_eq!(v, Vector2::new(1, 1));
    }

    #[test]
    fn test_sub() {
        assert_eq!(Vector2::new(1, 2) - Vector2::new(3, 2), Vector2::new(-2, 0));
    }

    #[test]
    fn test_sub_assign() {
        let mut v = Vector2::new(3, 1);
        v -= Vector2::new(2, 0);
        assert_eq!(v, Vector2::new(1, 1));
    }

    #[test]
    fn test_mul() {
        assert_eq!(Vector2::new(3, -2) * 2, Vector2::new(6, -4));
        assert_eq!(2 * Vector2::new(3, -2), Vector2::new(6, -4));
    }

    #[test]
    fn test_mul_assign() {
        let mut v = Vector2::new(-4, 2);
        v *= 3;
        assert_eq!(v, Vector2::new(-12, 6));
    }

    #[test]
    fn test_div() {
        assert_eq!(Vector2::new(12, 9) / 3, Vector2::new(4, 3));
    }

    #[test]
    fn test_div_assign() {
        let mut v = Vector2::new(16, 64);
        v /= 4;
        assert_eq!(v, Vector2::new(4, 16));
    }

    #[test]
    fn test_neg() {
        assert_eq!(-Vector2::new(1, 2), Vector2::new(-1, -2));
    }

    #[test]
    fn test_display() {
        let v = Vector2::new(19, -4);
        assert_eq!(v.to_string(), "Vector2(19, -4)");
    }

    #[test]
    fn test_from() {
        assert_eq!(
            2.0 * Vector2::<f64>::from(Vector2::new(1, 2)),
            Vector2::new(2.0, 4.0)
        );
    }
}
