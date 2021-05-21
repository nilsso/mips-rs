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
    // Device IO
    (Bdns, f_bdns),
    (Bdnsal, f_bdnsal),
    (Bdse, f_bdse),
    (Bdseal, f_bdseal),
    (Brdns, f_brdns),
    (Brdse, f_brdse),
    (L, f_l),
    (Lb, f_lb),
    (Lr, f_lr),
    (Ls, f_ls),
    (S, f_s),
    (Sb, f_sb),
    // Flow Control, Branches and Jumps
    (Bap, f_bap),
    (Bapal, f_bapal),
    (Bapz, f_bapz),
    (Bapzal, f_bapzal),
    (Beq, f_beq),
    (Beqal, f_beqal),
    (Beqz, f_beqz),
    (Beqzal, f_beqzal),
    (Bge, f_bge),
    (Bgeal, f_bgeal),
    (Bgez, f_bgez),
    (Bgezal, f_bgezal),
    (Bgt, f_bgt),
    (Bgtal, f_bgtal),
    (Bgtz, f_bgtz),
    (Bgtzal, f_bgtzal),
    (Ble, f_ble),
    (Bleal, f_bleal),
    (Blez, f_blez),
    (Blezal, f_blezal),
    (Blt, f_blt),
    (Bltal, f_bltal),
    (Bltz, f_bltz),
    (Bltzal, f_bltzal),
    (Bna, f_bna),
    (Bnaal, f_bnaal),
    (Bnaz, f_bnaz),
    (Bnazal, f_bnazal),
    (Bne, f_bne),
    (Bneal, f_bneal),
    (Bnez, f_bnez),
    (Bnezal, f_bnezal),
    (Brap, f_brap),
    (Brapz, f_brapz),
    (Breq, f_breq),
    (Breqz, f_breqz),
    (Brge, f_brge),
    (Brgez, f_brgez),
    (Brgt, f_brgt),
    (Brgtz, f_brgtz),
    (Brle, f_brle),
    (Brlez, f_brlez),
    (Brlt, f_brlt),
    (Brltz, f_brltz),
    (Brna, f_brna),
    (Brnaz, f_brnaz),
    (Brne, f_brne),
    (Brnez, f_brnez),
    (J, f_j),
    (Jal, f_jal),
    (Jr, f_jr),
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

