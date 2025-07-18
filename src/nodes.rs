
use std::rc::*;
#[derive(Debug,Clone)]
pub struct Node {
    pub kind: NodeKind,
    pub payload: NodePayload
}
/// Utilities for checking the payload or similarity of nodes
impl Node {
    pub fn is_none(self) -> bool {
        matches!(self.payload,NodePayload::None)
    }
    pub fn is_text(self) -> bool {
        matches!(self.payload,NodePayload::Text(_))
    }
    pub fn is_children(self) -> bool {
        matches!(self.payload,NodePayload::Children(_))
    }
    pub fn is_index(self) -> bool {
        matches!(self.payload,NodePayload::Index(_))
    }
    pub fn child_count(&self) -> u32 {
        match &self.payload {
            NodePayload::Children(a) => a.len() as u32,
            _ => 0
        }
    }
    pub fn first_child(&self) -> Option<Rc<Node>> {
        match &self.payload {
            NodePayload::Children(a) => a.first().cloned(),
            _=>None
        }
    }
    
    ///
    /// Simple equality check, however in the case of the payload being a vector of nodes, it only considers the length of the vector, not the inner contents, use == for slower but more accurate equality checks
    /// 
    pub fn is_similar(&self,other: &Node) -> bool {
        self.kind == other.kind && self.payload == other.payload
        
    }
    #[inline]
    pub fn create_type(node:Node) -> Rc<Node> {
         Rc::new(Node { kind: NodeKind::Type, payload:  NodePayload::Children(vec![Rc::new(node)])})
    }

    
}
impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        if !self.is_similar(other) {
            return false
        }
        match (&self.payload,&other.payload) {
            (NodePayload::Children(a),NodePayload::Children(b)) =>  {
                for (ca,cb )in a.iter().zip(b) {
                    // This recursively compares all the children
                    if  ca != cb {
                        return false;
                    }
                    
                }
                // There are no differences with any of the children
                true
            } ,
            // we already verified they are both the same with the is_similar check, and for every payload except from children a is_similar is all we need
            _=> true
        }
    }
}






#[derive(Debug,Clone)]
pub enum NodePayload {
    None,
    Text(String),
    Index(u32),
    Children(Vec<Rc<Node>>)
}
impl PartialEq for NodePayload {
    fn eq(&self, other: &Self) -> bool {
        match (self,other) {
            (NodePayload::Text(a),NodePayload::Text(b)) => a==b,
            (NodePayload::Index(a),NodePayload::Index(b) ) => a==b,
            (NodePayload::None,NodePayload::None) => true,
            (NodePayload::Children(a),NodePayload::Children(b)) => a.len() == b.len(),
            _=>false
        }
    }
}

pub mod predicates {
    use crate::nodes::*;
    macro_rules! node_is_x {
    ($x:ident,$($a:pat),*  ) => {
        pub fn $x(n:&Node) -> bool {
            match n.kind {
                $(
                    $a => true,
                )*
                NodeKind::Type => {
                match &n.payload {
                    NodePayload::Children(a) => { let firstchildopt = a.first(); match firstchildopt {
                        None => false,
                        Some(a) => predicates::$x(a)
                    } }
                    _=>false
                }
                }
                _=>false

            }


        }

    }
    }
    node_is_x!(is_class, NodeKind::Class, NodeKind::BoundGenericClass);
    node_is_x!(is_alias, NodeKind::TypeAlias);
    node_is_x!(is_enum, NodeKind::Enum, NodeKind::BoundGenericEnum);
    node_is_x!(
        is_protocol,
        NodeKind::Protocol,
        NodeKind::ProtocolSymbolicReference,
        NodeKind::ObjectiveCProtocolSymbolicReference
    );
    node_is_x!(
        is_struct,
        NodeKind::Structure,
        NodeKind::BoundGenericStructure
    );

}


