#[allow(dead_code)]
// Enums allow you to define a type by enumerating its possible variants
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[allow(dead_code)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(u32, u32, u32),
}

fn main() {
    // Create instances of each variant
    let ipv4 = IpAddr::V4(127, 0, 0, 1);
    let ipv6 = IpAddr::V6(String::from("::1"));

    let message = Message::ChangeColor(10, 7, 23);

    route(ipv4);
    route(ipv6);
    get_message(message);

    let a = [1, 2, 3, 4, 5];

    match does_exist(&a, 3) {
        Some(index) => println!("Target found at index [ {index} ]"),
        None => println!("Target not found"),
    };

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = x + y.unwrap();

    println!("Sum = {sum}");
    // or
    let _sum = match y {
        Some(y) => {
            let sum = x + y;
            println!("Sum = {sum}")
        }
        None => println!("Error"),
    };

    let secret_number = 32;

    // Using the catch-all pattern while throwing away the value
    match secret_number {
        4 => println!("Its 4"),
        10 => println!("Its 10"),
        _ => println!("Its some other number"),
        // Catch-all pattern while using the value (Can have any valid name)
        // other => println!("Number was {other}")
    }

    // Using 'if let' syntax
    let binary = Some(0b1001_1011);
    if let Some(decimal) = binary {
        println!("{decimal}")
    }
    // Is equivalent to this if we do not want to handle other cases
    let binary = Some(0b1001_1011);
    match binary {
        Some(decimal) => println!("{decimal}"),
        _ => (),
    }
}

fn does_exist(array: &[i32], target: i32) -> Option<usize> {
    for (index, element) in array.iter().enumerate() {
        if *element == target {
            return Some(index);
        }
    }
    None
}

fn route(ip_type: IpAddr) {
    match ip_type {
        IpAddr::V4(..) => println!("Version 4"),
        IpAddr::V6(..) => println!("Version 6"),
    }
}

fn get_message(message_type: Message) {
    match message_type {
        Message::Quit => println!("Quitting now"),
        Message::Move { .. } => println!("Moving"),
        Message::Write(..) => println!("Writing now"),
        Message::ChangeColor(..) => println!("Changing color"),
    }
}
