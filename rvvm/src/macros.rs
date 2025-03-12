#[macro_export]
macro_rules! impl_numeric_enum {
    {
    $valtype:ident, $(#[$inner:meta])* $vis:vis $name:ident [$($field_name: tt = $field_value: expr), + $(,)?]
    }  => {
        #[repr($valtype)]
        $(#[$inner])*
        $vis enum $name{
            $($field_name = $field_value,)*
            Unknown($valtype)
        }
        impl $name{
            pub const fn from_value(bits: $valtype) -> Self {
                match bits {
                    $($field_value => Self::$field_name,)*
                    _ => Self::Unknown(bits)
                }
            }
            pub const fn into_value(self) -> $valtype {
                match self {
                    $(Self::$field_name => $field_value,)*
                    Self::Unknown(x) => x
                }
            }
        }
        impl From<$valtype> for $name {
            fn from(value: $valtype) -> Self{
                Self::from_value(value)
            }
        }
        impl From<$name> for $valtype {
            fn from(value: $name) -> Self{
                value.into_value()
            }
        }
        impl From<usize> for $name {
            fn from(value: usize) -> Self {
                $name::from(value as u8)
            }
        }
        impl From<$name> for usize {
            fn from(value: $name) -> Self {
                $valtype::from(value) as _
            }
        }
    };
}
