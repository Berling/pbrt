impl_tuple2!(Vector2);

#[cfg(test)]
mod tests {
    use crate::math::{Vector2f32, Vector2i32};

    #[test]
    fn vector2_basics() {
        let v = Vector2f32::new(-1.0, 10.0);
        assert_eq!(v, Vector2i32::new(-1, 10).into());
        assert_ne!(v, Vector2f32::new(-1.0, 100.0));
        assert_eq!(Vector2f32::new(-2.0, 20.0), v + v);
        assert_eq!(Vector2f32::new(0.0, 0.0), v - v);
        assert_eq!(Vector2f32::new(-2.0, 20.0), v * 2.0);
        assert_eq!(Vector2f32::new(-2.0, 20.0), 2.0 * v);
        assert_eq!(Vector2f32::new(-0.5, 5.0), v / 2.0);
        assert_eq!(Vector2f32::new(1.0, 10.0), v.abs());
        assert_eq!(v, Vector2f32::new(-1.5, 9.9).ceil());
        assert_eq!(v, Vector2f32::new(-0.5, 10.01).floor());
        assert_eq!(
            Vector2f32::new(-20.0, 10.0),
            v.min(Vector2f32::new(-20.0, 20.0))
        );
        assert_eq!(
            Vector2f32::new(-1.0, 20.0),
            v.max(Vector2f32::new(-20.0, 20.0))
        );
        assert_eq!(-1.0, v.min_component_value());
        assert_eq!(-10.0, (-v).min_component_value());
        assert_eq!(10.0, v.max_component_value());
        assert_eq!(1.0, (-v).max_component_value());
        assert_eq!(0, v.min_component_index());
        assert_eq!(1, (-v).min_component_index());
        assert_eq!(1, v.max_component_index());
        assert_eq!(0, (-v).max_component_index());
        assert_eq!(v, v.permute(&[0, 1]));
        assert_eq!(Vector2f32::new(10.0, -1.0), v.permute(&[1, 0]));
        assert_eq!(Vector2f32::new(10.0, 10.0), v.permute(&[1, 1]));
    }
}
