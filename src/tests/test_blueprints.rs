use rstest::*;
use std::path::PathBuf;

use crate::blueprints::engine::BlueprintEngine;

#[fixture]
fn engine() -> BlueprintEngine {
    BlueprintEngine::init(PathBuf::new(), "test.lua", "test", false)
        .expect("engine should be initialized")
}

#[rstest]
fn test_engine_math(engine: BlueprintEngine) {
    let code = r#"
            assert(type(math.floor) == "function")
            assert(math.floor(1.9) == 1)
        "#;
    assert!(engine.run(code).is_ok())
}

#[rstest]
fn test_engine_string(engine: BlueprintEngine) {
    let code = r#"
            assert(type(string.upper) == "function")
            assert(string.upper("kanri") == "KANRI")
        "#;
    assert!(engine.run(code).is_ok())
}

#[rstest]
fn test_engine_utf8(engine: BlueprintEngine) {
    let code = r#"
            assert(type(utf8.len) == "function")
            assert(utf8.len("kanri") == 5)
        "#;
    assert!(engine.run(code).is_ok())
}

#[rstest]
fn test_engine_table(engine: BlueprintEngine) {
    let code = r#"
            assert(type(table.insert) == "function")
            local t = {}
            table.insert(t, "ok")
            assert(t[1] == "ok")
        "#;
    assert!(engine.run(code).is_ok())
}

#[rstest]
fn test_engine_system(engine: BlueprintEngine) {
    let code = format!(
        r#"
            assert(type(system.system) == "function")
            assert(system.system() == "{}")
            assert(type(os.getenv) == "function")
        "#,
        std::env::consts::OS
    );
    assert!(engine.run(&code).is_ok())
}

#[rstest]
fn test_engine_project(engine: BlueprintEngine) {
    let code = r#"
            assert(type(project.name) == "string")
            assert(project.name == "test")
        "#;
    assert!(engine.run(code).is_ok())
}

#[rstest]
fn test_engine_fs(engine: BlueprintEngine) {
    let code = r#"
            assert(type(fs.is_file) == "function")
            assert(fs.is_file("Cargo.toml") == true)
        "#;
    assert!(engine.run(code).is_ok())
}

#[rstest]
fn test_engine_path(engine: BlueprintEngine) {
    let code = r#"
        local separator = package.config:sub(1, 1)
        local source = path.join("src", "lib", "main.rs")
        assert(source == "src" .. separator .. "lib" .. separator .. "main.rs")
        assert(path.join() == ".")
        assert(path.parent(source) == path.join("src", "lib"))
        assert(path.parent("README.md") == ".")
        assert(path.basename(source) == "main.rs")
        assert(path.extension(source) == "rs")
        assert(path.extension("README") == nil)
        assert(path.stem(source) == "main")
        assert(path.is_absolute(system.temp_dir()))
        assert(not path.is_absolute(source))
    "#;

    engine.run(code).expect("path functions should work in Lua");
}
