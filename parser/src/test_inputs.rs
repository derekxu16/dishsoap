pub const PREFIX_OPERATION_NOT: &str = "
func test() -> P_bool {
    !true
}
";

pub const PREFIX_OPERATION_MINUS: &str = "
func test() -> P_i64 {
    -4
}
";

pub const ARITHMETIC_OPERATOR_PRECEDENCE: &str = "
func test() -> P_i64 {
    2 + 2 * 2
}
";

pub const IF_EXPRESSION: &str = "
func test() -> P_i64 {
    if (1 > 2) {
        3
    } else {
        4
    }
}
";

pub const OBJECT_INITIALIZATION_WITH_TYPE_ARGUMENTS_AND_FIELD_ACCESS: &str = "
class X<T> {c: T}

class Y<T> {a: P_bool, b: X<T>}

func test() -> P_i64 {
    let y: Y<P_i64> = Y<P_i64> {a: true, b: X<P_i64> {c: 123}};
    y.b.c
}
";

pub const OBJECT_INITIALIZATION_AND_FIELD_ACCESS: &str = "
class X {c: P_i64}

class Y {a: P_bool, b: X}

func test() -> P_i64 {
    let y: Y = Y {a: true, b: X {c: 123}};
    y.b.c
}
";

pub const VARIABLE_INITIALIZATION_AND_REFERENCE_INT: &str = "
func test() -> P_i64 {
    let a: P_i64 = 10;
    let b: P_i64 = a;
    b
}
";

pub const FUNCTION_DECLARATION_ADD: &str = "
func add(a: P_i64, b: P_i64) -> P_i64 {
    a + b
}
";

pub const FUNCTION_CALL_ADD: &str = "
func add(a: P_i64, b: P_i64) -> P_i64 {
    a + b
}

func test() -> P_i64 {
    add(11, 22)
}
";

pub const FUNCTION_CALL_UPDATE_STATE: &str = "
class C<T> {a: T}

func updateState(c: C<P_i64>) -> C<P_i64> {
    C<P_i64> {a: c.a + 1}
}

func test() -> P_i64 {
    let c: C<P_i64> = C<P_i64> {a: 0};
    updateState(c).a
}
";

// TODO(derekxu16): Declare a test() function in this input.
pub const FUNCTION_CALL_FIB: &str = "
func fib(n: P_i64) -> P_i64 {
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
