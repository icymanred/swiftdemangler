#[cfg(test)]
mod testing {
    use swiftdemangler::{demangler::Demangler, nodes::Node,};
    use swiftdemangler::reader::builtintypes::strings::{self, BUILTIN_TYPE_NAME_BRIDGEOBJECT, BUILTIN_TYPE_NAME_EXECUTOR, BUILTIN_TYPE_NAME_UNSAFEVALUEBUFFER};
    #[test]
    fn simpleBuiltins() {
        let mut dem = Demangler{text:"bBe".to_string(),..Default::default()};
        let bridgenode = Node{kind: swiftdemangler::nodes::NodeKind::BuiltinTypeName, payload: swiftdemangler::nodes::NodePayload::Text(String::from(BUILTIN_TYPE_NAME_BRIDGEOBJECT))};
        let unsafenode:Node = Node{kind: swiftdemangler::nodes::NodeKind::BuiltinTypeName, payload: swiftdemangler::nodes::NodePayload::Text(String::from(BUILTIN_TYPE_NAME_UNSAFEVALUEBUFFER))};
        let executornode = Node{kind: swiftdemangler::nodes::NodeKind::BuiltinTypeName, payload: swiftdemangler::nodes::NodePayload::Text(String::from(BUILTIN_TYPE_NAME_EXECUTOR))};
        let bridge = dem.demangle_builtin().unwrap();
        let unsfe = dem.demangle_builtin().unwrap();
        let executor = dem.demangle_builtin().unwrap();
        assert_eq!(*bridge,bridgenode);
        assert_eq!(*unsfe,unsafenode);
        assert_eq!(*executor,executornode);
        
    }
    #[test]
    fn sized_builtins() {
    let mut dem = Demangler{text:String::from("f32_i32_"),..Default::default()};
    let float_node = Node{kind: swiftdemangler::nodes::NodeKind::BuiltinTypeName, payload: swiftdemangler::nodes::NodePayload::Text(String::from("Builtin.FPIEEE32"))};
    let int_node = Node{kind: swiftdemangler::nodes::NodeKind::BuiltinTypeName, payload: swiftdemangler::nodes::NodePayload::Text(String::from("Builtin.Int32"))};
    let float = dem.demangle_builtin().unwrap();
    let int = dem.demangle_builtin().unwrap();
    dbg!(&float);
    dbg!(&int);
    assert_eq!(*float,float_node);
    assert_eq!(*int,int_node);
    
    }
}