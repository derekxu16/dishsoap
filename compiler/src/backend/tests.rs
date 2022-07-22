use crate::*;
use dishsoap_parser::test_inputs;
use llvm_sys::execution_engine::{
    LLVMCreateJITCompilerForModule, LLVMExecutionEngineRef, LLVMGenericValueToInt, LLVMLinkInMCJIT,
    LLVMRunFunction,
};
use llvm_sys::target::{
    LLVM_InitializeNativeAsmParser, LLVM_InitializeNativeAsmPrinter, LLVM_InitializeNativeTarget,
};
use serial_test::serial;
use std::ffi::CString;
use std::mem::MaybeUninit;

mod tests {
    use super::*;

    fn interpret(source: &str) -> i64 {
        unsafe {
            let context = LLVMContextCreate();
            let module = get_llvm_module_from_file(context, &source.to_string());

            LLVM_InitializeNativeTarget();
            LLVM_InitializeNativeAsmPrinter();
            LLVM_InitializeNativeAsmParser();
            LLVMLinkInMCJIT();

            let mut execution_engine = MaybeUninit::<LLVMExecutionEngineRef>::uninit();
            LLVMCreateJITCompilerForModule(
                execution_engine.as_mut_ptr(),
                module,
                0,
                (vec![]).as_mut_ptr(),
            );

            let name = CString::new("test").unwrap();
            // LLVMDumpModule(module);
            let test_function = LLVMGetNamedFunction(module, name.as_ptr());
            let llvm_result = LLVMRunFunction(
                execution_engine.assume_init_read(),
                test_function,
                0,
                (vec![]).as_mut_ptr(),
            );

            // TODO(derekxu16): The type is hardcoded, but it shouldn't be.
            let result = LLVMGenericValueToInt(llvm_result, true.into()) as i64;

            LLVMDisposeModule(module);
            LLVMContextDispose(context);

            result
        }
    }

    #[test]
    #[serial]
    fn prefix_expression_not() {
        let result = interpret(test_inputs::PREFIX_OPERATION_NOT);
        assert_eq!(result, 0);
    }

    #[test]
    #[serial]
    fn prefix_expression_minus() {
        let result = interpret(test_inputs::PREFIX_OPERATION_MINUS);
        assert_eq!(result, -4);
    }

    #[test]
    #[serial]
    fn arithmetic_expressions() {
        let result = interpret(test_inputs::ARITHMETIC_OPERATOR_PRECEDENCE);
        assert_eq!(result, 6);
    }

    #[test]
    #[serial]
    fn if_expressions() {
        let result = interpret(test_inputs::IF_EXPRESSION);
        assert_eq!(result, 4);
    }

    #[test]
    #[serial]
    fn function_calls() {
        let result = interpret(test_inputs::FUNCTION_CALL_ADD);
        assert_eq!(result, 33);
    }

    #[test]
    #[serial]
    fn record_initializations_and_field_accesses() {
        let result = interpret(test_inputs::RECORD_INITIALIZATION_AND_FIELD_ACCESS);
        assert_eq!(result, 123);
    }

    #[test]
    #[serial]
    fn variable_initializations_and_references() {
        let result = interpret(test_inputs::VARIABLE_INITIALIZATION_AND_REFERENCE_INT);
        assert_eq!(result, 10);
    }
}
