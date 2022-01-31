mod transaction;
use crate::transaction::Transaction;

fn transactions() {

    let file_ok = "data/transactions/transactions_ok.json";
    let file_with_error = "data/transactions/transactions_with_error.json";
    let file_not_exist = "data/transactions/file_not_exist.json";

    println!("{:?}", Transaction::get_with_match(file_ok));
    println!("{:?}", Transaction::get_with_and_then(file_ok));
    println!("{:?}", Transaction::get_with_own_error(file_ok));
    println!("{:?}", Transaction::get_with_question_mark(file_ok));

    println!("{:?}", Transaction::get_with_match(file_with_error));
    println!("{:?}", Transaction::get_with_and_then(file_with_error));
    println!("{:?}", Transaction::get_with_own_error(file_with_error));
    println!("{:?}", Transaction::get_with_question_mark(file_with_error));

    println!("{:?}", Transaction::get_with_match(file_not_exist));
    println!("{:?}", Transaction::get_with_and_then(file_not_exist));
    println!("{:?}", Transaction::get_with_own_error(file_not_exist));
    println!("{:?}", Transaction::get_with_question_mark(file_not_exist));
}

fn main() {

    transactions()

}

