use rmcp::transport::sse_server::{SseServer, SseServerConfig};

/*--------------------------------------------------------------------------------------
MCP Tools
--------------------------------------------------------------------------------------*/

mod tools;

/*--------------------------------------------------------------------------------------
  MCP Server
--------------------------------------------------------------------------------------*/

const BIND_ADDRESS: &str = "127.0.0.1:8123";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = SseServerConfig {
        bind: BIND_ADDRESS.parse()?,
        sse_path: "/sse".to_string(),
        post_path: "/message".to_string(),
        ct: tokio_util::sync::CancellationToken::new(),
        sse_keep_alive: None,
    };

    let (sse_server, router) = SseServer::new(config);

    // Do something with the router, e.g., add routes or middleware

    let listener = tokio::net::TcpListener::bind(sse_server.config.bind).await?;

    let ct = sse_server.config.ct.child_token();

    let server = axum::serve(listener, router).with_graceful_shutdown(async move {
        ct.cancelled().await;
    });

    tokio::spawn(async move {
        let _ = server.await;
    });

    let ct = sse_server.with_service(tools::Solver::new);

    tokio::signal::ctrl_c().await?;
    ct.cancel();
    Ok(())
}
