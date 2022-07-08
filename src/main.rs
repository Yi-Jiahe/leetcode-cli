extern crate dotenv;

use dotenv::dotenv;
use std::env;

use leetcode_cli::endpoints;

fn main() -> Result<(), std::env::VarError> {    
    dotenv().ok();

    let csrftoken = env::var("CSRFTOKEN")?;

    endpoints::get_daily(&csrftoken);

    Ok(())
}
