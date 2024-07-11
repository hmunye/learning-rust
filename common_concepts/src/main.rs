fn main() {
    // Once an immutable variable is bound to a value, that value associated with the variable can
    // not be changed
    //
    // Variables with the '_' prefix are not flagged for being unused
    let _x = 5;

    // Assigning a value twice to _x is not allowed
    // _x = 6;

    // We can make a variable mutable by using the keyword 'mut' before the variable name
    let mut _y = 3;

    // This is allowed since '_y' is mutable
    _y = 4;

    // Constants are similar to immutable variables in that
    // the value they are both bound to is not allowed to change
    //
    // Differences include:
    //
    // The 'mut' keyword can not be used with consts, making them always immutable
    // We must always annotate the type of a const
    // consts can be declared in any scope, including the global scope
    // conts can only be bound to a constant expression, not the result of a value that is computed
    // at runtime
    //
    // Naming convention is to use uppercase with '_' seperating words
    const _MY_CONST: usize = 4324554;

    let _z = 14;

    // Variables can be shadowed by including the 'let' keyword when declaring the new variable
    // with the same name
    //
    // The compiler will only see the second variable in this case. The second variable
    // 'overshadows' the first
    let _z = 10;

    // This inner scope shadows '_z', but then is dropped after the scope is over
    // The value printed for '_z' will therefore be 10
    {
        let _z = 13;
        println!("Inner scope value for _z: {_z}");
    }

    println!("Value for _z: {_z}");

    // The difference between 'mut' and 'shadowing' is that mut variable can not have their type
    // changed while shadowed variables can

    // Integer literals
    let _dec = 20_0124;
    let _hex = 0x32A;
    let _oct = 0o0347;
    let _bin = 0b1101_1001;
    // Only u8
    let _byte = b'C';

    // Tuples are a general way of grouping values with similar or distinct types into one compund
    // type
    let _tuple: (i16, i32, i64) = (3, 55, 193);

    println!("{}, {}, {}", _tuple.0, _tuple.1, _tuple.2);

    // With arrays, every value must be of the same type
    // Arrays in rust have a fixed length
    let _array = [1, 4, 5, 6, 7, 0];

    // When using type annotation, the first value is the type of the elements and the second value
    // is the number of elements
    let _array: [&str; 3] = ["test", "test", "test"];

    // Arrays can also be initialized with the same value for each element
    // This results in the array '[3, 3, 3, 3, 3]'
    let _array = [3; 5];

    let _first = _array[0];

    // Control Flow

    if _first < 4 {
        println!("True");
    } else if _first > 4 {
        println!("False");
    } else {
        println!("Unknown");
    }

    // if expression can be used in a let statement
    // The value will be bound based on the result of the if expression
    let _first = if _first < 3 { 6 } else { 4 };

    // Creates an infinite loop
    loop {
        println!("Looping");
        break;
    }

    let mut x = 0;
    // Loops can have labels to disambiguate between them
    let mut _result = 'counting: loop {
        loop {
            if x > 10 {
                break;
            } else {
                x += 1
            }
        }

        break 'counting x * 2;
    };

    println!("{_result}");

    // While loop
    while _result < 30 {
        _result += 1;
    }

    // For loop
    let _array = [1, 2, 4, 5, 6, 7];

    for element in _array {
        println!("{element}");
    }

    for number in (1..4).rev() {
        println!("{number}");
    }
}

// Functions and variable names are conventionally in 'snake case'
//
// Parameters passed must have type annotations (their types must be declared)
//
// The return type is specified with '->' with the type being returned
fn _add_numbers(x: i32, y: i32) -> i32 {
    // The value binded to variables can be an expression
    let z = {
        let num = 2;
        x + y + num
    };

    // This returns z
    z
}

// When a function ends with a statement, it implicitly returns the unit type '()' as there is no
// value to return
fn _some_fn(string: &str) {
    println!("{string}");
}
