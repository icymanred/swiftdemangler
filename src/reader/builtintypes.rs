use crate::{demangler::Demangler, reader::builtintypes::strings::{BUILTIN_TYPE_NAME_BRIDGEOBJECT, BUILTIN_TYPE_NAME_EXECUTOR, BUILTIN_TYPE_NAME_FLOAT, BUILTIN_TYPE_NAME_INT, BUILTIN_TYPE_NAME_UNSAFEVALUEBUFFER}};
use std::{fmt::format, rc::Rc};
use crate::nodes::Node;

macro_rules! textbuiltin {
    ($x:expr) => {
        Node { kind: (crate::nodes::NodeKind::BuiltinTypeName), payload: (crate::nodes::NodePayload::Text($x.to_string()))}
    };
}
const MAX_TYPE_SIZE:i32 = 4096; 

impl Demangler {
    pub fn demangle_builtin(&mut self)  -> Option<Rc<Node>>{
        let builtin = match self.next_char()? {
                'b' => textbuiltin!(BUILTIN_TYPE_NAME_BRIDGEOBJECT),
                'B' => textbuiltin!(BUILTIN_TYPE_NAME_UNSAFEVALUEBUFFER),
                'e' => textbuiltin!(BUILTIN_TYPE_NAME_EXECUTOR),
                'f' => {
                    let sz:i32 = self.demangle_index()? -1;
                    if !(0..MAX_TYPE_SIZE).contains(&sz) {
                        return None;
                    }
                    let txt = format!("{}{}",BUILTIN_TYPE_NAME_FLOAT,sz);
                    Node { kind: crate::nodes::NodeKind::BuiltinTypeName, payload: crate::nodes::NodePayload::Text(txt)}
                },
                'i' => {
                    let sz:i32 = self.demangle_index()? -1;
                    if !(0..MAX_TYPE_SIZE).contains(&sz) {
                        return None;
                    }
                    let txt = format!("{}{}",BUILTIN_TYPE_NAME_INT,sz);
                    Node { kind: crate::nodes::NodeKind::BuiltinTypeName, payload: crate::nodes::NodePayload::Text(txt)}
                
                }

                _=> return None
        };
         Some(Rc::new(builtin))
    }

}


#[allow(unused)]
/// All the string names of builtin swift types
pub mod strings {
    pub const BUILTIN_TYPE_NAME_INT:&str = "Builtin.Int";
/// The name of the Builtin type for Int8
pub const BUILTIN_TYPE_NAME_INT8:&str = "Builtin.Int8";
/// The name of the Builtin type for Int16
pub const BUILTIN_TYPE_NAME_INT16:&str = "Builtin.Int16";
/// The name of the Builtin type for Int32
pub const BUILTIN_TYPE_NAME_INT32:&str = "Builtin.Int32";
/// The name of the Builtin type for Int64
pub const BUILTIN_TYPE_NAME_INT64:&str = "Builtin.Int64";
/// The name of the Builtin type for Int128
pub const BUILTIN_TYPE_NAME_INT128:&str = "Builtin.Int128";
/// The name of the Builtin type for Int256
pub const BUILTIN_TYPE_NAME_INT256:&str = "Builtin.Int256";
/// The name of the Builtin type for Int512
pub const BUILTIN_TYPE_NAME_INT512:&str = "Builtin.Int512";
/// The name of the Builtin type for IntLiteral
pub const BUILTIN_TYPE_NAME_INTLITERAL:&str = "Builtin.IntLiteral";
/// The name of the Builtin type for IEEE Floating point types.
pub const BUILTIN_TYPE_NAME_FLOAT:&str = "Builtin.FPIEEE";
// The name of the builtin type for power pc specific floating point types.
pub const BUILTIN_TYPE_NAME_FLOAT_PPC:&str = "Builtin.FPPPC";
/// The name of the Builtin type for NativeObject
pub const BUILTIN_TYPE_NAME_NATIVEOBJECT:&str = "Builtin.NativeObject";
/// The name of the Builtin type for BridgeObject
pub const BUILTIN_TYPE_NAME_BRIDGEOBJECT:&str = "Builtin.BridgeObject";
/// The name of the Builtin type for RawPointer
pub const BUILTIN_TYPE_NAME_RAWPOINTER:&str = "Builtin.RawPointer";
/// The name of the Builtin type for RawUnsafeContinuation
pub const BUILTIN_TYPE_NAME_RAWUNSAFECONTINUATION:&str = "Builtin.RawUnsafeContinuation";
/// The name of the Builtin type for UnsafeValueBuffer
pub const BUILTIN_TYPE_NAME_UNSAFEVALUEBUFFER:&str = "Builtin.UnsafeValueBuffer";
/// The name of the Builtin type for Job
pub const BUILTIN_TYPE_NAME_JOB:&str = "Builtin.Job";
/// The name of the Builtin type for SerialExecutorRef
pub const BUILTIN_TYPE_NAME_EXECUTOR:&str = "Builtin.Executor";
/// The name of the Builtin type for DefaultActorStorage
pub const BUILTIN_TYPE_NAME_DEFAULTACTORSTORAGE:&str = "Builtin.DefaultActorStorage";
/// The name of the Builtin type for NonDefaultDistributedActorStorage
pub const BUILTIN_TYPE_NAME_NONDEFAULTDISTRIBUTEDACTORSTORAGE:&str = "Builtin.NonDefaultDistributedActorStorage";
/// The name of the Builtin type for UnknownObject
///
/// This no longer exists as an AST-accessible type, but it's still used for
/// fields shaped like AnyObject when ObjC interop is enabled.
pub const BUILTIN_TYPE_NAME_UNKNOWNOBJECT:&str = "Builtin.UnknownObject";
/// The name of the Builtin type for Vector
pub const BUILTIN_TYPE_NAME_VEC:&str = "Builtin.Vec";
pub const BUILTIN_TYPE_NAME_FIXEDARRAY:&str = "Builtin.FixedArray";
/// The name of the Builtin type for SILToken
pub const BUILTIN_TYPE_NAME_SILTOKEN:&str = "Builtin.SILToken";
/// The name of the Builtin type for Word
pub const BUILTIN_TYPE_NAME_WORD:&str = "Builtin.Word";
/// The name of the Builtin type for PackIndex
pub const BUILTIN_TYPE_NAME_PACKINDEX:&str = "Builtin.PackIndex";
pub const BUILTIN_TYPE_NAME_PREFIX:&str = "Builtin.";

}

