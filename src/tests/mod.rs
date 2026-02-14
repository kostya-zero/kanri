mod test_autocomplete;
mod test_library;

use std::path::Path;
use tempfile::TempDir;

struct TestContext {
    temp_dir: TempDir,
}

impl TestContext {
    fn setup() -> Self {
        let temp_dir = tempfile::tempdir().unwrap();
        Self { temp_dir }
    }

    fn path(&self) -> &Path {
        self.temp_dir.path()
    }
}
