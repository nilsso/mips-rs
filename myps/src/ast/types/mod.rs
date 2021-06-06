// mod var;
// pub use var::Var;
mod int;
pub use int::Int;

mod num;
pub use num::Num;

mod dev;
pub use dev::Dev;

pub trait Var {
    fn as_var(&self) -> Option<&String>;

    fn is_var(&self) -> bool {
        self.as_var().is_some()
    }

    fn is_lit(&self) -> bool {
        !self.is_var()
    }
}

macro_rules! impl_var {
    ($(($T:ty, $V:path)),*$(,)*) => {
        $(
            impl Var for $T {
                fn as_var(&self) -> Option<&String> {
                    if let $V(v) = self { Some(v) } else { None }
                }
            }
        )*
    };
}

impl_var!((Int, Int::Var), (Num, Num::Var), (Dev, Dev::Var));
