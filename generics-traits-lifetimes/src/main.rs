// Rust offers 'generics' or abstract stand-ins for concrete types or other properties
// Functions can take parameters of some generic type instead of concrete types like 'i32' or 'String'

use std::fmt::{Debug, Display};

struct NewsArticle {
    author: String,
    title: String,
    content: String,
}

// A 'trait' defines functionality a paticular type has and can share with other types
// Traits are somewhat similar to 'interfaces'
trait Summary {
    fn summarize(&self) -> String;

    fn default(&self) -> String {
        String::from("Read more...")
    }
}

// Implementation of the summarize function defined in the trait
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{} by {}: {}", self.author, self.title, self.content)
    }
}

fn main() {
    let list = vec![1, 2, 3, 4, 5, 6];

    let char_list = vec!['a', 'b', 'c', 'd', 'e'];

    let largest_num = largest(&list);
    let largest_char = largest(&char_list);

    println!("Largest value in list is {}", largest_num);
    println!("Largest value in char_list is {}", largest_char);

    ////////////////////////////////////////////////////////////////////////////////////

    let article = NewsArticle {
        author: String::from("John"),
        title: String::from("New Article"),
        content: String::from("Article content"),
    };

    println!("1 new atricle: {}", article.summarize());
    println!("{}", article.default());

    ////////////////////////////////////////////////////////////////////////////////////

    // The main goal of 'lifetimes' are to prevent dangling references
}

// Using lifetime annotations describe the relationships of the lifetimes 
// of multiple references to each other without affecting the lifetimes
fn _longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Example of extracting a function to remove and avoid duplication
//
// Here, this function uses generics so it can be reused on a slice of any type 'T'
// We restrict the type parameter 'T' by implementing the trait 'PartialOrd'
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for n in list {
        if n > largest {
            largest = n;
        }
    }

    largest
}

// Using Traits as parameters

fn _get_article(item: &impl Summary) {
    println!("{}", item.summarize());
}

// Trait bound syntax

fn _get_article_generic<T: Summary>(item: &T) {
    println!("{}", item.summarize());
}

// Specifying multiple trait bounds with '+'

fn _mutliple_trait_bounds<T: Summary + Display>(item: &T) {
    println!("{:?}", item.summarize());
}

// Using 'where' clause
fn _where_clause<T, U>(t: &T, u: &U)
where
    T: Display + Clone,
    U: Clone + Debug,
{
    println!("{}, {:?}", t, u);
}
