use std::collections::HashMap;
use std::io;

// Given a list of integers, use a vector and return the mean (the average value), median (when sorted, the value in the middle position), and mode (the value that occurs most often; a hash map will be helpful here) of the list.

#[derive(Debug)]
struct NumProps {
    mean: Option<f32>,
    median: Option<u32>,
    mode: Option<u32>,
}

fn to_median(nums: &Vec<u32>) -> u32 {
    let mut nums_copy: Vec<u32> = nums.clone();
    nums_copy.sort();
    nums_copy[nums_copy.len() / 2]
}

fn to_mode(nums: &Vec<u32>) -> u32 {
    let mut occurrences = HashMap::new();

    for &value in nums {
        *occurrences.entry(value).or_insert(0) += 1;
    }

    occurrences
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(val, _)| val)
        .expect("Cannot compute the mode of zero numbers")
}

fn num_properties(nums: &Vec<u32>) -> NumProps {
    let sum: u32 = nums.iter().sum();
    let mean: f32 = sum as f32 / nums.len() as f32;
    let median: u32 = to_median(&nums);
    let mode: u32 = to_mode(&nums);

    NumProps {
        mean: Some(mean),
        median: Some(median),
        mode: Some(mode),
    }
}

// Convert strings to pig latin. The first consonant of each word is moved to the end of the word and “ay” is added, so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!

fn pig_latin(phrase: &String) -> String {
    phrase
        .split_whitespace()
        .map(pigify_word)
        .collect::<Vec<String>>()
        .join(" ")
        .to_lowercase()
}

fn pigify_word(word: &str) -> String {
    let mut chars = word.chars();

    let first_char = match chars.next() {
        Some(c) => c,
        None => return String::new(),
    };

    match first_char {
        'a' | 'e' | 'i' | 'o' | 'u' | 'y' => format!("{}-hay", word),
        _ => format!("{}-{}ay", chars.as_str(), first_char),
    }
}

// Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company. For example, “Add Sally to Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.

fn init_employee_registry() {
    let mut registry: HashMap<String, Vec<String>> = HashMap::new();

    println!("Employee registry");
    println!("Available commands:");
    println!("\"add <Person> to <Department>\"");
    println!("\"list\" to show the registry");
    println!("\"exit\" to exit the program");

    loop {
        let mut input = String::new();

        println!("Insert command");

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let words = input.split_whitespace().collect::<Vec<&str>>();

        match words.as_slice() {
            ["add", name, "to", department] => {
                // TODO: Make personnel and department names consistent (capitalize)
                // TODO: Maybe remove person from another department if added to different department
                registry
                    .entry(department.to_string())
                    .and_modify(|e| add_if_does_not_exist(name.to_string(), e))
                    .or_insert(vec![name.to_string()]);
                println!("Added {} to {} department", name, department);
            }
            ["exit"] => {
                println!("Bye");
                break;
            }
            ["list"] => println!("Registry {:?}", registry),
            _ => println!("Invalid command"),
        };
    }
}

fn add_if_does_not_exist(word: String, vector: &mut Vec<String>) {
    match vector.iter().find(|x| **x == word) {
        Some(_) => (),
        None => vector.push(word),
    }
}

fn main() {
    let test_nums = vec![1, 1, 1, 1, 7, 7, 8, 7, 9];
    let test_phrase = String::from("Little boy fifteen apples and twelve oranges");

    let mut test_string_vec = vec![
        String::from("one"),
        String::from("two"),
        String::from("three"),
    ];
    add_if_does_not_exist(String::from("four"), &mut test_string_vec);

    println!("String vec {:?}", test_string_vec);

    println!("Numbers {:?}", test_nums);
    println!("Number properties: {:?}", num_properties(&test_nums));
    println!("Original phrase: {}", test_phrase);
    println!("Phrase in pig latin: {}", pig_latin(&test_phrase));
    println!("------------------------------------------------");
    init_employee_registry()
}
