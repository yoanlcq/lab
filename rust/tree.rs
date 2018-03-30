use std::collections::HashMap;

type NodeIdInternal = u32;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Tree<T> {
    nodes: HashMap<NodeId, Node<T>>,
    highest_id: NodeIdInternal,
    roots: Vec<NodeId>,
}

#[derive(Debug, Default, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Node<T> {
    pub value: T,
    parent: Option<NodeId>,
    children: Vec<NodeId>,
}

#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct NodeId(NodeIdInternal);


impl<T> Tree<T> {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            highest_id: 0,
            roots: vec![],
        }
    }
    pub fn node<'a>(&'a self, id: &NodeId) -> Option<&'a Node<T>> {
        self.nodes.get(&id)
    }
    pub fn node_mut<'a>(&'a mut self, id: &NodeId) -> Option<&'a mut Node<T>> {
        self.nodes.get_mut(&id)
    }
    fn new_node_id(&mut self) -> NodeId {
        self.highest_id += 1;
        NodeId(self.highest_id)
    }
    pub fn add_root(&mut self, value: T) -> NodeId {
        let node = Node { value, parent: None, children: vec![], };
        let id = self.new_node_id();
        self.nodes.insert(id, node);
        self.roots.push(id);
        id
    }
    pub fn add_child(&mut self, value: T, parent_id: &NodeId) -> Result<NodeId, ()> {
        let parent_node = self.node_mut(parent_id);
        if parent_node.is_none() {
            return Err(());
        }
        let node = Node { value, parent: Some(parent_id), children: vec![], };
        let id = self.new_node_id();
        parent_node.children.push(id);
        self.nodes.insert(id, node);
        Ok(id)
    }
    pub fn reparent(&mut self, id: &NodeId, new_parent_id: &NodeId) -> Result<(), ()> {
        let node = self.node_mut(id);
        if node.is_none() {
            return Err(());
        }
        let new_parent = self.node_mut(new_parent_id);
        if new_parent.is_none() {
            return Err(());
        }
        match node.parent {
            Some(parent) => {
                parent.children.remove(id);
            },
            None => {
                self.roots.remove(id);
            },
        };
        new_parent.children.push(id);
        node.parent = Some(new_parent_id);
    }
    pub fn unparent(&mut self, id: &NodeId) -> Result<(), ()> {
        let node = self.node_mut(id);
        if node.is_none() {
            return Err(());
        }
        if let Some(parent) = node.parent {
            parent.children.remove(id);
            node.parent = None;
            self.roots.push(id);
        }
        Ok(())
    }
    // FIXME
    pub fn remove_recursive(&mut self, id: &NodeId) -> Option<Node<T>> {
        let node = self.node(id);
        if node.is_none() {
            return None;
        }
        match node.parent {
            Some(parent) => parent.children.remove(id),
            None => self.roots.remove(id),
        };
        for child in node.children {
            self.remove(child);
        }
        self.nodes.remove(id)
    }
    // Removes a single node, reattaching its children to its parent.
    pub fn remove_and_reparent_children(&mut self, id: &NodeId) -> Option<Node<T>> {
        unimplemented!{}
    }
    // Removes a single node, making its children orphans.
    pub fn remove_and_make_orphans(&mut self, id: &NodeId) -> Option<Node<T>> {
        unimplemented!{}
    }
}

impl<T> Node<T> {
    pub fn children(&self) -> &[NodeId] {
        &self.children
    }
    pub fn parent(&self) -> Option<NodeId> {
        self.parent
    }
}

fn main() {

}
