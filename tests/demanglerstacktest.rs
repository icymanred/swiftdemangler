#[cfg(test)]
mod tests {
    
    use std::{rc::Rc, vec};

    use swiftdemangler::{demangler::*, nodes::{predicates, Node}};
    #[test]
    fn pop_test() {
        let mut demangler:Demangler = Demangler::default();
        let baseNode: Node = Node{kind : swiftdemangler::nodes::NodeKind::AccessibleFunctionRecord, payload: swiftdemangler::nodes::NodePayload::None};
        let classNode:Node = Node{kind: swiftdemangler::nodes::NodeKind::Class, payload: swiftdemangler::nodes::NodePayload::None};
        let typeNode : Node = Node{kind: swiftdemangler::nodes::NodeKind::Type, payload: swiftdemangler::nodes::NodePayload::Children(vec![Rc::new(Node { kind: swiftdemangler::nodes::NodeKind::Class, payload: swiftdemangler::nodes::NodePayload::None })])};
        let nodeD = Rc::new(typeNode);
        let nodeC = Rc::new(classNode);
        let nodeA = Rc::new(baseNode.clone());
        let nodeB = Rc::new( baseNode.clone() );
        demangler.node_stack.push(nodeD.clone());
        demangler.node_stack.push(nodeC.clone());
        demangler.node_stack.push(nodeA.clone());
        demangler.node_stack.push(nodeB.clone());
        assert!(Rc::ptr_eq(&nodeB, &(demangler.node_stack.pop().unwrap())));
        assert!(demangler.pop_kind(swiftdemangler::nodes::NodeKind::Allocator).is_none());
        assert!(Rc::ptr_eq(&nodeA, &demangler.pop_kind(swiftdemangler::nodes::NodeKind::AccessibleFunctionRecord).unwrap()));
        assert!(demangler.pop_pred(predicates::is_enum).is_none());
        assert!(demangler.pop_pred(predicates::is_class).is_some());
        assert!(demangler.pop_pred(predicates::is_enum).is_none());
        assert!(demangler.pop_pred(predicates::is_class).is_some());
    }
}