/*
Coded By Byron Dalberg
Guided by FreeCodeCamp 
Last Edit: 12/9/2022
note - lessons fcc1-fcc42 are in "./RUST-REPO/calculator/"
note - lessons fcc43-fcc55 are in "./RUST-REPO/properCalc/" << This Repo
*/
// use std::io; //for idiots check comment below
use std::env;

fn main() {
    //get user input for calculator (for idiots, maybe more convoluted than it needs to be for coders but if you try to multiply with * it returns file locations , this is a commandline function afterall)
        /*println!("Enter a simple calculation ex. '1 + 1'");
        let stdin = io::stdin();
        let mut user_input = String::new();
        stdin.read_line(&mut user_input).expect("Failed to read line");
        //parse user input
        let mut user_input = user_input.split_whitespace();
        let first_number = user_input.next().unwrap().parse::<i32>().unwrap();
        let operator = user_input.next().unwrap().chars().next().unwrap();
        let second_number = user_input.next().unwrap().parse::<i32>().unwrap();
        */
    let mut args = env::args();
    // for arg in args { 
    //     println!(":{}:", arg);
    // }

    //Your code Probs Doesnt Like Args if your using '*' in your math.  '*' lists and returns files in directory as env::args
    //That was part of your problem, the other part was that the 'nth(arg)' function adds its arg to current nth spot.
    let first_number = args.nth(1).unwrap().parse::<i32>().unwrap();
    let operator = args.nth(0).unwrap().parse::<char>().unwrap();
    let second_number = args.nth(0).unwrap().parse::<i32>().unwrap();
    // println!("{} {} {}", first_number, operator, second_number);
    // // let answer = calculate(first_number, operator, second_number);
    // //output
    // println!("output: {}", output(first_number, operator, second_number));
}
// fn calculate(first_number: i32, operator: char, second_number: i32) -> i32 {
//     let answer = match operator { //Better way of doing fcc33. Why use [if else] if you can just match. edit: this is fcc35.
//         '+' => first_number + second_number,
//         '-' => first_number - second_number,
//         '*' | 'x' | 'X'  => first_number * second_number, //fcc 36, 'or' operator
//         '/' => first_number / second_number,
//         _ => panic!("Invalid Operator"), //And this ones fcc34
//     };
//     answer
// }
    
// fn output(first_number: i32, operator: char, second_number: i32) -> String {
//     let answer = calculate(first_number, operator, second_number);
//     format!("{} {} {} = {}", first_number, operator, second_number, answer)
// }
