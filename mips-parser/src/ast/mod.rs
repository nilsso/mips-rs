//! Stationeers MIPS abstract syntax tree (AST) definition.
//
// <h1 style="color:red;">Update me</h1>
//
// ```[skip]
//            line-number
//                │
// Program(Vec<(usize, Expr)>)
// ┌────────────────────┘
// ├ Expr::Label(String)
// └ Expr::Function(Function, Vec<Arg>)
// ┌───────────────────────────────┘
// ├── Arg::Memory(Memory)
// │                 ├ Memory::MemBase(usize)
// │                 ├ Memory::MemAlias(String)
// │                 └ Memory::Mem(Box<Memory>)
// ├── Arg::Device(Device)
// │                 ├ Device::DevBase(usize)
// │                 ├ Device::DevAlias(String)
// │                 └ Device::Dev(Box<Memory>)
// ├─── Arg::Value(Value)
// │                 ├ Value::ValLit(f32)
// │                 └ Value::ValMem(Memory)
// └─── Arg::Token(String)
// ```
// An AST tree can be used in tandem with the state simulator module to simulate a
// functioning Stationeers IC10 chip.
//
// It is constructed out of the following nodes:
//
// ### Program
// A [`Program`] node represents an entire MIPS program
// and contains a vector of [expressions](#expressions) and their line numbers.
//
// ### Expressions
// Expressions in MIPS are either a label or a function call;
// an [`Expr`] node represents one of these.
//
// * [`Expr::ExprLabel`] - simply contains the `String` of the label
// * [`Expr::ExprFunc`] - contains the [function identifier](#functions) 
//     and a vector of [arguments](#arguments)
//
// ### Functions
// A [`Function`] node refers simply to a MIPS function.
// It enumerates all the functions available in Stationeers MIPS.
//
// ### Arguments
// Arguments to MIPS functions can be memory registers, device registers,
// numeric values or simple string tokens;
// an [`Arg`] node represents one of these.
//
// * [`Arg::ArgMem`] - contains a [memory register](#memory) node
// * [`Arg::ArgDev`] - contains a [device register](#device) node
// * [`Arg::ArgVal`] - contains a [numeric value](#value) node
// * [`Arg::ArgToken`] - contains a string token.
//
// Important to note is that arguments being valid for a particular function is enforced
// not by the AST tree construction but by the Pest parser.
//
// ### Memory
// A memory register node ([`Memory`]) refers to a value stored in memory
// and can be direct, an alias, or indirect
//
// * [`Memory::MemBase`] - a memory register indexed directly by a numeric value
// * [`Memory::MemAlias`] - an (unvalidated) alias to a memory register
// * [`Memory::Mem`] - a memory register indexed by a value in memory (indrectly)
//
// Examples:
//
// <table>
//     <tr><th>MIPS</th><th>AST Analog</th><th>Description</th></tr>
//     <tr>
//         <td><code>"r0"</code></td>
//         <td><code>Memory::MemBase</code></td>
//         <td>
//             Refers to the memory register indexed by zero; that is, the first (0th)
//             memory register.
//         </td>
//     </tr>
//     <tr>
//         <td><code>"x"</code></td>
//         <td><code>Memory::MemAlias</code></td>
//         <td>
//             Refers to a memory register aliased by the string <code>"x"</code>
//             (if such an alias was defined).
//         </td>
//     </tr>
//     <tr>
//         <td><code>"rr1"</code></td>
//         <td><code>Memory::Mem</code></td>
//         <td>
//             Refers to the memory register indexed by the value at
//             memory register <code>"r1"</code>.</br>
//             Suppose <code>"r1"</code> stores 5.
//             Then <code>"rr1"</code> reduces to <code>"r5"</code>.
//         </td>
//     </tr>
// </table>
//
// Note that a `Memory::MemAlias` cannot be used within a `Memory::Mem` as indirection
// via aliases is not part of the MIPS syntax.
//
// ### Device
// A device register node ([`Device`]) can also be direct, an alias, or indirect, but refer to a
// connected input-output (IO) device.
//
// * [`Device::DevBase`] - a device register indexed directly by a numeric value
// * [`Device::DevAlias`] - an (unvalidated) alias to a device register
// * [`Device::Dev`] - a device register indexed by a value in memory (indrectly)
//
// (See the above examples for `Memory`.)
//
// Note that a device register cannot be index indirectly via device registers since device
// registers do not store a numeric value but instead a connection to an IO device.
// For example:
//
// * `"dr2"` is fine, and refers to
//     the device indexed by the value stored at memory register `"r2"`, but
// * `"dd2"` means nothing, because `"d2"` refers to the 3rd device and not a number.
//
// ### Value
// A value argument in MIPS can be a literal numeric value or one stored in memory;
// a [`Value`] node represents one of these.
//
// * [`Value::ValLit`] - a literal floating-point value
// * [`Value::ValMem`] - a value stored in memory indexed via a [`Memory`] register
//
//
// [`Program`]: nodes::Program
// [`Expr`]: nodes::Expr
// [`Expr::ExprLabel`]: nodes::Expr::ExprLabel
// [`Expr::ExprFunc`]: nodes::Expr::ExprFunc
// [`Function`]: nodes::Function
// [`Arg`]: nodes::Arg
// [`Arg::ArgMem`]: nodes::Arg::ArgMem
// [`Arg::ArgDev`]: nodes::Arg::ArgDev
// [`Arg::ArgVal`]: nodes::Arg::ArgVal
// [`Arg::ArgToken`]: nodes::Arg::ArgToken
// [`Memory`]: nodes::Memory
// [`Memory::MemBase`]: nodes::Memory::MemBase
// [`Memory::MemAlias`]: nodes::Memory::MemAlias
// [`Memory::Mem`]: nodes::Memory::Mem
// [`Device`]: nodes::Device
// [`Device::DevBase`]: nodes::Device::DevBase
// [`Device::DevAlias`]: nodes::Device::DevAlias
// [`Device::Dev`]: nodes::Device::Dev
// [`Value`]: nodes::Value
// [`Value::ValLit`]: nodes::Value::ValLit
// [`Value::ValMem`]: nodes::Value::ValMem

pub mod nodes;

pub mod all_node_variants {
    pub use crate::ast::nodes::{
        Arg::*,
        Dev::*,
        Func::*,
        Mem::*,
        Val::*,
    };
}

// TODO: Not sure if I need a custom error type here. Parser might do well enough.
// use thiserror::Error;

// #[derive(Debug)]
// pub enum ASTError {
//     // Program
//     // Expr
//     // Function
//     // Arg
//     // Memory
//     // Device
//     // Value
// }

// #[derive(Clone, PartialEq, Debug, Error)]
// pub enum StateError {
//     #[error("Cannot find value a from register of type {0}")]
//     InvalidRegister(Register),
//     #[error("Invalid alias {0}")]
//     InvalidAlias(String),
//     #[error("Invalid memory index {0}")]
//     InvalidMemoryIndex(usize),
// }

