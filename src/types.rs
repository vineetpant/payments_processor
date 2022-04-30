use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub enum TransactionType {
    #[serde(rename = "deposit")]
    Deposit,
    #[serde(rename = "withdrawal")]
    Withdrawal,
    #[serde(rename = "dispute")]
    Dispute,
    #[serde(rename = "resolve")]
    Resolve,
    #[serde(rename = "chargeback")]
    Chargeback,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Transaction {
    pub r#type: TransactionType,
    pub client: u16,
    pub tx: u32,
    pub amount: Option<f32>,
}

#[derive(Debug, Clone)]
pub struct ClientAccount {
    pub client: u16,
    pub available: f32,
    pub held: f32,
    pub total: f32,
    pub locked: bool,
}
