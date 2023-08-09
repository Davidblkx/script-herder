#[macro_export]
macro_rules! enum_str {
    ($name:ident { $($variant:ident = $value:expr,)* }) => {
        #[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
        pub enum $name {
            $($variant,)*
        }

        impl $name {
            pub fn to_str(&self) -> &'static str {
                match self {
                    $($name::$variant => $value,)*
                }
            }

            pub fn from_str(value: &str) -> Option<$name> {
                match value {
                    $($value => Some($name::$variant),)*
                    _ => None,
                }
            }

            pub fn list() -> Vec<&'static str> {
                vec![$($value,)*]
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.to_str())
            }
        }
    };
}