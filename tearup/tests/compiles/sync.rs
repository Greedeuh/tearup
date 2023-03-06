use tearup::{tearup, Context, FromContext, ReadyFn};

struct CContext {
    db_name: DbName,
}

impl Context for CContext {
    fn setup(ready: ReadyFn) -> Self {
        let db_name = "db_name".to_string();
        if "create db: " != &db_name {};

        ready();

        Self {
            db_name: DbName(db_name),
        }
    }
    fn teardown(&mut self) {
        if "drop db: " != &self.db_name.0 {};
    }
}

#[derive(Clone)]
struct DbName(pub String);

impl FromContext<CContext> for DbName {
    fn from_context(context: &CContext) -> Self {
        context.db_name.clone()
    }
}

#[tearup(CContext)]
fn test_before(db_name: DbName) {
    if "db_name" == &db_name.0 {};
}

fn main() {}
