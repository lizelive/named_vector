#[macro_export]
macro_rules! named_vector {
    ($(#[$enum_meta:meta])* $vis:vis enum $name:ident { $($(#[$variant_meta:meta])* $variant:ident),+ $(,)? }) => {
        $(#[$enum_meta])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        $vis enum $name {
            $($(#[$variant_meta])* $variant),+
        }
        paste::paste! {
            impl $name {
                $(
                    pub fn [<is_ $variant:snake>](self) -> bool {
                        self == Self::$variant
                    }
                )*
                /// A slice containing all variants of the enum.
                pub const VALUES: &'static [Self] = &[
                    $(Self::$variant),+
                ];

                /// Returns a slice with all variants.
                pub fn values() -> &'static [Self] {
                    Self::VALUES
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    named_vector! {
        #[derive(Default)]
        pub enum UwU {
            /// hello world
            #[default]
            A,
            B,
            C,
        }
    }
    #[test]
    fn uwu_values_contains_all_variants() {
        assert_eq!(UwU::values(), &[UwU::A, UwU::B, UwU::C]);
    }

    #[test]
    fn uwu_values_len() {
        assert_eq!(UwU::values().len(), 3);
    }
}
