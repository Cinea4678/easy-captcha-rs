//! Axum & Tower_sessions 组合
//!
//! - Axum: [axum](https://docs.rs/axum)
//! - Tower Sessions: [axum](https://docs.rs/tower-sessions)

use crate::captcha::spec::SpecCaptcha;
use crate::extension::CaptchaUtil;
use crate::{AbstractCaptcha, NewCaptcha};
use async_trait::async_trait;
use axum::response::Response;
use tower_sessions::Session;

/// Axum & Tower_Sessions
#[async_trait]
pub trait CaptchaAxumTowerSessionExt {
    async fn get_captcha(session: Session) -> Response;
}

pub struct AxumTowerSessionCaptcha;

#[async_trait]
impl CaptchaAxumTowerSessionExt for AxumTowerSessionCaptcha {
    async fn get_captcha(session: Session) -> Response {
        let ans;
        let ct;
        let mut data = vec![];
        {
            let mut captcha = SpecCaptcha::new();
            captcha.out(&mut data).unwrap();
            ans = captcha.captcha.chars.clone().unwrap();
            ct = captcha.get_content_type();
        }

        session.insert("Captcha ans", ans).await.unwrap();
        Response::builder()
            .header("Content-Type", ct)
            .body(data.into())
            .unwrap()
    }
}
