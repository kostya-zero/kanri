use rstest::*;

use crate::autocomplete::{CompletionResult, suggest_completion};

#[fixture]
fn items() -> [&'static str; 3] {
    ["apple", "orange", "watermelon"]
}

#[rstest]
pub fn test_autocomplete_found_similar(items: [&str; 3]) {
    let result = suggest_completion("ap", &items);
    assert_eq!(
        result,
        CompletionResult::FoundSimilar(String::from("apple"))
    )
}

#[rstest]
pub fn test_autocomplete_found(items: [&str; 3]) {
    let result = suggest_completion("apple", &items);
    assert_eq!(result, CompletionResult::Found)
}

#[rstest]
pub fn test_autocomplete_nothing(items: [&str; 3]) {
    let result = suggest_completion("kanri", &items);
    assert_eq!(result, CompletionResult::Nothing)
}
