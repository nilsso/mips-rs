use std::convert::TryFrom;

use mips_parser::prelude::{AstError, Arg};
use crate::state::{AliasKind, ICState, ICStateError, ICStateResult};

/// Argument reducer helper type.
#[derive(Debug)]
pub struct ArgReducer<'args, 'dk, const STACKSIZE: usize> {
    state: &'dk ICState<'dk, STACKSIZE>,
    args: &'args Vec<Arg>,
}

impl<'args, 'dk, const STACKSIZE: usize> ArgReducer<'args, 'dk, STACKSIZE> {
    pub fn new(state: &'dk ICState<'dk, STACKSIZE>, args: &'args Vec<Arg>) -> Self {
        Self { state, args }
    }
}

/// Trait for attempting to reduce an argument into a type.
pub trait TryReduce<'args, 'dk, const STACKSIZE: usize>
where
    Self: Sized
{
    fn try_reduce(arg: &Arg, reducer: &ArgReducer<'args, 'dk, STACKSIZE>) -> ICStateResult<Self>;
}

// Implement TryFrom for tuple of below helper types from ArgReducer
macro_rules! impl_tuple_try_from_reducer {
    ($( [ ($( $L:tt ),*), ($( $l:tt ),*), ($( $n:literal ),*), $N:literal ] ),*$(,)*) => {
        $(
            impl<'args, 'dk, $($L),*, const STACKSIZE: usize> TryFrom<&ArgReducer<'args, 'dk, STACKSIZE>> for ($($L),*,)
            where
                $(
                    $L: TryReduce<'args, 'dk, STACKSIZE>, 
                )*
            {
                type Error = ICStateError;

                fn try_from(reducer: &ArgReducer<'args, 'dk, STACKSIZE>) -> ICStateResult<($($L),*,)> {
                    let args = reducer.args;
                    if args.len() == $N {
                        $( let $l = <$L>::try_reduce(&args[$n], &reducer)?;)*
                        Ok(($($l),*,))
                    } else {
                        let e = format!("Expected {}, found {}", $N, args.len());
                        Err(ICStateError::WrongNumberOfArgs(e))
                    }
                }
            }
        )*
    };
}

impl_tuple_try_from_reducer!(
    [(A),             (a),             (0),             1],
    [(A, B),          (a, b),          (0, 1),          2],
    [(A, B, C),       (a, b, c),       (0, 1, 2),       3],
    [(A, B, C, D),    (a, b, c, d),    (0, 1, 2, 3),    4],
    [(A, B, C, D, E), (a, b, c, d, e), (0, 1, 2, 3, 4), 5]
);

// ================================================================================================
// Memory reducer helper type
// ================================================================================================

pub struct M(pub usize);

impl<'args, 'dk, const STACKSIZE: usize> TryReduce<'args, 'dk, STACKSIZE> for M {
    fn try_reduce(arg: &Arg, reducer: &ArgReducer<'args, 'dk, STACKSIZE>) -> ICStateResult<M> {
        let m = arg.mem()?;
        Ok(M(reducer.state.mem_reduce(m)?))
    }
}

// ================================================================================================
// Device reducer helper type
// ================================================================================================

pub struct D(pub usize);

impl<'args, 'dk, const STACKSIZE: usize> TryReduce<'args, 'dk, STACKSIZE> for D {
    fn try_reduce(arg: &Arg, reducer: &ArgReducer<'args, 'dk, STACKSIZE>) -> ICStateResult<D> {
        let d = arg.dev()?;
        Ok(D(reducer.state.dev_reduce(d)?))
    }
}

// ================================================================================================
// Register reducer helper type
// ================================================================================================

pub struct R(pub AliasKind);

impl<'args, 'dk, const STACKSIZE: usize> TryReduce<'args, 'dk, STACKSIZE> for R {
    fn try_reduce(arg: &Arg, reducer: &ArgReducer<'args, 'dk, STACKSIZE>) -> ICStateResult<R> {
        match arg {
            Arg::ArgMem(m) => {
                let i = reducer.state.mem_reduce(m)?;
                Ok(R(AliasKind::MemId(i as usize)))
            }
            Arg::ArgDev(d) => {
                let i = reducer.state.dev_reduce(d)?;
                Ok(R(AliasKind::DevId(i as usize)))
            }
            _ => Err(AstError::WrongArg(format!("Expected ArgMem or ArgDev, found {}", arg)))?,
        }
    }
}

// ================================================================================================
// Value reducer helper type
// ================================================================================================

pub struct V(pub f64);

impl<'args, 'dk, const STACKSIZE: usize> TryReduce<'args, 'dk, STACKSIZE> for V {
    fn try_reduce(arg: &Arg, reducer: &ArgReducer<'args, 'dk, STACKSIZE>) -> ICStateResult<V> {
        let v = arg.val()?;
        Ok(V(reducer.state.val_reduce(v)?))
    }
}

// ================================================================================================
// Token reducer helper type
// ================================================================================================

pub struct T(pub String);

impl<'args, 'dk, const STACKSIZE: usize> TryReduce<'args, 'dk, STACKSIZE> for T {
    fn try_reduce(arg: &Arg, _reducer: &ArgReducer<'args, 'dk, STACKSIZE>) -> ICStateResult<T> {
        Ok(T(arg.token().cloned()?))
    }
}

#[test]
fn test_reducer() {
    use mips_parser::prelude::*;

    let state = ICState::default()
        .with_mem(0, 9)
        .with_mem(9, 3)
        .with_mem(3, 6);
    let args = vec![
        Arg::ArgVal(Val::ValMem(Mem::MemLit(0, 0))), // r0 -> 9
        Arg::ArgVal(Val::ValMem(Mem::MemLit(0, 1))), // rr0 -> r9 -> 3
        Arg::ArgVal(Val::ValMem(Mem::MemLit(0, 2))), // rrr0 -> rr9 -> r3 -> 6
    ];
    let reducer = ArgReducer {
        state: &state,
        args: &args,
    };

    let _a: (V, D, D);

    let (V(a), V(b), V(c)) = <(V, V, V)>::try_from(&reducer).unwrap();

    assert_eq!(a, 9.0);
    assert_eq!(b, 3.0);
    assert_eq!(c, 6.0);
}
