use std::net::TcpListener;

use newletter_api::run;
#[tokio::main]
async fn main() -> std::io::Result<()> {
    let server = TcpListener::bind("127.0.0.1:8000")?;
    run(server)?.await
}
