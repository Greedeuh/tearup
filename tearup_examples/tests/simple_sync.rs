use tearup::{tearup_test, Context, SharedContext};

#[tearup_test(SimpleContextX)]
fn it_setup_a_fake_db(mut db: DbClient) {
    db.execute("some action with a side effect on DB");
    assert_eq!("some res", db.query("some query to assert the side effect"));
}

struct SimpleContextX {}

impl Context for SimpleContextX {
    fn setup(shared_context: &mut SharedContext) -> Self {
        let mut db_client = DbClient::new("random_db_name");

        db_client.create_db();

        shared_context.register(db_client.clone());

        Self {}
    }

    fn teardown(self, shared_context: &mut SharedContext) {
        shared_context.get::<DbClient>().unwrap().drop_db();
    }
}

#[derive(Clone)]
pub struct DbClient {
    #[allow(unused)]
    name: String,
}

impl DbClient {
    pub fn new(db_name: &str) -> Self {
        DbClient {
            name: db_name.to_string(),
        }
    }
    pub fn create_db(&mut self) {}
    pub fn drop_db(&mut self) {}
    pub fn execute(&mut self, _query: &str) {}
    pub fn query(&mut self, _query: &str) -> String {
        "some res".to_string()
    }
}
