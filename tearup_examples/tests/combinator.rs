use tearup::{tearup_test, ContextCombinator, SharedContext, SimpleContext};

type CombinationContext = ContextCombinator<DbContext, ServerContext>;

#[tearup_test(CombinationContext)]
fn it_setup_a_fake_db_and_a_server(mut db: DbClient, url: Url) {
    db.execute("some action with a side effect on DB");
    assert_eq!("some res", db.query("some query to assert the side effect"));

    http_post(url);
}

struct DbContext {}

impl SimpleContext for DbContext {
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

struct ServerContext {}

impl SimpleContext for ServerContext {
    fn setup(shared_context: &mut SharedContext) -> Self {
        // use db provided by DbContext
        let db = shared_context.get::<DbClient>().unwrap();

        let url = launch_server(&db);

        shared_context.register(url);

        Self {}
    }

    fn teardown(self, _shared_context: &mut SharedContext) {}
}

fn launch_server(_db: &DbClient) -> Url {
    Url("localhost:randomPort/".to_owned())
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

#[derive(Clone)]
struct Url(String);

fn http_post(_url: Url) {}
