use std::{
    collections::HashMap,
    convert::AsRef,
    fmt
};
use crate::{
    error::{AgcResult, AgcError, AgcErrorKind},
    traits::{AgcHashable, AgcNumberLike}
};

/// The type of edge.
/// 
/// The edge can go from left to right, right to left or be bidirectional.
/// 
/// This enum is used in `Edge`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EdgeKind {
    ToRight,
    ToLeft,
    Bidirectional
}

impl fmt::Display for EdgeKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// An edge in a graph, with a `left` node, a `right` node and a cost to
/// traverse the 2 nodes in a certain direction specified by
/// `edge_kind: EdgeKind
#[derive(Clone, PartialEq, Eq)]
pub struct Edge<N, C>
where
    N: AgcHashable + Clone,
    C: AgcNumberLike
{
    pub left: N,
    pub right: N,
    pub cost: C,
    pub edge_kind: EdgeKind
}

impl<N, C> Edge<N, C>
where
    N: AgcHashable + Clone,
    C: AgcNumberLike
{
    /// Create a new `Edge`.
    /// 
    /// # Example
    /// ```
    ///     use algocol::graph::Edge;
    ///     Edge::new(0, 1, 5);
    /// ```
    /// 
    /// # Panics
    /// If `left` and `right` are the same, this function will panic because
    /// an edge that starts and end in the same node does not exist.
    /// 
    /// ```ignore
    ///     use algocol::graph::Edge;
    ///     Edge::new(0, 1, 5); // Does not panic
    ///     Edge::new(0, 0, 5); // Panics!
    /// ```
    pub fn new(left: N, right: N, cost: C, edge_kind: EdgeKind) -> Self {
        Self::try_new(left, right, cost, edge_kind).unwrap()
    }

    /// Attempt to create a new `Edge`.
    /// 
    /// If `left == right`, `Err` is returned, otherwise `Ok(Edge)` is
    /// returned.
    /// 
    /// # Example
    /// ```
    ///     use algocol::graph::Edge;
    ///     assert!(matches!(Edge::try_new(0, 1, 5), Ok(_)));
    ///     assert!(matches!(Edge::try_new(0, 0, 5), Err(_)));
    /// ```
    pub fn try_new(
        left: N,
        right: N,
        cost: C,
        edge_kind: EdgeKind
    ) -> AgcResult<Self> {
        if left == right {
            Err(AgcError::new(
                AgcErrorKind::SameNode,
                "left cannot be the same as right."
            ))
        } else {
            Ok(Self {left, right, cost, edge_kind})
        }
    }
}

/// An `AdjacencyMatrix` maps each node to all the adjacent nodes in connects
/// to with the cost to get there. This is done with the field called `matrix`.
/// `matrix` is annotated as `HashMap<K, HashMap<K, V>>`. The top-level
/// `HashMap` uses the origin node as the key, and all its adjacent nodes
/// as the value stored in a `HashMap<K, V>`. The bottom-level `HashMap`
/// uses the destination node as its key and the cost to get there as the
/// value. Hence if you want to iterate over the adjacent nodes from any
/// arbitrary node `a`, you can do this:
/// 
/// ```
///     use algocol::graph::AdjacencyMatrix;
///     let mut matrix = AdjacencyMatrix::<i32, i32>::new();
///     if let Some(a) = matrix.get_adjacent(&0) {
///         for (b, cost) in a.iter() {
///             println!("Cost to get from {} to {}: {}", a, b, cost);
///         }
///     }
/// ```
/// 
/// If you need to access the cost of an edge more directly, you can use
/// `self.get_edge` or `self.get_mut_edge`.
#[derive(Clone)]
pub struct AdjacencyMatrix<K, V>
where
    K: AgcHashable + Clone,
    V: AgcNumberLike
{
    matrix: HashMap<K, HashMap<K, V>>
}

impl<K, V> AdjacencyMatrix<K, V>
where
    K: AgcHashable + Clone,
    V: AgcNumberLike
{
    /// Create a new `AdjacencyMatrix`.
    pub fn new() -> Self {
        Self {matrix: HashMap::new()}
    }

    /// Create a new `AdjacencyMatrix` with nodes pre-registered.
    pub fn with_nodes<T>(nodes: &T) -> Self
    where
        T: AsRef<[K]>
    {
        let mut matrix = Self::new();
        for node in nodes.as_ref() {
           matrix.register_node(node);
        }
        matrix
    }

    /// Get the `HashMap` pointing to the adjacent nodes of `node` with their
    /// cost.
    pub fn get_adjacent(&self, node: &K) -> Option<&HashMap<K, V>> {
        self.matrix.get(node)
    }

    /// Get a mutable reference to the `HashMap` pointing to the adjacent
    /// nodes of `node` with their cost.
    pub fn get_mut_adjacent(&mut self, node: &K) -> Option<&mut HashMap<K, V>> {
        self.matrix.get_mut(node)
    }

    /// Get the cost to go from node `a` (from) to node `b` (to).
    pub fn get_edge(&self, from: &K, to: &K) -> Option<&V> {
        self.matrix.get(from)?.get(to)
    }

    /// Get a mutable reference to the cost to go from node `a` (from) to node
    /// `b` (to).
    pub fn get_mut_edge(&mut self, from: &K, to: &K) -> Option<&mut V> {
        self.matrix.get_mut(from)?.get_mut(to)
    }

    /// Check if a node is a key in `self.matrix`.
    pub fn registered(&self, node: &K) -> bool {
        self.matrix.contains_key(&node)
    }

    /// Add a node as a key to `self.matrix` if it has not already been added
    /// and get a mutable reference to the `HashMap` of adjacent nodes to it.
    pub fn register_node(&mut self, node: &K) -> &mut HashMap<K, V> {
        if !self.registered(node) {
            self.matrix.insert(node.clone(), HashMap::new());
        }
        self.matrix.get_mut(node).unwrap()
    }

    fn push_raw(&mut self, from: &K, to: &K, cost: V) -> AgcResult<()> {
        if from == to {
            return Err(AgcError::new(
                AgcErrorKind::SameNode,
                "from cannot be the same as to"
            ));
        }
        if let Some(edge) = self.get_mut_edge(from, to) {
            if cost < *edge {
                *edge = cost;
            }
        } else {
            self.register_node(from).insert(to.clone(), cost);
        }
        Ok(())
    }

    /// Push an edge into the `AdjacencyMatrix`.
    /// 
    /// This method returns an `Err` if `edge.left == edge.right`.
    pub fn push(&mut self, edge: Edge<K, V>) -> AgcResult<()> {
        use EdgeKind::*;
        match edge.edge_kind {
            ToRight => self.push_raw(&edge.left, &edge.right, edge.cost),
            ToLeft => self.push_raw(&edge.right, &edge.left, edge.cost),
            Bidirectional => {
                self.push_raw(&edge.left, &edge.right, edge.cost)?;
                self.push_raw(&edge.right, &edge.left, edge.cost)
            }
        }
    }
}
