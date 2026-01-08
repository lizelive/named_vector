#[macro_export]
macro_rules! named_vector {
    ($(#[$enum_meta:meta])* $vis:vis vec $name:ident { $($(#[$axes_meta:meta])* $axes:ident),+ $(,)? }) => {
        $(#[$enum_meta])*
        #[derive(Default, Debug, Clone, Copy, PartialEq)]
        $vis struct $name<T=f32>{
            $($(#[$axes_meta])* pub $axes : T),+
        }
        paste::paste! {
            $(#[$enum_meta])*
            #[repr(u8)]
            #[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
            $vis enum [<$name Axes>]  {
                #[default]
                $($(#[$axes_meta])* [<$axes:camel>],)+
                Count
            }

            impl [<$name Axes>] {
                $(
                    pub fn [<is_ $axes>](self) -> bool {
                        self == Self::[<$axes:camel>]
                    }
                )*

                pub const COUNT: usize = Self::Count as usize;
                /// A slice containing all axess of the enum.
                pub const VALUES: [Self; Self::COUNT] = [
                    $(Self::[<$axes:camel>]),+
                ];

                /// Returns a slice with all axess.
                pub const fn values() -> &'static [Self] {
                    & Self::VALUES
                }
            }
        }
        // ---- arithmetic impls ----
        impl<T> ::core::ops::Add for $name<T>
        where T: ::core::ops::Add<Output = T>
        {
            type Output = Self;
            fn add(self, rhs: Self) -> Self::Output {
                Self { $($axes: self.$axes + rhs.$axes),+ }
            }
        }

        impl<T> ::core::ops::Sub for $name<T>
        where T: ::core::ops::Sub<Output = T>
        {
            type Output = Self;
            fn sub(self, rhs: Self) -> Self::Output {
                Self { $($axes: self.$axes - rhs.$axes),+ }
            }
        }

        impl<T> ::core::ops::Mul for $name<T>
        where T: ::core::ops::Mul<Output = T>
        {
            type Output = Self;
            fn mul(self, rhs: Self) -> Self::Output {
                Self { $($axes: self.$axes * rhs.$axes),+ }
            }
        }

        impl<T> ::core::ops::Div for $name<T>
        where T: ::core::ops::Div<Output = T>
        {
            type Output = Self;
            fn div(self, rhs: Self) -> Self::Output {
                Self { $($axes: self.$axes / rhs.$axes),+ }
            }
        }

        impl<T> ::core::ops::Neg for $name<T>
        where T: ::core::ops::Neg<Output = T>
        {
            type Output = Self;
            fn neg(self) -> Self::Output {
                Self { $($axes: -self.$axes),+ }
            }
        }

        // ---- num_traits implementations ----
        impl<T> ::num_traits::Zero for $name<T>
        where T: ::num_traits::Zero + Copy + ::core::ops::Add<Output = T>
        {
            fn zero() -> Self {
                Self { $($axes: ::num_traits::Zero::zero()),+ }
            }

            fn is_zero(&self) -> bool {
                $( ::num_traits::Zero::is_zero(&self.$axes) )&&+
            }
        }

        impl<T> ::num_traits::One for $name<T>
        where T: ::num_traits::One + Copy + ::core::ops::Mul<Output = T>
        {
            fn one() -> Self {
                Self { $($axes: ::num_traits::One::one()),+ }
            }
        }

        impl<T> ::num_traits::Bounded for $name<T>
        where T: ::num_traits::Bounded + Copy
        {
            fn min_value() -> Self {
                Self { $($axes: ::num_traits::Bounded::min_value()),+ }
            }

            fn max_value() -> Self {
                Self { $($axes: ::num_traits::Bounded::max_value()),+ }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    named_vector! {
        pub vec UwU {
            /// hello world
            x,
            y,
            z,
        }
    }
    #[test]
    fn uwu_values_contains_all_axess() {
        assert_eq!(UwUAxes::values(), &[UwUAxes::X, UwUAxes::Y, UwUAxes::Z]);
    }

    #[test]
    fn uwu_values_len() {
        assert_eq!(UwUAxes::values().len(), 3);
    }
    #[test]
    fn uwu_count_const() {
        assert_eq!(UwUAxes::COUNT, 3);
        assert_eq!(UwUAxes::values().len(), UwUAxes::COUNT);
    }

    #[test]
    fn uwu_is_methods() {
        // X axes
        assert!(UwUAxes::X.is_x());
        assert!(!UwUAxes::X.is_y());
        assert!(!UwUAxes::X.is_z());

        // Y axes
        assert!(!UwUAxes::Y.is_x());
        assert!(UwUAxes::Y.is_y());
        assert!(!UwUAxes::Y.is_z());

        // Z axes
        assert!(!UwUAxes::Z.is_x());
        assert!(!UwUAxes::Z.is_y());
        assert!(UwUAxes::Z.is_z());
    }

    #[test]
    fn uwu_zero_and_one() {
        use ::num_traits::{One, Zero};
        assert_eq!(
            UwU::<f32>::zero(),
            UwU {
                x: 0.0f32,
                y: 0.0f32,
                z: 0.0f32
            }
        );
        assert!(UwU::<f32>::zero().is_zero());
        assert_eq!(
            UwU::<f32>::one(),
            UwU {
                x: 1.0f32,
                y: 1.0f32,
                z: 1.0f32
            }
        );
    }

    #[test]
    fn uwu_bounded() {
        let min = <UwU as ::num_traits::Bounded>::min_value();
        let max = <UwU as ::num_traits::Bounded>::max_value();
        assert!(min.x <= max.x && min.y <= max.y && min.z <= max.z);
    }

    #[test]
    fn uwu_arithmetic() {
        let a = UwU::<f32> {
            x: 1.0f32,
            y: 2.0f32,
            z: 3.0f32,
        };
        let b = UwU::<f32> {
            x: 4.0f32,
            y: 5.0f32,
            z: 6.0f32,
        };
        assert_eq!(
            a + b,
            UwU::<f32> {
                x: 5.0f32,
                y: 7.0f32,
                z: 9.0f32
            }
        );
        assert_eq!(
            b - a,
            UwU::<f32> {
                x: 3.0f32,
                y: 3.0f32,
                z: 3.0f32
            }
        );
        assert_eq!(
            a * UwU::<f32> {
                x: 2.0f32,
                y: 2.0f32,
                z: 2.0f32
            },
            UwU::<f32> {
                x: 2.0f32,
                y: 4.0f32,
                z: 6.0f32
            }
        );
        assert_eq!(
            -a,
            UwU::<f32> {
                x: -1.0f32,
                y: -2.0f32,
                z: -3.0f32
            }
        );
    }
}
