// From: https://github.com/bitshifter/pathtrace-rs/blob/vec3_sse/src/vmath.rs

#![allow(dead_code)]

#[cfg(target_feature = "sse2")]
pub use self::sse2::*;

#[cfg(not(target_feature = "sse2"))]
pub use self::scalar::*;

mod sse2 {
    #[cfg(target_arch = "x86")]
    use std::arch::x86::*;
    #[cfg(target_arch = "x86_64")]
    use std::arch::x86_64::*;
    use std::f32;
    use std::fmt;
    use std::ops::*;

    //-------------------------------------------------------------------------
    // Float3
    //-------------------------------------------------------------------------
    #[derive(Clone, Copy, Debug)]
    #[repr(C)]
    pub struct Float3(__m128);

    #[inline]
    pub fn float3(x: f32, y: f32, z: f32) -> Float3 {
        Float3::new(x, y, z)
    }

    impl Float3 {
        #[inline]
        pub fn zero() -> Float3 {
            unsafe { Float3(_mm_set1_ps(0.0)) }
        }

        #[inline]
        pub fn unwrap(self) -> __m128 {
            self.0
        }

        #[inline]
        pub fn new(x: f32, y: f32, z: f32) -> Float3 {
            unsafe { Float3(_mm_set_ps(z, z, y, x)) }
        }

        #[inline]
        pub fn get_x(self) -> f32 {
            unsafe { _mm_cvtss_f32(self.0) }
        }

        #[inline]
        pub fn get_y(self) -> f32 {
            unsafe { _mm_cvtss_f32(_mm_shuffle_ps(self.0, self.0, 0b01_01_01_01)) }
        }

        #[inline]
        pub fn get_z(self) -> f32 {
            unsafe { _mm_cvtss_f32(_mm_shuffle_ps(self.0, self.0, 0b10_10_10_10)) }
        }

        #[inline]
        pub fn yzx(self) -> Float3 {
            unsafe { Float3(_mm_shuffle_ps(self.0, self.0, 0b00_00_10_01)) }
        }

        #[inline]
        pub fn zxy(self) -> Float3 {
            unsafe { Float3(_mm_shuffle_ps(self.0, self.0, 0b01_01_00_10)) }
        }

        #[inline]
        pub fn set_x(&mut self, x: f32) {
            unsafe {
                self.0 = _mm_move_ss(self.0, _mm_set_ss(x));
            }
        }

        #[inline]
        pub fn set_y(&mut self, y: f32) {
            unsafe {
                let mut t = _mm_move_ss(self.0, _mm_set_ss(y));
                t = _mm_shuffle_ps(t, t, 0b11_10_00_00);
                self.0 = _mm_move_ss(t, self.0);
            }
        }

        #[inline]
        pub fn set_z(&mut self, z: f32) {
            unsafe {
                let mut t = _mm_move_ss(self.0, _mm_set_ss(z));
                t = _mm_shuffle_ps(t, t, 0b11_00_01_00);
                self.0 = _mm_move_ss(t, self.0);
            }
        }

        #[inline]
        pub fn sum(self) -> f32 {
            self.get_x() + self.get_y() + self.get_z()
        }

        #[inline]
        pub fn dot(self, rhs: Float3) -> f32 {
            (self * rhs).sum()
        }

        #[inline]
        pub fn cross(self, rhs: Float3) -> Float3 {
            // x  <-  a.y*b.z - a.z*b.y
            // y  <-  a.z*b.x - a.x*b.z
            // z  <-  a.x*b.y - a.y*b.x
            // We can save a shuffle by grouping it in this wacky order:
            (self.zxy() * rhs - self * rhs.zxy()).zxy()
        }

        #[inline]
        pub fn length(self) -> f32 {
            self.dot(self).sqrt()
        }

        #[inline]
        pub fn normalize(self) -> Float3 {
            let inv_length = 1.0 / self.dot(self).sqrt();
            self * inv_length
        }
    }

    impl fmt::Display for Float3 {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "[{}, {}, {}]", self.get_x(), self.get_y(), self.get_z())
        }
    }

    impl Div<f32> for Float3 {
        type Output = Float3;
        #[inline]
        fn div(self, rhs: f32) -> Float3 {
            unsafe { Float3(_mm_div_ps(self.0, _mm_set1_ps(rhs))) }
        }
    }

    impl DivAssign<f32> for Float3 {
        #[inline]
        fn div_assign(&mut self, rhs: f32) {
            unsafe { self.0 = _mm_div_ps(self.0, _mm_set1_ps(rhs)) }
        }
    }

    impl Mul<Float3> for Float3 {
        type Output = Float3;
        #[inline]
        fn mul(self, rhs: Float3) -> Float3 {
            unsafe { Float3(_mm_mul_ps(self.0, rhs.0)) }
        }
    }

    impl MulAssign<Float3> for Float3 {
        #[inline]
        fn mul_assign(&mut self, rhs: Float3) {
            unsafe {
                self.0 = _mm_mul_ps(self.0, rhs.0);
            }
        }
    }

    impl Mul<f32> for Float3 {
        type Output = Float3;
        #[inline]
        fn mul(self, rhs: f32) -> Float3 {
            unsafe { Float3(_mm_mul_ps(self.0, _mm_set1_ps(rhs))) }
        }
    }

    impl MulAssign<f32> for Float3 {
        #[inline]
        fn mul_assign(&mut self, rhs: f32) {
            unsafe { self.0 = _mm_mul_ps(self.0, _mm_set1_ps(rhs)) }
        }
    }

    impl Mul<Float3> for f32 {
        type Output = Float3;
        #[inline]
        fn mul(self, rhs: Float3) -> Float3 {
            unsafe { Float3(_mm_mul_ps(_mm_set1_ps(self), rhs.0)) }
        }
    }

    impl Add for Float3 {
        type Output = Float3;
        #[inline]
        fn add(self, rhs: Float3) -> Float3 {
            unsafe { Float3(_mm_add_ps(self.0, rhs.0)) }
        }
    }

    impl AddAssign for Float3 {
        #[inline]
        fn add_assign(&mut self, rhs: Float3) {
            unsafe { self.0 = _mm_add_ps(self.0, rhs.0) }
        }
    }

    impl Sub for Float3 {
        type Output = Float3;
        #[inline]
        fn sub(self, rhs: Float3) -> Float3 {
            unsafe { Float3(_mm_sub_ps(self.0, rhs.0)) }
        }
    }

    impl SubAssign for Float3 {
        #[inline]
        fn sub_assign(&mut self, rhs: Float3) {
            unsafe { self.0 = _mm_sub_ps(self.0, rhs.0) }
        }
    }

    impl Neg for Float3 {
        type Output = Float3;
        #[inline]
        fn neg(self) -> Float3 {
            unsafe { Float3(_mm_sub_ps(_mm_set1_ps(0.0), self.0)) }
        }
    }
}

