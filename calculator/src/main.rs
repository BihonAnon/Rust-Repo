use std::io;

fn main() {
    //fcc28 asks us to do some bad stuff, (take varibles directly to the main function), on the online version i guess it doesnt matter cuz thers already a main function running in the background
    //this is rusts suggested way of collecting user input. (i think)
    println!("Hello, world!");
    println!("Calculator! Input your simple eq (ex. 1 + 1)");
    let mut user_input = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut user_input).expect("Failed to read line");
    let mut user_input = user_input.split_whitespace(); //THIS FUNCTIONS AWESOME
    let first_number = user_input.next().unwrap().parse::<f32>().unwrap();
    let operator = user_input.next().unwrap().chars().next().unwrap();
    let second_number = user_input.next().unwrap().parse::<f32>().unwrap();
    println!(
        "{}",
        fcc28(first_number, operator, second_number)
    );
//I dont know what this does, but i think its a code safty thing?
} 

// fn first_name() {
//     //fcc 6
//     let first_name = "John";
//     //fcc 7 string concatenation VVV
//     println!("My name is {}", first_name);
// }
// fn fcc8() {
//     //fcc 8
//     /*
//     println! is a macro that prints text to the console.
//     this makes it an excelent debugging tool, as you can
//     print the value of a variable at various points in your
//     code to see how it changes.
//     --DOCS-- https://doc.rust-lang.org/rust-by-example/hello/print.html
//     */
//     let first_name = "John";
//     let last_name = "Doe";
//     println!("My name is {} {}", first_name, last_name);
// }

// fn fcc9() {
//     //fcc 9
//     //
//     // The type of first_name is &str
//     // str is a primitive type, and the ampersand (&) is a reference. indicates the type is a refrence.
//     // A key aspect of rust is memory use and allocation (ownership)
//     // another common type is String, since its heap allocated, it means the size can be unkown at complie time (dynamic)
//     let example = String::from("Hello, Campers!");
//     let name = example;
//     //fcc10, This some werid engagement. I would call this a poin^er but im pretty confused.
//     //NEVER MIND: IF VARIABLE IS MOVED IN MEMORY, YOU CANT ACCESS IT ANYMORE. THIS MAKES SO MUCH SENSE
//     println!("{}", name);
//     println!("{}", example); //YE(S I GET AN ERROR FCC12
// }

// fn fcc13() {
//     let example = String::from("Hello, Campers!");
//     let refrence_name = &example; // << (fcc13) Call as refrence (using &) and the varibale wont be moved in memory, so it can both can be accessed.
//     println!("example : {} refrence : {}", example, refrence_name);
// } //Code Works, thats fcc13 & fcc14

// fn fcc15() {
//     let firstname = String::from("John");
//     let owned_string = firstname.to_owned() + " Doe";
//     println!("{}", owned_string);
// }//Code Works, thats fcc15/

// fn fcc16() //Generalized fcc16, for my own sanity later when i refrence this.
//{
//     let mut my_string = String::from("thisIsAString");
//     my_string.push_str(" AndThisIsMore");
//     println!("{}", my_string); 
// } //Code Works, thats fcc16 & fcc17 & fcc18 & fcc19

// fn fcc20() {
//     //fcc20 explains that strings can be thought of as collections (arrays) of char (Unicode Scalar Values (USV) ex. [ U+221E = ∞ ])
//     let first = "J"; //fcc20 done
//     println!("focus[{}] .len() {} // .chars().count() {}", first, first.len(), first.chars().count()); //fcc21, but sanity added
//     //fcc22 => The len method returns the length in bytes for the str. The chars method returns an iterator over the chars in the string slice, and the count method returns the number of elements in the iterator.
//     let first = "∞"; //fcc23
//     println!("focus[{}] .len() {} // .chars().count() {}", first, first.len(), first.chars().count()); //fcc23 again, sanity added


//     let mut char_array = String::from(first);
//     char_array.push_str("I  ");
//     println!("{}", first);
//     //Processing.org
//     //Natron
//     //Houdini
//     //affinity
// }

// fn fcc26() -> usize { //PURPOSE??? IM LOST??? nevermind, 27 has to do with data types, intro lesson
//     24 
// } //fcc26 Done

// #[cfg(test)]
// mod tests {
//   use crate::fcc27;
//   #[test]
//   #[should_panic] //THIS MAKES THE PROGRAM ERROR OUT? OR SHOULD ERROR OUT? Testing is werid.
//   fn main_panics_with_i() {
//     assert_eq!(main() as usize as f32, main() as f32);
//   }
//   #[test]
//   fn main_returns_f() {
//     assert_eq!(main() as f32, 24.5);
//   }
// } //fcc27 DONE

// fn fcc28alt() -> io::Result<()> { //ALRIGHT THIS IS USER INPUT. ERRR How do i make it read at the same time as run?
//     //Got it, wrong implementation, but i got it.
//     let mut user_input = String::new();
//     io::stdin().read_line(&mut user_input)?; // we get Stdin from the user
//     println!("You entered: {}", user_input);
//     Ok(())
// }

fn fcc28(first_number: f32, operator: char, second_number: f32) -> String {
    // println!("Your Input: {} {} {}", first_number, operator, second_number);
    // println!("Calculating");
    let answer = match operator {
        '+' => first_number + second_number,
        '-' => first_number - second_number,
        '*' => first_number * second_number,
        '/' => first_number / second_number,
        _ => panic!("Invalid Operator"),
    };
    format!("Result: {} {} {} = {}", first_number, operator, second_number, answer) //fcc29 using this result! funciton, if you want it to return a string you need to not have a semicolon on the end.
}