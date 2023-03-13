use do_notation::m;

pub struct Tree<T> {
    v: Vec<Node<T>>,
}

struct Node<T> {
    pub v: T,
    pub children: Vec<NodeId>,
    pub parent: Option<NodeId>,
}

pub type NodeId = usize;

impl<T> Node<T> {
    pub fn new(v: T) -> Node<T> {
        Node {
            v: v,
            children: Vec::new(),
            parent: None,
        }
    }
    pub fn new_with_parent_id(v: T, parent: NodeId) -> Node<T> {
        Node {
            v: v,
            children: Vec::new(),
            parent: Some(parent),
        }
    }
}

impl<T> Tree<T> {
    pub fn new() -> Tree<T> {
        Tree { v: Vec::new() }
    }

    pub fn root(&self) -> Option<NodeId> {
        if self.v.is_empty() {
            None
        } else {
            Some(0)
        }
    }

    /// get the child nodes of the specifed node.
    pub fn children(&self, top_node_id: NodeId) -> Vec<&T> {
        match self.v.get(top_node_id) {
            Some(top_node) => top_node
                .children
                .iter()
                .map(|child_id| match self.get(*child_id) {
                    Some(child) => vec![child],
                    None => vec![],
                })
                .flatten()
                .collect(),
            None => Vec::new(),
        }
    }

    pub fn add(&mut self, node_id: NodeId, val: T) -> Option<NodeId> {
        if self.v.is_empty() {
            let new_node = Node::new(val);
            self.v.push(new_node);
            Some(0)
        } else if self.v.len() <= node_id {
            None
        } else {
            let new_node_id = self.v.len();
            self.v[node_id].children.push(new_node_id);
            self.v.push(Node::new_with_parent_id(val, node_id));
            Some(new_node_id)
        }
    }

    pub fn get(&self, node_id: NodeId) -> Option<&T> {
        if node_id < self.v.len() {
            Some(&self.v[node_id].v)
        } else {
            None
        }
    }

    pub fn update<F>(&mut self, node_id: NodeId, f: F) -> bool
    where
        F: FnOnce(&mut T) -> (),
    {
        if self.v.len() <= node_id {
            return false;
        }

        let old = &mut self.v[node_id].v;
        f(old);
        true
    }

    pub fn parent_id(&self, node_id: NodeId) -> Option<NodeId> {
        m! {
            node <- self.v.get(node_id);
            node.parent
        }
    }

    pub fn parent(&self, node_id: NodeId) -> Option<(NodeId, &T)> {
        m! {
            node <- self.v.get(node_id);
            parent_id <- node.parent;
            parent <- self.v.get(parent_id);
            return (parent_id, &parent.v);
        }
    }
}
