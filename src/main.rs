mod check_balance;

#[tokio::main]
async fn main() {
    check_balance::check_balance().await;
}
