use axum::{extract::Query, response::Html, routing::get, Router};
use serde::Deserialize;
use std::collections::HashSet;
use std::fs::*;
use std::{collections::HashMap, fs, net::SocketAddr};

/// # Program that serves a Form
/// containing three parameters
/// 1. the language of choice
/// 2. option to include or not include input character count
/// 3. the inout to be checked for misspelling
/// ## Limitations
/// - german: capitalisation is not considered (as i dont have a dictionary for all german words,
/// that is capitalised correctly)
///
/// that's pretty much it tho 🥰
#[allow(unused)]
#[tokio::main]
async fn main() {

    // defined routes and handles
    let app = Router::new()
        .route("/", get(handler))
        .route("/check_spelling", get(handler_input));

    // attach to socket
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

/// # Form data
/// - `language` the language that the user chose to correct against
/// - `letter_count` the option to include a letter count of initial input
/// - `input` the input that is to be checked
#[allow(unused)]
#[derive(Deserialize)]
struct RequestParameters {
    language: String,
    letter_count: usize,
    input: String,
}

/// # Creates a dictionary of the langauge
/// by adding every word in the dictionary to a `HashSet`
/// which is the used via
/// ```
/// HashSet.contains(word)
/// ```
/// to check if the word is written correctly
///
/// Fun Fact: dictionary set around 48 bytes at runtime, HOW ? I have no idea
fn create_dictionary_set(language: String) -> HashSet<String> {
    let mut res: HashSet<String> = HashSet::new();

    if language == "english".to_string() {
        let paths = fs::read_dir("src/dictionary_en").expect("Upps i didnt find the english dict");
        for path in paths {
            for line in fs::read_to_string(path.expect("Path was unvalid").path()).expect("line was invalid utf-8").lines() {
                res.insert(line.to_string());
            }
        }
        return res;
    } else if language == "german".to_string() {
        let paths = fs::read_dir("src/dictionary_ger").expect("Uups i didnt find the english dict");
        for path in paths {
            // IMPORTANT! If error occurs, it can be due to macos adding .DS_STORE file into a folder which cant be read as it doesn't contain anything
            for line in fs::read_to_string(path.as_ref().expect("Path was unvalid").path()).expect(&format!("line wasn't valid utf-8 at path {}", path.as_ref().unwrap().path().display())).lines() {
                res.insert(line.to_string());
            }
        }

        // Reference to the size of dictionary at runtime 🫙
        println!("{}", std::mem::size_of_val(&res));
        return res;
    }
    return res;
}

/// # Checks if words in input are spelled correctly
/// returns a `String` of wrongly spelled words
/// operated by a comma
/// as a "side effect" it also cleans the string of any
/// leading or trailing whitespaces
/// as well as special characters such as ,.?!
fn check_spelling_and_return_string_of_wrong_spelled_words(
    dic: HashSet<String>,
    input: &String,
) -> String {
    let mut list_of_wrong_words: Vec<String> = Vec::new();

    let input = clean_input_to_check(input);
    for word in input.split(" ") {
        if !dic.contains(word) {
            list_of_wrong_words.push(word.to_string());
        }
    }
    list_of_wrong_words.join(",").to_string()
}

/// # Cleans any .,!? symbols from the input
/// if the symbol occurs at the end of the word
fn clean_input_to_check(input: &String) -> String {
    let special_chars = ['.', ',', '!', '?'];
    let mut res = String::new();
    if input.is_empty() {
        return "Nothing to see here 👀".to_string();
    }

    let input = input.trim();
    let input = input.to_lowercase();


    for mut word in input.split(" ") {
        let last_char = word.chars().last().unwrap();
        if special_chars.contains(&last_char) {
            let last_index = word.len() - 1;
            res.push_str(&word[0..word.len() - 1]);
            res.push(' ');
        } else {
            res.push_str(&word[0..word.len()]);
            res.push(' ');
        }
    }
    res
}

/// # Initial Route
/// grabs form data such as:
/// - langauge: the language of the text
/// - input: the text input to be checked
async fn handler() -> Html<String> {
    Html(r#"<!DOCTYPE html>
<html lang="en">

<head>
    <meta charset="UTF-8">
    <link rel="preconnect" href="https://fonts.googleapis.com">
    <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
    <link href="https://fonts.googleapis.com/css2?family=Roboto&display=swap" rel="stylesheet">

    <title>Spellinator</title>
</head>
<body>

    <div class="container">
      <h1 class="head_line" style="font-family: 'Roboto', sans-serif; font-weight: bold; text-align: center;">Spellinator</h1>
    </div>
    <div class="main_content" style="margin: auto; width: 60%; font-weight: bold;">
        <form action="check_spelling">
            <label for="language" style="font-weight: bold;
            color: #558b6e; text-align: center; font-family: 'Roboto', sans-serif;">What language do you wish to check</label>
            <select id="language" name="language" class="lang">
                <option value="english">english 🏴󠁧󠁢󠁥󠁮󠁧󠁿</option>
                <option value="german">german 🇩🇪</option>
            </select><br>
            <label for="letter_count" style="font-weight: bold; text-align: center; font-family: 'Roboto', sans-serif;">Show input letter count ?</label>
            <select id="letter_count" name="letter_count" class="text">
                <option value="1">Yes please, I'd love a letter count! 🥰</option>
                <option value="0">No thank you, I like to not know my letter count 🥷🏼</option>
            </select><br>
            <label for="input" style="font-family: 'Roboto', sans-serif; font-weight: bold; text-align: center;">Please enter the text to check</label>
            <input type="text" id="input" name="input" style="width: 100%;
            padding: 12px 20px;
            margin: 8px 0;
            color: #558b6e;
            border-color: #558b6e;
            font-weight: bold;
            box-sizing: border-box;
            border-radius: 10px;" value=""><br>
            <input type="submit" value="check" class="submit_button" style="font-weight: bold; color: #ffffff; background-color: #558b6e; border-radius: 5px; height: 2rem; width: 5rem; margin: 20px; border: none;">
            <h3 style="color: #558b6e; font-family: 'Roboto', sans-serif; font-weight: bold; text-align: center;">Nothing to see here <h3>
        </form>
    </div>
</body>
</html>"#.to_string())
}

/// # Route for submitted data
/// uses data collected from form
/// to display a text with what was spelled wrongly
#[allow(unused)]
async fn handler_input(
    Query(form_params): Query<RequestParameters>) -> Html<String> {
    // Parameters
    let lang = form_params.language;
    let letter_count = form_params.letter_count;
    let input_text = form_params.input;
    let mut list_of_wrong_words = String::new();
    let mut letter_counted_number = 0;

    let dict = create_dictionary_set(lang);
    list_of_wrong_words = check_spelling_and_return_string_of_wrong_spelled_words(dict, &input_text);

    if input_text.is_empty() {
        list_of_wrong_words = "Nothing to see here 👀".to_string();
    }

    if list_of_wrong_words.is_empty() {
        list_of_wrong_words = "Nothing to see here 👀".to_string();
    }

    for c in input_text.chars() {
        if c != ' ' {
            letter_counted_number += 1;
        }
    }

    if letter_count == 1 {
        list_of_wrong_words = format!("{}: letter_count = {}", list_of_wrong_words, letter_counted_number);
    }

    Html(format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <link rel="preconnect" href="https://fonts.googleapis.com">
    <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
    <link href="https://fonts.googleapis.com/css2?family=Roboto&display=swap" rel="stylesheet">

    <title>Spellinator</title>
</head>
<body>
    <div class="container">
      <h1 class="head_line" style="font-family: 'Roboto', sans-serif; font-weight: bold; text-align: center;">Spellinator</h1>
    </div>
    <div class="main_content" style="margin: auto; width: 60%; font-weight: bold;">
        <form action="check_spelling">
            <label for="language" style="font-weight: bold;
            color: #558b6e; text-align: center; font-family: 'Roboto', sans-serif;">What language do you wish to check</label>
            <select id="language" name="language" class="lang">
                <option value="english">english 🏴󠁧󠁢󠁥󠁮󠁧󠁿</option>
                <option value="german">german 🇩🇪</option>
            </select><br>
            <label for="letter_count" style="font-weight: bold; text-align: center; font-family: 'Roboto', sans-serif;">Show input letter count ?</label>
            <select id="letter_count" name="letter_count" class="text">
                <option value="1">Yes please, I'd love a letter count! 🥰</option>
                <option value="0">No thank you, I like to not know my letter count 🥷🏼</option>
            </select><br>
            <label for="input" style="font-family: 'Roboto', sans-serif; font-weight: bold; text-align: center;">Please enter the text to check</label>
            <input type="text" id="input" name="input" style="width: 100%;
            padding: 12px 20px;
            margin: 8px 0;
            color: #558b6e;
            border-color: #558b6e;
            font-weight: bold;
            box-sizing: border-box;
            border-radius: 10px;" value=""><br>
            <input type="submit" value="check" class="submit_button" style="font-weight: bold; color: #ffffff; background-color: #558b6e; border-radius: 5px; height: 2rem; width: 5rem; margin: 20px; border: none;">
            <h3 style="color: #558b6e; font-family: 'Roboto', sans-serif; font-weight: bold; text-align: center;">{}<h3>
        </form>
    </div>
</body>
</html>"#,
        list_of_wrong_words
    ))
}

/// # Tests passed: 3/3 🪩
#[cfg(test)]
mod test {
    use super::*;

    /// # Testing creation of dictionary
    #[test]
    fn test_dict() {
        let english_dict = create_dictionary_set("english".to_string());
        let german_dict = create_dictionary_set("german".to_string());

        assert!(english_dict.contains("a"));
        assert!(german_dict.contains("hallo"));
    }

    /// # Testing if input gets properly cleaned
    #[test]
    fn test_clean_input() {
        let input = "hello. my. cutie pie".to_string();
        let cleaned_input = clean_input_to_check(&input);
        assert_eq!(cleaned_input, "hello my cutie pie ".to_string());
    }

    /// # Testing if dictionary finds wrongly spelled words
    #[test]
    fn test_check_spelling() {
        let english_dict = create_dictionary_set("english".to_string());
        let german_dict = create_dictionary_set("german".to_string());

        assert_eq!(
            check_spelling_and_return_string_of_wrong_spelled_words(
                english_dict,
                &"hello woorld how are yoouuu".to_string(),
            ),
            "woorld,yoouuu,".to_string()
        );
    }
}
