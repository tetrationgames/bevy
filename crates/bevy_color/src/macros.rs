//! Macros shared by the color types.

/// Generates the `const` counterparts of [`Alpha`](crate::Alpha), [`Mix`](crate::Mix), and
/// (optionally) [`Hue`](crate::Hue) on a color type, and implements those traits by
/// delegating to them.
///
/// List every channel except `alpha`, then append `hue` when the type has a hue channel:
///
/// ```text
/// impl_const_color_ops!(Srgba, [red, green, blue]);
/// impl_const_color_ops!(Hsla, [saturation, lightness], hue);
/// ```
macro_rules! impl_const_color_ops {
    ($ty: ident, [$($element: ident),+] $(, $hue: ident)?) => {
        impl $ty {
            /// Return a new version of this color with the given alpha value.
            ///
            /// This is the `const` counterpart of
            /// [`Alpha::with_alpha`](crate::Alpha::with_alpha).
            #[inline]
            pub const fn with_alpha(&self, alpha: f32) -> Self {
                Self { alpha, ..*self }
            }

            /// Return the alpha component of this color.
            ///
            /// This is the `const` counterpart of [`Alpha::alpha`](crate::Alpha::alpha).
            #[inline]
            pub const fn alpha(&self) -> f32 {
                self.alpha
            }

            /// Return a new version of this color with the hue channel set to the given value.
            ///
            /// This is the `const` counterpart of
            /// [`Hue::with_hue`](crate::Hue::with_hue).
            $(
                #[inline]
                pub const fn with_hue(&self, hue: f32) -> Self {
                    Self { $hue: hue, ..*self }
                }
            )?

            /// Return a new version of this color with the hue channel rotated by the given
            /// degrees.
            ///
            /// This is the `const` counterpart of
            /// [`Hue::rotate_hue`](crate::Hue::rotate_hue).
            $(
                #[inline]
                pub const fn rotate_hue(&self, degrees: f32) -> Self {
                    Self {
                        $hue: $crate::color_ops::rotate_hue(self.$hue, degrees),
                        ..*self
                    }
                }
            )?

            /// Linearly interpolate between this and another color, by factor.
            /// Factor should be between 0.0 and 1.0.
            ///
            /// This is the `const` counterpart of [`Mix::mix`](crate::Mix::mix).
            #[inline]
            pub const fn mix(&self, other: &Self, factor: f32) -> Self {
                let n_factor = 1.0 - factor;
                Self {
                    $($element: self.$element * n_factor + other.$element * factor,)+
                    $($hue: $crate::color_ops::lerp_hue(self.$hue, other.$hue, factor),)?
                    alpha: self.alpha * n_factor + other.alpha * factor,
                }
            }
        }

        impl $crate::Alpha for $ty {
            #[inline]
            fn with_alpha(&self, alpha: f32) -> Self {
                Self::with_alpha(self, alpha)
            }

            #[inline]
            fn alpha(&self) -> f32 {
                Self::alpha(self)
            }

            #[inline]
            fn set_alpha(&mut self, alpha: f32) {
                self.alpha = alpha;
            }
        }

        impl $crate::Mix for $ty {
            #[inline]
            fn mix(&self, other: &Self, factor: f32) -> Self {
                Self::mix(self, other, factor)
            }
        }

        $(
            impl $crate::Hue for $ty {
                #[inline]
                fn with_hue(&self, hue: f32) -> Self {
                    Self::with_hue(self, hue)
                }

                #[inline]
                fn hue(&self) -> f32 {
                    self.$hue
                }

                #[inline]
                fn set_hue(&mut self, hue: f32) {
                    self.$hue = hue;
                }
            }
        )?
    };
}

macro_rules! impl_componentwise_vector_space {
    ($ty: ident, [$($element: ident),+]) => {
        impl core::ops::Add<Self> for $ty {
            type Output = Self;

            fn add(self, rhs: Self) -> Self::Output {
                Self::Output {
                    $($element: self.$element + rhs.$element,)+
                }
            }
        }

        impl core::ops::AddAssign<Self> for $ty {
            fn add_assign(&mut self, rhs: Self) {
                *self = *self + rhs;
            }
        }

        impl core::ops::Neg for $ty {
            type Output = Self;

            fn neg(self) -> Self::Output {
                Self::Output {
                    $($element: -self.$element,)+
                }
            }
        }

        impl core::ops::Sub<Self> for $ty {
            type Output = Self;

            fn sub(self, rhs: Self) -> Self::Output {
                Self::Output {
                    $($element: self.$element - rhs.$element,)+
                }
            }
        }

        impl core::ops::SubAssign<Self> for $ty {
            fn sub_assign(&mut self, rhs: Self) {
                *self = *self - rhs;
            }
        }

        impl core::ops::Mul<f32> for $ty {
            type Output = Self;

            fn mul(self, rhs: f32) -> Self::Output {
                Self::Output {
                    $($element: self.$element * rhs,)+
                }
            }
        }

        impl core::ops::Mul<$ty> for f32 {
            type Output = $ty;

            fn mul(self, rhs: $ty) -> Self::Output {
                Self::Output {
                    $($element: self * rhs.$element,)+
                }
            }
        }

        impl core::ops::MulAssign<f32> for $ty {
            fn mul_assign(&mut self, rhs: f32) {
                *self = *self * rhs;
            }
        }

        impl core::ops::Div<f32> for $ty {
            type Output = Self;

            fn div(self, rhs: f32) -> Self::Output {
                Self::Output {
                    $($element: self.$element / rhs,)+
                }
            }
        }

        impl core::ops::DivAssign<f32> for $ty {
            fn div_assign(&mut self, rhs: f32) {
                *self = *self / rhs;
            }
        }

        impl bevy_math::VectorSpace for $ty {
            type Scalar = f32;
            const ZERO: Self = Self {
                $($element: 0.0,)+
            };
        }

        impl bevy_math::StableInterpolate for $ty {
            fn interpolate_stable(&self, other: &Self, t: f32) -> Self {
                bevy_math::VectorSpace::lerp(*self, *other, t)
            }
        }
    };
}

pub(crate) use impl_componentwise_vector_space;
pub(crate) use impl_const_color_ops;
