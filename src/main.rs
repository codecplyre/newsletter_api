use newletter_api::run;
#[tokio::main]
async fn main() -> std::io::Result<()> {
    run()?.await
}
