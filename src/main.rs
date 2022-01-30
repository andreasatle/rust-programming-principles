#[derive(Debug)]
pub struct Transaction {
    from: String,
    to: String,
    amount: u64,
}

fn main() {
    println!("Hello, world!");
    let trans = get_transaction("test_data/transactions.json");
    println!("{:?}", trans);
}

fn get_transaction(fname: &str) -> Result<Vec<Transaction>, String> {
    Ok(Vec::new())
}
