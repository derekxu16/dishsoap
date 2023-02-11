mod backend;
mod types;
mod utils;
mod visitor;

use std::ffi::CString;

use clap::Parser as clap_Parser;
use dishsoap_parser::ast::{Node, UntypedNodeCommonFields};
use dishsoap_parser::Parser;
use llvm_sys::bit_writer::LLVMWriteBitcodeToFile;
use llvm_sys::core::*;
use llvm_sys::prelude::{LLVMContextRef, LLVMModuleRef};
use types::{
    build_environment_from_top_level_declarations,
    build_type_environment_from_top_level_declarations,
};
// use llvm_sys::target_machine::LLVMGetDefaultTargetTriple;
use crate::types::TypeChecker;
use backend::builder::Builder;
use utils::string_to_c_string;
use visitor::{PostOrderVisitor, PreOrderVisitor};

#[derive(clap_Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(long)]
    dump_ir: bool,

    file_path: String,
}

fn parse_file(file_content: &String) -> Node<UntypedNodeCommonFields> {
    let mut parser = Parser::new(file_content);

    parser.parse()
}

pub fn get_llvm_module_from_file(context: LLVMContextRef, file_content: &String) -> LLVMModuleRef {
    let untyped_ast = &parse_file(file_content);
    let mut type_checker = TypeChecker::new(
        &build_environment_from_top_level_declarations(untyped_ast),
        &build_type_environment_from_top_level_declarations(untyped_ast),
    );
    let typed_ast = type_checker.visit(&untyped_ast);

    unsafe {
        let module_name = CString::new("main").unwrap();
        let module = LLVMModuleCreateWithName(module_name.as_ptr());
        let llvm_builder = LLVMCreateBuilderInContext(context);

        // let log_func_type = LLVMFunctionType(
        //     LLVMVoidType(),
        //     [LLVMPointerType(LLVMInt8Type(), 0)].as_ptr() as *mut _,
        //     1,
        //     0,
        // );
        // let log_func = LLVMAddFunction(module, string_to_c_str(&"log".to_owned()), log_func_type);

        let mut builder = Builder::new(&context, &module, &llvm_builder);
        builder.visit(&typed_ast);

        LLVMDisposeBuilder(llvm_builder);

        module
    }
}

fn main() {
    let cli = Cli::parse();
    let file_content =
        std::fs::read_to_string(cli.file_path).expect("Error: could not read source file");

    unsafe {
        let context = LLVMContextCreate();
        let module = get_llvm_module_from_file(context, &file_content);
        LLVMSetTarget(
            module,
            string_to_c_string("wasm32-unknown-unknown-wasm".to_owned()).as_ptr(),
        );
        // LLVMSetTarget(module, LLVMGetDefaultTargetTriple());
        if cli.dump_ir {
            LLVMDumpModule(module);
        } else {
            LLVMWriteBitcodeToFile(module, string_to_c_string("main.bc".to_owned()).as_ptr());
        }

        LLVMDisposeModule(module);
        LLVMContextDispose(context);
    }
}
