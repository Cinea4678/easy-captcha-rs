use axum::routing::get;
use axum::Router;
use easy_captcha::captcha::gif::GifCaptcha;
use easy_captcha::NewCaptcha;
use time::Duration;
use tower_sessions::Expiry;

#[tokio::main]
async fn main() {
    // 初始化tower_sessions
    let session = tower_sessions::MemoryStore::default();
    let session_layer = tower_sessions::SessionManagerLayer::new(session)
        .with_secure(false)
        .with_expiry(Expiry::OnInactivity(Duration::seconds(20)));

    // 初始化路由
    use easy_captcha::extension::axum_tower_sessions::{
        AxumTowerSessionCaptcha, CaptchaAxumTowerSessionExt,
    };
    let router = Router::new()
        .route("/captcha", get(AxumTowerSessionCaptcha::get_captcha))
        .route_layer(session_layer);

    // 启动程序
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8010")
        .await
        .unwrap();
    axum::serve(listener, router).await.unwrap();
}
