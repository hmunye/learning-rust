// Defining a struct
struct User {
    name: String,
    age: u32,
}

// Tuple struct
struct _Color(i32, i32, i32);

// Unit-like struct
struct _IsEqual;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Creating an implementation block for Rectangle. Everything within this block is associated with
// the Rectangle type
impl Rectangle {
    // Methods and functions are similar. Methods are however, defined within the context of a
    // struct
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn new(width: u32, height: u32) -> Self {
        Rectangle { width, height }
    }

    fn can_hold(&self, rec: &Rectangle) -> bool {
        self.width > rec.width && self.height > rec.height
    }
}

fn main() {
    // Creating an instance of the struct
    let mut _user1 = User {
        name: String::from("John"),
        age: 30,
    };

    // The entire instance must be mutable to change any field's value
    _user1.age = 40;

    let _user2 = make_user(String::from("Jason"), 28);

    // Sets a new age for the instance but uses the rest of the values (name) from '_user1'
    //
    // With this assignment, '_user1' could no longer be used because the String in 'name' was
    // moved from '_user1' to '_user3'
    //    let _user3 = User {
    //        age: 20,
    //        .._user1
    //    };

    // Accessing fields from the instance using dot notation
    println!("{} is {} years old", _user1.name, _user1.age);
    println!("{} is {} years old", _user2.name, _user2.age);

    let rec = Rectangle {
        width: 30,
        height: 10,
    };

    let rec2 = Rectangle::new(20, 40);

    println!("{}", rec.can_hold(&rec2));

    let _area = Rectangle::area(&rec);
    // or
    let _area1 = rec.area();

    dbg!(&rec);
}

fn make_user(name: String, age: u32) -> User {
    // Since the parameter names and struct fields have the same name, we can use the init
    // shorthand to initialize each field
    User { name, age }
}
