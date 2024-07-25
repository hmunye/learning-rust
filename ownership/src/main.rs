fn plus_one(x: i32) -> i32 {
    // The value passed in to the function (5) is placed on the stack now within the 'plus_one'
    // stack-frame
    //
    // This function is dropped after executing, so the stack will only include the 'main'
    // stack-frame
    x + 1
}

fn main() {
    // x and its bound value 5 are placed on the stack within the 'main' function stack-frame
    let x = 5;
    // y calls the plus_one() function, which is placed on the stack
    // The returned value from 'plus_one' is then placed in the 'main' stack-frame bound to y
    let y = plus_one(x);

    println!("{y}");

    ////////////////////////////////////////////////////////////////////////////////////////

    // Here, the value of a is copied into b, and a is left unchanged, as they are now seperate
    // variables
    //
    // Copying like this can be costly
    let _a = 5;

    let mut _b = _a;

    _b += 1;

    // Here, the array holds 1 million elements
    let _a = [0; 1_000_000];
    // When the value of a is copied into b, the 'main' stack-frame now contains 2 million elements
    let _b = _a;

    // 'Box' provides a way for allocating data on the heap
    //
    // Here '_a' owns the box and is responsible for its memory management
    let _a = Box::new([0; 10]);
    // This assignment copies the heap pointer to _b
    //
    // Without '&', '_b' would be the new owner of the box and Rust will deallocate the box on
    // behalf of b, only dropping once instead of twice. '_a' could no longer be used to access the
    // heap data
    //
    // '&' allows '_b' to borrow (reference) the value of _a (heap pointer) so that both can now point to the
    // box, with '_a' keeping ownership of the box
    let _b = &_a;

    // Another way to avoid moving is to clone the data. This returns a new box (heap allocation)
    // with the data as its contents
    let _b = _a.clone();

    let _c = Box::new(4);

    // This explicitly drops '_c' from memory
    drop(_c);

    // Using ':p' (Pointer formatter)
    println!("_a: {:?}, _b: {:?}, address: {:p}", _a, _b, _b);

    ////////////////////////////////////////////////////////////////////////////////////////

    let mut _x: Box<i32> = Box::new(1);
    let _a: i32 = *_x; // *_x reads the heap value, so _a = 1
    *_x += 1; // *_x on the left-side modifies the heap value,
              //     so _x points to the value 2

    let _r1: &Box<i32> = &_x; // _r1 points to _x on the stack
    let _b: i32 = **_r1; // two dereferences get us to the heap value

    let _r2: &i32 = &*_x; // _r2 points to the heap value directly
    let _c: i32 = *_r2; // so only one dereference is needed to read it

    ////////////////////////////////////////////////////////////////////////////////////////

    // Swapping values of variables
    let mut _a: Box<i32> = Box::new(10000000);
    let mut _b: Box<i32> = Box::new(20000000);

    println!("Before swap: _a = {}, _b = {}", _a, _b);

    let mut _ptr_a: &mut Box<i32> = &mut _a;
    let mut _ptr_b: &mut Box<i32> = &mut _b;

    **_ptr_a = **_ptr_a + **_ptr_b;
    **_ptr_b = **_ptr_a - **_ptr_b;
    **_ptr_a = **_ptr_a - **_ptr_b;

    println!("After swap: _a = {}, _b = {}", _a, _b);

    ////////////////////////////////////////////////////////////////////////////////////////

    // The borrow check looks for unsafe operations involving references
    //
    // In this case, 'v.push()' before the println! macro invalidates the reference 'num' because a new 
    // heap allocation needs to be created for a bigger Vec with the right capacity
    //
    // 'Shared references' are read-only immutable references, while 'unique references' are mutable
    // references
    let mut v: Vec<i32> = vec![1, 2, 3];

    //let num: &i32 = &v[0];

    let num_mut: &mut i32 = &mut v[1];

    *num_mut += 2;

    println!("{}", *num_mut);

    v.push(4);

    ////////////////////////////////////////////////////////////////////////////////////////

    let mut a = [0, 1, 2, 3];

    let x = &mut a[1];

    *x += 1;

    println!("{a:?}");

    let word = "Hello World";

    let first_word = first_word(&word);

    println!("First Word is [ {first_word} ]");

    println!(
        "&String={} &str={}",
        std::mem::size_of::<&String>(),
        std::mem::size_of::<&str>(),
    );
}

// The issue with this function is that once it returns the reference to 's', the function is
// dropped along with its local variables, so 's' is also dropped, leading '&s' to point to
// deallocated memory
//fn return_string() -> &String {
//    let s = String::from("Hello");
//    &s
//}

// To fix this we can return the String, moving ownership outside of the function
fn _return_string_own() -> String {
    let s = String::from("Hello");
    s
}

// Or return a string literal, with the 'static lifetime
fn _return_string_literal() -> &'static str {
    "Hello"
}

// Or defer borrow-checking to the runtime by using garbage collection. This can be dont using a
// reference-counted pointer
fn _return_string_rc() -> std::rc::Rc<String> {
    let s = std::rc::Rc::new(String::from("Hello"));
    // 'Rc::clone' only clones a pointer to s and not the data. At runtime, Rc checks when the last
    // Rc pointing to data has been dropped, then deallocates the data
    std::rc::Rc::clone(&s)
}

// Or have the caller of the function provide a 'slot' to put the string in using a mutable
// reference
fn _return_string_slot(output: &mut String) {
    output.replace_range(.., "Hello");
}

fn first_word(str: &str) -> &str {
    let bytes = str.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &str[..i];
        }
    }

    &str[..]
}
