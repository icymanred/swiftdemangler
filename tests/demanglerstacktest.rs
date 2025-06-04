#[cfg(test)]
mod tests {
    
    use std::{rc::Rc, vec};

    use swiftdemangler::{demangler::*, nodes::{predicates, Node}};
    #[test]
    fn pop_test() {
        let mut demangler:Demangler = Demangler::default();
        let base_node: Node = Node{kind : swiftdemangler::nodes::NodeKind::AccessibleFunctionRecord, payload: swiftdemangler::nodes::NodePayload::None};
        let class_node:Node = Node{kind: swiftdemangler::nodes::NodeKind::Class, payload: swiftdemangler::nodes::NodePayload::None};
        let type_node : Node = Node{kind: swiftdemangler::nodes::NodeKind::Type, payload: swiftdemangler::nodes::NodePayload::Children(vec![Rc::new(Node { kind: swiftdemangler::nodes::NodeKind::Class, payload: swiftdemangler::nodes::NodePayload::None })])};
        let node_d = Rc::new(type_node);
        let node_c = Rc::new(class_node);
        let node_a = Rc::new(base_node.clone());
        let node_b = Rc::new( base_node.clone() );
        demangler.node_stack.push(node_d.clone());
        demangler.node_stack.push(node_c.clone());
        demangler.node_stack.push(node_a.clone());
        demangler.node_stack.push(node_b.clone());
        assert!(Rc::ptr_eq(&node_b, &(demangler.node_stack.pop().unwrap())));
        assert!(demangler.pop_kind(swiftdemangler::nodes::NodeKind::Allocator).is_none());
        assert!(Rc::ptr_eq(&node_a, &demangler.pop_kind(swiftdemangler::nodes::NodeKind::AccessibleFunctionRecord).unwrap()));
        assert!(demangler.pop_pred(predicates::is_enum).is_none());
        assert!(demangler.pop_pred(predicates::is_class).is_some());
        assert!(demangler.pop_pred(predicates::is_enum).is_none());
        assert!(demangler.pop_pred(predicates::is_class).is_some());
    }
    #[test]
    fn single_natural_test() {
        let mut b = Demangler{text:"3".to_string(),..Default::default()};
        b.text = "3".to_string();
        assert!(b.demangle_natural().is_some_and(|v| v == 3));
    }
    #[test]
    fn natural_test() {
        let mut b = Demangler{text:"345429".to_string(),..Default::default()};
        assert!(b.demangle_natural().is_some_and(|v| v == 345429));
    }
    #[test]
    fn natural_init_fail() {
        let mut b = Demangler{text:"j".to_string(),..Default::default()};
        assert!(b.demangle_natural().is_none());
    }
    #[test]
    fn natural_stop() {
        let mut b = Demangler{text:"93j4".to_string(),..Default::default()};
        assert!(b.demangle_natural().is_some_and(|v| v == 93));

    }
}