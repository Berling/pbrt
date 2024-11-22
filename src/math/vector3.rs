use super::{Normal3, Point3};

impl_tuple3!(Vector3);

impl<T, U> From<Point3<U>> for Vector3<T>
where
    Vector3<T>: From<Vector3<U>>,
{
    fn from(value: Point3<U>) -> Self {
        Vector3::<U>::new(value.x, value.y, value.z).into()
    }
}

impl<T, U> From<Normal3<U>> for Vector3<T>
where
    Vector3<T>: From<Vector3<U>>,
{
    fn from(value: Normal3<U>) -> Self {
        Vector3::<U>::new(value.x, value.y, value.z).into()
    }
}

#[cfg(test)]
mod tests {
    use crate::math::{Vector3f32, Vector3i32};

    #[test]
    fn vector3_basics() {
        let v = Vector3f32::new(-1.0, 10.0, 2.0);
        assert_eq!(v, Vector3i32::new(-1, 10, 2).into());
        assert_ne!(v, Vector3f32::new(-1.0, 100.0, 2.0));
        assert_eq!(Vector3f32::new(-2.0, 20.0, 4.0), v + v);
        assert_eq!(Vector3f32::new(0.0, 0.0, 0.0), v - v);
        assert_eq!(Vector3f32::new(-2.0, 20.0, 4.0), v * 2.0);
        assert_eq!(Vector3f32::new(-2.0, 20.0, 4.0), 2.0 * v);
        assert_eq!(Vector3f32::new(-0.5, 5.0, 1.0), v / 2.0);
        assert_eq!(Vector3f32::new(1.0, 10.0, 2.0), v.abs());
        assert_eq!(v, Vector3f32::new(-1.5, 9.9, 1.01).ceil());
        assert_eq!(v, Vector3f32::new(-0.5, 10.01, 2.99).floor());
        assert_eq!(
            Vector3f32::new(-20.0, 10.0, 1.5),
            v.min(Vector3f32::new(-20.0, 20.0, 1.5))
        );
        assert_eq!(
            Vector3f32::new(-1.0, 20.0, 2.0),
            v.max(Vector3f32::new(-20.0, 20.0, 0.0))
        );
        assert_eq!(-1.0, v.min_component_value());
        assert_eq!(-10.0, (-v).min_component_value());
        assert_eq!(10.0, v.max_component_value());
        assert_eq!(1.0, (-v).max_component_value());
        assert_eq!(0, v.min_component_index());
        assert_eq!(1, (-v).min_component_index());
        assert_eq!(1, v.max_component_index());
        assert_eq!(0, (-v).max_component_index());
        assert_eq!(v, v.permute(&[0, 1, 2]));
        assert_eq!(Vector3f32::new(10.0, -1.0, 2.0), v.permute(&[1, 0, 2]));
        assert_eq!(Vector3f32::new(2.0, -1.0, 10.0), v.permute(&[2, 0, 1]));
        assert_eq!(Vector3f32::new(10.0, 10.0, -1.0), v.permute(&[1, 1, 0]));
    }
}
