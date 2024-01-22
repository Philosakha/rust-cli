mod check_balance;
mod check_mina;

#[tokio::main]
async fn main() {
    // check_balance::check_balance().await;

    check_mina::check_mina().await;
}
