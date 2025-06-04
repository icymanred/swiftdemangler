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
         self.text.chars().nth(self.index as usize)
    }
    pub fn next_char(&mut self) -> Option<char> {
        if self.index as usize >= self.text.len() {
            return None
        }
        let curind:u32 = self.index;
        self.index+=1;
         self.text.chars().nth(curind as usize)

    }
    pub fn demangle_natural(&mut self) -> Option<i32> {
        if !self.peek_char()?.is_ascii_digit() {
            return None
        }
        let mut num:i32 = 0;
        loop {
            let curchar = self.peek_char();
            if curchar.is_none_or(|a| !a.is_ascii_digit()) {
                return Some(num);
            }
            let curchar = curchar.unwrap();
            let newnum:i32 = (10 * num) + ((curchar as i32 ) - ('0' as i32));
            num = newnum;
            if newnum < num {
                return None
            }
            self.next_char();

        }
    }
}