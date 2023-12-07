﻿use crate::ucount;
use std::ops::{Add, Range};

/// 图的基本形式。
#[derive(Clone, Debug)]
pub struct Graph<N, E> {
    /// 节点和边的拓扑结构。
    pub topology: GraphTopo,
    /// 所有节点的信息。
    pub nodes: Vec<N>,
    /// 所有边的信息。
    pub edges: Vec<E>,
}

/// 图拓扑结构。
#[derive(Clone, Default, Debug)]
pub struct GraphTopo {
    /// 全图输入边的数量。
    pub(super) global_inputs_len: ucount,
    /// 全图输出边的数量。
    pub(super) global_outputs_len: ucount,
    /// 节点集合。
    ///
    /// 节点集合是图中节点的一种符合拓扑序的排列。
    pub(super) nodes: Vec<Node>,
    /// 边的目标集合。
    ///
    /// 该集合的排列顺序是由节点集合决定的。
    /// 首先是每个节点的输入边，最后是全图的输出边。
    ///
    /// 即：`connections.len() == nodes.sum(inputs_len) + global_outputs_lenws`。
    pub(super) connections: Vec<OutputEdge>,
}

/// 作为节点输入的边序号。
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct OutputEdge(pub ucount);

/// 用于遍历图拓扑的结构。
pub struct Iter<'a> {
    inner: &'a GraphTopo,
    i: ucount,
    pass_connections: ucount,
    pass_edges: ucount,
}

/// 节点结构。
#[derive(Clone, PartialEq, Eq, Debug)]
pub(super) struct Node {
    pub(super) local_edges_len: ucount,
    pub(super) inputs_len: ucount,
    pub(super) outputs_len: ucount,
}

impl GraphTopo {
    /// 遍历迭代器。
    #[inline]
    pub const fn traverse(&self) -> Iter {
        Iter {
            inner: self,
            i: 0,
            pass_connections: self.global_outputs_len,
            pass_edges: self.global_inputs_len,
        }
    }

    /// 遍历迭代器。
    #[inline]
    pub const fn iter(&self) -> Iter {
        self.traverse()
    }

    /// 节点数量。
    pub fn nodes_len(&self) -> usize {
        self.nodes.len()
    }

    /// 计算图中边的总数。
    pub fn calculate_edges_len(&self) -> usize {
        self.nodes
            .iter()
            .map(|node| node.local_edges_len as usize + node.outputs_len as usize)
            .sum::<usize>()
            .add(self.global_outputs_len as usize)
    }

    /// 全图输入边的数量。
    pub const fn global_inputs_len(&self) -> usize {
        self.global_inputs_len as _
    }

    /// 全图输出边的数量。
    pub const fn global_outputs_len(&self) -> usize {
        self.global_outputs_len as _
    }

    /// 全图输入边集。
    pub const fn global_inputs(&self) -> Range<ucount> {
        0..self.global_inputs_len
    }

    /// 全图输出边集。
    pub fn global_outputs(&self) -> &[OutputEdge] {
        &self.connections[..self.global_outputs_len as _]
    }
}

impl<'a> IntoIterator for &'a GraphTopo {
    type Item = (usize, &'a [OutputEdge], Range<usize>);

    type IntoIter = Iter<'a>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.traverse()
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = (usize, &'a [OutputEdge], Range<usize>);

    fn next(&mut self) -> Option<Self::Item> {
        let i = self.i as usize;
        let node = self.inner.nodes.get(i)?;
        self.i += 1;

        let first_input = self.pass_connections as usize;
        self.pass_connections += node.inputs_len;
        self.pass_edges += node.local_edges_len;
        let first_edge = self.pass_edges as usize;
        self.pass_edges += node.outputs_len;

        Some((
            i,
            &self.inner.connections[first_input..self.pass_connections as usize],
            first_edge..self.pass_edges as usize,
        ))
    }
}
