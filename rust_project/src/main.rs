use std::io::{stdin, stdout, Write};

fn input() -> Result<String, String> {
    //! # Get user input
    //! 1. print UI
    //! 2. get input
    //! 3. test
    //! 4. return result
    // UI
    print!("?> ");
    stdout().flush().ok();
    // Input
    let mut input: String = "".to_string();
    stdin().read_line(&mut input).unwrap();
    // if exit
    println!("{input}");
    if input == "q".to_string(){
        println!("QQQ");
        return Err(input);
    }
    // test and return result
    match input.is_empty() {
        true => {
            return Err("!! Error: empty input".to_string());
        }
        false => {
            return Ok(input);
        }
    }
}
// TODO unit test

fn input_parser_slicer(raw_problem: Result<String, String>) -> Result<Vec<String>, String> {
    //! # Just make vector of strings.
    //! 1. remove white spaces
    //! 2. add spaces before operators (sin tan)
    //! 3. trim spaces
    //! 4. test
    //! 5. return result
    match raw_problem {
        Err(error) => {
            return Err(error);
        }
        Ok(raw_problem) => {
            // BODY

            return Ok(vec!["123".to_string()]);
        }
    }
}
// TODO unit test

fn prioritizatron(wrong_problem: Result<Vec<String>, String>) -> Result<Vec<String>, String> {
    //! # Make prioritization of problem
    //! 1. iterates over vector
    //! 2. find operators
    //! 3. add brackets to make priorit
    //! 4. test
    //! 5. return result
    match wrong_problem {
        Err(error) => {
            return Err(error);
        }
        Ok(wrong_problem) => {
            // BODY

            return Ok(wrong_problem);
        }
    }
}
// TODO unit test

fn parser_calculaser(problem: Result<Vec<String>, String>) -> Result<f64, String> {
    //! # Parse problem and calculate
    //! 1. race
    match problem {
        Err(error) => {
            return Err(error);
        }
        Ok(problem) => {
            // BODY

            return Ok(-1.0);
        }
    }
}
// TODO unit test

fn answer(answer: Result<f64, String>) -> char {
    //! Print answer or error
    match answer {
        Err(error) => {
            if error == "q".to_string() {
                println!("qqqqqqq");
                return 'q';
                // FIXME exit loop
            }
            print!("!! {error}\n\n");
            stdout().flush().ok();
            return 'e';
        }
        Ok(answer) => {
            // BODY

            print!("=> {answer}\n\n");
            stdout().flush().ok();
            return 'o';
        }
    }
}
// TODO unit test

fn main() -> () {
    println!("Type problem or q to quit. 😉");
    loop {
        if answer(parser_calculaser(prioritizatron(input_parser_slicer(
            input(),
        )))) == 'q'
        {
            return ();
        }
    }
}
