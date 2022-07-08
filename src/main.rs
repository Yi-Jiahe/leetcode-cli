extern crate dotenv;

use dotenv::dotenv;
use std::env;

use leetcode_cli::endpoints;

fn main() {    
    dotenv().ok();

    let csrftoken = match env::var("CSRFTOKEN") {
        Ok(value) => value,
        Err(error) => match error {
            env::VarError::NotPresent => panic!("CSRFTOKEN variable not set"),
            env::VarError::NotUnicode(_value) => panic!("CSRFTOKEN value: invalid"),
        },
    };

    let daily_question = match endpoints::get_daily(&csrftoken) {
        Ok(result) => result, 
        Err(error) => panic!("{}", error),
    };
}
