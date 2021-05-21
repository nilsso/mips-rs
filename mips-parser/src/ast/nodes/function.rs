use crate::Rule;

macro_rules! functions {
    {$(($enum_variant:ident, $rule:ident)),*$(,)*} => {
        /// Function node.
        ///
        /// Contains variants for all functions available in Stationeers MIPS.
        #[derive(PartialEq, Debug)]
        pub enum Function {
            $( $enum_variant, )*
            /// TODO: Remove once all functions have been added.
            Unknown,
        }

        impl Function {
            pub fn new(rule: Rule) -> Self {
                match rule {
                    $( Rule::$rule => Function::$enum_variant, )*
                    _ => Function::Unknown,
                }
            }
        }
    };
}

functions!{
    // Variable Selection
    (Sap, f_sap),
    (Sapz, f_sapz),
    (Sdns, f_sdns),
    (Sdse, f_sdse),
    (Select, f_select),
    (Seq, f_seq),
    (Seqz, f_seqz),
    (Sge, f_sge),
    (Sgez, f_sgez),
    (Sgt, f_sgt),
    (Sgtz, f_sgtz),
    (Sle, f_sle),
    (Slez, f_slez),
    (Slt, f_slt),
    (Sltz, f_sltz),
    (Sna, f_sna),
    (Snaz, f_snaz),
    (Sne, f_sne),
    (Snez, f_snez),
    // Mathematical Operations
    (Abs, f_abs),
    (Acos, f_acos),
    (Add, f_add),
    (Asin, f_asin),
    (Atan, f_atan),
    (Ceil, f_ceil),
    (Cos, f_cos),
    (Div, f_div),
    (Exp, f_exp),
    (Floor, f_floor),
    (Log, f_log),
    (Max, f_max),
    (Min, f_min),
    (Mod, f_mod),
    (Mul, f_mul),
    (Rand, f_rand),
    (Round, f_round),
    (Sin, f_sin),
    (Sqrt, f_sqrt),
    (Sub, f_sub),
    (Tan, f_tan),
    (Trunc, f_trunc),
    // Logic
    (And, f_and),
    (Nor, f_nor),
    (Or, f_or),
    (Xor, f_xor),
    // Stack
    ( Peek,   f_peek   ),
    ( Pop,    f_pop    ),
    ( Push,   f_push   ),
    // Misc
    ( Alias,  f_alias  ),
    ( Define, f_define ),
    ( Hcf,    f_hcf    ),
    ( Move,   f_move   ),
    ( Sleep,  f_sleep  ),
    ( Yield,  f_yield  ),
}

