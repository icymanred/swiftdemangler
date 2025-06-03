use std::rc::Rc;
use crate::nodes::Node;
pub struct Demangler {
    pub text: String,
    pub node_stack: Vec<Rc<Node>>,
    pub substitutions: Vec<Rc<Node>>,
    pub index: u32

}