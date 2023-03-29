use async_trait::async_trait;
use tearup::{tearup_test, AnyMap, AsyncSimpleContext};

#[tearup_test(DbContext)]
async fn it_setup_a_fake_db(mut db: DbClient) {
    db.execute("some action with a side effect on DB").await;
    assert_eq!(
        "some res",
        db.query("some query to assert the side effect").await
    );
}

struct DbContext {
    db_client: DbClient,
}

#[async_trait]
impl<'a> AsyncSimpleContext<'a> for DbContext {
    async fn setup() -> Self {
        let mut db_client = DbClient::new("random_db_name");

        db_client.create_db().await;

        Self { db_client }
    }

    fn public_context(&mut self) -> AnyMap {
        let mut map = AnyMap::new();
        map.insert(self.db_client.clone());
        map
    }

    async fn teardown(mut self) {
        self.db_client.drop_db().await;
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
    pub async fn create_db(&mut self) {}
    pub async fn drop_db(&mut self) {}
    pub async fn execute(&mut self, _query: &str) {}
    pub async fn query(&mut self, _query: &str) -> String {
        "some res".to_string()
    }
}
