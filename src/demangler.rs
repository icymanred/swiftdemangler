use std::rc::Rc;

use crate::nodes::*;
#[derive(Default)]
pub struct Demangler {
    pub text: String,
    pub node_stack: Vec<Rc<Node>>,
    pub substitutions: Vec<Rc<Node>>,
    pub index: u32

}

impl Demangler {
    pub fn pop_kind(&mut self,kind:NodeKind) -> Option<Rc<Node>> {
        let last = self.node_stack.last()?;
        if last.kind == kind {
             return self.node_stack.pop()        
        }
        None
    }
    pub fn pop_pred<T: Fn(&Node) -> bool>(&mut self, pred: T) -> Option<Rc<Node>> {
        let last = self.node_stack.last()?;
        if pred(last) {
            return self.node_stack.pop()
        }
        None
    }
    pub fn peek_char(&self) -> Option<char> {
        return self.text.chars().last()
    }
    pub fn next_char(&mut self) -> Option<char> {
        if self.index as usize >= self.text.len() {
            return None
        }
        let curind:u32 = self.index;
        self.index+=1;
        return self.text.chars().last();

    }
    
}