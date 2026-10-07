use crate::{
    library::{Library, LibraryError},
    tests::TestContext,
};
use rstest::*;
use std::{fs, path::PathBuf};

#[fixture]
fn context() -> TestContext {
    TestContext::setup()
}

#[rstest]
fn test_library_new(context: TestContext) {
    let library = Library::new(context.path(), false).unwrap();
    assert!(library.is_empty());
    assert!(context.path().exists());
    assert!(matches!(
        Library::new(PathBuf::from("/non/existent/path"), false),
        Err(LibraryError::InvalidPath)
    ));
}

#[rstest]
fn test_library_create_project(context: TestContext) {
    let mut library = Library::new(context.path(), false).unwrap();

    assert!(library.create("new_project").is_ok());
    assert!(context.path().join("new_project").exists());
    assert!(matches!(
        library.create("new_project"),
        Err(LibraryError::AlreadyExists)
    ));
}

#[rstest]
fn test_library_contains(context: TestContext) {
    fs::create_dir(context.path().join("test_project")).unwrap();

    let library = Library::new(context.path(), false).unwrap();
    assert!(library.contains("test_project"));
    assert!(!library.contains("non_existent_project"));
}

#[rstest]
fn test_library_get(context: TestContext) {
    fs::create_dir(context.path().join("test_project")).unwrap();

    let library = Library::new(context.path(), false).unwrap();
    assert!(library.get("test_project").is_some());
    assert!(library.get("non_existent_project").is_none());
}

#[rstest]
fn test_hidden_projects(context: TestContext) {
    fs::create_dir(context.path().join("visible_project")).unwrap();
    fs::create_dir(context.path().join(".hidden_project")).unwrap();

    let library = Library::new(context.path(), false).unwrap();
    assert!(library.contains("visible_project"));
    assert!(!library.contains(".hidden_project"));

    let library_with_hidden = Library::new(context.path(), true).unwrap();
    assert!(library_with_hidden.contains("visible_project"));
    assert!(library_with_hidden.contains(".hidden_project"));
}

#[test]
fn test_library_rename() {
    let context = TestContext::setup();
    let path = context.path().to_path_buf();

    let mut library = Library::new(&path, false).unwrap();
    library.create("test").unwrap();
    assert!(library.get("test").is_some());

    library.rename("test", "new_test").unwrap();
    assert!(library.get("new_test").is_some());
    assert!(library.get("test").is_none());
}

#[rstest]
fn test_context_cleanup() {
    let temp_path;
    {
        let context = TestContext::setup();
        temp_path = context.path().to_path_buf();
        fs::write(context.path().join("test.txt"), "test").unwrap();
        assert!(temp_path.exists());
    }

    assert!(!temp_path.exists());
}