#[derive(PartialEq, Eq,Debug,Clone, Copy)]
pub enum NodeKind {
    Allocator,
    AnonymousContext,
    AnyProtocolConformanceList,
    ArgumentTuple,
    AssociatedType,
    AssociatedTypeRef,
    AssociatedTypeMetadataAccessor,
    DefaultAssociatedTypeMetadataAccessor,
    AccessorAttachedMacroExpansion,
    AssociatedTypeWitnessTableAccessor,
    BaseWitnessTableAccessor,
    AutoClosureType,
    BodyAttachedMacroExpansion,
    BoundGenericClass,
    BoundGenericEnum,
    BoundGenericStructure,
    BoundGenericProtocol,
    BoundGenericOtherNominalType,
    BoundGenericTypeAlias,
    BoundGenericFunction,
    BuiltinTypeName,
    BuiltinTupleType,
    BuiltinFixedArray,
    CFunctionPointer,
    ClangType,
    Class,
    ClassMetadataBaseOffset,
    ConcreteProtocolConformance,
    PackProtocolConformance,
    ConformanceAttachedMacroExpansion,
    Constructor,
    CoroutineContinuationPrototype,
    Deallocator,
    DeclContext,
    DefaultArgumentInitializer,
    DependentAssociatedConformance,
    DependentAssociatedTypeRef,
    DependentGenericConformanceRequirement,
    DependentGenericParamCount,
    DependentGenericParamType,
    DependentGenericSameTypeRequirement,
    DependentGenericSameShapeRequirement,
    DependentGenericLayoutRequirement,
    DependentGenericParamPackMarker,
    DependentGenericSignature,
    DependentGenericType,
    DependentMemberType,
    DependentPseudogenericSignature,
    DependentProtocolConformanceRoot,
    DependentProtocolConformanceInherited,
    DependentProtocolConformanceAssociated,
    DependentProtocolConformanceOpaque,
    Destructor,
    DidSet,
    Directness,
    DistributedThunk,
    DistributedAccessor,
    DynamicAttribute,
    DirectMethodReferenceAttribute,
    DynamicSelf,
    DynamicallyReplaceableFunctionImpl,
    DynamicallyReplaceableFunctionKey,
    DynamicallyReplaceableFunctionVar,
    Enum,
    EnumCase,
    ErrorType,
    EscapingAutoClosureType,
    NoEscapeFunctionType,
    ConcurrentFunctionType,
    GlobalActorFunctionType,
    DifferentiableFunctionType,
    ExistentialMetatype,
    ExplicitClosure,
    Extension,
    ExtensionAttachedMacroExpansion,
    FieldOffset,
    FreestandingMacroExpansion,
    FullTypeMetadata,
    Function,
    FunctionSignatureSpecialization,
    FunctionSignatureSpecializationParam,
    FunctionSignatureSpecializationReturn,
    FunctionSignatureSpecializationParamKind,
    FunctionSignatureSpecializationParamPayload,
    FunctionType,
    ConstrainedExistential,
    ConstrainedExistentialRequirementList,
    ConstrainedExistentialSelf,
    GenericPartialSpecialization,
    GenericPartialSpecializationNotReAbstracted,
    GenericProtocolWitnessTable,
    GenericProtocolWitnessTableInstantiationFunction,
    ResilientProtocolWitnessTable,
    GenericSpecialization,
    GenericSpecializationNotReAbstracted,
    GenericSpecializationInResilienceDomain,
    GenericSpecializationParam,
    GenericSpecializationPrespecialized,
    InlinedGenericFunction,
    GenericTypeMetadataPattern,
    Getter,
    Global,
    GlobalGetter,
    Identifier,
    Index,
    IVarInitializer,
    IVarDestroyer,
    ImplEscaping,
    ImplConvention,
    ImplDifferentiabilityKind,
    ImplErasedIsolation,
    ImplSendingResult,
    ImplParameterResultDifferentiability,
    ImplParameterSending,
    ImplFunctionAttribute,
    ImplFunctionConvention,
    ImplFunctionConventionName,
    ImplFunctionType,
    ImplCoroutineKind,
    ImplInvocationSubstitutions,
    ImplicitClosure,
    ImplParameter,
    ImplPatternSubstitutions,
    ImplResult,
    ImplYield,
    ImplErrorResult,
    InOut,
    InfixOperator,
    Initializer,
    InitAccessor,
    Isolated,
    IsolatedDeallocator,
    Sending,
    IsolatedAnyFunctionType,
    NonIsolatedCallerFunctionType,
    SendingResultFunctionType,
    KeyPathGetterThunkHelper,
    KeyPathSetterThunkHelper,
    KeyPathUnappliedMethodThunkHelper,
    KeyPathAppliedMethodThunkHelper,
    KeyPathEqualsThunkHelper,
    KeyPathHashThunkHelper,
    LazyProtocolWitnessTableAccessor,
    LazyProtocolWitnessTableCacheVariable,
    LocalDeclName,
    Macro,
    MacroExpansionLoc,
    MacroExpansionUniqueName,
    MaterializeForSet,
    MemberAttachedMacroExpansion,
    MemberAttributeAttachedMacroExpansion,
    MergedFunction,
    Metatype,
    MetatypeRepresentation,
    Metaclass,
    MethodLookupFunction,
    ObjCMetadataUpdateFunction,
    ObjCResilientClassStub,
    FullObjCResilientClassStub,
    ModifyAccessor,
    Modify2Accessor,
    Module,
    NativeOwningAddressor,
    NativeOwningMutableAddressor,
    NativePinningAddressor,
    NativePinningMutableAddressor,
    NominalTypeDescriptor,
    NominalTypeDescriptorRecord,
    NonObjCAttribute,
    Number,
    ObjCAsyncCompletionHandlerImpl,
    PredefinedObjCAsyncCompletionHandlerImpl,
    ObjCAttribute,
    ObjCBlock,
    EscapingObjCBlock,
    OtherNominalType,
    OwningAddressor,
    OwningMutableAddressor,
    PartialApplyForwarder,
    PartialApplyObjCForwarder,
    PeerAttachedMacroExpansion,
    PostfixOperator,
    PreambleAttachedMacroExpansion,
    PrefixOperator,
    PrivateDeclName,
    PropertyDescriptor,
    PropertyWrapperBackingInitializer,
    PropertyWrapperInitFromProjectedValue,
    Protocol,
    ProtocolSymbolicReference,
    ProtocolConformance,
    ProtocolConformanceRefInTypeModule,
    ProtocolConformanceRefInProtocolModule,
    ProtocolConformanceRefInOtherModule,
    ProtocolDescriptor,
    ProtocolDescriptorRecord,
    ProtocolConformanceDescriptor,
    ProtocolConformanceDescriptorRecord,
    ProtocolList,
    ProtocolListWithClass,
    ProtocolListWithAnyObject,
    ProtocolSelfConformanceDescriptor,
    ProtocolSelfConformanceWitness,
    ProtocolSelfConformanceWitnessTable,
    ProtocolWitness,
    ProtocolWitnessTable,
    ProtocolWitnessTableAccessor,
    ProtocolWitnessTablePattern,
    ReabstractionThunk,
    ReabstractionThunkHelper,
    ReabstractionThunkHelperWithSelf,
    ReabstractionThunkHelperWithGlobalActor,
    ReadAccessor,
    Read2Accessor,
    RelatedEntityDeclName,
    RetroactiveConformance,
    ReturnType,
    Shared,
    Owned,
    SILBoxType,
    SILBoxTypeWithLayout,
    SILBoxLayout,
    SILBoxMutableField,
    SILBoxImmutableField,
    Setter,
    SpecializationPassID,
    IsSerialized,
    Static,
    Structure,
    Subscript,
    Suffix,
    ThinFunctionType,
    Tuple,
    TupleElement,
    TupleElementName,
    Pack,
    SILPackDirect,
    SILPackIndirect,
    PackExpansion,
    PackElement,
    PackElementLevel,
    Type,
    TypeSymbolicReference,
    TypeAlias,
    TypeList,
    TypeMangling,
    TypeMetadata,
    TypeMetadataAccessFunction,
    TypeMetadataCompletionFunction,
    TypeMetadataInstantiationCache,
    TypeMetadataInstantiationFunction,
    TypeMetadataSingletonInitializationCache,
    TypeMetadataDemanglingCache,
    TypeMetadataLazyCache,
    UncurriedFunctionType,
    UnknownIndex,
    Weak,
    Unowned,
    Unmanaged,
    UnsafeAddressor,
    UnsafeMutableAddressor,
    ValueWitness,
    ValueWitnessTable,
    Variable,
    VTableThunk,
    VTableAttribute,
    WillSet,
    ReflectionMetadataBuiltinDescriptor,
    ReflectionMetadataFieldDescriptor,
    ReflectionMetadataAssocTypeDescriptor,
    ReflectionMetadataSuperclassDescriptor,
    GenericTypeParamDecl,
    CurryThunk,
    SILThunkIdentity,
    DispatchThunk,
    MethodDescriptor,
    ProtocolRequirementsBaseDescriptor,
    AssociatedConformanceDescriptor,
    DefaultAssociatedConformanceAccessor,
    BaseConformanceDescriptor,
    AssociatedTypeDescriptor,
    AsyncAnnotation,
    ThrowsAnnotation,
    TypedThrowsAnnotation,
    EmptyList,
    FirstElementMarker,
    VariadicMarker,
    OutlinedBridgedMethod,
    OutlinedCopy,
    OutlinedConsume,
    OutlinedRetain,
    OutlinedRelease,
    OutlinedInitializeWithTake,
    OutlinedInitializeWithCopy,
    OutlinedAssignWithTake,
    OutlinedAssignWithCopy,
    OutlinedDestroy,
    OutlinedVariable,
    OutlinedReadOnlyObject,
    AssocTypePath,
    LabelList,
    ModuleDescriptor,
    ExtensionDescriptor,
    AnonymousDescriptor,
    AssociatedTypeGenericParamRef,
    SugaredOptional,
    SugaredArray,
    SugaredDictionary,
    SugaredInlineArray,
    SugaredParen,
    AccessorFunctionReference,
    OpaqueType,
    OpaqueTypeDescriptorSymbolicReference,
    OpaqueTypeDescriptor,
    OpaqueTypeDescriptorRecord,
    OpaqueTypeDescriptorAccessor,
    OpaqueTypeDescriptorAccessorImpl,
    OpaqueTypeDescriptorAccessorKey,
    OpaqueTypeDescriptorAccessorVar,
    OpaqueReturnType,
    OpaqueReturnTypeOf,
    CanonicalSpecializedGenericMetaclass,
    CanonicalSpecializedGenericTypeMetadataAccessFunction,
    MetadataInstantiationCache,
    NoncanonicalSpecializedGenericTypeMetadata,
    NoncanonicalSpecializedGenericTypeMetadataCache,
    GlobalVariableOnceFunction,
    GlobalVariableOnceToken,
    GlobalVariableOnceDeclList,
    CanonicalPrespecializedGenericTypeCachingOnceToken,
    AsyncFunctionPointer,
    AutoDiffFunction,
    AutoDiffFunctionKind,
    AutoDiffSelfReorderingReabstractionThunk,
    AutoDiffSubsetParametersThunk,
    AutoDiffDerivativeVTableThunk,
    DifferentiabilityWitness,
    NoDerivative,
    IndexSubset,
    AsyncAwaitResumePartialFunction,
    AsyncSuspendResumePartialFunction,
    AccessibleFunctionRecord,
    CompileTimeLiteral,
    BackDeploymentThunk,
    BackDeploymentFallback,
    ExtendedExistentialTypeShape,
    Uniquable,
    UniqueExtendedExistentialTypeShapeSymbolicReference,
    NonUniqueExtendedExistentialTypeShapeSymbolicReference,
    SymbolicExtendedExistentialType,
    DroppedArgument,
    HasSymbolQuery,
    OpaqueReturnTypeIndex,
    OpaqueReturnTypeParent,
    OutlinedEnumTagStore,
    OutlinedEnumProjectDataForLoad,
    OutlinedEnumGetTag,
    AsyncRemoved,
    ObjectiveCProtocolSymbolicReference,
    OutlinedInitializeWithCopyNoValueWitness,
    OutlinedAssignWithTakeNoValueWitness,
    OutlinedAssignWithCopyNoValueWitness,
    OutlinedDestroyNoValueWitness,
    DependentGenericInverseConformanceRequirement,
    Integer,
    NegativeInteger,
    DependentGenericParamValueMarker,
    CoroFunctionPointer,
    DefaultOverride,
    ConstValue,
}
