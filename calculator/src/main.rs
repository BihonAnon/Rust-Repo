fn main() {
    println!("Hello, world!");
    first_name();
}

fn first_name() {
    //fcc 6
    let first_name = "John";
    //fcc 7 string concatenation VVV
    println!("My name is {}", first_name);
}
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