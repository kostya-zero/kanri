use std::path::PathBuf;

use crate::blueprints::engine::BlueprintEngine;

#[test]
fn test_engine_initialization() {
    let engine = BlueprintEngine::init(PathBuf::new(), "test.lua", "test", false);
    assert!(engine.is_ok())
}

#[test]
fn test_engine_math() {
    let engine = BlueprintEngine::init(PathBuf::new(), "test.lua", "test", false);
    assert!(engine.is_ok());

    let code = r#"
            assert(type(math.floor) == "function")
            assert(math.floor(1.9) == 1)
        "#;
    assert!(engine.unwrap().run(code).is_ok())
}

#[test]
fn test_engine_string() {
    let engine = BlueprintEngine::init(PathBuf::new(), "test.lua", "test", false);
    assert!(engine.is_ok());

    let code = r#"
            assert(type(string.upper) == "function")
            assert(string.upper("kanri") == "KANRI")
        "#;
    assert!(engine.unwrap().run(code).is_ok())
}

#[test]
fn test_engine_utf8() {
    let engine = BlueprintEngine::init(PathBuf::new(), "test.lua", "test", false);
    assert!(engine.is_ok());

    let code = r#"
            assert(type(utf8.len) == "function")
            assert(utf8.len("kanri") == 5)
        "#;
    assert!(engine.unwrap().run(code).is_ok())
}

#[test]
fn test_engine_table() {
    let engine = BlueprintEngine::init(PathBuf::new(), "test.lua", "test", false);
    assert!(engine.is_ok());

    let code = r#"
            assert(type(table.insert) == "function")
            local t = {}
            table.insert(t, "ok")
            assert(t[1] == "ok")
        "#;
    assert!(engine.unwrap().run(code).is_ok())
}

#[test]
fn test_engine_os() {
    let engine = BlueprintEngine::init(PathBuf::new(), "test.lua", "test", false);
    assert!(engine.is_ok());

    let code: &str;

    #[cfg(windows)]
    {
        code = r#"
            assert(type(os.family) == "function")
            assert(os.family() == "windows")
            "#;
    }

    #[cfg(not(windows))]
    {
        code = r#"
            assert(type(os.family) == "function")
            assert(os.family() == "unix")
            "#;
    }

    assert!(engine.unwrap().run(code).is_ok())
}

#[test]
fn test_engine_project() {
    let engine = BlueprintEngine::init(PathBuf::new(), "test.lua", "test", false);
    assert!(engine.is_ok());

    let code = r#"
            assert(type(project.name) == "function")
            assert(project.name() == "test")
        "#;
    assert!(engine.unwrap().run(code).is_ok())
}

#[test]
fn test_engine_fs() {
    let engine = BlueprintEngine::init(PathBuf::new(), "test.lua", "test", false);
    assert!(engine.is_ok());

    let code = r#"
            assert(type(fs.is_file) == "function")
            assert(fs.is_file("Cargo.toml") == true)
        "#;
    assert!(engine.unwrap().run(code).is_ok())
}
