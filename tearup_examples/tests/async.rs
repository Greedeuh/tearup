use async_trait::async_trait;
use tearup::{tearup_test, AsyncContext, FromAsyncContext, ReadyFn};

#[tearup_test(SimpleContext)]
async fn it_setup_a_fake_db(mut db: DbClient) {
    db.execute("some action with a side effect on DB").await;
    assert_eq!(
        "some res",
        db.query("some query to assert the side effect").await
    );
}

struct SimpleContext {
    db_client: DbClient,
}

#[async_trait]
impl<'a> AsyncContext<'a> for SimpleContext {
    async fn setup(ready: ReadyFn) -> Self {
        let mut db_client = DbClient {
            name: "random_db_name".to_string(),
        };

        db_client.create_db().await;

        ready();

        Self { db_client }
    }

    async fn teardown(&mut self) {
        self.db_client.drop_db().await;
    }
}

#[derive(Clone)]
pub struct DbClient {
    #[allow(unused)]
    name: String,
}

impl DbClient {
    pub async fn create_db(&mut self) {}
    pub async fn drop_db(&mut self) {}
    pub async fn execute(&mut self, _query: &str) {}
    pub async fn query(&mut self, _query: &str) -> String {
        "some res".to_string()
    }
}

#[async_trait]
impl FromAsyncContext<'_, SimpleContext> for DbClient {
    async fn from_setup(context: &SimpleContext) -> Self {
        context.db_client.clone()
    }
}
