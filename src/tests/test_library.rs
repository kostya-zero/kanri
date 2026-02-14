use crate::{
    library::{Library, LibraryError},
    tests::TestContext,
};
use std::{fs, path::PathBuf};

#[test]
fn test_library_new() {
    let context = TestContext::setup();
    let path = context.path().to_path_buf();

    let library = Library::new(&path, false).unwrap();
    assert!(library.is_empty());
    assert!(context.path().exists());
    assert!(matches!(
        Library::new(&PathBuf::from("/non/existent/path"), false),
        Err(LibraryError::InvalidPath)
    ));
}

#[test]
fn test_library_create_project() {
    let context = TestContext::setup();
    let path = context.path().to_path_buf();

    let mut library = Library::new(&path, false).unwrap();

    assert!(library.create("new_project").is_ok());
    assert!(context.path().join("new_project").exists());

    assert!(matches!(
        library.create("new_project"),
        Err(LibraryError::AlreadyExists)
    ));
}

#[test]
fn test_library_contains() {
    let context = TestContext::setup();
    let path = context.path().to_path_buf();

    fs::create_dir(context.path().join("test_project")).unwrap();

    let library = Library::new(&path, false).unwrap();
    assert!(library.contains("test_project"));
    assert!(!library.contains("non_existent_project"));
}

#[test]
fn test_library_get() {
    let context = TestContext::setup();
    let path = context.path().to_path_buf();

    fs::create_dir(context.path().join("test_project")).unwrap();

    let library = Library::new(&path, false).unwrap();
    assert!(library.get("test_project").is_some());
    assert!(library.get("non_existent_project").is_none());
}

#[test]
fn test_hidden_projects() {
    let context = TestContext::setup();
    let path = context.path().to_path_buf();

    fs::create_dir(context.path().join("visible_project")).unwrap();
    fs::create_dir(context.path().join(".hidden_project")).unwrap();

    let library = Library::new(&path, false).unwrap();
    assert!(library.contains("visible_project"));
    assert!(!library.contains(".hidden_project"));

    let library_with_hidden = Library::new(&path, true).unwrap();
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

#[test]
fn test_cleanup() {
    let temp_path;
    {
        let context = TestContext::setup();
        temp_path = context.path().to_path_buf();

        fs::write(context.path().join("test.txt"), "test").unwrap();

        assert!(temp_path.exists());
    }

    assert!(!temp_path.exists());
}
