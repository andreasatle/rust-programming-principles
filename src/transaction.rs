use serde_derive::*;

/// Use our own Error type.
#[derive(Debug)]
pub enum TransactionError {
    LoadError(std::io::Error),
    ParseError(serde_json::Error),
}

/// Make sure that from is implemented from each error that can occur.
impl From<std::io::Error> for TransactionError {
    fn from(e: std::io::Error) -> Self {
        TransactionError::LoadError(e)
    }
}

/// Make sure that from is implemented from each error that can occur.
impl From<serde_json::Error> for TransactionError {
    fn from(e: serde_json::Error) -> Self {
        TransactionError::ParseError(e)
    }
}

/// A Transaction with from, to and amount.
#[derive(Serialize, Deserialize, Debug)]
pub struct Transaction {
    from: String,
    to: String,
    amount: u64,
}

impl Transaction {
    /// The most straightforward but wordy way of reading
    /// transactions from a JSON-file.
    pub fn get_with_match(fname: &str) -> Result<Vec<Transaction>, String> {
        let s = match std::fs::read_to_string(&fname) {
            Ok(v) => v,
            Err(e) => return Err(e.to_string()),
        };
        let t: Vec<Transaction> = match serde_json::from_str(&s) {
            Ok(v) => v,
            Err(e) => return Err(e.to_string()),
        };
        Ok(t)
    }

    // Read transactions from JSON file without using match.
    // Use methods on Result:
    // map_err(|e|f): Result<T, E> -> Result<T, F>
    // and_then(|t|g(t)): Ok(T) -> g(T), else return Err(E)
    pub fn get_with_and_then(fname: &str) -> Result<Vec<Transaction>, String> {
        std::fs::read_to_string(fname)
            .map_err(|e| e.to_string())
            .and_then(|ld| serde_json::from_str(&ld).map_err(|e| e.to_string()))
    }

    // If From trait implemented, then we can use into here.
    pub fn get_with_own_error(fname: &str) -> Result<Vec<Transaction>, TransactionError> {
        std::fs::read_to_string(fname)
            .map_err(|e| e.into())
            .and_then(|ld| serde_json::from_str(&ld).map_err(|e| e.into()))
    }

    // With From trait, we can shorten even further with question marks.
    pub fn get_with_question_mark(fname: &str) -> Result<Vec<Transaction>, TransactionError> {
        Ok(serde_json::from_str(&std::fs::read_to_string(fname)?)?)
    }
}