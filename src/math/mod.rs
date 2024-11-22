#[macro_use]
mod macros;
mod partial_pre_ord;
mod vector2;
mod vector3;

pub use partial_pre_ord::PartialPreOrd;
pub use vector2::Vector2;
pub use vector3::Vector3;

pub type Vector2f32 = Vector2<f32>;
pub type Vector2i32 = Vector2<i32>;
pub type Vector3f32 = Vector3<f32>;
pub type Vector3i32 = Vector3<i32>;
