use diesel::{prelude::*, sql_types::Bool, Connection, PgConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use tearup::{tearup, Context, SharedContext};
use uuid::Uuid;

// or #[tearup_test(SimpleContextX)]
#[tearup(SimpleContextX)]
fn it_setup_a_fake_db(mut db: DbClient) {
    db.execute("some action with a side effect on DB");
    assert_eq!("some res", db.query("some query to assert the side effect"));
}

struct SimpleContextX {}

impl Context for SimpleContextX {
    fn setup(shared_context: &mut SharedContext) -> Self {
        let db_name = dbg!(format!("go_{}", Uuid::new_v4().simple()));

        setup_db(&db_name);

        shared_context.register(DbClient(db_name.clone()));
        shared_context.register(DbName(db_name));

        Self {}
    }

    fn teardown(self, shared_context: &mut SharedContext) {
        let db_name = shared_context.get::<DbName>().unwrap();
        drop_db(&db_name.0);
    }
}

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

pub fn setup_db(db_name: &str) {
    let mut pg =
        PgConnection::establish("postgres://postgres:postgres@localhost:6543/postgres").unwrap();
    diesel::dsl::sql::<Bool>(&format!("CREATE DATABASE {};", db_name))
        .execute(&mut pg)
        .unwrap();

    let mut db = DbClient(db_name.to_owned()).pg_conn();

    db.run_pending_migrations(MIGRATIONS).unwrap();
}

pub fn drop_db(db_name: &str) {
    let mut pg =
        PgConnection::establish("postgres://postgres:postgres@localhost:6543/postgres").unwrap();
    diesel::dsl::sql::<Bool>(&format!(
        "SELECT pg_terminate_backend(pg_stat_activity.pid)
        FROM pg_stat_activity
        WHERE pg_stat_activity.datname = '{}';",
        db_name
    ))
    .execute(&mut pg)
    .unwrap();
    diesel::dsl::sql::<Bool>(&format!("DROP DATABASE {};", db_name))
        .execute(&mut pg)
        .unwrap();
}

#[derive(Clone)]
struct DbClient(String);
#[allow(dead_code)]
impl DbClient {
    pub fn pg_conn(&self) -> PgConnection {
        PgConnection::establish(&format!(
            "postgres://postgres:postgres@localhost:6543/{}",
            self.0
        ))
        .unwrap()
    }
    pub fn execute(&mut self, _query: &str) {}
    pub fn query(&mut self, _query: &str) -> String {
        "some res".to_string()
    }
}

#[derive(Clone)]
struct DbName(pub String);
