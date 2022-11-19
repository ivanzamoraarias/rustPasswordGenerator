use std::env;
use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
use std::fs;
use chrono::Datelike;
mod errorsEnum;
mod defaults;

#[derive(Serialize, Deserialize)]
struct PasswordData {
    names: Vec<String>,
    lastNames: Vec<String>
}

#[derive(Serialize, Deserialize)]
struct ResultData {
    password: String,
    userName: String
}

fn main() {
    println!("=========PASSWORD GENERATOR=======");

    let args: Vec<String> = env::args().collect();

    let (user_name, file_path, result_path) = get_parsed_args(args);

    let rand_year = get_random_year();

    let password_data: PasswordData = get_password_data_from_file(file_path);

    let (rand_name_index, rand_surname_index) = get_random_index_from_password_data(&password_data);

    let result = format!(
        "{}{}@{}", 
        password_data.names[rand_name_index], 
        password_data.lastNames[rand_surname_index], 
        rand_year
    );
    
    println!("Your new password {}", result);

    let result_string = get_result_string(
        ResultData {
            password: result,
            userName: user_name
       }
    );

    println!("Result {}", result_string);

    fs::write(result_path, result_string).expect(errorsEnum::ERROR_UNABLE_TO_WRITE_FILE);

}

fn get_random_year() -> i32 {
    let current_date = chrono::Utc::now();
    let year = current_date.year();
    let rand_year = rand::thread_rng().gen_range(1991..year);

    return rand_year;
}

fn get_result_string(result_json: ResultData) -> String {
    let result_string = serde_json::to_string(&result_json).expect(errorsEnum::ERROR_FORMAT_JSON);
    return result_string;
}

fn get_random_index_from_password_data(password_data: &PasswordData) -> (usize, usize) {
    let rand_name_index = rand::thread_rng().gen_range(0..password_data.names.len());
    let rand_surname_index = rand::thread_rng().gen_range(0..password_data.lastNames.len());

    (rand_name_index, rand_surname_index)
}

fn get_password_data_from_file(file: String) -> PasswordData {
    let data = fs::read_to_string(file).expect(errorsEnum::ERROR_UNABLE_TO_READ_FILE);
    let json: PasswordData = serde_json::from_str(&data).expect(errorsEnum::ERROR_FORMAT_JSON);
    return json;
}

fn get_parsed_args(args: Vec<String>) -> (String, String, String) {
    let user_name = if args.len() > 1 { &args[1] } else { defaults::DEFAULT_USER_NAME };
    let file_path = if args.len() > 2 { &args[2] } else { defaults::DEFAULT_FILE_PATH };
    let result_path = if args.len() > 3 { &args[3] } else { defaults::DEFAULT_RESULT_PATH };

    (user_name.to_string(), file_path.to_string(), result_path.to_string())
}