mod scalar {
    use std::f32;
    use std::fmt;
    use std::ops::*;
    #[derive(Clone, Copy, Debug)]
    #[repr(C)]
    pub struct Float3(f32, f32, f32);

    #[inline]
    pub fn float3(x: f32, y: f32, z: f32) -> Float3 {
        Float3(x, y, z)
    }

    impl Float3 {
        #[inline]
        pub fn zero() -> Float3 {
            Float3(0.0, 0.0, 0.0)
        }

        #[inline]
        pub fn unwrap(self) -> (f32, f32, f32) {
            (self.0, self.1, self.2)
        }

        #[inline]
        pub fn new(x: f32, y: f32, z: f32) -> Float3 {
            Float3(x, y, z)
        }

        #[inline]
        pub fn get_x(self) -> f32 {
            self.0
        }

        #[inline]
        pub fn get_y(self) -> f32 {
            self.1
        }

        #[inline]
        pub fn get_z(self) -> f32 {
            self.2
        }

        #[inline]
        pub fn dot(self, rhs: Float3) -> f32 {
            (self.0 * rhs.0) + (self.1 * rhs.1) + (self.2 * rhs.2)
        }

        #[inline]
        pub fn cross(self, rhs: Float3) -> Float3 {
            Float3(
                self.1 * rhs.2 - rhs.1 * self.2,
                self.2 * rhs.0 - rhs.2 * self.0,
                self.0 * rhs.1 - rhs.0 * self.1,
            )
        }

        #[inline]
        pub fn length(self) -> f32 {
            self.dot(self).sqrt()
        }

        #[inline]
        pub fn normalize(self) -> Float3 {
            let inv_length = 1.0 / self.dot(self).sqrt();
            self * inv_length
        }
    }

    impl fmt::Display for Float3 {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "[{}, {}, {}]", self.0, self.1, self.2)
        }
    }

    impl Div<f32> for Float3 {
        type Output = Float3;
        #[inline]
        fn div(self, rhs: f32) -> Float3 {
            Float3(self.0 / rhs, self.1 / rhs, self.2 / rhs)
        }
    }

    impl DivAssign<f32> for Float3 {
        #[inline]
        fn div_assign(&mut self, rhs: f32) {
            *self = Float3(self.0 / rhs, self.1 / rhs, self.2 / rhs)
        }
    }

    impl Mul<Float3> for Float3 {
        type Output = Float3;
        #[inline]
        fn mul(self, rhs: Float3) -> Float3 {
            Float3(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
        }
    }

    impl MulAssign<Float3> for Float3 {
        #[inline]
        fn mul_assign(&mut self, rhs: Float3) {
            *self = Float3(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
        }
    }

    impl Mul<f32> for Float3 {
        type Output = Float3;
        #[inline]
        fn mul(self, rhs: f32) -> Float3 {
            Float3(self.0 * rhs, self.1 * rhs, self.2 * rhs)
        }
    }

    impl MulAssign<f32> for Float3 {
        #[inline]
        fn mul_assign(&mut self, rhs: f32) {
            *self = Float3(self.0 * rhs, self.1 * rhs, self.2 * rhs)
        }
    }

    impl Mul<Float3> for f32 {
        type Output = Float3;
        #[inline]
        fn mul(self, rhs: Float3) -> Float3 {
            Float3(self * rhs.0, self * rhs.1, self * rhs.2)
        }
    }

    impl Add for Float3 {
        type Output = Float3;
        #[inline]
        fn add(self, rhs: Float3) -> Float3 {
            Float3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
        }
    }

    impl AddAssign for Float3 {
        #[inline]
        fn add_assign(&mut self, rhs: Float3) {
            *self = Float3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
        }
    }

    impl Sub for Float3 {
        type Output = Float3;
        #[inline]
        fn sub(self, rhs: Float3) -> Float3 {
            Float3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
        }
    }

    impl SubAssign for Float3 {
        #[inline]
        fn sub_assign(&mut self, rhs: Float3) {
            *self = Float3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
        }
    }

    impl Neg for Float3 {
        type Output = Float3;
        #[inline]
        fn neg(self) -> Float3 {
            Float3(-self.0, -self.1, -self.2)
        }
    }
}
