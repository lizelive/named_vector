#[macro_export]
macro_rules! named_vector {
    ($(#[$enum_meta:meta])* $vis:vis vec $name:ident { $($(#[$axes_meta:meta])* $axes:ident),+ $(,)? }) => {
        $(#[$enum_meta])*
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
}
