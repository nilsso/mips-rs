#![feature(trait_alias)]
pub mod traits;
pub mod test_utils;

#[macro_export]
macro_rules! is_as_inner {
    ($Self:ty, $Error:ty, $error:path,
        [$( ($self:path, $is:ident, $as:ident, $inner:ident, $Inner:ty, $msg:literal) ),*$(,)*]
    ) => {
        $(
            pub fn $is(this: &Self) -> bool {
                matches!($self, this)
            }

            pub fn $as(&self) -> Option<&Self> {
                <$Self>::$is(self).then_some(self)
            }

            pub fn $inner(&self) -> Result<$Inner, $Error> {
                match self {
                    $self(inner) => Ok(inner),
                    _ => Err($error(format!($msg, self))),
                }
            }
        )*
    };
}

#[macro_export]
macro_rules! impl_from_error {
    ($T:ty, $($E:tt),*$(,)*) => {
        $(
            impl From<$E> for $T {
                fn from(e: $E) -> Self {
                    <$T>::$E(e)
                }
            }
        )*
    }
}

