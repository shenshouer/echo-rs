use axum::{
    body::Body,
    extract::ConnectInfo,
    http::{header::HeaderValue, HeaderMap, Request, StatusCode},
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use serde_json::{self, Value};
use std::{collections::HashMap, net::SocketAddr};
use tokio::signal;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new().route("/", get(root));

    let addr = "0.0.0.0:3000";
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn root(ConnectInfo(addr): ConnectInfo<SocketAddr>, req: Request<Body>) -> impl IntoResponse {
    let mut value = convert(req.headers());
    merge_kv(&mut value, "remote_ip", &addr.to_string());
    (StatusCode::OK, Json(value))
}

fn convert(headers: &HeaderMap<HeaderValue>) -> serde_json::Value {
    serde_json::from_str(&format!("{:?}", headers)).expect("不能将http headers 转换成json")
}

pub fn merge_kv(v: &mut Value, key: &str, value: &str) {
    match v {
        Value::Object(m) => {
            m.insert(key.to_string(), Value::String(value.to_string()));
        }
        _ => {}
    }
}

pub fn merge(v: &Value, fields: &HashMap<String, String>) -> Value {
    match v {
        Value::Object(m) => {
            let mut m = m.clone();
            for (k, v) in fields {
                m.insert(k.clone(), Value::String(v.clone()));
            }
            Value::Object(m)
        }
        v => v.clone(),
    }
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    tracing::debug!("接收到停止信号，开始优雅关闭");
}
