fn main() {
    println!("Hello, world!");
    fcc8();
    fcc9();
}

// fn first_name() {
//     //fcc 6
//     let first_name = "John";
//     //fcc 7 string concatenation VVV
//     println!("My name is {}", first_name);
// }
fn fcc8() {
    //fcc 8
    /*
    println! is a macro that prints text to the console.
    this makes it an excelent debugging tool, as you can
    print the value of a variable at various points in your
    code to see how it changes.
    --DOCS-- https://doc.rust-lang.org/rust-by-example/hello/print.html
    */
    let first_name = "John";
    let last_name = "Doe";
    println!("My name is {} {}", first_name, last_name);
}

fn fcc9() {
    //fcc 9
/*
The type of first_name is &str
str is a primitive type, and the ampersand (&) is a reference. indicates the type is a refrence.
A key aspect of rust is memory use and allocation (ownership)
another common type is String, since its heap allocated, it means the size can be unkown at complie time (dynamic)
*/
    let example = String::from("Hello, Campers!");
    let name = example;
    //fcc10, This some werid engagement. I would call this a poin^er but im pretty confused.
    //NEVER MIND: IF VARIABLE IS MOVED IN MEMORY, YOU CANT ACCESS IT ANYMORE. THIS MAKES SO MUCH SENSE
    println!("{}", name);
    println!("{}", example);
}