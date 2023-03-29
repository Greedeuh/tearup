use tearup::{tearup_test, AnyMap, SimpleContext};

#[tearup_test(SimpleContextX)]
fn it_setup_a_fake_db(mut db: DbClient) {
    db.execute("some action with a side effect on DB");
    assert_eq!("some res", db.query("some query to assert the side effect"));
}

struct SimpleContextX {
    db_client: DbClient,
}

impl SimpleContext for SimpleContextX {
    fn setup() -> Self {
        let mut db_client = DbClient::new("random_db_name");

        db_client.create_db();

        Self { db_client }
    }

    fn public_context(&mut self) -> AnyMap {
        let mut map = AnyMap::new();
        map.insert(self.db_client.clone());
        map
    }

    fn teardown(mut self) {
        self.db_client.drop_db();
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
