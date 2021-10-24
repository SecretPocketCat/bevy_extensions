use bevy::math::{IVec2, IVec3, UVec2, UVec3, Vec2, Vec3};

macro_rules! impl_vec2_extensions {
    ($t: ty, $axis_t: ty) => {
        impl VecAxis<$axis_t> for $t {
            fn get_axis_value(&self, is_horizontal: bool) -> $axis_t {
                if is_horizontal {
                    self.x
                } else {
                    self.y
                }
            }

            fn get_axis_value_mut(&mut self, is_horizontal: bool) -> &mut $axis_t {
                if is_horizontal {
                    &mut self.x
                } else {
                    &mut self.y
                }
            }

            fn get_axis(&self, is_horizontal: bool) -> Self
            where
                Self: Sized + Copy,
            {
                if is_horizontal {
                    self.transpose()
                } else {
                    self.clone()
                }
            }
        }

        impl Vec2Extensions for $t {
            fn transpose(&self) -> Self
            where
                Self: Sized,
            {
                Self::new(self.y, self.x)
            }
        }
    };
}

macro_rules! impl_vec2_conversion {
    ($t: ty, $vec3_t: ty, $z: expr) => {
        impl Vec2Conversion<$vec3_t> for $t {
            fn to_vec3(&self) -> $vec3_t {
                <$vec3_t>::new(self.x, self.y, $z)
            }
        }
    };
}

pub trait VecAxis<TAxis> {
    fn get_axis_value(&self, is_horizontal: bool) -> TAxis;

    fn get_axis_value_mut(&mut self, is_horizontal: bool) -> &mut TAxis;

    fn get_axis(&self, is_horizontal: bool) -> Self
    where
        Self: Sized + Copy;
}

pub trait Vec2Extensions {
    fn transpose(&self) -> Self
    where
        Self: Sized;
}

pub trait Vec2Conversion<TVec3> {
    fn to_vec3(&self) -> TVec3;
}

impl_vec2_extensions!(Vec2, f32);
impl_vec2_conversion!(Vec2, Vec3, 0.);
impl_vec2_extensions!(IVec2, i32);
impl_vec2_conversion!(IVec2, IVec3, 0);
impl_vec2_extensions!(UVec2, u32);
impl_vec2_conversion!(UVec2, UVec3, 0);

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn get_axis_value() {
        let vec = Vec2::new(1., 2.);

        assert_eq!(vec.x, vec.get_axis_value(true));
        assert_eq!(vec.y, vec.get_axis_value(false));
    }

    #[test]
    fn get_axis() {
        let vec = Vec2::new(1., 2.);

        assert_eq!(Vec2::new(vec.y, vec.x), vec.get_axis(true));
        assert_eq!(vec.clone(), vec.get_axis(false));
    }

    #[test]
    fn transpose() {
        let vec = Vec2::new(1., 2.);

        assert_eq!(Vec2::new(vec.y, vec.x), vec.transpose());
    }

    #[test]
    fn to_vec3() {
        let vec = Vec2::new(1., 2.);

        assert_eq!(Vec3::new(vec.x, vec.y, 0.), vec.to_vec3());
    }
}
