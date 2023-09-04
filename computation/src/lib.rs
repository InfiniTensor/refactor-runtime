//! # Computation

// #![deny(warnings)]
#![deny(missing_docs)]
#![allow(implied_bounds_entailment)]

mod operators;
mod tensor;

use std::rc::Rc;

pub use operators::{OpType, Operator};
pub use tensor::{DimExpr, Shape, Tensor};

extern crate common;
extern crate itertools;
extern crate smallvec;

/// 节点是一个算子，它可以有多个输入和多个输出。
///
/// 作为图表示的一种优化，具有相同信息的节点可以共享节点信息对象。
pub type Node = Rc<dyn Operator>;

/// 在优化过程中，边可能在不同子图间共享。共享的只是信息，拓扑结构是不会共享的。
pub type Edge = Rc<Tensor>;
