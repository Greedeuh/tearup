use std::sync::Mutex;

use async_trait::async_trait;
use lazy_static::lazy_static;
use reqwest::StatusCode;
use rocket::fairing::AdHoc;
use tearup::{tearup_test, AsyncReadyFn, AsyncSharedContext, AsyncSimpleContext, AsyncTimeGate};
use tearup_examples::rocket;

#[tearup_test(RocketContext)]
async fn it_launches_the_server(url: BaseUrl) {
    let response = reqwest::get(&url.0).await.unwrap();

    assert_eq!(StatusCode::OK, response.status())
}

struct RocketContext {
    _srv_life: ServerLife,
    port: u16,
}

pub type ServerLife = rocket::tokio::task::JoinHandle<rocket::Rocket<rocket::Ignite>>;

lazy_static! {
    static ref AVAILABLE_PORTS: Mutex<Vec<u16>> = Mutex::new((8001..9000).collect());
}

#[async_trait]
impl<'a> AsyncSimpleContext<'a> for RocketContext {
    async fn setup(shared_context: AsyncSharedContext) -> Self {
        let port = choose_port().await;

        let gate = AsyncTimeGate::new();

        let _srv_life = launch_server_then_notif_ready(port, gate.notifier()).await;

        gate.wait_signal().await;

        shared_context
            .register(BaseUrl(format!("http://localhost:{}/", port)))
            .await;

        Self { _srv_life, port }
    }

    async fn teardown(mut self, _shared_context: AsyncSharedContext) {
        free_port(self.port).await;
    }
}

async fn choose_port() -> u16 {
    AVAILABLE_PORTS.lock().unwrap().remove(0)
}

async fn free_port(port: u16) {
    AVAILABLE_PORTS.lock().unwrap().push(port);
}

async fn launch_server_then_notif_ready<'a>(port: u16, ready: AsyncReadyFn<'static>) -> ServerLife {
    tokio::task::spawn(async move {
        rocket(port)
            .attach(AdHoc::on_liftoff("Liftoff notifier", |_| {
                Box::pin(async move { ready().await })
            }))
            .launch()
            .await
            .unwrap()
    })
}

#[derive(Clone)]
struct BaseUrl(pub String);
