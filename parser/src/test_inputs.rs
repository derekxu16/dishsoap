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

pub const RAW_VEC: &str = "
class RawVec {
    capacity: P_i64,
    start_pointer: P_i64,
}

func RawVec_new() -> RawVec {
    RawVec {
        capacity: 0,
        start_pointer: 0,
    }
}

func RawVec_growBy(v: RawVec, growBy: P_i64) -> RawVec {
    let next_capacity: P_i64 = v.capacity + growBy;
    let next_start_pointer: P_i64 = __malloc(next_capacity);

    let _: P_unit = __memMove(next_start_pointer, v.start_pointer, v.capacity);

    let _: P_unit = __free(v.start_pointer);

    RawVec {
        capacity: next_capacity,
        start_pointer: next_start_pointer,
    }
}

// TODO(derekxu16): This should actually be
// `func RawVec_store<T>(v: RawVec<T>, offset: P_i64, element: T) -> P_unit`.
func RawVec_store(v: RawVec, index: P_i64, element: P_i64) -> P_unit {
    __memStore(v.start_pointer, index, element)
}

func RawVec_load(v: RawVec, index: P_i64) -> P_i64 {
    __memLoad(v.start_pointer, index)
}

func test() -> P_i64 {
    let v: RawVec = RawVec_new();
    let v2: RawVec = RawVec_growBy(v, 6);
    let _: P_unit = RawVec_store(v2, 5, 123);
    let v3: RawVec = RawVec_growBy(v2, 3);
    RawVec_load(v3, 5)
}
";
