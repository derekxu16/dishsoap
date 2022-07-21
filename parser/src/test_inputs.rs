pub const PREFIX_OPERATION_NOT: &str = "
func test() -> P_bool {
    !true
}
";

pub const PREFIX_OPERATION_MINUS: &str = "
func test() -> P_i32 {
    -4
}
";

pub const ARITHMETIC_OPERATOR_PRECEDENCE: &str = "
func test() -> P_i32 {
    2 + 2 * 2
}
";

pub const IF_EXPRESSION: &str = "
func test() -> P_i32 {
    if (1 > 2) {
        3
    } else {
        4
    }
}
";

// TODO(derekxu16): Write tests that use this input.
pub const FUNCTION_CALL_FIB: &str = "
func fib(n: P_i32) -> P_i32 {
    if (n == 0) {
        0
    } else {
        if (n == 1) {
            1
        } else {
            fib(n - 1) + fib(n - 2)
        }
    }
}
";

pub const FUNCTION_CALL_ADD: &str = "
func add(a: P_i32, b: P_i32) -> P_i32 {
    a + b
}

func test() -> P_i32 {
    add(11, 22)
}
";

pub const RECORD_INITIALIZATION_AND_FIELD_ACCESS: &str = "
func test() -> P_i32 {
    let x: {a: P_bool, b: {c: P_i32}} = {a: true, b: {c: 123}};
    x.b.c
}
";

pub const VARIABLE_INITIALIZATION_AND_REFERENCE_INT: &str = "
func test() -> P_i32 {
    let a: P_i32 = 10;
    let b: P_i32 = a;
    b
}
";

pub const FUNCTION_DECLARATION_ADD: &str = "
func add(a: P_i32, b: P_i32) -> P_i32 {
    a + b
}
";
