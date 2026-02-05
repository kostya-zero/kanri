use library::Project;

use super::*;

#[test]
fn test_project_new() {
    let context = TestContext::setup();
    let path = context.path().to_path_buf();
    let project = Project {
        name: "test_project".to_string(),
        path: path.clone(),
    };

    assert_eq!(project.name.as_str(), "test_project");
    assert_eq!(project.path.to_str().unwrap(), path.to_str().unwrap());
}

#[test]
fn test_project_get_name() {
    let context = TestContext::setup();
    let project = Project {
        name: "test_project".to_string(),
        path: context.path().to_path_buf(),
    };
    assert_eq!(project.name.as_str(), "test_project");
}

#[test]
fn test_project_get_path_str() {
    let context = TestContext::setup();
    let path = context.path().to_path_buf();
    let path_str = path.to_str().unwrap();
    let project = Project {
        name: "test_project".to_string(),
        path: context.path().to_path_buf(),
    };
    assert_eq!(project.path.to_str().unwrap(), path_str);
}
