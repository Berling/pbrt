use crate::math::macros::n_tuple_impl;

n_tuple_impl! {Vector3, x, y, z}

#[cfg(test)]
mod tests {
    use crate::math::vector3::Vector3;

    #[test]
    fn test_new() {
        let v = Vector3::new(1, 2, 3);
        assert_eq!(v.x, 1);
        assert_eq!(v.y, 2);
        assert_eq!(v.z, 3);
    }

    #[test]
    fn test_has_nan() {
        assert!(!Vector3::new(1, 2, 3).has_nan());
        assert!(!Vector3::new(1.0, 2.0, 3.0).has_nan());
        let mut v = Vector3::new(1.0, 2.0, 3.0);
        v.x = f64::NAN;
        assert!(v.has_nan());
        let mut v = Vector3::new(1.0, 2.0, 3.0);
        v.y = f64::NAN;
        assert!(v.has_nan());
        let mut v = Vector3::new(1.0, 2.0, 3.0);
        v.z = f64::NAN;
        assert!(v.has_nan());
    }

    #[test]
    fn test_abs() {
        assert_eq!(Vector3::new(-1, 4, 19).abs(), Vector3::new(1, 4, 19));
        assert_eq!(
            Vector3::new(-4.6, -0.3, 42.3).abs(),
            Vector3::new(4.6, 0.3, 42.3)
        );
    }

    #[test]
    fn test_floor() {
        assert_eq!(
            Vector3::new(1.2, 0.4, 10.8).floor(),
            Vector3::new(1.0, 0.0, 10.0)
        );
    }

    #[test]
    fn test_ceil() {
        assert_eq!(
            Vector3::new(1.2, 0.4, 10.8).ceil(),
            Vector3::new(2.0, 1.0, 11.0)
        );
    }

    #[test]
    fn test_lerp() {
        assert_eq!(
            Vector3::new(1.0, 2.0, 4.0).lerp(Vector3::new(-2.0, 1.0, 5.0), 0.2),
            Vector3::new(0.4, 1.8, 4.2)
        );
    }

    #[test]
    fn test_min() {
        assert_eq!(
            Vector3::new(10, -4, 8).min(Vector3::new(-3, -2, 90)),
            Vector3::new(-3, -4, 8)
        );
        assert_eq!(
            Vector3::new(2.4, -3.1, -12.45).min(Vector3::new(1.2, -4.5, 9.8)),
            Vector3::new(1.2, -4.5, -12.45)
        );
    }

    #[test]
    fn test_max() {
        assert_eq!(
            Vector3::new(10, -4, 8).max(Vector3::new(-3, -2, 90)),
            Vector3::new(10, -2, 90)
        );
        assert_eq!(
            Vector3::new(2.4, -3.1, -12.45).max(Vector3::new(1.2, -4.5, 9.8)),
            Vector3::new(2.4, -3.1, 9.8)
        );
    }

    #[test]
    fn test_min_value() {
        assert_eq!(Vector3::new(10, -4, 8).min_value(), -4);
        assert_eq!(Vector3::new(2.4, -3.1, -12.45).min_value(), -12.45);
    }

    #[test]
    fn test_max_value() {
        assert_eq!(Vector3::new(10, -4, 8).max_value(), 10);
        assert_eq!(Vector3::new(2.4, -3.1, -12.45).max_value(), 2.4);
    }

    #[test]
    fn test_min_index() {
        assert_eq!(Vector3::new(10, -4, 8).min_index(), 1);
        assert_eq!(Vector3::new(2.4, -3.1, -12.45).min_index(), 2);
    }

    #[test]
    fn test_max_index() {
        assert_eq!(Vector3::new(10, -4, 8).max_index(), 0);
        assert_eq!(Vector3::new(2.4, -3.1, -12.45).max_index(), 0);
    }

    #[test]
    fn test_mul_add() {
        assert_eq!(
            Vector3::new(1, -4, 0).mul_add(Vector3::new(2, 1, 3), Vector3::new(0, 2, -1)),
            Vector3::new(2, -2, -1)
        );
    }

    #[test]
    fn test_permute() {
        assert_eq!(
            Vector3::new(1.2, -0.5, 2.4).permute(&[0, 2, 1]),
            Vector3::new(1.2, 2.4, -0.5)
        );
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn test_permute_out_of_bounds() {
        Vector3::new(1.2, -0.5, 2.4).permute(&[0, 20, 1]);
    }

    #[test]
    fn test_h_prod() {
        assert_eq!(Vector3::new(0.5, 1.2, -2.4).h_prod(), -1.44);
    }

    #[test]
    fn test_index() {
        let v = Vector3::new(-10, 4, 42);
        assert_eq!(v[0], -10);
        assert_eq!(v[1], 4);
        assert_eq!(v[2], 42);
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn test_index_out_of_bounds() {
        let v = Vector3::new(-10, 4, 42);
        v[42];
    }

    #[test]
    fn test_index_mut() {
        let mut v = Vector3::new(-10, 4, 42);
        v[0] = 19;
        v[1] = 2;
        v[2] = -5;
        assert_eq!(v[0], 19);
        assert_eq!(v[1], 2);
        assert_eq!(v[2], -5);
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn test_index_mut_out_of_bounds() {
        let mut v = Vector3::new(-10, 4, 42);
        v[10] = 14;
    }

    #[test]
    fn test_add() {
        assert_eq!(
            Vector3::new(1.2, 0.4, 0.5) + Vector3::new(0.2, 0.3, -0.2),
            Vector3::new(1.4, 0.7, 0.3)
        );
    }

    #[test]
    fn test_add_assign() {
        let mut v = Vector3::new(-1, 0, 3);
        v += Vector3::new(2, 1, -5);
        assert_eq!(v, Vector3::new(1, 1, -2));
    }

    #[test]
    fn test_sub() {
        assert_eq!(
            Vector3::new(1, 2, 3) - Vector3::new(3, 2, 1),
            Vector3::new(-2, 0, 2)
        );
    }

    #[test]
    fn test_sub_assign() {
        let mut v = Vector3::new(3, 1, 6);
        v -= Vector3::new(2, 0, 8);
        assert_eq!(v, Vector3::new(1, 1, -2));
    }

    #[test]
    fn test_mul() {
        assert_eq!(Vector3::new(3, -2, 0) * 2, Vector3::new(6, -4, 0));
        assert_eq!(2 * Vector3::new(3, -2, 0), Vector3::new(6, -4, 0));
    }

    #[test]
    fn test_mul_assign() {
        let mut v = Vector3::new(-4, 2, 1);
        v *= 3;
        assert_eq!(v, Vector3::new(-12, 6, 3));
    }

    #[test]
    fn test_div() {
        assert_eq!(Vector3::new(12, 9, 21) / 3, Vector3::new(4, 3, 7));
    }

    #[test]
    fn test_div_assign() {
        let mut v = Vector3::new(16, 64, 128);
        v /= 4;
        assert_eq!(v, Vector3::new(4, 16, 32));
    }

    #[test]
    fn test_neg() {
        assert_eq!(-Vector3::new(1, 2, 3), Vector3::new(-1, -2, -3));
    }

    #[test]
    fn test_display() {
        let v = Vector3::new(19, -4, 0);
        assert_eq!(v.to_string(), "Vector3(19, -4, 0)");
    }

    #[test]
    fn test_from() {
        assert_eq!(
            2.0 * Vector3::<f64>::from(Vector3::new(1, 2, 3)),
            Vector3::new(2.0, 4.0, 6.0)
        );
    }
}
