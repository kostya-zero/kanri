use crate::{config::Config, terminal::ask_dialog};

#[derive(Debug, Eq, PartialEq)]
pub enum CompletionResult {
    Found,
    FoundSimilar(String),
    Nothing,
}

pub fn autocomplete(word: &str, words_list: Vec<&str>, config: &Config) -> Option<String> {
    let suggested = suggest_completion(word, words_list.clone());

    match suggested {
        CompletionResult::Found => Some(word.to_string()),
        CompletionResult::FoundSimilar(name) => {
            if config.autocomplete.always_accept {
                return Some(name);
            }
            let answer = ask_dialog(&format!("Did you mean '{name}'?"), true, false);
            if answer { Some(name) } else { None }
        }
        CompletionResult::Nothing => None,
    }
}

pub fn suggest_completion(word: &str, words_list: Vec<&str>) -> CompletionResult {
    if words_list.contains(&word) {
        return CompletionResult::Found;
    }

    let word_lowercase = word.to_lowercase();
    if let Some(similar) = words_list
        .iter()
        .find(|entry| entry.to_lowercase().starts_with(&word_lowercase))
    {
        CompletionResult::FoundSimilar(similar.to_string())
    } else {
        CompletionResult::Nothing
    }
}
