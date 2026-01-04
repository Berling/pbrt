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
    fn test_display() {
        let v = Vector3::new(19, -4, 0);
        assert_eq!(v.to_string(), "Vector3(19, -4, 0)");
    }
}
