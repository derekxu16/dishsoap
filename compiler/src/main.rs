mod builder;
mod utils;

use builder::*;
use dishsoap_parser::Parser;
use llvm_sys::bit_writer::LLVMWriteBitcodeToFile;
use llvm_sys::core::{
    LLVMAddFunction, LLVMContextCreate, LLVMContextDispose, LLVMCreateBuilderInContext,
    LLVMDisposeBuilder, LLVMDisposeModule, LLVMFunctionType, LLVMInt8Type,
    LLVMModuleCreateWithName, LLVMPointerType, LLVMSetTarget, LLVMVoidType,
};
use llvm_sys::target_machine::LLVMGetDefaultTargetTriple;
use utils::string_to_c_str;

fn main() {
    let path = std::env::args()
        .nth(1)
        .expect("Error: no source file path specified");
    let content = std::fs::read_to_string(path).expect("Error: could not read source file");

    let mut parser = Parser::new(&content);
    let ast = parser.parse();

    unsafe {
        let context = LLVMContextCreate();
        let module = LLVMModuleCreateWithName(string_to_c_str(&"main".to_owned()));
        let llvm_builder = LLVMCreateBuilderInContext(context);

        let log_func_type = LLVMFunctionType(
            LLVMVoidType(),
            [LLVMPointerType(LLVMInt8Type(), 0)].as_ptr() as *mut _,
            1,
            0,
        );
        // let log_func = LLVMAddFunction(module, string_to_c_str(&"log".to_owned()), log_func_type);

        let mut builder = Builder::new(&context, &module, &llvm_builder);
        builder.build(&ast);

        LLVMSetTarget(
            module,
            string_to_c_str(&"wasm32-unknown-unknown-wasm".to_owned()),
        );
        // LLVMSetTarget(module, LLVMGetDefaultTargetTriple());
        LLVMWriteBitcodeToFile(module, string_to_c_str(&"main.bc".to_owned()));

        LLVMDisposeBuilder(llvm_builder);
        LLVMDisposeModule(module);
        LLVMContextDispose(context);
    }
}

#[cfg(test)]
mod tests {}
