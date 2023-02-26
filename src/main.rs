use std::process;
use std::io::{self, BufRead};

fn add(x: i32, y: i32) -> i32 {
    return x + y;
}

fn sub(x: i32, y: i32) -> i32 {
    return x - y;
}

fn mul(x: i32, y: i32) -> i32 {
    return x * y;
}

fn div(x: i32, y: i32) -> i32 {
    if y == 0 {
        println!("cannot divid by 0");
        process::exit(0);
    }
    return x / y;
}

fn evaluate(buffer: String) -> i32 {
    let index: i32 = 0;
    let parts: Vec<&str>;
    
    if buffer.contains("+") {
        parts = buffer.split('+').collect();
        return add(parts[0].trim().parse().expect("error: failed to parse number"), parts[1].trim().parse().expect("error: failed to parse number"));
    } else if buffer.contains("-") {
        parts = buffer.split('-').collect();
        return sub(parts[0].trim().parse().expect("error: failed to parse number"), parts[1].trim().parse().expect("error: failed to parse number"));
    } else if buffer.contains("*") {
        parts = buffer.split('*').collect();
        return mul(parts[0].trim().parse().expect("error: failed to parse number"), parts[1].trim().parse().expect("error: failed to parse number"));
    } else if buffer.contains("/") {
        parts = buffer.split('/').collect();
        return div(parts[0].trim().parse().expect("error: failed to parse number"), parts[1].trim().parse().expect("error: failed to parse number"));
    }
    return index;
}

fn is_input_valid(buffer: &mut String) -> bool {
    let mut one_operator: i32 = 0;
    
    for c in buffer.chars() {
        if matches!(c, '+' | '-' | '*' | '/') {
            one_operator += 1;
        }
        if !c.is_digit(10) && !c.is_whitespace() && !matches!(c, '+' | '-' | '*' | '/') {
            return false;
        }
    }
    if one_operator != 1 {
        return false
    }
    return true;
}

fn main() {
    let stdin = io::stdin();
    let mut reader = stdin.lock();
    let mut buffer = String::new();
    let result: i32;
    println!("Input should be only spaces, numbers or these math operators '+', '-', '*', '/'");
    reader.read_line(&mut buffer).expect("error: failed to read line");
    if !is_input_valid(&mut buffer) {
        println!("error: input must contain only whitespaces, numbers or one of these math operators '+', '-', '*', '/'");
        return ;
    }
    result = evaluate(buffer);
    println!("result is {}", result);
}


/*
1. check if still an operation to do and if not return the result
2. if still an operation to do:
    if there is a * or / in buffer cut string in two around operators and recursively call itself: eval(left) operators eval(right)
    if there is a - or + in buffer cut string in two around operators and recursively call itself: eval(left) operators eval(right)
    somehow add to the / part a condition to avoid case where right ends up evaluating to zero


fn evaluate(buffer: String) -> i32 {
 to ignore compilator errors:
    let test: i32 = 0;
    return test;

    if (!buffer.contains("+") && !buffer.contains("-") && !buffer.contains("*") && !buffer.contains("/")) {
        return buffer.parse().expect("Failed to parse number");
    }

}

syntax incorrect? if !buffer.matches(['+', '-', '*', '/']) {}

testing:
1 + 2
1 + 2 * 3
1 + 2 * 3 + 1

1 + 2 * 3 + 1

(1 + 2) * (3 + 1)

2 * 4 + 1 / 2 - 3
2 * 4
8 + 1 / 2 - 3
1 / 2
8 + 0.5 - 3
8 + 0.5
8.5 - 3
5.5
 */
