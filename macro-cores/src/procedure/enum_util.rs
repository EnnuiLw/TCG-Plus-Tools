#[macro_export]
macro_rules! impl_value {
    (
        $(
            $(#[$outer:meta])*
            $vis:vis enum $name:ident { 
                $(
                    $(#[doc = $doc:literal])*
                    $(#[cfg $($cfg:tt)*])?
                    $(#[default $($dummy:tt)?])?
                    $variant:ident => $val:expr
                ),* $(,)?
            }
        )*
    ) => {
        $(
            $(#[$outer])*
            $vis enum $name { 
                $(
                    $(#[doc = $doc:literal])*
                    $(#[cfg $($cfg:tt)*])?
                    $(#[default $($dummy:tt)?])?
                    $variant
                ),*
            }

            impl $name {
                pub fn value(&self) -> &'static str {
                    match self {
                        $(Self::$variant => $val),*
                    }
                }

                pub fn from_value(val: &str) -> Option<Self> {
                    match val {
                        $($val => Some(Self::$variant)),+,
                        _ => None
                    }
                }

                pub fn variants() -> &'static [&'static str] {
                    &[$($val),+]
                }
            }
        )*  
    };
}