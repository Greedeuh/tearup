use tearup::{tearup_test, FromContext, ReadyFn, WaitingContext};

#[tearup_test(SimpleContextX)]
fn it_setup_a_fake_db(mut db: DbClient) {
    db.execute("some action with a side effect on DB");
    assert_eq!("some res", db.query("some query to assert the side effect"));
}

struct SimpleContextX {
    db_client: DbClient,
}

impl WaitingContext for SimpleContextX {
    fn setup(ready: ReadyFn) -> Self {
        let mut db_client = DbClient {
            name: "random_db_name".to_string(),
        };

        db_client.create_db();

        ready();

        Self { db_client }
    }

    fn teardown(&mut self, ready: ReadyFn) {
        self.db_client.drop_db();
        ready();
    }
}

#[derive(Clone)]
pub struct DbClient {
    #[allow(unused)]
    name: String,
}

impl DbClient {
    pub fn create_db(&mut self) {}
    pub fn drop_db(&mut self) {}
    pub fn execute(&mut self, _query: &str) {}
    pub fn query(&mut self, _query: &str) -> String {
        "some res".to_string()
    }
}

impl FromContext<SimpleContextX> for DbClient {
    fn from_context(context: &SimpleContextX) -> Self {
        context.db_client.clone()
    }
}
