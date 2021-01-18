use llvm_sys::analysis::*;
use llvm_sys::bit_reader::*;
use llvm_sys::bit_writer::*;
use llvm_sys::comdat::*;
use llvm_sys::core::*;
use llvm_sys::debuginfo::*;
use llvm_sys::disassembler::*;
use llvm_sys::error::*;
use llvm_sys::error_handling::*;
use llvm_sys::execution_engine::*;
use llvm_sys::initialization::*;
use llvm_sys::linker::*;
use llvm_sys::object::*;
use llvm_sys::orc::*;
use llvm_sys::orc2::*;
use llvm_sys::prelude::*;
use llvm_sys::remarks::*;
use llvm_sys::support::*;
use llvm_sys::target::*;
use llvm_sys::target_machine::*;
use llvm_sys::transforms::aggressive_instcombine::*;
use llvm_sys::transforms::coroutines::*;
use llvm_sys::transforms::ipo::*;
use llvm_sys::transforms::pass_manager_builder::*;
use llvm_sys::transforms::scalar::*;
use llvm_sys::transforms::util::*;
use llvm_sys::transforms::vectorize::*;
use llvm_sys::*;
impl OrcSymbolStringPoolEntryRef {
    pub fn orc_release_symbol_string_pool_entry(self) -> () {
        unsafe { LLVMOrcReleaseSymbolStringPoolEntry(self.0) }
    }
}
impl TargetRef {
    pub fn get_next_target(self) -> TargetRef {
        TargetRef(unsafe { LLVMGetNextTarget(self.0) })
    }
    pub fn get_target_name(self) -> *const ::libc::c_char {
        unsafe { LLVMGetTargetName(self.0) }
    }
    pub fn get_target_description(self) -> *const ::libc::c_char {
        unsafe { LLVMGetTargetDescription(self.0) }
    }
    pub fn target_has_jit(self) -> LLVMBool {
        unsafe { LLVMTargetHasJIT(self.0) }
    }
    pub fn target_has_target_machine(self) -> LLVMBool {
        unsafe { LLVMTargetHasTargetMachine(self.0) }
    }
    pub fn target_has_asm_backend(self) -> LLVMBool {
        unsafe { LLVMTargetHasAsmBackend(self.0) }
    }
}
impl NamedMDNodeRef {
    pub fn get_next_named_metadata(self) -> NamedMDNodeRef {
        NamedMDNodeRef(unsafe { LLVMGetNextNamedMetadata(self.0) })
    }
    pub fn get_previous_named_metadata(self) -> NamedMDNodeRef {
        NamedMDNodeRef(unsafe { LLVMGetPreviousNamedMetadata(self.0) })
    }
}
impl BuilderRef {
    pub fn position_builder_before(self, a0: ValueRef) -> () {
        unsafe { LLVMPositionBuilderBefore(self.0, a0.0) }
    }
    pub fn position_builder_at_end(self, a0: BasicBlockRef) -> () {
        unsafe { LLVMPositionBuilderAtEnd(self.0, a0.0) }
    }
    pub fn get_insert_block(self) -> BasicBlockRef {
        BasicBlockRef(unsafe { LLVMGetInsertBlock(self.0) })
    }
    pub fn clear_insertion_position(self) -> () {
        unsafe { LLVMClearInsertionPosition(self.0) }
    }
    pub fn insert_into_builder(self, a0: ValueRef) -> () {
        unsafe { LLVMInsertIntoBuilder(self.0, a0.0) }
    }
    pub fn dispose_builder(self) -> () {
        unsafe { LLVMDisposeBuilder(self.0) }
    }
    pub fn get_current_debug_location_2(self) -> MetadataRef {
        MetadataRef(unsafe { LLVMGetCurrentDebugLocation2(self.0) })
    }
    pub fn set_current_debug_location_2(self, a0: MetadataRef) -> () {
        unsafe { LLVMSetCurrentDebugLocation2(self.0, a0.0) }
    }
    pub fn set_inst_debug_location(self, a0: ValueRef) -> () {
        unsafe { LLVMSetInstDebugLocation(self.0, a0.0) }
    }
    pub fn builder_get_default_fp_math_tag(self) -> MetadataRef {
        MetadataRef(unsafe { LLVMBuilderGetDefaultFPMathTag(self.0) })
    }
    pub fn builder_set_default_fp_math_tag(self, a0: MetadataRef) -> () {
        unsafe { LLVMBuilderSetDefaultFPMathTag(self.0, a0.0) }
    }
    pub fn set_current_debug_location(self, a0: ValueRef) -> () {
        unsafe { LLVMSetCurrentDebugLocation(self.0, a0.0) }
    }
    pub fn get_current_debug_location(self) -> ValueRef {
        ValueRef(unsafe { LLVMGetCurrentDebugLocation(self.0) })
    }
    pub fn build_ret_void(self) -> ValueRef {
        ValueRef(unsafe { LLVMBuildRetVoid(self.0) })
    }
    pub fn build_ret(self, a0: ValueRef) -> ValueRef {
        ValueRef(unsafe { LLVMBuildRet(self.0, a0.0) })
    }
    pub fn build_br(self, a0: BasicBlockRef) -> ValueRef {
        ValueRef(unsafe { LLVMBuildBr(self.0, a0.0) })
    }
    pub fn build_unreachable(self) -> ValueRef {
        ValueRef(unsafe { LLVMBuildUnreachable(self.0) })
    }
    pub fn build_resume(self, a0: ValueRef) -> ValueRef {
        ValueRef(unsafe { LLVMBuildResume(self.0, a0.0) })
    }
    pub fn build_free(self, a0: ValueRef) -> ValueRef {
        ValueRef(unsafe { LLVMBuildFree(self.0, a0.0) })
    }
}
impl BinaryRef {
    pub fn dispose_binary(self) -> () {
        unsafe { LLVMDisposeBinary(self.0) }
    }
    pub fn binary_copy_memory_buffer(self) -> MemoryBufferRef {
        MemoryBufferRef(unsafe { LLVMBinaryCopyMemoryBuffer(self.0) })
    }
    pub fn binary_get_type(self) -> LLVMBinaryType {
        unsafe { LLVMBinaryGetType(self.0) }
    }
    pub fn object_file_copy_section_iterator(self) -> SectionIteratorRef {
        SectionIteratorRef(unsafe { LLVMObjectFileCopySectionIterator(self.0) })
    }
    pub fn object_file_copy_symbol_iterator(self) -> SymbolIteratorRef {
        SymbolIteratorRef(unsafe { LLVMObjectFileCopySymbolIterator(self.0) })
    }
}
impl OrcJITTargetMachineBuilderRef {
    pub fn orc_dispose_jit_target_machine_builder(self) -> () {
        unsafe { LLVMOrcDisposeJITTargetMachineBuilder(self.0) }
    }
}
impl AttributeRef {
    pub fn get_enum_attribute_kind(self) -> ::libc::c_uint {
        unsafe { LLVMGetEnumAttributeKind(self.0) }
    }
    pub fn get_enum_attribute_value(self) -> u64 {
        unsafe { LLVMGetEnumAttributeValue(self.0) }
    }
    pub fn is_enum_attribute(self) -> LLVMBool {
        unsafe { LLVMIsEnumAttribute(self.0) }
    }
    pub fn is_string_attribute(self) -> LLVMBool {
        unsafe { LLVMIsStringAttribute(self.0) }
    }
}
impl TargetMachineRef {
    pub fn dispose_target_machine(self) -> () {
        unsafe { LLVMDisposeTargetMachine(self.0) }
    }
    pub fn get_target_machine_target(self) -> TargetRef {
        TargetRef(unsafe { LLVMGetTargetMachineTarget(self.0) })
    }
    pub fn get_target_machine_triple(self) -> *mut ::libc::c_char {
        unsafe { LLVMGetTargetMachineTriple(self.0) }
    }
    pub fn get_target_machine_cpu(self) -> *mut ::libc::c_char {
        unsafe { LLVMGetTargetMachineCPU(self.0) }
    }
    pub fn get_target_machine_feature_string(self) -> *mut ::libc::c_char {
        unsafe { LLVMGetTargetMachineFeatureString(self.0) }
    }
    pub fn create_target_data_layout(self) -> TargetDataRef {
        TargetDataRef(unsafe { LLVMCreateTargetDataLayout(self.0) })
    }
    pub fn set_target_machine_asm_verbosity(self, a0: LLVMBool) -> () {
        unsafe { LLVMSetTargetMachineAsmVerbosity(self.0, a0) }
    }
    pub fn add_analysis_passes(self, a0: PassManagerRef) -> () {
        unsafe { LLVMAddAnalysisPasses(self.0, a0.0) }
    }
    pub fn orc_create_instance(self) -> OrcJITStackRef {
        OrcJITStackRef(unsafe { LLVMOrcCreateInstance(self.0) })
    }
}
impl ValueRef {
    pub fn get_subprogram(self) -> MetadataRef {
        MetadataRef(unsafe { LLVMGetSubprogram(self.0) })
    }
    pub fn set_subprogram(self, a0: MetadataRef) -> () {
        unsafe { LLVMSetSubprogram(self.0, a0.0) }
    }
    pub fn instruction_get_debug_loc(self) -> MetadataRef {
        MetadataRef(unsafe { LLVMInstructionGetDebugLoc(self.0) })
    }
    pub fn instruction_set_debug_loc(self, a0: MetadataRef) -> () {
        unsafe { LLVMInstructionSetDebugLoc(self.0, a0.0) }
    }
    pub fn get_comdat(self) -> ComdatRef {
        ComdatRef(unsafe { LLVMGetComdat(self.0) })
    }
    pub fn set_comdat(self, a0: ComdatRef) -> () {
        unsafe { LLVMSetComdat(self.0, a0.0) }
    }
    pub fn verify_function(self, a0: LLVMVerifierFailureAction) -> LLVMBool {
        unsafe { LLVMVerifyFunction(self.0, a0) }
    }
    pub fn view_function_cfg(self) -> () {
        unsafe { LLVMViewFunctionCFG(self.0) }
    }
    pub fn view_function_cfg_only(self) -> () {
        unsafe { LLVMViewFunctionCFGOnly(self.0) }
    }
    pub fn get_debug_loc_line(self) -> ::libc::c_uint {
        unsafe { LLVMGetDebugLocLine(self.0) }
    }
    pub fn get_debug_loc_column(self) -> ::libc::c_uint {
        unsafe { LLVMGetDebugLocColumn(self.0) }
    }
    pub fn get_next_function(self) -> ValueRef {
        ValueRef(unsafe { LLVMGetNextFunction(self.0) })
    }
    pub fn get_previous_function(self) -> ValueRef {
        ValueRef(unsafe { LLVMGetPreviousFunction(self.0) })
    }
    pub fn get_value_kind(self) -> LLVMValueKind {
        unsafe { LLVMGetValueKind(self.0) }
    }
    pub fn type_of(self) -> TypeRef {
        TypeRef(unsafe { LLVMTypeOf(self.0) })
    }
    pub fn get_value_name(self) -> *const ::libc::c_char {
        unsafe { LLVMGetValueName(self.0) }
    }
    pub fn set_value_name(self, a0: *const ::libc::c_char) -> () {
        unsafe { LLVMSetValueName(self.0, a0) }
    }
    pub fn dump_value(self) -> () {
        unsafe { LLVMDumpValue(self.0) }
    }
    pub fn print_value_to_string(self) -> *mut ::libc::c_char {
        unsafe { LLVMPrintValueToString(self.0) }
    }
    pub fn replace_all_uses_with(self, a0: ValueRef) -> () {
        unsafe { LLVMReplaceAllUsesWith(self.0, a0.0) }
    }
    pub fn is_constant(self) -> LLVMBool {
        unsafe { LLVMIsConstant(self.0) }
    }
    pub fn is_undef(self) -> LLVMBool {
        unsafe { LLVMIsUndef(self.0) }
    }
    pub fn is_amd_node(self) -> ValueRef {
        ValueRef(unsafe { LLVMIsAMDNode(self.0) })
    }
    pub fn is_amd_string(self) -> ValueRef {
        ValueRef(unsafe { LLVMIsAMDString(self.0) })
    }
    pub fn get_first_use(self) -> UseRef {
        UseRef(unsafe { LLVMGetFirstUse(self.0) })
    }
    pub fn get_operand(self, a0: ::libc::c_uint) -> ValueRef {
        ValueRef(unsafe { LLVMGetOperand(self.0, a0) })
    }
    pub fn get_operand_use(self, a0: ::libc::c_uint) -> UseRef {
        UseRef(unsafe { LLVMGetOperandUse(self.0, a0) })
    }
    pub fn set_operand(self, a0: ::libc::c_uint, a1: ValueRef) -> () {
        unsafe { LLVMSetOperand(self.0, a0, a1.0) }
    }
    pub fn get_num_operands(self) -> ::libc::c_int {
        unsafe { LLVMGetNumOperands(self.0) }
    }
    pub fn is_null(self) -> LLVMBool {
        unsafe { LLVMIsNull(self.0) }
    }
    pub fn const_int_get_z_ext_value(self) -> ::libc::c_ulonglong {
        unsafe { LLVMConstIntGetZExtValue(self.0) }
    }
    pub fn const_int_get_s_ext_value(self) -> ::libc::c_longlong {
        unsafe { LLVMConstIntGetSExtValue(self.0) }
    }
    pub fn is_constant_string(self) -> LLVMBool {
        unsafe { LLVMIsConstantString(self.0) }
    }
    pub fn get_as_string(self, a0: *mut ::libc::size_t) -> *const ::libc::c_char {
        unsafe { LLVMGetAsString(self.0, a0) }
    }
    pub fn get_element_as_constant(self, a0: ::libc::c_uint) -> ValueRef {
        ValueRef(unsafe { LLVMGetElementAsConstant(self.0, a0) })
    }
    pub fn get_const_opcode(self) -> LLVMOpcode {
        unsafe { LLVMGetConstOpcode(self.0) }
    }
    pub fn const_neg(self) -> ValueRef {
        ValueRef(unsafe { LLVMConstNeg(self.0) })
    }
    pub fn const_nsw_neg(self) -> ValueRef {
        ValueRef(unsafe { LLVMConstNSWNeg(self.0) })
    }
    pub fn const_nuw_neg(self) -> ValueRef {
        ValueRef(unsafe { LLVMConstNUWNeg(self.0) })
    }
    pub fn const_f_neg(self) -> ValueRef {
        ValueRef(unsafe { LLVMConstFNeg(self.0) })
    }
    pub fn const_not(self) -> ValueRef {
        ValueRef(unsafe { LLVMConstNot(self.0) })
    }
    pub fn const_add(self, a0: ValueRef) -> ValueRef {
        ValueRef(unsafe { LLVMConstAdd(self.0, a0.0) })
    }
    pub fn const_nsw_add(self, a0: ValueRef) -> ValueRef {
        ValueRef(unsafe { LLVMConstNSWAdd(self.0, a0.0) })
    }
    pub fn const_nuw_add(self, a0: ValueRef) -> ValueRef {
        ValueRef(unsafe { LLVMConstNUWAdd(self.0, a0.0) })
    }
    pub fn const_f_add(self, a0: ValueRef) -> ValueRef {
        ValueRef(unsafe { LLVMConstFAdd(self.0, a0.0) })
    }
    pub fn const_sub(self, a0: ValueRef) -> ValueRef {
        ValueRef(unsafe { LLVMConstSub(self.0, a0.0) })
    }
    pub fn const_nsw_sub(self, a0: ValueRef) -> ValueRef {
        ValueRef(unsafe { LLVMConstNSWSub(self.0, a0.0) })
    }
    pub fn const_nuw_sub(self, a0: ValueRef) -> ValueRef {
        ValueRef(unsafe { LLVMConstNUWSub(self.0, a0.0) })
    }
    pub fn const_f_sub(self, a0: ValueRef) -> ValueRef {
        ValueRef(unsafe { LLVMConstFSub(self.0, a0.0) })
    }
    pub fn const_mul(self, a0: ValueRef) -> ValueRef {
        ValueRef(unsafe { LLVMConstMul(self.0, a0.0) })
    }
    pub fn const_nsw_mul(self, a0: ValueRef) -> ValueRef {
        ValueRef(unsafe { LLVMConstNSWMul(self.0, a0.0) })
    }
    pub fn const_nuw_mul(self, a0: ValueRef) -> ValueRef {
        ValueRef(unsafe { LLVMConstNUWMul(self.0, a0.0) })
    }
    pub fn const_f_mul(self, a0: ValueRef) -> ValueRef {
        ValueRef(unsafe { LLVMConstFMul(self.0, a0.0) })
    }
    pub fn const_u_div(self, a0: ValueRef) -> ValueRef {
        ValueRef(unsafe { LLVMConstUDiv(self.0, a0.0) })
    }
    pub fn const_exact_u_div(self, a0: ValueRef) -> ValueRef {
        ValueRef(unsafe { LLVMConstExactUDiv(self.0, a0.0) })
    }
    pub fn const_s_div(self, a0: ValueRef) -> ValueRef {
        ValueRef(unsafe { LLVMConstSDiv(self.0, a0.0) })
    }
    pub fn const_exact_s_div(self, a0: ValueRef) -> ValueRef {
        ValueRef(unsafe { LLVMConstExactSDiv(self.0, a0.0) })
    }
    pub fn const_f_div(self, a0: ValueRef) -> ValueRef {
        ValueRef(unsafe { LLVMConstFDiv(self.0, a0.0) })
    }
    pub fn const_u_rem(self, a0: ValueRef) -> ValueRef {
        ValueRef(unsafe { LLVMConstURem(self.0, a0.0) })
    }
    pub fn const_s_rem(self, a0: ValueRef) -> ValueRef {
        ValueRef(unsafe { LLVMConstSRem(self.0, a0.0) })
    }
    pub fn const_f_rem(self, a0: ValueRef) -> ValueRef {
        ValueRef(unsafe { LLVMConstFRem(self.0, a0.0) })
    }
    pub fn const_and(self, a0: ValueRef) -> ValueRef {
        ValueRef(unsafe { LLVMConstAnd(self.0, a0.0) })
    }
    pub fn const_or(self, a0: ValueRef) -> ValueRef {
        ValueRef(unsafe { LLVMConstOr(self.0, a0.0) })
    }
    pub fn const_xor(self, a0: ValueRef) -> ValueRef {
        ValueRef(unsafe { LLVMConstXor(self.0, a0.0) })
    }
    pub fn const_shl(self, a0: ValueRef) -> ValueRef {
        ValueRef(unsafe { LLVMConstShl(self.0, a0.0) })
    }
    pub fn const_l_shr(self, a0: ValueRef) -> ValueRef {
        ValueRef(unsafe { LLVMConstLShr(self.0, a0.0) })
    }
    pub fn const_a_shr(self, a0: ValueRef) -> ValueRef {
        ValueRef(unsafe { LLVMConstAShr(self.0, a0.0) })
    }
    pub fn const_trunc(self, a0: TypeRef) -> ValueRef {
        ValueRef(unsafe { LLVMConstTrunc(self.0, a0.0) })
    }
    pub fn const_s_ext(self, a0: TypeRef) -> ValueRef {
        ValueRef(unsafe { LLVMConstSExt(self.0, a0.0) })
    }
    pub fn const_z_ext(self, a0: TypeRef) -> ValueRef {
        ValueRef(unsafe { LLVMConstZExt(self.0, a0.0) })
    }
    pub fn const_fp_trunc(self, a0: TypeRef) -> ValueRef {
        ValueRef(unsafe { LLVMConstFPTrunc(self.0, a0.0) })
    }
    pub fn const_fp_ext(self, a0: TypeRef) -> ValueRef {
        ValueRef(unsafe { LLVMConstFPExt(self.0, a0.0) })
    }
    pub fn const_ui_to_fp(self, a0: TypeRef) -> ValueRef {
        ValueRef(unsafe { LLVMConstUIToFP(self.0, a0.0) })
    }
    pub fn const_si_to_fp(self, a0: TypeRef) -> ValueRef {
        ValueRef(unsafe { LLVMConstSIToFP(self.0, a0.0) })
    }
    pub fn const_fp_to_ui(self, a0: TypeRef) -> ValueRef {
        ValueRef(unsafe { LLVMConstFPToUI(self.0, a0.0) })
    }
    pub fn const_fp_to_si(self, a0: TypeRef) -> ValueRef {
        ValueRef(unsafe { LLVMConstFPToSI(self.0, a0.0) })
    }
    pub fn const_ptr_to_int(self, a0: TypeRef) -> ValueRef {
        ValueRef(unsafe { LLVMConstPtrToInt(self.0, a0.0) })
    }
    pub fn const_int_to_ptr(self, a0: TypeRef) -> ValueRef {
        ValueRef(unsafe { LLVMConstIntToPtr(self.0, a0.0) })
    }
    pub fn const_bit_cast(self, a0: TypeRef) -> ValueRef {
        ValueRef(unsafe { LLVMConstBitCast(self.0, a0.0) })
    }
    pub fn const_addr_space_cast(self, a0: TypeRef) -> ValueRef {
        ValueRef(unsafe { LLVMConstAddrSpaceCast(self.0, a0.0) })
    }
    pub fn const_z_ext_or_bit_cast(self, a0: TypeRef) -> ValueRef {
        ValueRef(unsafe { LLVMConstZExtOrBitCast(self.0, a0.0) })
    }
    pub fn const_s_ext_or_bit_cast(self, a0: TypeRef) -> ValueRef {
        ValueRef(unsafe { LLVMConstSExtOrBitCast(self.0, a0.0) })
    }
    pub fn const_trunc_or_bit_cast(self, a0: TypeRef) -> ValueRef {
        ValueRef(unsafe { LLVMConstTruncOrBitCast(self.0, a0.0) })
    }
    pub fn const_pointer_cast(self, a0: TypeRef) -> ValueRef {
        ValueRef(unsafe { LLVMConstPointerCast(self.0, a0.0) })
    }
    pub fn const_fp_cast(self, a0: TypeRef) -> ValueRef {
        ValueRef(unsafe { LLVMConstFPCast(self.0, a0.0) })
    }
    pub fn block_address(self, a0: BasicBlockRef) -> ValueRef {
        ValueRef(unsafe { LLVMBlockAddress(self.0, a0.0) })
    }
    pub fn get_global_parent(self) -> ModuleRef {
        ModuleRef(unsafe { LLVMGetGlobalParent(self.0) })
    }
    pub fn is_declaration(self) -> LLVMBool {
        unsafe { LLVMIsDeclaration(self.0) }
    }
    pub fn get_linkage(self) -> LLVMLinkage {
        unsafe { LLVMGetLinkage(self.0) }
    }
    pub fn set_linkage(self, a0: LLVMLinkage) -> () {
        unsafe { LLVMSetLinkage(self.0, a0) }
    }
    pub fn get_section(self) -> *const ::libc::c_char {
        unsafe { LLVMGetSection(self.0) }
    }
    pub fn set_section(self, a0: *const ::libc::c_char) -> () {
        unsafe { LLVMSetSection(self.0, a0) }
    }
    pub fn get_visibility(self) -> LLVMVisibility {
        unsafe { LLVMGetVisibility(self.0) }
    }
    pub fn set_visibility(self, a0: LLVMVisibility) -> () {
        unsafe { LLVMSetVisibility(self.0, a0) }
    }
    pub fn get_dll_storage_class(self) -> LLVMDLLStorageClass {
        unsafe { LLVMGetDLLStorageClass(self.0) }
    }
    pub fn set_dll_storage_class(self, a0: LLVMDLLStorageClass) -> () {
        unsafe { LLVMSetDLLStorageClass(self.0, a0) }
    }
    pub fn get_unnamed_address(self) -> LLVMUnnamedAddr {
        unsafe { LLVMGetUnnamedAddress(self.0) }
    }
    pub fn set_unnamed_address(self, a0: LLVMUnnamedAddr) -> () {
        unsafe { LLVMSetUnnamedAddress(self.0, a0) }
    }
    pub fn global_get_value_type(self) -> TypeRef {
        TypeRef(unsafe { LLVMGlobalGetValueType(self.0) })
    }
    pub fn has_unnamed_addr(self) -> LLVMBool {
        unsafe { LLVMHasUnnamedAddr(self.0) }
    }
    pub fn set_unnamed_addr(self, a0: LLVMBool) -> () {
        unsafe { LLVMSetUnnamedAddr(self.0, a0) }
    }
    pub fn get_alignment(self) -> ::libc::c_uint {
        unsafe { LLVMGetAlignment(self.0) }
    }
    pub fn set_alignment(self, a0: ::libc::c_uint) -> () {
        unsafe { LLVMSetAlignment(self.0, a0) }
    }
    pub fn global_set_metadata(self, a0: ::libc::c_uint, a1: MetadataRef) -> () {
        unsafe { LLVMGlobalSetMetadata(self.0, a0, a1.0) }
    }
    pub fn global_erase_metadata(self, a0: ::libc::c_uint) -> () {
        unsafe { LLVMGlobalEraseMetadata(self.0, a0) }
    }
    pub fn global_clear_metadata(self) -> () {
        unsafe { LLVMGlobalClearMetadata(self.0) }
    }
    pub fn get_next_global(self) -> ValueRef {
        ValueRef(unsafe { LLVMGetNextGlobal(self.0) })
    }
    pub fn get_previous_global(self) -> ValueRef {
        ValueRef(unsafe { LLVMGetPreviousGlobal(self.0) })
    }
    pub fn delete_global(self) -> () {
        unsafe { LLVMDeleteGlobal(self.0) }
    }
    pub fn get_initializer(self) -> ValueRef {
        ValueRef(unsafe { LLVMGetInitializer(self.0) })
    }
    pub fn set_initializer(self, a0: ValueRef) -> () {
        unsafe { LLVMSetInitializer(self.0, a0.0) }
    }
    pub fn is_thread_local(self) -> LLVMBool {
        unsafe { LLVMIsThreadLocal(self.0) }
    }
    pub fn set_thread_local(self, a0: LLVMBool) -> () {
        unsafe { LLVMSetThreadLocal(self.0, a0) }
    }
    pub fn is_global_constant(self) -> LLVMBool {
        unsafe { LLVMIsGlobalConstant(self.0) }
    }
    pub fn set_global_constant(self, a0: LLVMBool) -> () {
        unsafe { LLVMSetGlobalConstant(self.0, a0) }
    }
    pub fn get_thread_local_mode(self) -> LLVMThreadLocalMode {
        unsafe { LLVMGetThreadLocalMode(self.0) }
    }
    pub fn set_thread_local_mode(self, a0: LLVMThreadLocalMode) -> () {
        unsafe { LLVMSetThreadLocalMode(self.0, a0) }
    }
    pub fn is_externally_initialized(self) -> LLVMBool {
        unsafe { LLVMIsExternallyInitialized(self.0) }
    }
    pub fn set_externally_initialized(self, a0: LLVMBool) -> () {
        unsafe { LLVMSetExternallyInitialized(self.0, a0) }
    }
    pub fn get_next_global_alias(self) -> ValueRef {
        ValueRef(unsafe { LLVMGetNextGlobalAlias(self.0) })
    }
    pub fn get_previous_global_alias(self) -> ValueRef {
        ValueRef(unsafe { LLVMGetPreviousGlobalAlias(self.0) })
    }
    pub fn alias_get_aliasee(self) -> ValueRef {
        ValueRef(unsafe { LLVMAliasGetAliasee(self.0) })
    }
    pub fn alias_set_aliasee(self, a0: ValueRef) -> () {
        unsafe { LLVMAliasSetAliasee(self.0, a0.0) }
    }
    pub fn delete_function(self) -> () {
        unsafe { LLVMDeleteFunction(self.0) }
    }
    pub fn has_personality_fn(self) -> LLVMBool {
        unsafe { LLVMHasPersonalityFn(self.0) }
    }
    pub fn get_personality_fn(self) -> ValueRef {
        ValueRef(unsafe { LLVMGetPersonalityFn(self.0) })
    }
    pub fn set_personality_fn(self, a0: ValueRef) -> () {
        unsafe { LLVMSetPersonalityFn(self.0, a0.0) }
    }
    pub fn get_intrinsic_id(self) -> ::libc::c_uint {
        unsafe { LLVMGetIntrinsicID(self.0) }
    }
    pub fn get_function_call_conv(self) -> ::libc::c_uint {
        unsafe { LLVMGetFunctionCallConv(self.0) }
    }
    pub fn set_function_call_conv(self, a0: ::libc::c_uint) -> () {
        unsafe { LLVMSetFunctionCallConv(self.0, a0) }
    }
    pub fn get_gc(self) -> *const ::libc::c_char {
        unsafe { LLVMGetGC(self.0) }
    }
    pub fn set_gc(self, a0: *const ::libc::c_char) -> () {
        unsafe { LLVMSetGC(self.0, a0) }
    }
    pub fn add_attribute_at_index(self, a0: LLVMAttributeIndex, a1: AttributeRef) -> () {
        unsafe { LLVMAddAttributeAtIndex(self.0, a0, a1.0) }
    }
    pub fn get_attribute_count_at_index(self, a0: LLVMAttributeIndex) -> ::libc::c_uint {
        unsafe { LLVMGetAttributeCountAtIndex(self.0, a0) }
    }
    pub fn count_params(self) -> ::libc::c_uint {
        unsafe { LLVMCountParams(self.0) }
    }
    pub fn get_params(self, a0: *mut LLVMValueRef) -> () {
        unsafe { LLVMGetParams(self.0, a0) }
    }
    pub fn get_param(self, a0: ::libc::c_uint) -> ValueRef {
        ValueRef(unsafe { LLVMGetParam(self.0, a0) })
    }
    pub fn get_param_parent(self) -> ValueRef {
        ValueRef(unsafe { LLVMGetParamParent(self.0) })
    }
    pub fn get_first_param(self) -> ValueRef {
        ValueRef(unsafe { LLVMGetFirstParam(self.0) })
    }
    pub fn get_last_param(self) -> ValueRef {
        ValueRef(unsafe { LLVMGetLastParam(self.0) })
    }
    pub fn get_next_param(self) -> ValueRef {
        ValueRef(unsafe { LLVMGetNextParam(self.0) })
    }
    pub fn get_previous_param(self) -> ValueRef {
        ValueRef(unsafe { LLVMGetPreviousParam(self.0) })
    }
    pub fn set_param_alignment(self, a0: ::libc::c_uint) -> () {
        unsafe { LLVMSetParamAlignment(self.0, a0) }
    }
    pub fn get_next_global_i_func(self) -> ValueRef {
        ValueRef(unsafe { LLVMGetNextGlobalIFunc(self.0) })
    }
    pub fn get_previous_global_i_func(self) -> ValueRef {
        ValueRef(unsafe { LLVMGetPreviousGlobalIFunc(self.0) })
    }
    pub fn get_global_i_func_resolver(self) -> ValueRef {
        ValueRef(unsafe { LLVMGetGlobalIFuncResolver(self.0) })
    }
    pub fn set_global_i_func_resolver(self, a0: ValueRef) -> () {
        unsafe { LLVMSetGlobalIFuncResolver(self.0, a0.0) }
    }
    pub fn erase_global_i_func(self) -> () {
        unsafe { LLVMEraseGlobalIFunc(self.0) }
    }
    pub fn remove_global_i_func(self) -> () {
        unsafe { LLVMRemoveGlobalIFunc(self.0) }
    }
    pub fn value_as_metadata(self) -> MetadataRef {
        MetadataRef(unsafe { LLVMValueAsMetadata(self.0) })
    }
    pub fn get_md_string(self, a0: *mut ::libc::c_uint) -> *const ::libc::c_char {
        unsafe { LLVMGetMDString(self.0, a0) }
    }
    pub fn get_md_node_num_operands(self) -> ::libc::c_uint {
        unsafe { LLVMGetMDNodeNumOperands(self.0) }
    }
    pub fn get_md_node_operands(self, a0: *mut LLVMValueRef) -> () {
        unsafe { LLVMGetMDNodeOperands(self.0, a0) }
    }
    pub fn value_is_basic_block(self) -> LLVMBool {
        unsafe { LLVMValueIsBasicBlock(self.0) }
    }
    pub fn value_as_basic_block(self) -> BasicBlockRef {
        BasicBlockRef(unsafe { LLVMValueAsBasicBlock(self.0) })
    }
    pub fn count_basic_blocks(self) -> ::libc::c_uint {
        unsafe { LLVMCountBasicBlocks(self.0) }
    }
    pub fn get_basic_blocks(self, a0: *mut LLVMBasicBlockRef) -> () {
        unsafe { LLVMGetBasicBlocks(self.0, a0) }
    }
    pub fn get_first_basic_block(self) -> BasicBlockRef {
        BasicBlockRef(unsafe { LLVMGetFirstBasicBlock(self.0) })
    }
    pub fn get_last_basic_block(self) -> BasicBlockRef {
        BasicBlockRef(unsafe { LLVMGetLastBasicBlock(self.0) })
    }
    pub fn get_entry_basic_block(self) -> BasicBlockRef {
        BasicBlockRef(unsafe { LLVMGetEntryBasicBlock(self.0) })
    }
    pub fn append_existing_basic_block(self, a0: BasicBlockRef) -> () {
        unsafe { LLVMAppendExistingBasicBlock(self.0, a0.0) }
    }
    pub fn append_basic_block(self, a0: *const ::libc::c_char) -> BasicBlockRef {
        BasicBlockRef(unsafe { LLVMAppendBasicBlock(self.0, a0) })
    }
    pub fn has_metadata(self) -> ::libc::c_int {
        unsafe { LLVMHasMetadata(self.0) }
    }
    pub fn get_metadata(self, a0: ::libc::c_uint) -> ValueRef {
        ValueRef(unsafe { LLVMGetMetadata(self.0, a0) })
    }
    pub fn set_metadata(self, a0: ::libc::c_uint, a1: ValueRef) -> () {
        unsafe { LLVMSetMetadata(self.0, a0, a1.0) }
    }
    pub fn get_instruction_parent(self) -> BasicBlockRef {
        BasicBlockRef(unsafe { LLVMGetInstructionParent(self.0) })
    }
    pub fn get_next_instruction(self) -> ValueRef {
        ValueRef(unsafe { LLVMGetNextInstruction(self.0) })
    }
    pub fn get_previous_instruction(self) -> ValueRef {
        ValueRef(unsafe { LLVMGetPreviousInstruction(self.0) })
    }
    pub fn instruction_remove_from_parent(self) -> () {
        unsafe { LLVMInstructionRemoveFromParent(self.0) }
    }
    pub fn instruction_erase_from_parent(self) -> () {
        unsafe { LLVMInstructionEraseFromParent(self.0) }
    }
    pub fn get_instruction_opcode(self) -> LLVMOpcode {
        unsafe { LLVMGetInstructionOpcode(self.0) }
    }
    pub fn get_i_cmp_predicate(self) -> LLVMIntPredicate {
        unsafe { LLVMGetICmpPredicate(self.0) }
    }
    pub fn get_f_cmp_predicate(self) -> LLVMRealPredicate {
        unsafe { LLVMGetFCmpPredicate(self.0) }
    }
    pub fn instruction_clone(self) -> ValueRef {
        ValueRef(unsafe { LLVMInstructionClone(self.0) })
    }
    pub fn is_a_terminator_inst(self) -> ValueRef {
        ValueRef(unsafe { LLVMIsATerminatorInst(self.0) })
    }
    pub fn get_num_arg_operands(self) -> ::libc::c_uint {
        unsafe { LLVMGetNumArgOperands(self.0) }
    }
    pub fn set_instruction_call_conv(self, a0: ::libc::c_uint) -> () {
        unsafe { LLVMSetInstructionCallConv(self.0, a0) }
    }
    pub fn get_instruction_call_conv(self) -> ::libc::c_uint {
        unsafe { LLVMGetInstructionCallConv(self.0) }
    }
    pub fn add_call_site_attribute(self, a0: LLVMAttributeIndex, a1: AttributeRef) -> () {
        unsafe { LLVMAddCallSiteAttribute(self.0, a0, a1.0) }
    }
    pub fn get_called_function_type(self) -> TypeRef {
        TypeRef(unsafe { LLVMGetCalledFunctionType(self.0) })
    }
    pub fn get_called_value(self) -> ValueRef {
        ValueRef(unsafe { LLVMGetCalledValue(self.0) })
    }
    pub fn is_tail_call(self) -> LLVMBool {
        unsafe { LLVMIsTailCall(self.0) }
    }
    pub fn set_tail_call(self, a0: LLVMBool) -> () {
        unsafe { LLVMSetTailCall(self.0, a0) }
    }
    pub fn get_normal_dest(self) -> BasicBlockRef {
        BasicBlockRef(unsafe { LLVMGetNormalDest(self.0) })
    }
    pub fn get_unwind_dest(self) -> BasicBlockRef {
        BasicBlockRef(unsafe { LLVMGetUnwindDest(self.0) })
    }
    pub fn set_normal_dest(self, a0: BasicBlockRef) -> () {
        unsafe { LLVMSetNormalDest(self.0, a0.0) }
    }
    pub fn set_unwind_dest(self, a0: BasicBlockRef) -> () {
        unsafe { LLVMSetUnwindDest(self.0, a0.0) }
    }
    pub fn get_num_successors(self) -> ::libc::c_uint {
        unsafe { LLVMGetNumSuccessors(self.0) }
    }
    pub fn get_successor(self, a0: ::libc::c_uint) -> BasicBlockRef {
        BasicBlockRef(unsafe { LLVMGetSuccessor(self.0, a0) })
    }
    pub fn set_successor(self, a0: ::libc::c_uint, a1: BasicBlockRef) -> () {
        unsafe { LLVMSetSuccessor(self.0, a0, a1.0) }
    }
    pub fn is_conditional(self) -> LLVMBool {
        unsafe { LLVMIsConditional(self.0) }
    }
    pub fn get_condition(self) -> ValueRef {
        ValueRef(unsafe { LLVMGetCondition(self.0) })
    }
    pub fn set_condition(self, a0: ValueRef) -> () {
        unsafe { LLVMSetCondition(self.0, a0.0) }
    }
    pub fn get_switch_default_dest(self) -> BasicBlockRef {
        BasicBlockRef(unsafe { LLVMGetSwitchDefaultDest(self.0) })
    }
    pub fn get_allocated_type(self) -> TypeRef {
        TypeRef(unsafe { LLVMGetAllocatedType(self.0) })
    }
    pub fn is_in_bounds(self) -> LLVMBool {
        unsafe { LLVMIsInBounds(self.0) }
    }
    pub fn set_is_in_bounds(self, a0: LLVMBool) -> () {
        unsafe { LLVMSetIsInBounds(self.0, a0) }
    }
    pub fn count_incoming(self) -> ::libc::c_uint {
        unsafe { LLVMCountIncoming(self.0) }
    }
    pub fn get_incoming_value(self, a0: ::libc::c_uint) -> ValueRef {
        ValueRef(unsafe { LLVMGetIncomingValue(self.0, a0) })
    }
    pub fn get_incoming_block(self, a0: ::libc::c_uint) -> BasicBlockRef {
        BasicBlockRef(unsafe { LLVMGetIncomingBlock(self.0, a0) })
    }
    pub fn is_a_argument(self) -> ValueRef {
        ValueRef(unsafe { LLVMIsAArgument(self.0) })
    }
    pub fn is_a_basic_block(self) -> ValueRef {
        ValueRef(unsafe { LLVMIsABasicBlock(self.0) })
    }
    pub fn is_a_inline_asm(self) -> ValueRef {
        ValueRef(unsafe { LLVMIsAInlineAsm(self.0) })
    }
    pub fn is_a_user(self) -> ValueRef {
        ValueRef(unsafe { LLVMIsAUser(self.0) })
    }
    pub fn is_a_constant(self) -> ValueRef {
        ValueRef(unsafe { LLVMIsAConstant(self.0) })
    }
    pub fn is_a_block_address(self) -> ValueRef {
        ValueRef(unsafe { LLVMIsABlockAddress(self.0) })
    }
    pub fn is_a_constant_aggregate_zero(self) -> ValueRef {
        ValueRef(unsafe { LLVMIsAConstantAggregateZero(self.0) })
    }
    pub fn is_a_constant_array(self) -> ValueRef {
        ValueRef(unsafe { LLVMIsAConstantArray(self.0) })
    }
    pub fn is_a_constant_data_sequential(self) -> ValueRef {
        ValueRef(unsafe { LLVMIsAConstantDataSequential(self.0) })
    }
    pub fn is_a_constant_data_array(self) -> ValueRef {
        ValueRef(unsafe { LLVMIsAConstantDataArray(self.0) })
    }
    pub fn is_a_constant_data_vector(self) -> ValueRef {
        ValueRef(unsafe { LLVMIsAConstantDataVector(self.0) })
    }
    pub fn is_a_constant_expr(self) -> ValueRef {
        ValueRef(unsafe { LLVMIsAConstantExpr(self.0) })
    }
    pub fn is_a_constant_fp(self) -> ValueRef {
        ValueRef(unsafe { LLVMIsAConstantFP(self.0) })
    }
    pub fn is_a_constant_int(self) -> ValueRef {
        ValueRef(unsafe { LLVMIsAConstantInt(self.0) })
    }
    pub fn is_a_constant_pointer_null(self) -> ValueRef {
        ValueRef(unsafe { LLVMIsAConstantPointerNull(self.0) })
    }
    pub fn is_a_constant_struct(self) -> ValueRef {
        ValueRef(unsafe { LLVMIsAConstantStruct(self.0) })
    }
    pub fn is_a_constant_token_none(self) -> ValueRef {
        ValueRef(unsafe { LLVMIsAConstantTokenNone(self.0) })
    }
    pub fn is_a_constant_vector(self) -> ValueRef {
        ValueRef(unsafe { LLVMIsAConstantVector(self.0) })
    }
    pub fn is_a_global_value(self) -> ValueRef {
        ValueRef(unsafe { LLVMIsAGlobalValue(self.0) })
    }
    pub fn is_a_global_alias(self) -> ValueRef {
        ValueRef(unsafe { LLVMIsAGlobalAlias(self.0) })
    }
    pub fn is_a_global_i_func(self) -> ValueRef {
        ValueRef(unsafe { LLVMIsAGlobalIFunc(self.0) })
    }
    pub fn is_a_global_object(self) -> ValueRef {
        ValueRef(unsafe { LLVMIsAGlobalObject(self.0) })
    }
    pub fn is_a_function(self) -> ValueRef {
        ValueRef(unsafe { LLVMIsAFunction(self.0) })
    }
    pub fn is_a_global_variable(self) -> ValueRef {
        ValueRef(unsafe { LLVMIsAGlobalVariable(self.0) })
    }
    pub fn is_a_undef_value(self) -> ValueRef {
        ValueRef(unsafe { LLVMIsAUndefValue(self.0) })
    }
    pub fn is_a_instruction(self) -> ValueRef {
        ValueRef(unsafe { LLVMIsAInstruction(self.0) })
    }
    pub fn is_a_unary_operator(self) -> ValueRef {
        ValueRef(unsafe { LLVMIsAUnaryOperator(self.0) })
    }
    pub fn is_a_binary_operator(self) -> ValueRef {
        ValueRef(unsafe { LLVMIsABinaryOperator(self.0) })
    }
    pub fn is_a_call_inst(self) -> ValueRef {
        ValueRef(unsafe { LLVMIsACallInst(self.0) })
    }
    pub fn is_a_intrinsic_inst(self) -> ValueRef {
        ValueRef(unsafe { LLVMIsAIntrinsicInst(self.0) })
    }
    pub fn is_a_dbg_info_intrinsic(self) -> ValueRef {
        ValueRef(unsafe { LLVMIsADbgInfoIntrinsic(self.0) })
    }
    pub fn is_a_dbg_variable_intrinsic(self) -> ValueRef {
        ValueRef(unsafe { LLVMIsADbgVariableIntrinsic(self.0) })
    }
    pub fn is_a_dbg_declare_inst(self) -> ValueRef {
        ValueRef(unsafe { LLVMIsADbgDeclareInst(self.0) })
    }
    pub fn is_a_dbg_label_inst(self) -> ValueRef {
        ValueRef(unsafe { LLVMIsADbgLabelInst(self.0) })
    }
    pub fn is_a_mem_intrinsic(self) -> ValueRef {
        ValueRef(unsafe { LLVMIsAMemIntrinsic(self.0) })
    }
    pub fn is_a_mem_cpy_inst(self) -> ValueRef {
        ValueRef(unsafe { LLVMIsAMemCpyInst(self.0) })
    }
    pub fn is_a_mem_move_inst(self) -> ValueRef {
        ValueRef(unsafe { LLVMIsAMemMoveInst(self.0) })
    }
    pub fn is_a_mem_set_inst(self) -> ValueRef {
        ValueRef(unsafe { LLVMIsAMemSetInst(self.0) })
    }
    pub fn is_a_cmp_inst(self) -> ValueRef {
        ValueRef(unsafe { LLVMIsACmpInst(self.0) })
    }
    pub fn is_af_cmp_inst(self) -> ValueRef {
        ValueRef(unsafe { LLVMIsAFCmpInst(self.0) })
    }
    pub fn is_ai_cmp_inst(self) -> ValueRef {
        ValueRef(unsafe { LLVMIsAICmpInst(self.0) })
    }
    pub fn is_a_extract_element_inst(self) -> ValueRef {
        ValueRef(unsafe { LLVMIsAExtractElementInst(self.0) })
    }
    pub fn is_a_get_element_ptr_inst(self) -> ValueRef {
        ValueRef(unsafe { LLVMIsAGetElementPtrInst(self.0) })
    }
    pub fn is_a_insert_element_inst(self) -> ValueRef {
        ValueRef(unsafe { LLVMIsAInsertElementInst(self.0) })
    }
    pub fn is_a_insert_value_inst(self) -> ValueRef {
        ValueRef(unsafe { LLVMIsAInsertValueInst(self.0) })
    }
    pub fn is_a_landing_pad_inst(self) -> ValueRef {
        ValueRef(unsafe { LLVMIsALandingPadInst(self.0) })
    }
    pub fn is_aphi_node(self) -> ValueRef {
        ValueRef(unsafe { LLVMIsAPHINode(self.0) })
    }
    pub fn is_a_select_inst(self) -> ValueRef {
        ValueRef(unsafe { LLVMIsASelectInst(self.0) })
    }
    pub fn is_a_shuffle_vector_inst(self) -> ValueRef {
        ValueRef(unsafe { LLVMIsAShuffleVectorInst(self.0) })
    }
    pub fn is_a_store_inst(self) -> ValueRef {
        ValueRef(unsafe { LLVMIsAStoreInst(self.0) })
    }
    pub fn is_a_branch_inst(self) -> ValueRef {
        ValueRef(unsafe { LLVMIsABranchInst(self.0) })
    }
    pub fn is_a_indirect_br_inst(self) -> ValueRef {
        ValueRef(unsafe { LLVMIsAIndirectBrInst(self.0) })
    }
    pub fn is_a_invoke_inst(self) -> ValueRef {
        ValueRef(unsafe { LLVMIsAInvokeInst(self.0) })
    }
    pub fn is_a_return_inst(self) -> ValueRef {
        ValueRef(unsafe { LLVMIsAReturnInst(self.0) })
    }
    pub fn is_a_switch_inst(self) -> ValueRef {
        ValueRef(unsafe { LLVMIsASwitchInst(self.0) })
    }
    pub fn is_a_unreachable_inst(self) -> ValueRef {
        ValueRef(unsafe { LLVMIsAUnreachableInst(self.0) })
    }
    pub fn is_a_resume_inst(self) -> ValueRef {
        ValueRef(unsafe { LLVMIsAResumeInst(self.0) })
    }
    pub fn is_a_cleanup_return_inst(self) -> ValueRef {
        ValueRef(unsafe { LLVMIsACleanupReturnInst(self.0) })
    }
    pub fn is_a_catch_return_inst(self) -> ValueRef {
        ValueRef(unsafe { LLVMIsACatchReturnInst(self.0) })
    }
    pub fn is_a_catch_switch_inst(self) -> ValueRef {
        ValueRef(unsafe { LLVMIsACatchSwitchInst(self.0) })
    }
    pub fn is_a_call_br_inst(self) -> ValueRef {
        ValueRef(unsafe { LLVMIsACallBrInst(self.0) })
    }
    pub fn is_a_funclet_pad_inst(self) -> ValueRef {
        ValueRef(unsafe { LLVMIsAFuncletPadInst(self.0) })
    }
    pub fn is_a_catch_pad_inst(self) -> ValueRef {
        ValueRef(unsafe { LLVMIsACatchPadInst(self.0) })
    }
    pub fn is_a_cleanup_pad_inst(self) -> ValueRef {
        ValueRef(unsafe { LLVMIsACleanupPadInst(self.0) })
    }
    pub fn is_a_unary_instruction(self) -> ValueRef {
        ValueRef(unsafe { LLVMIsAUnaryInstruction(self.0) })
    }
    pub fn is_a_alloca_inst(self) -> ValueRef {
        ValueRef(unsafe { LLVMIsAAllocaInst(self.0) })
    }
    pub fn is_a_cast_inst(self) -> ValueRef {
        ValueRef(unsafe { LLVMIsACastInst(self.0) })
    }
    pub fn is_a_addr_space_cast_inst(self) -> ValueRef {
        ValueRef(unsafe { LLVMIsAAddrSpaceCastInst(self.0) })
    }
    pub fn is_a_bit_cast_inst(self) -> ValueRef {
        ValueRef(unsafe { LLVMIsABitCastInst(self.0) })
    }
    pub fn is_afp_ext_inst(self) -> ValueRef {
        ValueRef(unsafe { LLVMIsAFPExtInst(self.0) })
    }
    pub fn is_afp_to_si_inst(self) -> ValueRef {
        ValueRef(unsafe { LLVMIsAFPToSIInst(self.0) })
    }
    pub fn is_afp_to_ui_inst(self) -> ValueRef {
        ValueRef(unsafe { LLVMIsAFPToUIInst(self.0) })
    }
    pub fn is_afp_trunc_inst(self) -> ValueRef {
        ValueRef(unsafe { LLVMIsAFPTruncInst(self.0) })
    }
    pub fn is_a_int_to_ptr_inst(self) -> ValueRef {
        ValueRef(unsafe { LLVMIsAIntToPtrInst(self.0) })
    }
    pub fn is_a_ptr_to_int_inst(self) -> ValueRef {
        ValueRef(unsafe { LLVMIsAPtrToIntInst(self.0) })
    }
    pub fn is_as_ext_inst(self) -> ValueRef {
        ValueRef(unsafe { LLVMIsASExtInst(self.0) })
    }
    pub fn is_asi_to_fp_inst(self) -> ValueRef {
        ValueRef(unsafe { LLVMIsASIToFPInst(self.0) })
    }
    pub fn is_a_trunc_inst(self) -> ValueRef {
        ValueRef(unsafe { LLVMIsATruncInst(self.0) })
    }
    pub fn is_aui_to_fp_inst(self) -> ValueRef {
        ValueRef(unsafe { LLVMIsAUIToFPInst(self.0) })
    }
    pub fn is_az_ext_inst(self) -> ValueRef {
        ValueRef(unsafe { LLVMIsAZExtInst(self.0) })
    }
    pub fn is_a_extract_value_inst(self) -> ValueRef {
        ValueRef(unsafe { LLVMIsAExtractValueInst(self.0) })
    }
    pub fn is_a_load_inst(self) -> ValueRef {
        ValueRef(unsafe { LLVMIsALoadInst(self.0) })
    }
    pub fn is_ava_arg_inst(self) -> ValueRef {
        ValueRef(unsafe { LLVMIsAVAArgInst(self.0) })
    }
    pub fn is_a_freeze_inst(self) -> ValueRef {
        ValueRef(unsafe { LLVMIsAFreezeInst(self.0) })
    }
    pub fn is_a_atomic_cmp_xchg_inst(self) -> ValueRef {
        ValueRef(unsafe { LLVMIsAAtomicCmpXchgInst(self.0) })
    }
    pub fn is_a_atomic_rmw_inst(self) -> ValueRef {
        ValueRef(unsafe { LLVMIsAAtomicRMWInst(self.0) })
    }
    pub fn is_a_fence_inst(self) -> ValueRef {
        ValueRef(unsafe { LLVMIsAFenceInst(self.0) })
    }
    pub fn get_num_indices(self) -> ::libc::c_uint {
        unsafe { LLVMGetNumIndices(self.0) }
    }
    pub fn get_indices(self) -> *const ::libc::c_uint {
        unsafe { LLVMGetIndices(self.0) }
    }
    pub fn add_case(self, a0: ValueRef, a1: BasicBlockRef) -> () {
        unsafe { LLVMAddCase(self.0, a0.0, a1.0) }
    }
    pub fn add_destination(self, a0: BasicBlockRef) -> () {
        unsafe { LLVMAddDestination(self.0, a0.0) }
    }
    pub fn get_num_clauses(self) -> ::libc::c_uint {
        unsafe { LLVMGetNumClauses(self.0) }
    }
    pub fn get_clause(self, a0: ::libc::c_uint) -> ValueRef {
        ValueRef(unsafe { LLVMGetClause(self.0, a0) })
    }
    pub fn add_clause(self, a0: ValueRef) -> () {
        unsafe { LLVMAddClause(self.0, a0.0) }
    }
    pub fn is_cleanup(self) -> LLVMBool {
        unsafe { LLVMIsCleanup(self.0) }
    }
    pub fn set_cleanup(self, a0: LLVMBool) -> () {
        unsafe { LLVMSetCleanup(self.0, a0) }
    }
    pub fn add_handler(self, a0: BasicBlockRef) -> () {
        unsafe { LLVMAddHandler(self.0, a0.0) }
    }
    pub fn get_num_handlers(self) -> ::libc::c_uint {
        unsafe { LLVMGetNumHandlers(self.0) }
    }
    pub fn get_handlers(self, a0: *mut LLVMBasicBlockRef) -> () {
        unsafe { LLVMGetHandlers(self.0, a0) }
    }
    pub fn get_arg_operand(self, a0: ::libc::c_uint) -> ValueRef {
        ValueRef(unsafe { LLVMGetArgOperand(self.0, a0) })
    }
    pub fn set_arg_operand(self, a0: ::libc::c_uint, a1: ValueRef) -> () {
        unsafe { LLVMSetArgOperand(self.0, a0, a1.0) }
    }
    pub fn get_parent_catch_switch(self) -> ValueRef {
        ValueRef(unsafe { LLVMGetParentCatchSwitch(self.0) })
    }
    pub fn set_parent_catch_switch(self, a0: ValueRef) -> () {
        unsafe { LLVMSetParentCatchSwitch(self.0, a0.0) }
    }
    pub fn get_volatile(self) -> LLVMBool {
        unsafe { LLVMGetVolatile(self.0) }
    }
    pub fn set_volatile(self, a0: LLVMBool) -> () {
        unsafe { LLVMSetVolatile(self.0, a0) }
    }
    pub fn get_weak(self) -> LLVMBool {
        unsafe { LLVMGetWeak(self.0) }
    }
    pub fn set_weak(self, a0: LLVMBool) -> () {
        unsafe { LLVMSetWeak(self.0, a0) }
    }
    pub fn get_ordering(self) -> LLVMAtomicOrdering {
        unsafe { LLVMGetOrdering(self.0) }
    }
    pub fn set_ordering(self, a0: LLVMAtomicOrdering) -> () {
        unsafe { LLVMSetOrdering(self.0, a0) }
    }
    pub fn get_atomic_rmw_bin_op(self) -> LLVMAtomicRMWBinOp {
        unsafe { LLVMGetAtomicRMWBinOp(self.0) }
    }
    pub fn set_atomic_rmw_bin_op(self, a0: LLVMAtomicRMWBinOp) -> () {
        unsafe { LLVMSetAtomicRMWBinOp(self.0, a0) }
    }
    pub fn get_num_mask_elements(self) -> ::libc::c_uint {
        unsafe { LLVMGetNumMaskElements(self.0) }
    }
    pub fn get_mask_value(self, a0: ::libc::c_uint) -> ::libc::c_int {
        unsafe { LLVMGetMaskValue(self.0, a0) }
    }
    pub fn is_atomic_single_thread(self) -> LLVMBool {
        unsafe { LLVMIsAtomicSingleThread(self.0) }
    }
    pub fn set_atomic_single_thread(self, a0: LLVMBool) -> () {
        unsafe { LLVMSetAtomicSingleThread(self.0, a0) }
    }
    pub fn get_cmp_xchg_success_ordering(self) -> LLVMAtomicOrdering {
        unsafe { LLVMGetCmpXchgSuccessOrdering(self.0) }
    }
    pub fn set_cmp_xchg_success_ordering(self, a0: LLVMAtomicOrdering) -> () {
        unsafe { LLVMSetCmpXchgSuccessOrdering(self.0, a0) }
    }
    pub fn get_cmp_xchg_failure_ordering(self) -> LLVMAtomicOrdering {
        unsafe { LLVMGetCmpXchgFailureOrdering(self.0) }
    }
    pub fn set_cmp_xchg_failure_ordering(self, a0: LLVMAtomicOrdering) -> () {
        unsafe { LLVMSetCmpXchgFailureOrdering(self.0, a0) }
    }
}
impl BasicBlockRef {
    pub fn basic_block_as_value(self) -> ValueRef {
        ValueRef(unsafe { LLVMBasicBlockAsValue(self.0) })
    }
    pub fn get_basic_block_name(self) -> *const ::libc::c_char {
        unsafe { LLVMGetBasicBlockName(self.0) }
    }
    pub fn get_basic_block_parent(self) -> ValueRef {
        ValueRef(unsafe { LLVMGetBasicBlockParent(self.0) })
    }
    pub fn get_basic_block_terminator(self) -> ValueRef {
        ValueRef(unsafe { LLVMGetBasicBlockTerminator(self.0) })
    }
    pub fn get_next_basic_block(self) -> BasicBlockRef {
        BasicBlockRef(unsafe { LLVMGetNextBasicBlock(self.0) })
    }
    pub fn get_previous_basic_block(self) -> BasicBlockRef {
        BasicBlockRef(unsafe { LLVMGetPreviousBasicBlock(self.0) })
    }
    pub fn delete_basic_block(self) -> () {
        unsafe { LLVMDeleteBasicBlock(self.0) }
    }
    pub fn remove_basic_block_from_parent(self) -> () {
        unsafe { LLVMRemoveBasicBlockFromParent(self.0) }
    }
    pub fn move_basic_block_before(self, a0: BasicBlockRef) -> () {
        unsafe { LLVMMoveBasicBlockBefore(self.0, a0.0) }
    }
    pub fn move_basic_block_after(self, a0: BasicBlockRef) -> () {
        unsafe { LLVMMoveBasicBlockAfter(self.0, a0.0) }
    }
    pub fn get_first_instruction(self) -> ValueRef {
        ValueRef(unsafe { LLVMGetFirstInstruction(self.0) })
    }
    pub fn get_last_instruction(self) -> ValueRef {
        ValueRef(unsafe { LLVMGetLastInstruction(self.0) })
    }
}
impl ExecutionEngineRef {
    pub fn dispose_execution_engine(self) -> () {
        unsafe { LLVMDisposeExecutionEngine(self.0) }
    }
    pub fn run_static_constructors(self) -> () {
        unsafe { LLVMRunStaticConstructors(self.0) }
    }
    pub fn run_static_destructors(self) -> () {
        unsafe { LLVMRunStaticDestructors(self.0) }
    }
    pub fn free_machine_code_for_function(self, a0: ValueRef) -> () {
        unsafe { LLVMFreeMachineCodeForFunction(self.0, a0.0) }
    }
    pub fn add_module(self, a0: ModuleRef) -> () {
        unsafe { LLVMAddModule(self.0, a0.0) }
    }
    pub fn get_execution_engine_target_data(self) -> TargetDataRef {
        TargetDataRef(unsafe { LLVMGetExecutionEngineTargetData(self.0) })
    }
    pub fn get_execution_engine_target_machine(self) -> TargetMachineRef {
        TargetMachineRef(unsafe { LLVMGetExecutionEngineTargetMachine(self.0) })
    }
    pub fn get_function_address(self, a0: *const ::libc::c_char) -> u64 {
        unsafe { LLVMGetFunctionAddress(self.0, a0) }
    }
}
impl OrcLLJITRef {
    pub fn orc_dispose_lljit(self) -> ErrorRef {
        ErrorRef(unsafe { LLVMOrcDisposeLLJIT(self.0) })
    }
    pub fn orc_lljit_get_execution_session(self) -> LLVMOrcExecutionSessionRef {
        unsafe { LLVMOrcLLJITGetExecutionSession(self.0) }
    }
    pub fn orc_lljit_get_main_jit_dylib(self) -> LLVMOrcJITDylibRef {
        unsafe { LLVMOrcLLJITGetMainJITDylib(self.0) }
    }
    pub fn orc_lljit_get_triple_string(self) -> *const ::libc::c_char {
        unsafe { LLVMOrcLLJITGetTripleString(self.0) }
    }
    pub fn orc_lljit_get_global_prefix(self) -> ::libc::c_char {
        unsafe { LLVMOrcLLJITGetGlobalPrefix(self.0) }
    }
}
impl OrcLLJITBuilderRef {
    pub fn orc_dispose_lljit_builder(self) -> () {
        unsafe { LLVMOrcDisposeLLJITBuilder(self.0) }
    }
}
impl SymbolIteratorRef {
    pub fn dispose_symbol_iterator(self) -> () {
        unsafe { LLVMDisposeSymbolIterator(self.0) }
    }
    pub fn move_to_next_symbol(self) -> () {
        unsafe { LLVMMoveToNextSymbol(self.0) }
    }
    pub fn get_symbol_name(self) -> *const ::libc::c_char {
        unsafe { LLVMGetSymbolName(self.0) }
    }
    pub fn get_symbol_address(self) -> u64 {
        unsafe { LLVMGetSymbolAddress(self.0) }
    }
    pub fn get_symbol_size(self) -> u64 {
        unsafe { LLVMGetSymbolSize(self.0) }
    }
}
impl MCJITMemoryManagerRef {
    pub fn dispose_mcjit_memory_manager(self) -> () {
        unsafe { LLVMDisposeMCJITMemoryManager(self.0) }
    }
}
impl PassManagerRef {
    pub fn add_lower_switch_pass(self) -> () {
        unsafe { LLVMAddLowerSwitchPass(self.0) }
    }
    pub fn add_promote_memory_to_register_pass(self) -> () {
        unsafe { LLVMAddPromoteMemoryToRegisterPass(self.0) }
    }
}
impl TargetDataRef {
    pub fn copy_string_rep_of_target_data(self) -> *mut ::libc::c_char {
        unsafe { LLVMCopyStringRepOfTargetData(self.0) }
    }
    pub fn byte_order(self) -> LLVMByteOrdering {
        unsafe { LLVMByteOrder(self.0) }
    }
    pub fn pointer_size(self) -> ::libc::c_uint {
        unsafe { LLVMPointerSize(self.0) }
    }
    pub fn pointer_size_for_as(self, a0: ::libc::c_uint) -> ::libc::c_uint {
        unsafe { LLVMPointerSizeForAS(self.0, a0) }
    }
    pub fn int_ptr_type(self) -> TypeRef {
        TypeRef(unsafe { LLVMIntPtrType(self.0) })
    }
    pub fn int_ptr_type_for_as(self, a0: ::libc::c_uint) -> TypeRef {
        TypeRef(unsafe { LLVMIntPtrTypeForAS(self.0, a0) })
    }
    pub fn size_of_type_in_bits(self, a0: TypeRef) -> ::libc::c_ulonglong {
        unsafe { LLVMSizeOfTypeInBits(self.0, a0.0) }
    }
    pub fn store_size_of_type(self, a0: TypeRef) -> ::libc::c_ulonglong {
        unsafe { LLVMStoreSizeOfType(self.0, a0.0) }
    }
    pub fn abi_size_of_type(self, a0: TypeRef) -> ::libc::c_ulonglong {
        unsafe { LLVMABISizeOfType(self.0, a0.0) }
    }
    pub fn abi_alignment_of_type(self, a0: TypeRef) -> ::libc::c_uint {
        unsafe { LLVMABIAlignmentOfType(self.0, a0.0) }
    }
    pub fn call_frame_alignment_of_type(self, a0: TypeRef) -> ::libc::c_uint {
        unsafe { LLVMCallFrameAlignmentOfType(self.0, a0.0) }
    }
    pub fn preferred_alignment_of_type(self, a0: TypeRef) -> ::libc::c_uint {
        unsafe { LLVMPreferredAlignmentOfType(self.0, a0.0) }
    }
    pub fn dispose_target_data(self) -> () {
        unsafe { LLVMDisposeTargetData(self.0) }
    }
}
impl ModuleProviderRef {
    pub fn dispose_module_provider(self) -> () {
        unsafe { LLVMDisposeModuleProvider(self.0) }
    }
    pub fn create_function_pass_manager(self) -> PassManagerRef {
        PassManagerRef(unsafe { LLVMCreateFunctionPassManager(self.0) })
    }
}
impl OrcThreadSafeContextRef {
    pub fn orc_thread_safe_context_get_context(self) -> ContextRef {
        ContextRef(unsafe { LLVMOrcThreadSafeContextGetContext(self.0) })
    }
    pub fn orc_dispose_thread_safe_context(self) -> () {
        unsafe { LLVMOrcDisposeThreadSafeContext(self.0) }
    }
}
impl OrcJITStackRef {
    pub fn orc_get_error_msg(self) -> *const ::libc::c_char {
        unsafe { LLVMOrcGetErrorMsg(self.0) }
    }
    pub fn orc_dispose_instance(self) -> ErrorRef {
        ErrorRef(unsafe { LLVMOrcDisposeInstance(self.0) })
    }
}
impl PassManagerRef {
    pub fn run_pass_manager(self, a0: ModuleRef) -> LLVMBool {
        unsafe { LLVMRunPassManager(self.0, a0.0) }
    }
    pub fn initialize_function_pass_manager(self) -> LLVMBool {
        unsafe { LLVMInitializeFunctionPassManager(self.0) }
    }
    pub fn run_function_pass_manager(self, a0: ValueRef) -> LLVMBool {
        unsafe { LLVMRunFunctionPassManager(self.0, a0.0) }
    }
    pub fn finalize_function_pass_manager(self) -> LLVMBool {
        unsafe { LLVMFinalizeFunctionPassManager(self.0) }
    }
    pub fn dispose_pass_manager(self) -> () {
        unsafe { LLVMDisposePassManager(self.0) }
    }
    pub fn add_aggressive_inst_combiner_pass(self) -> () {
        unsafe { LLVMAddAggressiveInstCombinerPass(self.0) }
    }
    pub fn add_argument_promotion_pass(self) -> () {
        unsafe { LLVMAddArgumentPromotionPass(self.0) }
    }
    pub fn add_constant_merge_pass(self) -> () {
        unsafe { LLVMAddConstantMergePass(self.0) }
    }
    pub fn add_merge_functions_pass(self) -> () {
        unsafe { LLVMAddMergeFunctionsPass(self.0) }
    }
    pub fn add_called_value_propagation_pass(self) -> () {
        unsafe { LLVMAddCalledValuePropagationPass(self.0) }
    }
    pub fn add_dead_arg_elimination_pass(self) -> () {
        unsafe { LLVMAddDeadArgEliminationPass(self.0) }
    }
    pub fn add_function_attrs_pass(self) -> () {
        unsafe { LLVMAddFunctionAttrsPass(self.0) }
    }
    pub fn add_function_inlining_pass(self) -> () {
        unsafe { LLVMAddFunctionInliningPass(self.0) }
    }
    pub fn add_always_inliner_pass(self) -> () {
        unsafe { LLVMAddAlwaysInlinerPass(self.0) }
    }
    pub fn add_global_dce_pass(self) -> () {
        unsafe { LLVMAddGlobalDCEPass(self.0) }
    }
    pub fn add_global_optimizer_pass(self) -> () {
        unsafe { LLVMAddGlobalOptimizerPass(self.0) }
    }
    pub fn add_ip_constant_propagation_pass(self) -> () {
        unsafe { LLVMAddIPConstantPropagationPass(self.0) }
    }
    pub fn add_prune_eh_pass(self) -> () {
        unsafe { LLVMAddPruneEHPass(self.0) }
    }
    pub fn add_ipsccp_pass(self) -> () {
        unsafe { LLVMAddIPSCCPPass(self.0) }
    }
    pub fn add_internalize_pass(self, a0: ::libc::c_uint) -> () {
        unsafe { LLVMAddInternalizePass(self.0, a0) }
    }
    pub fn add_strip_dead_prototypes_pass(self) -> () {
        unsafe { LLVMAddStripDeadPrototypesPass(self.0) }
    }
    pub fn add_strip_symbols_pass(self) -> () {
        unsafe { LLVMAddStripSymbolsPass(self.0) }
    }
    pub fn add_instruction_combining_pass(self) -> () {
        unsafe { LLVMAddInstructionCombiningPass(self.0) }
    }
    pub fn add_aggressive_dce_pass(self) -> () {
        unsafe { LLVMAddAggressiveDCEPass(self.0) }
    }
    pub fn add_dce_pass(self) -> () {
        unsafe { LLVMAddDCEPass(self.0) }
    }
    pub fn add_bit_tracking_dce_pass(self) -> () {
        unsafe { LLVMAddBitTrackingDCEPass(self.0) }
    }
    pub fn add_alignment_from_assumptions_pass(self) -> () {
        unsafe { LLVMAddAlignmentFromAssumptionsPass(self.0) }
    }
    pub fn add_cfg_simplification_pass(self) -> () {
        unsafe { LLVMAddCFGSimplificationPass(self.0) }
    }
    pub fn add_dead_store_elimination_pass(self) -> () {
        unsafe { LLVMAddDeadStoreEliminationPass(self.0) }
    }
    pub fn add_scalarizer_pass(self) -> () {
        unsafe { LLVMAddScalarizerPass(self.0) }
    }
    pub fn add_merged_load_store_motion_pass(self) -> () {
        unsafe { LLVMAddMergedLoadStoreMotionPass(self.0) }
    }
    pub fn add_gvn_pass(self) -> () {
        unsafe { LLVMAddGVNPass(self.0) }
    }
    pub fn add_new_gvn_pass(self) -> () {
        unsafe { LLVMAddNewGVNPass(self.0) }
    }
    pub fn add_ind_var_simplify_pass(self) -> () {
        unsafe { LLVMAddIndVarSimplifyPass(self.0) }
    }
    pub fn add_jump_threading_pass(self) -> () {
        unsafe { LLVMAddJumpThreadingPass(self.0) }
    }
    pub fn add_licm_pass(self) -> () {
        unsafe { LLVMAddLICMPass(self.0) }
    }
    pub fn add_loop_deletion_pass(self) -> () {
        unsafe { LLVMAddLoopDeletionPass(self.0) }
    }
    pub fn add_loop_idiom_pass(self) -> () {
        unsafe { LLVMAddLoopIdiomPass(self.0) }
    }
    pub fn add_loop_rotate_pass(self) -> () {
        unsafe { LLVMAddLoopRotatePass(self.0) }
    }
    pub fn add_loop_reroll_pass(self) -> () {
        unsafe { LLVMAddLoopRerollPass(self.0) }
    }
    pub fn add_loop_unroll_pass(self) -> () {
        unsafe { LLVMAddLoopUnrollPass(self.0) }
    }
    pub fn add_loop_unroll_and_jam_pass(self) -> () {
        unsafe { LLVMAddLoopUnrollAndJamPass(self.0) }
    }
    pub fn add_loop_unswitch_pass(self) -> () {
        unsafe { LLVMAddLoopUnswitchPass(self.0) }
    }
    pub fn add_lower_atomic_pass(self) -> () {
        unsafe { LLVMAddLowerAtomicPass(self.0) }
    }
    pub fn add_mem_cpy_opt_pass(self) -> () {
        unsafe { LLVMAddMemCpyOptPass(self.0) }
    }
    pub fn add_partially_inline_lib_calls_pass(self) -> () {
        unsafe { LLVMAddPartiallyInlineLibCallsPass(self.0) }
    }
    pub fn add_reassociate_pass(self) -> () {
        unsafe { LLVMAddReassociatePass(self.0) }
    }
    pub fn add_sccp_pass(self) -> () {
        unsafe { LLVMAddSCCPPass(self.0) }
    }
    pub fn add_scalar_repl_aggregates_pass(self) -> () {
        unsafe { LLVMAddScalarReplAggregatesPass(self.0) }
    }
    pub fn add_scalar_repl_aggregates_pass_ssa(self) -> () {
        unsafe { LLVMAddScalarReplAggregatesPassSSA(self.0) }
    }
    pub fn add_simplify_lib_calls_pass(self) -> () {
        unsafe { LLVMAddSimplifyLibCallsPass(self.0) }
    }
    pub fn add_tail_call_elimination_pass(self) -> () {
        unsafe { LLVMAddTailCallEliminationPass(self.0) }
    }
    pub fn add_constant_propagation_pass(self) -> () {
        unsafe { LLVMAddConstantPropagationPass(self.0) }
    }
    pub fn add_demote_memory_to_register_pass(self) -> () {
        unsafe { LLVMAddDemoteMemoryToRegisterPass(self.0) }
    }
    pub fn add_verifier_pass(self) -> () {
        unsafe { LLVMAddVerifierPass(self.0) }
    }
    pub fn add_correlated_value_propagation_pass(self) -> () {
        unsafe { LLVMAddCorrelatedValuePropagationPass(self.0) }
    }
    pub fn add_early_cse_pass(self) -> () {
        unsafe { LLVMAddEarlyCSEPass(self.0) }
    }
    pub fn add_early_cse_mem_ssa_pass(self) -> () {
        unsafe { LLVMAddEarlyCSEMemSSAPass(self.0) }
    }
    pub fn add_lower_expect_intrinsic_pass(self) -> () {
        unsafe { LLVMAddLowerExpectIntrinsicPass(self.0) }
    }
    pub fn add_lower_constant_intrinsics_pass(self) -> () {
        unsafe { LLVMAddLowerConstantIntrinsicsPass(self.0) }
    }
    pub fn add_type_based_alias_analysis_pass(self) -> () {
        unsafe { LLVMAddTypeBasedAliasAnalysisPass(self.0) }
    }
    pub fn add_scoped_no_alias_aa_pass(self) -> () {
        unsafe { LLVMAddScopedNoAliasAAPass(self.0) }
    }
    pub fn add_basic_alias_analysis_pass(self) -> () {
        unsafe { LLVMAddBasicAliasAnalysisPass(self.0) }
    }
    pub fn add_unify_function_exit_nodes_pass(self) -> () {
        unsafe { LLVMAddUnifyFunctionExitNodesPass(self.0) }
    }
    pub fn add_coro_early_pass(self) -> () {
        unsafe { LLVMAddCoroEarlyPass(self.0) }
    }
    pub fn add_coro_split_pass(self) -> () {
        unsafe { LLVMAddCoroSplitPass(self.0) }
    }
    pub fn add_coro_elide_pass(self) -> () {
        unsafe { LLVMAddCoroElidePass(self.0) }
    }
    pub fn add_coro_cleanup_pass(self) -> () {
        unsafe { LLVMAddCoroCleanupPass(self.0) }
    }
    pub fn add_add_discriminators_pass(self) -> () {
        unsafe { LLVMAddAddDiscriminatorsPass(self.0) }
    }
    pub fn add_loop_vectorize_pass(self) -> () {
        unsafe { LLVMAddLoopVectorizePass(self.0) }
    }
    pub fn add_slp_vectorize_pass(self) -> () {
        unsafe { LLVMAddSLPVectorizePass(self.0) }
    }
}
impl PassManagerBuilderRef {
    pub fn pass_manager_builder_dispose(self) -> () {
        unsafe { LLVMPassManagerBuilderDispose(self.0) }
    }
}
impl SectionIteratorRef {
    pub fn dispose_section_iterator(self) -> () {
        unsafe { LLVMDisposeSectionIterator(self.0) }
    }
    pub fn move_to_next_section(self) -> () {
        unsafe { LLVMMoveToNextSection(self.0) }
    }
    pub fn move_to_containing_section(self, a0: SymbolIteratorRef) -> () {
        unsafe { LLVMMoveToContainingSection(self.0, a0.0) }
    }
    pub fn get_section_name(self) -> *const ::libc::c_char {
        unsafe { LLVMGetSectionName(self.0) }
    }
    pub fn get_section_size(self) -> u64 {
        unsafe { LLVMGetSectionSize(self.0) }
    }
    pub fn get_section_contents(self) -> *const ::libc::c_char {
        unsafe { LLVMGetSectionContents(self.0) }
    }
    pub fn get_section_address(self) -> u64 {
        unsafe { LLVMGetSectionAddress(self.0) }
    }
    pub fn get_relocations(self) -> RelocationIteratorRef {
        RelocationIteratorRef(unsafe { LLVMGetRelocations(self.0) })
    }
}
impl RemarkParserRef {
    pub fn remark_parser_get_next(self) -> RemarkEntryRef {
        RemarkEntryRef(unsafe { LLVMRemarkParserGetNext(self.0) })
    }
    pub fn remark_parser_has_error(self) -> LLVMBool {
        unsafe { LLVMRemarkParserHasError(self.0) }
    }
    pub fn remark_parser_get_error_message(self) -> *const ::libc::c_char {
        unsafe { LLVMRemarkParserGetErrorMessage(self.0) }
    }
    pub fn remark_parser_dispose(self) -> () {
        unsafe { LLVMRemarkParserDispose(self.0) }
    }
}
impl ObjectFileRef {
    pub fn dispose_object_file(self) -> () {
        unsafe { LLVMDisposeObjectFile(self.0) }
    }
    pub fn get_sections(self) -> SectionIteratorRef {
        SectionIteratorRef(unsafe { LLVMGetSections(self.0) })
    }
    pub fn get_symbols(self) -> SymbolIteratorRef {
        SymbolIteratorRef(unsafe { LLVMGetSymbols(self.0) })
    }
}
impl MetadataRef {
    pub fn di_location_get_line(self) -> ::libc::c_uint {
        unsafe { LLVMDILocationGetLine(self.0) }
    }
    pub fn di_location_get_column(self) -> ::libc::c_uint {
        unsafe { LLVMDILocationGetColumn(self.0) }
    }
    pub fn di_location_get_scope(self) -> MetadataRef {
        MetadataRef(unsafe { LLVMDILocationGetScope(self.0) })
    }
    pub fn di_location_get_inlined_at(self) -> MetadataRef {
        MetadataRef(unsafe { LLVMDILocationGetInlinedAt(self.0) })
    }
    pub fn di_scope_get_file(self) -> MetadataRef {
        MetadataRef(unsafe { LLVMDIScopeGetFile(self.0) })
    }
    pub fn di_type_get_size_in_bits(self) -> u64 {
        unsafe { LLVMDITypeGetSizeInBits(self.0) }
    }
    pub fn di_type_get_offset_in_bits(self) -> u64 {
        unsafe { LLVMDITypeGetOffsetInBits(self.0) }
    }
    pub fn di_type_get_align_in_bits(self) -> u32 {
        unsafe { LLVMDITypeGetAlignInBits(self.0) }
    }
    pub fn di_type_get_line(self) -> ::libc::c_uint {
        unsafe { LLVMDITypeGetLine(self.0) }
    }
    pub fn di_type_get_flags(self) -> LLVMDIFlags {
        unsafe { LLVMDITypeGetFlags(self.0) }
    }
    pub fn di_global_variable_expression_get_variable(self) -> MetadataRef {
        MetadataRef(unsafe { LLVMDIGlobalVariableExpressionGetVariable(self.0) })
    }
    pub fn di_global_variable_expression_get_expression(self) -> MetadataRef {
        MetadataRef(unsafe { LLVMDIGlobalVariableExpressionGetExpression(self.0) })
    }
    pub fn di_variable_get_file(self) -> MetadataRef {
        MetadataRef(unsafe { LLVMDIVariableGetFile(self.0) })
    }
    pub fn di_variable_get_scope(self) -> MetadataRef {
        MetadataRef(unsafe { LLVMDIVariableGetScope(self.0) })
    }
    pub fn di_variable_get_line(self) -> ::libc::c_uint {
        unsafe { LLVMDIVariableGetLine(self.0) }
    }
    pub fn dispose_temporary_md_node(self) -> () {
        unsafe { LLVMDisposeTemporaryMDNode(self.0) }
    }
    pub fn di_subprogram_get_line(self) -> ::libc::c_uint {
        unsafe { LLVMDISubprogramGetLine(self.0) }
    }
    pub fn get_metadata_kind(self) -> LLVMMetadataKind {
        unsafe { LLVMGetMetadataKind(self.0) }
    }
}
impl RemarkEntryRef {
    pub fn remark_entry_dispose(self) -> () {
        unsafe { LLVMRemarkEntryDispose(self.0) }
    }
    pub fn remark_entry_get_type(self) -> LLVMRemarkType {
        unsafe { LLVMRemarkEntryGetType(self.0) }
    }
    pub fn remark_entry_get_pass_name(self) -> RemarkStringRef {
        RemarkStringRef(unsafe { LLVMRemarkEntryGetPassName(self.0) })
    }
    pub fn remark_entry_get_remark_name(self) -> RemarkStringRef {
        RemarkStringRef(unsafe { LLVMRemarkEntryGetRemarkName(self.0) })
    }
    pub fn remark_entry_get_function_name(self) -> RemarkStringRef {
        RemarkStringRef(unsafe { LLVMRemarkEntryGetFunctionName(self.0) })
    }
    pub fn remark_entry_get_debug_loc(self) -> RemarkDebugLocRef {
        RemarkDebugLocRef(unsafe { LLVMRemarkEntryGetDebugLoc(self.0) })
    }
    pub fn remark_entry_get_hotness(self) -> u64 {
        unsafe { LLVMRemarkEntryGetHotness(self.0) }
    }
    pub fn remark_entry_get_num_args(self) -> u32 {
        unsafe { LLVMRemarkEntryGetNumArgs(self.0) }
    }
    pub fn remark_entry_get_first_arg(self) -> RemarkArgRef {
        RemarkArgRef(unsafe { LLVMRemarkEntryGetFirstArg(self.0) })
    }
}
impl FatalErrorHandler {
    pub fn install_fatal_error_handler(self) -> () {
        unsafe { LLVMInstallFatalErrorHandler(self.0) }
    }
}
impl DiagnosticInfoRef {
    pub fn get_diag_info_description(self) -> *mut ::libc::c_char {
        unsafe { LLVMGetDiagInfoDescription(self.0) }
    }
    pub fn get_diag_info_severity(self) -> LLVMDiagnosticSeverity {
        unsafe { LLVMGetDiagInfoSeverity(self.0) }
    }
}
impl PassRegistryRef {
    pub fn initialize_core(self) -> () {
        unsafe { LLVMInitializeCore(self.0) }
    }
    pub fn initialize_transform_utils(self) -> () {
        unsafe { LLVMInitializeTransformUtils(self.0) }
    }
    pub fn initialize_scalar_opts(self) -> () {
        unsafe { LLVMInitializeScalarOpts(self.0) }
    }
    pub fn initialize_obj_carc_opts(self) -> () {
        unsafe { LLVMInitializeObjCARCOpts(self.0) }
    }
    pub fn initialize_vectorization(self) -> () {
        unsafe { LLVMInitializeVectorization(self.0) }
    }
    pub fn initialize_inst_combine(self) -> () {
        unsafe { LLVMInitializeInstCombine(self.0) }
    }
    pub fn initialize_aggressive_inst_combiner(self) -> () {
        unsafe { LLVMInitializeAggressiveInstCombiner(self.0) }
    }
    pub fn initialize_ipo(self) -> () {
        unsafe { LLVMInitializeIPO(self.0) }
    }
    pub fn initialize_instrumentation(self) -> () {
        unsafe { LLVMInitializeInstrumentation(self.0) }
    }
    pub fn initialize_analysis(self) -> () {
        unsafe { LLVMInitializeAnalysis(self.0) }
    }
    pub fn initialize_ipa(self) -> () {
        unsafe { LLVMInitializeIPA(self.0) }
    }
    pub fn initialize_code_gen(self) -> () {
        unsafe { LLVMInitializeCodeGen(self.0) }
    }
    pub fn initialize_target(self) -> () {
        unsafe { LLVMInitializeTarget(self.0) }
    }
}
impl UseRef {
    pub fn get_next_use(self) -> UseRef {
        UseRef(unsafe { LLVMGetNextUse(self.0) })
    }
    pub fn get_user(self) -> ValueRef {
        ValueRef(unsafe { LLVMGetUser(self.0) })
    }
    pub fn get_used_value(self) -> ValueRef {
        ValueRef(unsafe { LLVMGetUsedValue(self.0) })
    }
}
impl OrcJITDylibDefinitionGeneratorRef {
    pub fn orc_dispose_jit_dylib_definition_generator(self) -> () {
        unsafe { LLVMOrcDisposeJITDylibDefinitionGenerator(self.0) }
    }
}
impl OrcThreadSafeModuleRef {
    pub fn orc_dispose_thread_safe_module(self) -> () {
        unsafe { LLVMOrcDisposeThreadSafeModule(self.0) }
    }
}
impl TargetLibraryInfoRef {
    pub fn add_target_library_info(self, a0: PassManagerRef) -> () {
        unsafe { LLVMAddTargetLibraryInfo(self.0, a0.0) }
    }
}
impl TypeRef {
    pub fn get_type_kind(self) -> LLVMTypeKind {
        unsafe { LLVMGetTypeKind(self.0) }
    }
    pub fn type_is_sized(self) -> LLVMBool {
        unsafe { LLVMTypeIsSized(self.0) }
    }
    pub fn get_type_context(self) -> ContextRef {
        ContextRef(unsafe { LLVMGetTypeContext(self.0) })
    }
    pub fn dump_type(self) -> () {
        unsafe { LLVMDumpType(self.0) }
    }
    pub fn print_type_to_string(self) -> *mut ::libc::c_char {
        unsafe { LLVMPrintTypeToString(self.0) }
    }
    pub fn get_int_type_width(self) -> ::libc::c_uint {
        unsafe { LLVMGetIntTypeWidth(self.0) }
    }
    pub fn is_function_var_arg(self) -> LLVMBool {
        unsafe { LLVMIsFunctionVarArg(self.0) }
    }
    pub fn get_return_type(self) -> TypeRef {
        TypeRef(unsafe { LLVMGetReturnType(self.0) })
    }
    pub fn count_param_types(self) -> ::libc::c_uint {
        unsafe { LLVMCountParamTypes(self.0) }
    }
    pub fn get_param_types(self, a0: *mut LLVMTypeRef) -> () {
        unsafe { LLVMGetParamTypes(self.0, a0) }
    }
    pub fn get_struct_name(self) -> *const ::libc::c_char {
        unsafe { LLVMGetStructName(self.0) }
    }
    pub fn count_struct_element_types(self) -> ::libc::c_uint {
        unsafe { LLVMCountStructElementTypes(self.0) }
    }
    pub fn get_struct_element_types(self, a0: *mut LLVMTypeRef) -> () {
        unsafe { LLVMGetStructElementTypes(self.0, a0) }
    }
    pub fn struct_get_type_at_index(self, a0: ::libc::c_uint) -> TypeRef {
        TypeRef(unsafe { LLVMStructGetTypeAtIndex(self.0, a0) })
    }
    pub fn is_packed_struct(self) -> LLVMBool {
        unsafe { LLVMIsPackedStruct(self.0) }
    }
    pub fn is_opaque_struct(self) -> LLVMBool {
        unsafe { LLVMIsOpaqueStruct(self.0) }
    }
    pub fn is_literal_struct(self) -> LLVMBool {
        unsafe { LLVMIsLiteralStruct(self.0) }
    }
    pub fn get_element_type(self) -> TypeRef {
        TypeRef(unsafe { LLVMGetElementType(self.0) })
    }
    pub fn get_subtypes(self, a0: *mut LLVMTypeRef) -> () {
        unsafe { LLVMGetSubtypes(self.0, a0) }
    }
    pub fn get_num_contained_types(self) -> ::libc::c_uint {
        unsafe { LLVMGetNumContainedTypes(self.0) }
    }
    pub fn array_type(self, a0: ::libc::c_uint) -> TypeRef {
        TypeRef(unsafe { LLVMArrayType(self.0, a0) })
    }
    pub fn get_array_length(self) -> ::libc::c_uint {
        unsafe { LLVMGetArrayLength(self.0) }
    }
    pub fn pointer_type(self, a0: ::libc::c_uint) -> TypeRef {
        TypeRef(unsafe { LLVMPointerType(self.0, a0) })
    }
    pub fn get_pointer_address_space(self) -> ::libc::c_uint {
        unsafe { LLVMGetPointerAddressSpace(self.0) }
    }
    pub fn vector_type(self, a0: ::libc::c_uint) -> TypeRef {
        TypeRef(unsafe { LLVMVectorType(self.0, a0) })
    }
    pub fn get_vector_size(self) -> ::libc::c_uint {
        unsafe { LLVMGetVectorSize(self.0) }
    }
    pub fn const_null(self) -> ValueRef {
        ValueRef(unsafe { LLVMConstNull(self.0) })
    }
    pub fn const_all_ones(self) -> ValueRef {
        ValueRef(unsafe { LLVMConstAllOnes(self.0) })
    }
    pub fn get_undef(self) -> ValueRef {
        ValueRef(unsafe { LLVMGetUndef(self.0) })
    }
    pub fn const_pointer_null(self) -> ValueRef {
        ValueRef(unsafe { LLVMConstPointerNull(self.0) })
    }
    pub fn const_real(self, a0: ::libc::c_double) -> ValueRef {
        ValueRef(unsafe { LLVMConstReal(self.0, a0) })
    }
    pub fn const_real_of_string(self, a0: *const ::libc::c_char) -> ValueRef {
        ValueRef(unsafe { LLVMConstRealOfString(self.0, a0) })
    }
    pub fn align_of(self) -> ValueRef {
        ValueRef(unsafe { LLVMAlignOf(self.0) })
    }
    pub fn size_of(self) -> ValueRef {
        ValueRef(unsafe { LLVMSizeOf(self.0) })
    }
}
impl ComdatRef {
    pub fn get_comdat_selection_kind(self) -> LLVMComdatSelectionKind {
        unsafe { LLVMGetComdatSelectionKind(self.0) }
    }
    pub fn set_comdat_selection_kind(self, a0: LLVMComdatSelectionKind) -> () {
        unsafe { LLVMSetComdatSelectionKind(self.0, a0) }
    }
}
impl RemarkDebugLocRef {
    pub fn remark_debug_loc_get_source_file_path(self) -> RemarkStringRef {
        RemarkStringRef(unsafe { LLVMRemarkDebugLocGetSourceFilePath(self.0) })
    }
    pub fn remark_debug_loc_get_source_line(self) -> u32 {
        unsafe { LLVMRemarkDebugLocGetSourceLine(self.0) }
    }
    pub fn remark_debug_loc_get_source_column(self) -> u32 {
        unsafe { LLVMRemarkDebugLocGetSourceColumn(self.0) }
    }
}
impl ContextRef {
    pub fn int_ptr_type_in_context(self, a0: TargetDataRef) -> TypeRef {
        TypeRef(unsafe { LLVMIntPtrTypeInContext(self.0, a0.0) })
    }
    pub fn context_get_diagnostic_handler(self) -> LLVMDiagnosticHandler {
        unsafe { LLVMContextGetDiagnosticHandler(self.0) }
    }
    pub fn context_get_diagnostic_context(self) -> *mut ::libc::c_void {
        unsafe { LLVMContextGetDiagnosticContext(self.0) }
    }
    pub fn context_should_discard_value_names(self) -> LLVMBool {
        unsafe { LLVMContextShouldDiscardValueNames(self.0) }
    }
    pub fn context_set_discard_value_names(self, a0: LLVMBool) -> () {
        unsafe { LLVMContextSetDiscardValueNames(self.0, a0) }
    }
    pub fn context_dispose(self) -> () {
        unsafe { LLVMContextDispose(self.0) }
    }
    pub fn int_1_type_in_context(self) -> TypeRef {
        TypeRef(unsafe { LLVMInt1TypeInContext(self.0) })
    }
    pub fn int_8_type_in_context(self) -> TypeRef {
        TypeRef(unsafe { LLVMInt8TypeInContext(self.0) })
    }
    pub fn int_16_type_in_context(self) -> TypeRef {
        TypeRef(unsafe { LLVMInt16TypeInContext(self.0) })
    }
    pub fn int_32_type_in_context(self) -> TypeRef {
        TypeRef(unsafe { LLVMInt32TypeInContext(self.0) })
    }
    pub fn int_64_type_in_context(self) -> TypeRef {
        TypeRef(unsafe { LLVMInt64TypeInContext(self.0) })
    }
    pub fn int_128_type_in_context(self) -> TypeRef {
        TypeRef(unsafe { LLVMInt128TypeInContext(self.0) })
    }
    pub fn int_type_in_context(self, a0: ::libc::c_uint) -> TypeRef {
        TypeRef(unsafe { LLVMIntTypeInContext(self.0, a0) })
    }
    pub fn half_type_in_context(self) -> TypeRef {
        TypeRef(unsafe { LLVMHalfTypeInContext(self.0) })
    }
    pub fn b_float_type_in_context(self) -> TypeRef {
        TypeRef(unsafe { LLVMBFloatTypeInContext(self.0) })
    }
    pub fn float_type_in_context(self) -> TypeRef {
        TypeRef(unsafe { LLVMFloatTypeInContext(self.0) })
    }
    pub fn double_type_in_context(self) -> TypeRef {
        TypeRef(unsafe { LLVMDoubleTypeInContext(self.0) })
    }
    pub fn x86fp80_type_in_context(self) -> TypeRef {
        TypeRef(unsafe { LLVMX86FP80TypeInContext(self.0) })
    }
    pub fn fp128_type_in_context(self) -> TypeRef {
        TypeRef(unsafe { LLVMFP128TypeInContext(self.0) })
    }
    pub fn ppcfp128_type_in_context(self) -> TypeRef {
        TypeRef(unsafe { LLVMPPCFP128TypeInContext(self.0) })
    }
    pub fn struct_create_named(self, a0: *const ::libc::c_char) -> TypeRef {
        TypeRef(unsafe { LLVMStructCreateNamed(self.0, a0) })
    }
    pub fn void_type_in_context(self) -> TypeRef {
        TypeRef(unsafe { LLVMVoidTypeInContext(self.0) })
    }
    pub fn label_type_in_context(self) -> TypeRef {
        TypeRef(unsafe { LLVMLabelTypeInContext(self.0) })
    }
    pub fn x86mmx_type_in_context(self) -> TypeRef {
        TypeRef(unsafe { LLVMX86MMXTypeInContext(self.0) })
    }
    pub fn token_type_in_context(self) -> TypeRef {
        TypeRef(unsafe { LLVMTokenTypeInContext(self.0) })
    }
    pub fn metadata_type_in_context(self) -> TypeRef {
        TypeRef(unsafe { LLVMMetadataTypeInContext(self.0) })
    }
    pub fn metadata_as_value(self, a0: MetadataRef) -> ValueRef {
        ValueRef(unsafe { LLVMMetadataAsValue(self.0, a0.0) })
    }
    pub fn create_builder_in_context(self) -> BuilderRef {
        BuilderRef(unsafe { LLVMCreateBuilderInContext(self.0) })
    }
}
impl RemarkArgRef {
    pub fn remark_arg_get_key(self) -> RemarkStringRef {
        RemarkStringRef(unsafe { LLVMRemarkArgGetKey(self.0) })
    }
    pub fn remark_arg_get_value(self) -> RemarkStringRef {
        RemarkStringRef(unsafe { LLVMRemarkArgGetValue(self.0) })
    }
    pub fn remark_arg_get_debug_loc(self) -> RemarkDebugLocRef {
        RemarkDebugLocRef(unsafe { LLVMRemarkArgGetDebugLoc(self.0) })
    }
}
impl MemoryBufferRef {
    pub fn create_object_file(self) -> ObjectFileRef {
        ObjectFileRef(unsafe { LLVMCreateObjectFile(self.0) })
    }
    pub fn get_bitcode_module_2(self, a0: *mut LLVMModuleRef) -> LLVMBool {
        unsafe { LLVMGetBitcodeModule2(self.0, a0) }
    }
    pub fn get_buffer_start(self) -> *const ::libc::c_char {
        unsafe { LLVMGetBufferStart(self.0) }
    }
    pub fn get_buffer_size(self) -> ::libc::size_t {
        unsafe { LLVMGetBufferSize(self.0) }
    }
    pub fn dispose_memory_buffer(self) -> () {
        unsafe { LLVMDisposeMemoryBuffer(self.0) }
    }
}
impl DisasmContextRef {
    pub fn set_disasm_options(self, a0: u64) -> ::libc::c_int {
        unsafe { LLVMSetDisasmOptions(self.0, a0) }
    }
    pub fn disasm_dispose(self) -> () {
        unsafe { LLVMDisasmDispose(self.0) }
    }
}
impl RelocationIteratorRef {
    pub fn dispose_relocation_iterator(self) -> () {
        unsafe { LLVMDisposeRelocationIterator(self.0) }
    }
    pub fn move_to_next_relocation(self) -> () {
        unsafe { LLVMMoveToNextRelocation(self.0) }
    }
    pub fn get_relocation_offset(self) -> u64 {
        unsafe { LLVMGetRelocationOffset(self.0) }
    }
    pub fn get_relocation_symbol(self) -> SymbolIteratorRef {
        SymbolIteratorRef(unsafe { LLVMGetRelocationSymbol(self.0) })
    }
    pub fn get_relocation_type(self) -> u64 {
        unsafe { LLVMGetRelocationType(self.0) }
    }
    pub fn get_relocation_type_name(self) -> *const ::libc::c_char {
        unsafe { LLVMGetRelocationTypeName(self.0) }
    }
    pub fn get_relocation_value_string(self) -> *const ::libc::c_char {
        unsafe { LLVMGetRelocationValueString(self.0) }
    }
}
impl GenericValueRef {
    pub fn generic_value_int_width(self) -> ::libc::c_uint {
        unsafe { LLVMGenericValueIntWidth(self.0) }
    }
    pub fn generic_value_to_pointer(self) -> *mut ::libc::c_void {
        unsafe { LLVMGenericValueToPointer(self.0) }
    }
    pub fn dispose_generic_value(self) -> () {
        unsafe { LLVMDisposeGenericValue(self.0) }
    }
}
impl ErrorRef {
    pub fn get_error_type_id(self) -> LLVMErrorTypeId {
        unsafe { LLVMGetErrorTypeId(self.0) }
    }
    pub fn consume_error(self) -> () {
        unsafe { LLVMConsumeError(self.0) }
    }
    pub fn get_error_message(self) -> *mut ::libc::c_char {
        unsafe { LLVMGetErrorMessage(self.0) }
    }
}
impl RemarkStringRef {
    pub fn remark_string_get_data(self) -> *const ::libc::c_char {
        unsafe { LLVMRemarkStringGetData(self.0) }
    }
    pub fn remark_string_get_len(self) -> u32 {
        unsafe { LLVMRemarkStringGetLen(self.0) }
    }
}
impl DIBuilderRef {
    pub fn dispose_di_builder(self) -> () {
        unsafe { LLVMDisposeDIBuilder(self.0) }
    }
    pub fn di_builder_finalize(self) -> () {
        unsafe { LLVMDIBuilderFinalize(self.0) }
    }
    pub fn di_builder_create_null_ptr_type(self) -> MetadataRef {
        MetadataRef(unsafe { LLVMDIBuilderCreateNullPtrType(self.0) })
    }
}
impl ModuleRef {
    pub fn link_modules_2(self, a0: ModuleRef) -> LLVMBool {
        unsafe { LLVMLinkModules2(self.0, a0.0) }
    }
    pub fn get_module_data_layout(self) -> TargetDataRef {
        TargetDataRef(unsafe { LLVMGetModuleDataLayout(self.0) })
    }
    pub fn set_module_data_layout(self, a0: TargetDataRef) -> () {
        unsafe { LLVMSetModuleDataLayout(self.0, a0.0) }
    }
    pub fn get_module_debug_metadata_version(self) -> ::libc::c_uint {
        unsafe { LLVMGetModuleDebugMetadataVersion(self.0) }
    }
    pub fn strip_module_debug_info(self) -> LLVMBool {
        unsafe { LLVMStripModuleDebugInfo(self.0) }
    }
    pub fn create_di_builder_disallow_unresolved(self) -> DIBuilderRef {
        DIBuilderRef(unsafe { LLVMCreateDIBuilderDisallowUnresolved(self.0) })
    }
    pub fn create_di_builder(self) -> DIBuilderRef {
        DIBuilderRef(unsafe { LLVMCreateDIBuilder(self.0) })
    }
    pub fn get_or_insert_comdat(self, a0: *const ::libc::c_char) -> ComdatRef {
        ComdatRef(unsafe { LLVMGetOrInsertComdat(self.0, a0) })
    }
    pub fn write_bitcode_to_file(self, a0: *const ::libc::c_char) -> ::libc::c_int {
        unsafe { LLVMWriteBitcodeToFile(self.0, a0) }
    }
    pub fn write_bitcode_to_file_handle(self, a0: ::libc::c_int) -> ::libc::c_int {
        unsafe { LLVMWriteBitcodeToFileHandle(self.0, a0) }
    }
    pub fn write_bitcode_to_memory_buffer(self) -> MemoryBufferRef {
        MemoryBufferRef(unsafe { LLVMWriteBitcodeToMemoryBuffer(self.0) })
    }
    pub fn clone_module(self) -> ModuleRef {
        ModuleRef(unsafe { LLVMCloneModule(self.0) })
    }
    pub fn dispose_module(self) -> () {
        unsafe { LLVMDisposeModule(self.0) }
    }
    pub fn get_data_layout(self) -> *const ::libc::c_char {
        unsafe { LLVMGetDataLayout(self.0) }
    }
    pub fn get_data_layout_str(self) -> *const ::libc::c_char {
        unsafe { LLVMGetDataLayoutStr(self.0) }
    }
    pub fn set_data_layout(self, a0: *const ::libc::c_char) -> () {
        unsafe { LLVMSetDataLayout(self.0, a0) }
    }
    pub fn get_target(self) -> *const ::libc::c_char {
        unsafe { LLVMGetTarget(self.0) }
    }
    pub fn set_target(self, a0: *const ::libc::c_char) -> () {
        unsafe { LLVMSetTarget(self.0, a0) }
    }
    pub fn dump_module(self) -> () {
        unsafe { LLVMDumpModule(self.0) }
    }
    pub fn print_module_to_string(self) -> *mut ::libc::c_char {
        unsafe { LLVMPrintModuleToString(self.0) }
    }
    pub fn set_module_inline_asm(self, a0: *const ::libc::c_char) -> () {
        unsafe { LLVMSetModuleInlineAsm(self.0, a0) }
    }
    pub fn get_module_context(self) -> ContextRef {
        ContextRef(unsafe { LLVMGetModuleContext(self.0) })
    }
    pub fn get_type_by_name(self, a0: *const ::libc::c_char) -> TypeRef {
        TypeRef(unsafe { LLVMGetTypeByName(self.0, a0) })
    }
    pub fn get_first_named_metadata(self) -> NamedMDNodeRef {
        NamedMDNodeRef(unsafe { LLVMGetFirstNamedMetadata(self.0) })
    }
    pub fn get_last_named_metadata(self) -> NamedMDNodeRef {
        NamedMDNodeRef(unsafe { LLVMGetLastNamedMetadata(self.0) })
    }
    pub fn get_named_function(self, a0: *const ::libc::c_char) -> ValueRef {
        ValueRef(unsafe { LLVMGetNamedFunction(self.0, a0) })
    }
    pub fn get_first_function(self) -> ValueRef {
        ValueRef(unsafe { LLVMGetFirstFunction(self.0) })
    }
    pub fn get_last_function(self) -> ValueRef {
        ValueRef(unsafe { LLVMGetLastFunction(self.0) })
    }
    pub fn get_named_global(self, a0: *const ::libc::c_char) -> ValueRef {
        ValueRef(unsafe { LLVMGetNamedGlobal(self.0, a0) })
    }
    pub fn get_first_global(self) -> ValueRef {
        ValueRef(unsafe { LLVMGetFirstGlobal(self.0) })
    }
    pub fn get_last_global(self) -> ValueRef {
        ValueRef(unsafe { LLVMGetLastGlobal(self.0) })
    }
    pub fn get_first_global_alias(self) -> ValueRef {
        ValueRef(unsafe { LLVMGetFirstGlobalAlias(self.0) })
    }
    pub fn get_last_global_alias(self) -> ValueRef {
        ValueRef(unsafe { LLVMGetLastGlobalAlias(self.0) })
    }
    pub fn get_first_global_i_func(self) -> ValueRef {
        ValueRef(unsafe { LLVMGetFirstGlobalIFunc(self.0) })
    }
    pub fn get_last_global_i_func(self) -> ValueRef {
        ValueRef(unsafe { LLVMGetLastGlobalIFunc(self.0) })
    }
    pub fn create_module_provider_for_existing_module(self) -> ModuleProviderRef {
        ModuleProviderRef(unsafe { LLVMCreateModuleProviderForExistingModule(self.0) })
    }
    pub fn create_function_pass_manager_for_module(self) -> PassManagerRef {
        PassManagerRef(unsafe { LLVMCreateFunctionPassManagerForModule(self.0) })
    }
}
pub fn initialize_all_disassemblers() -> () {
    unsafe { LLVM_InitializeAllDisassemblers() }
}
pub fn initialize_mips_target() -> () {
    unsafe { LLVMInitializeMipsTarget() }
}
pub fn create_intel_jit_event_listener() -> LLVMJITEventListenerRef {
    unsafe { LLVMCreateIntelJITEventListener() }
}
pub fn initialize_sparc_target_mc() -> () {
    unsafe { LLVMInitializeSparcTargetMC() }
}
pub fn initialize_arm_target_mc() -> () {
    unsafe { LLVMInitializeARMTargetMC() }
}
pub fn initialize_system_z_target_mc() -> () {
    unsafe { LLVMInitializeSystemZTargetMC() }
}
pub fn initialize_arm_target_info() -> () {
    unsafe { LLVMInitializeARMTargetInfo() }
}
pub fn search_for_address_of_symbol(a0: *const ::libc::c_char) -> *mut ::libc::c_void {
    unsafe { LLVMSearchForAddressOfSymbol(a0) }
}
pub fn initialize_amdgpu_target_info() -> () {
    unsafe { LLVMInitializeAMDGPUTargetInfo() }
}
pub fn initialize_hexagon_asm_printer() -> () {
    unsafe { LLVMInitializeHexagonAsmPrinter() }
}
pub fn initialize_web_assembly_disassembler() -> () {
    unsafe { LLVMInitializeWebAssemblyDisassembler() }
}
pub fn initialize_x86_disassembler() -> () {
    unsafe { LLVMInitializeX86Disassembler() }
}
pub fn initialize_riscv_target() -> () {
    unsafe { LLVMInitializeRISCVTarget() }
}
pub fn initialize_mips_target_info() -> () {
    unsafe { LLVMInitializeMipsTargetInfo() }
}
pub fn initialize_riscv_target_info() -> () {
    unsafe { LLVMInitializeRISCVTargetInfo() }
}
pub fn initialize_web_assembly_asm_printer() -> () {
    unsafe { LLVMInitializeWebAssemblyAsmPrinter() }
}
pub fn initialize_a_arch_64_target() -> () {
    unsafe { LLVMInitializeAArch64Target() }
}
pub fn initialize_lanai_disassembler() -> () {
    unsafe { LLVMInitializeLanaiDisassembler() }
}
pub fn initialize_lanai_asm_printer() -> () {
    unsafe { LLVMInitializeLanaiAsmPrinter() }
}
pub fn initialize_all_target_infos() -> () {
    unsafe { LLVM_InitializeAllTargetInfos() }
}
pub fn initialize_mips_asm_parser() -> () {
    unsafe { LLVMInitializeMipsAsmParser() }
}
pub fn initialize_sparc_asm_printer() -> () {
    unsafe { LLVMInitializeSparcAsmPrinter() }
}
pub fn x86mmx_type() -> TypeRef {
    TypeRef(unsafe { LLVMX86MMXType() })
}
pub fn start_multithreaded() -> LLVMBool {
    unsafe { LLVMStartMultithreaded() }
}
pub fn module_create_with_name(a0: *const ::libc::c_char) -> ModuleRef {
    ModuleRef(unsafe { LLVMModuleCreateWithName(a0) })
}
pub fn initialize_native_asm_printer() -> LLVMBool {
    unsafe { LLVM_InitializeNativeAsmPrinter() }
}
pub fn intrinsic_is_overloaded(a0: ::libc::c_uint) -> LLVMBool {
    unsafe { LLVMIntrinsicIsOverloaded(a0) }
}
pub fn fp128_type() -> TypeRef {
    TypeRef(unsafe { LLVMFP128Type() })
}
pub fn reset_fatal_error_handler() -> () {
    unsafe { LLVMResetFatalErrorHandler() }
}
pub fn get_default_target_triple() -> *mut ::libc::c_char {
    unsafe { LLVMGetDefaultTargetTriple() }
}
pub fn md_string(a0: *const ::libc::c_char, a1: ::libc::c_uint) -> ValueRef {
    ValueRef(unsafe { LLVMMDString(a0, a1) })
}
pub fn initialize_x86_target() -> () {
    unsafe { LLVMInitializeX86Target() }
}
pub fn initialize_system_z_target_info() -> () {
    unsafe { LLVMInitializeSystemZTargetInfo() }
}
pub fn initialize_arm_disassembler() -> () {
    unsafe { LLVMInitializeARMDisassembler() }
}
pub fn half_type() -> TypeRef {
    TypeRef(unsafe { LLVMHalfType() })
}
pub fn initialize_msp430_target_info() -> () {
    unsafe { LLVMInitializeMSP430TargetInfo() }
}
pub fn pass_manager_builder_create() -> PassManagerBuilderRef {
    PassManagerBuilderRef(unsafe { LLVMPassManagerBuilderCreate() })
}
pub fn initialize_a_arch_64_target_mc() -> () {
    unsafe { LLVMInitializeAArch64TargetMC() }
}
pub fn initialize_nvptx_target_mc() -> () {
    unsafe { LLVMInitializeNVPTXTargetMC() }
}
pub fn create_perf_jit_event_listener() -> LLVMJITEventListenerRef {
    unsafe { LLVMCreatePerfJITEventListener() }
}
pub fn normalize_target_triple(a0: *const ::libc::c_char) -> *mut ::libc::c_char {
    unsafe { LLVMNormalizeTargetTriple(a0) }
}
pub fn initialize_hexagon_target() -> () {
    unsafe { LLVMInitializeHexagonTarget() }
}
pub fn initialize_x_core_disassembler() -> () {
    unsafe { LLVMInitializeXCoreDisassembler() }
}
pub fn get_global_pass_registry() -> PassRegistryRef {
    PassRegistryRef(unsafe { LLVMGetGlobalPassRegistry() })
}
pub fn get_first_target() -> TargetRef {
    TargetRef(unsafe { LLVMGetFirstTarget() })
}
pub fn initialize_a_arch_64_disassembler() -> () {
    unsafe { LLVMInitializeAArch64Disassembler() }
}
pub fn stop_multithreaded() -> () {
    unsafe { LLVMStopMultithreaded() }
}
pub fn remark_version() -> u32 {
    unsafe { LLVMRemarkVersion() }
}
pub fn dispose_module_flags_metadata(a0: *mut LLVMModuleFlagEntry) -> () {
    unsafe { LLVMDisposeModuleFlagsMetadata(a0) }
}
pub fn initialize_sparc_target_info() -> () {
    unsafe { LLVMInitializeSparcTargetInfo() }
}
pub fn initialize_bpf_disassembler() -> () {
    unsafe { LLVMInitializeBPFDisassembler() }
}
pub fn initialize_lanai_asm_parser() -> () {
    unsafe { LLVMInitializeLanaiAsmParser() }
}
pub fn initialize_sparc_target() -> () {
    unsafe { LLVMInitializeSparcTarget() }
}
pub fn create_pass_manager() -> PassManagerRef {
    PassManagerRef(unsafe { LLVMCreatePassManager() })
}
pub fn initialize_riscv_target_mc() -> () {
    unsafe { LLVMInitializeRISCVTargetMC() }
}
pub fn create_o_profile_jit_event_listener() -> LLVMJITEventListenerRef {
    unsafe { LLVMCreateOProfileJITEventListener() }
}
pub fn x86fp80_type() -> TypeRef {
    TypeRef(unsafe { LLVMX86FP80Type() })
}
pub fn debug_metadata_version() -> ::libc::c_uint {
    unsafe { LLVMDebugMetadataVersion() }
}
pub fn initialize_x86_target_mc() -> () {
    unsafe { LLVMInitializeX86TargetMC() }
}
pub fn initialize_web_assembly_target_mc() -> () {
    unsafe { LLVMInitializeWebAssemblyTargetMC() }
}
pub fn link_in_mcjit() -> () {
    unsafe { LLVMLinkInMCJIT() }
}
pub fn initialize_power_pc_asm_parser() -> () {
    unsafe { LLVMInitializePowerPCAsmParser() }
}
pub fn get_string_error_type_id() -> LLVMErrorTypeId {
    unsafe { LLVMGetStringErrorTypeId() }
}
pub fn initialize_amdgpu_target_mc() -> () {
    unsafe { LLVMInitializeAMDGPUTargetMC() }
}
pub fn initialize_hexagon_target_mc() -> () {
    unsafe { LLVMInitializeHexagonTargetMC() }
}
pub fn initialize_msp430_target() -> () {
    unsafe { LLVMInitializeMSP430Target() }
}
pub fn initialize_hexagon_disassembler() -> () {
    unsafe { LLVMInitializeHexagonDisassembler() }
}
pub fn initialize_a_arch_64_asm_printer() -> () {
    unsafe { LLVMInitializeAArch64AsmPrinter() }
}
pub fn orc_create_lljit_builder() -> OrcLLJITBuilderRef {
    OrcLLJITBuilderRef(unsafe { LLVMOrcCreateLLJITBuilder() })
}
pub fn initialize_power_pc_target_info() -> () {
    unsafe { LLVMInitializePowerPCTargetInfo() }
}
pub fn get_host_cpu_features() -> *mut ::libc::c_char {
    unsafe { LLVMGetHostCPUFeatures() }
}
pub fn create_target_data(a0: *const ::libc::c_char) -> TargetDataRef {
    TargetDataRef(unsafe { LLVMCreateTargetData(a0) })
}
pub fn initialize_power_pc_disassembler() -> () {
    unsafe { LLVMInitializePowerPCDisassembler() }
}
pub fn get_target_from_name(a0: *const ::libc::c_char) -> TargetRef {
    TargetRef(unsafe { LLVMGetTargetFromName(a0) })
}
pub fn initialize_native_disassembler() -> LLVMBool {
    unsafe { LLVM_InitializeNativeDisassembler() }
}
pub fn initialize_web_assembly_target_info() -> () {
    unsafe { LLVMInitializeWebAssemblyTargetInfo() }
}
pub fn initialize_arm_asm_printer() -> () {
    unsafe { LLVMInitializeARMAsmPrinter() }
}
pub fn initialize_x_core_target_info() -> () {
    unsafe { LLVMInitializeXCoreTargetInfo() }
}
pub fn initialize_hexagon_target_info() -> () {
    unsafe { LLVMInitializeHexagonTargetInfo() }
}
pub fn is_multithreaded() -> LLVMBool {
    unsafe { LLVMIsMultithreaded() }
}
pub fn context_create() -> ContextRef {
    ContextRef(unsafe { LLVMContextCreate() })
}
pub fn initialize_bpf_asm_printer() -> () {
    unsafe { LLVMInitializeBPFAsmPrinter() }
}
pub fn initialize_web_assembly_asm_parser() -> () {
    unsafe { LLVMInitializeWebAssemblyAsmParser() }
}
pub fn initialize_native_target() -> LLVMBool {
    unsafe { LLVM_InitializeNativeTarget() }
}
pub fn initialize_native_asm_parser() -> LLVMBool {
    unsafe { LLVM_InitializeNativeAsmParser() }
}
pub fn initialize_a_arch_64_asm_parser() -> () {
    unsafe { LLVMInitializeAArch64AsmParser() }
}
pub fn initialize_power_pc_target_mc() -> () {
    unsafe { LLVMInitializePowerPCTargetMC() }
}
pub fn initialize_x_core_target_mc() -> () {
    unsafe { LLVMInitializeXCoreTargetMC() }
}
pub fn initialize_amdgpu_asm_parser() -> () {
    unsafe { LLVMInitializeAMDGPUAsmParser() }
}
pub fn int_32_type() -> TypeRef {
    TypeRef(unsafe { LLVMInt32Type() })
}
pub fn add_symbol(a0: *const ::libc::c_char, a1: *mut ::libc::c_void) -> () {
    unsafe { LLVMAddSymbol(a0, a1) }
}
pub fn initialize_bpf_target_info() -> () {
    unsafe { LLVMInitializeBPFTargetInfo() }
}
pub fn ppcfp128_type() -> TypeRef {
    TypeRef(unsafe { LLVMPPCFP128Type() })
}
pub fn initialize_x86_target_info() -> () {
    unsafe { LLVMInitializeX86TargetInfo() }
}
pub fn initialize_all_target_m_cs() -> () {
    unsafe { LLVM_InitializeAllTargetMCs() }
}
pub fn initialize_mips_asm_printer() -> () {
    unsafe { LLVMInitializeMipsAsmPrinter() }
}
pub fn initialize_x86_asm_printer() -> () {
    unsafe { LLVMInitializeX86AsmPrinter() }
}
pub fn create_gdb_registration_listener() -> LLVMJITEventListenerRef {
    unsafe { LLVMCreateGDBRegistrationListener() }
}
pub fn float_type() -> TypeRef {
    TypeRef(unsafe { LLVMFloatType() })
}
pub fn orc_create_new_thread_safe_context() -> OrcThreadSafeContextRef {
    OrcThreadSafeContextRef(unsafe { LLVMOrcCreateNewThreadSafeContext() })
}
pub fn int_128_type() -> TypeRef {
    TypeRef(unsafe { LLVMInt128Type() })
}
pub fn void_type() -> TypeRef {
    TypeRef(unsafe { LLVMVoidType() })
}
pub fn initialize_system_z_target() -> () {
    unsafe { LLVMInitializeSystemZTarget() }
}
pub fn md_node(a0: *mut LLVMValueRef, a1: ::libc::c_uint) -> ValueRef {
    ValueRef(unsafe { LLVMMDNode(a0, a1) })
}
pub fn initialize_system_z_disassembler() -> () {
    unsafe { LLVMInitializeSystemZDisassembler() }
}
pub fn get_md_kind_id(a0: *const ::libc::c_char, a1: ::libc::c_uint) -> ::libc::c_uint {
    unsafe { LLVMGetMDKindID(a0, a1) }
}
pub fn initialize_nvptx_target() -> () {
    unsafe { LLVMInitializeNVPTXTarget() }
}
pub fn initialize_arm_asm_parser() -> () {
    unsafe { LLVMInitializeARMAsmParser() }
}
pub fn get_undef_mask_elem() -> ::libc::c_int {
    unsafe { LLVMGetUndefMaskElem() }
}
pub fn initialize_lanai_target() -> () {
    unsafe { LLVMInitializeLanaiTarget() }
}
pub fn orc_dispose_mangled_symbol(a0: *mut ::libc::c_char) -> () {
    unsafe { LLVMOrcDisposeMangledSymbol(a0) }
}
pub fn initialize_x_core_asm_printer() -> () {
    unsafe { LLVMInitializeXCoreAsmPrinter() }
}
pub fn initialize_system_z_asm_printer() -> () {
    unsafe { LLVMInitializeSystemZAsmPrinter() }
}
pub fn initialize_arm_target() -> () {
    unsafe { LLVMInitializeARMTarget() }
}
pub fn initialize_all_asm_parsers() -> () {
    unsafe { LLVM_InitializeAllAsmParsers() }
}
pub fn create_message(a0: *const ::libc::c_char) -> *mut ::libc::c_char {
    unsafe { LLVMCreateMessage(a0) }
}
pub fn label_type() -> TypeRef {
    TypeRef(unsafe { LLVMLabelType() })
}
pub fn initialize_bpf_target_mc() -> () {
    unsafe { LLVMInitializeBPFTargetMC() }
}
pub fn initialize_msp430_target_mc() -> () {
    unsafe { LLVMInitializeMSP430TargetMC() }
}
pub fn initialize_lanai_target_mc() -> () {
    unsafe { LLVMInitializeLanaiTargetMC() }
}
pub fn initialize_bpf_target() -> () {
    unsafe { LLVMInitializeBPFTarget() }
}
pub fn link_in_interpreter() -> () {
    unsafe { LLVMLinkInInterpreter() }
}
pub fn int_16_type() -> TypeRef {
    TypeRef(unsafe { LLVMInt16Type() })
}
pub fn initialize_mips_target_mc() -> () {
    unsafe { LLVMInitializeMipsTargetMC() }
}
pub fn initialize_nvptx_asm_printer() -> () {
    unsafe { LLVMInitializeNVPTXAsmPrinter() }
}
pub fn get_global_context() -> ContextRef {
    ContextRef(unsafe { LLVMGetGlobalContext() })
}
pub fn dispose_error_message(a0: *mut ::libc::c_char) -> () {
    unsafe { LLVMDisposeErrorMessage(a0) }
}
pub fn initialize_amdgpu_target() -> () {
    unsafe { LLVMInitializeAMDGPUTarget() }
}
pub fn initialize_lanai_target_info() -> () {
    unsafe { LLVMInitializeLanaiTargetInfo() }
}
pub fn int_64_type() -> TypeRef {
    TypeRef(unsafe { LLVMInt64Type() })
}
pub fn dispose_value_metadata_entries(a0: *mut LLVMValueMetadataEntry) -> () {
    unsafe { LLVMDisposeValueMetadataEntries(a0) }
}
pub fn load_library_permanently(a0: *const ::libc::c_char) -> LLVMBool {
    unsafe { LLVMLoadLibraryPermanently(a0) }
}
pub fn initialize_a_arch_64_target_info() -> () {
    unsafe { LLVMInitializeAArch64TargetInfo() }
}
pub fn initialize_all_targets() -> () {
    unsafe { LLVM_InitializeAllTargets() }
}
pub fn initialize_power_pc_asm_printer() -> () {
    unsafe { LLVMInitializePowerPCAsmPrinter() }
}
pub fn get_host_cpu_name() -> *mut ::libc::c_char {
    unsafe { LLVMGetHostCPUName() }
}
pub fn initialize_web_assembly_target() -> () {
    unsafe { LLVMInitializeWebAssemblyTarget() }
}
pub fn dispose_message(a0: *mut ::libc::c_char) -> () {
    unsafe { LLVMDisposeMessage(a0) }
}
pub fn initialize_x_core_target() -> () {
    unsafe { LLVMInitializeXCoreTarget() }
}
pub fn remark_parser_create_yaml(a0: *const ::libc::c_void, a1: u64) -> RemarkParserRef {
    RemarkParserRef(unsafe { LLVMRemarkParserCreateYAML(a0, a1) })
}
pub fn initialize_system_z_asm_parser() -> () {
    unsafe { LLVMInitializeSystemZAsmParser() }
}
pub fn create_generic_value_of_pointer(a0: *mut ::libc::c_void) -> GenericValueRef {
    GenericValueRef(unsafe { LLVMCreateGenericValueOfPointer(a0) })
}
pub fn initialize_amdgpu_asm_printer() -> () {
    unsafe { LLVMInitializeAMDGPUAsmPrinter() }
}
pub fn initialize_all_asm_printers() -> () {
    unsafe { LLVM_InitializeAllAsmPrinters() }
}
pub fn initialize_power_pc_target() -> () {
    unsafe { LLVMInitializePowerPCTarget() }
}
pub fn int_8_type() -> TypeRef {
    TypeRef(unsafe { LLVMInt8Type() })
}
pub fn enable_pretty_stack_trace() -> () {
    unsafe { LLVMEnablePrettyStackTrace() }
}
pub fn int_1_type() -> TypeRef {
    TypeRef(unsafe { LLVMInt1Type() })
}
pub fn double_type() -> TypeRef {
    TypeRef(unsafe { LLVMDoubleType() })
}
pub fn get_last_enum_attribute_kind() -> ::libc::c_uint {
    unsafe { LLVMGetLastEnumAttributeKind() }
}
pub fn initialize_msp430_asm_printer() -> () {
    unsafe { LLVMInitializeMSP430AsmPrinter() }
}
pub fn initialize_sparc_disassembler() -> () {
    unsafe { LLVMInitializeSparcDisassembler() }
}
pub fn int_type(a0: ::libc::c_uint) -> TypeRef {
    TypeRef(unsafe { LLVMIntType(a0) })
}
pub fn initialize_nvptx_target_info() -> () {
    unsafe { LLVMInitializeNVPTXTargetInfo() }
}
pub fn shutdown() -> () {
    unsafe { LLVMShutdown() }
}
pub fn create_builder() -> BuilderRef {
    BuilderRef(unsafe { LLVMCreateBuilder() })
}
pub fn initialize_mips_disassembler() -> () {
    unsafe { LLVMInitializeMipsDisassembler() }
}
pub fn initialize_sparc_asm_parser() -> () {
    unsafe { LLVMInitializeSparcAsmParser() }
}
pub fn b_float_type() -> TypeRef {
    TypeRef(unsafe { LLVMBFloatType() })
}
pub fn initialize_x86_asm_parser() -> () {
    unsafe { LLVMInitializeX86AsmParser() }
}
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct RemarkDebugLocRef(pub LLVMRemarkDebugLocRef);
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct PassRegistryRef(pub LLVMPassRegistryRef);
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct ContextRef(pub LLVMContextRef);
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct OrcThreadSafeModuleRef(pub LLVMOrcThreadSafeModuleRef);
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct DiagnosticInfoRef(pub LLVMDiagnosticInfoRef);
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct OrcJITDylibDefinitionGeneratorRef(pub LLVMOrcJITDylibDefinitionGeneratorRef);
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct NamedMDNodeRef(pub LLVMNamedMDNodeRef);
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct BasicBlockRef(pub LLVMBasicBlockRef);
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct OrcJITTargetMachineBuilderRef(pub LLVMOrcJITTargetMachineBuilderRef);
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct SymbolIteratorRef(pub LLVMSymbolIteratorRef);
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct BinaryRef(pub LLVMBinaryRef);
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct ExecutionEngineRef(pub LLVMExecutionEngineRef);
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct OrcSymbolStringPoolEntryRef(pub LLVMOrcSymbolStringPoolEntryRef);
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct TargetDataRef(pub LLVMTargetDataRef);
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct RemarkParserRef(pub LLVMRemarkParserRef);
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct MCJITMemoryManagerRef(pub LLVMMCJITMemoryManagerRef);
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct RemarkArgRef(pub LLVMRemarkArgRef);
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct TypeRef(pub LLVMTypeRef);
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct ErrorRef(pub LLVMErrorRef);
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct BuilderRef(pub LLVMBuilderRef);
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct AttributeRef(pub LLVMAttributeRef);
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct ModuleProviderRef(pub LLVMModuleProviderRef);
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct PassManagerBuilderRef(pub LLVMPassManagerBuilderRef);
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct ModuleRef(pub LLVMModuleRef);
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct RemarkEntryRef(pub LLVMRemarkEntryRef);
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct RelocationIteratorRef(pub LLVMRelocationIteratorRef);
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct RemarkStringRef(pub LLVMRemarkStringRef);
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct TargetMachineRef(pub LLVMTargetMachineRef);
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct UseRef(pub LLVMUseRef);
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct MemoryBufferRef(pub LLVMMemoryBufferRef);
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct OrcJITStackRef(pub LLVMOrcJITStackRef);
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct OrcLLJITBuilderRef(pub LLVMOrcLLJITBuilderRef);
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct GenericValueRef(pub LLVMGenericValueRef);
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct DIBuilderRef(pub LLVMDIBuilderRef);
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct PassManagerRef(pub LLVMPassManagerRef);
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct ObjectFileRef(pub LLVMObjectFileRef);
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct ValueRef(pub LLVMValueRef);
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct ComdatRef(pub LLVMComdatRef);
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct TargetLibraryInfoRef(pub LLVMTargetLibraryInfoRef);
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct FatalErrorHandler(pub LLVMFatalErrorHandler);
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct MetadataRef(pub LLVMMetadataRef);
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct OrcLLJITRef(pub LLVMOrcLLJITRef);
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct OrcThreadSafeContextRef(pub LLVMOrcThreadSafeContextRef);
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct SectionIteratorRef(pub LLVMSectionIteratorRef);
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct TargetRef(pub LLVMTargetRef);
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct DisasmContextRef(pub LLVMDisasmContextRef);
