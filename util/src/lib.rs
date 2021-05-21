#[macro_export]
macro_rules! impl_is_as_inner {
    ($T:ty, $variant:path, $is:ident, $as:ident, $inner:ident, $inner_type:ty) => {
        impl $T {
            #[allow(unused_variables)]
            pub fn $is(this: &Self) -> bool {
                matches!($variant, this)
            }

            pub fn $as(&self) -> Option<&Self> {
                <$T>::$is(self).then_some(self)
            }

            pub fn $inner(&self) -> Option<$inner_type> {
                match self {
                    $variant(inner) => Some(inner.clone()),
                    _ => None,
                }
            }
        }
    };
}

