use axum::extract::Query;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::Router;
use easy_captcha::captcha::gif::GifCaptcha;
use easy_captcha::extension::axum_tower_sessions::{
    CaptchaAxumTowerSessionExt, CaptchaAxumTowerSessionStaticExt,
};
use easy_captcha::extension::CaptchaUtil;
use easy_captcha::NewCaptcha;
use std::collections::HashMap;
use time::Duration;
use tower_sessions::{Expiry, Session};

/// 接口：获取验证码
async fn get_captcha(session: Session) -> Result<Response, StatusCode> {
    let mut captcha: CaptchaUtil<GifCaptcha> = CaptchaUtil::with_size_and_len(127, 48, 4);
    match captcha.out(&session).await {
        Ok(response) => Ok(response),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

/// 接口：验证验证码
async fn verify_captcha(
    session: Session,
    Query(query): Query<HashMap<String, String>>,
) -> Response {
    // 从请求中获取验证码
    if let Some(code) = query.get("code") {
        // 调用CaptchaUtil的静态方法验证验证码是否正确
        if CaptchaUtil::ver(code, &session).await {
            CaptchaUtil::clear(&session).await; // 如果愿意的话，你可以从Session中清理掉验证码
            "Your code is valid, thank you.".into_response()
        } else {
            "Your code is not valid, I'm sorry.".into_response()
        }
    } else {
        "You didn't provide the code.".into_response()
    }
}

#[tokio::main]
async fn main() {
    // 初始化tower_sessions
    let session = tower_sessions::MemoryStore::default();
    let session_layer = tower_sessions::SessionManagerLayer::new(session)
        .with_secure(false)
        .with_expiry(Expiry::OnInactivity(Duration::seconds(20)));

    // 初始化路由
    let router = Router::new()
        .route("/captcha", get(get_captcha))
        .route("/verify", get(verify_captcha))
        .route_layer(session_layer);

    // 启动程序
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8010")
        .await
        .unwrap();
    axum::serve(listener, router).await.unwrap();
}
