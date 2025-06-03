#[allow(unused)]
use swiftdemangler::*;
use std::rc::Rc;
#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use swiftdemangler::nodes::Node;
    #[test]
     fn similararity_pass() {
        let simple:Node = Node{kind: swiftdemangler::nodes::NodeKind::AccessibleFunctionRecord, payload: swiftdemangler::nodes::NodePayload::None};
        let simplesame: Node = Node{kind: swiftdemangler::nodes::NodeKind::AccessibleFunctionRecord, payload: swiftdemangler::nodes::NodePayload::None};
        assert_eq!(simple,simplesame);
        let diff_kind:Node = Node{kind: swiftdemangler::nodes::NodeKind::AccessorAttachedMacroExpansion,payload:swiftdemangler::nodes::NodePayload::None};
        assert_ne!(diff_kind,simple);
        let diff_payload: Node = Node{kind: swiftdemangler::nodes::NodeKind::AccessibleFunctionRecord,payload: swiftdemangler::nodes::NodePayload::Index(0)};
        assert_ne!(diff_payload,simple);
     }
     #[test]
     fn deep_eq() {
      let simple = Rc::new(Node{kind: swiftdemangler::nodes::NodeKind::AccessibleFunctionRecord, payload: swiftdemangler::nodes::NodePayload::None});
      let simpledif = Rc::new(Node{kind: swiftdemangler::nodes::NodeKind::Allocator, payload: swiftdemangler::nodes::NodePayload::None});

      let single_child:Node = Node{kind: swiftdemangler::nodes::NodeKind::Type,payload: swiftdemangler::nodes::NodePayload::Children( vec![simple.clone()])};
      let single_child_same:Node = Node{kind: swiftdemangler::nodes::NodeKind::Type,payload: swiftdemangler::nodes::NodePayload::Children( vec![simple.clone()])};
      assert_eq!(single_child,single_child_same);
      let single_dif_child = Node{kind: swiftdemangler::nodes::NodeKind::Type,payload: swiftdemangler::nodes::NodePayload::Children(vec![simpledif.clone()])};
      assert_ne!(single_child,single_dif_child);
     }
}