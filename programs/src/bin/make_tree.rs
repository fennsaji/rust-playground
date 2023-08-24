use std::collections::HashMap;
use serde_json::Result;
use serde_derive::{Deserialize, Serialize};

type TxHash = String;

type TransactionsMap = HashMap<String, Box<TransactionNode>>;
trait Key<T: PartialEq + Clone> {
    fn current_key(&self) -> T;

    fn previous_key(&self) -> Option<T>;
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Transaction {
    previous_tx_hash: Option<String>,
    amount: i32,
    from_did: String,
    to_did: String,
    issued_at: String,
    expiry_at: String,
    tx_hash: String,
    signed_tx_hash: String,
}

impl Key<TxHash> for Transaction {
    fn current_key(&self) -> TxHash {
        self.tx_hash.clone()
    }
    fn previous_key(&self) -> Option<TxHash> {
        self.previous_tx_hash.clone()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
struct TransactionNode {
    previous_tx_hash: Option<TxHash>,
    amount: i32,
    from_did: String,
    to_did: String,
    issued_at: String,
    expiry_at: String,
    tx_hash: TxHash,
    signed_tx_hash: String,
    children: Vec<TxHash>,
}

impl Key<TxHash> for TransactionNode {
    fn current_key(&self) -> TxHash {
        self.tx_hash.clone()
    }
    fn previous_key(&self) -> Option<TxHash> {
        self.previous_tx_hash.clone()
    }
}

// Try into from Transaction to TransactonNode
impl From<Transaction> for TransactionNode {
    fn from(transaction: Transaction) -> Self {
        TransactionNode {
            previous_tx_hash: transaction.previous_tx_hash,
            amount: transaction.amount,
            from_did: transaction.from_did,
            to_did: transaction.to_did,
            issued_at: transaction.issued_at,
            expiry_at: transaction.expiry_at,
            tx_hash: transaction.tx_hash,
            signed_tx_hash: transaction.signed_tx_hash,
            children: Vec::new(),
        }
    }
}

fn build_transaction_tree(transactions: Vec<Transaction>) -> (TxHash, TransactionsMap) {
    let mut transaction_map: TransactionsMap = HashMap::new();
    let mut root_node: TxHash = Default::default();

    // Convert transaction to hashmap
    for transaction in transactions.clone() {
        let tx_hash = transaction.current_key();
        transaction_map.insert(tx_hash.clone(), Box::new(transaction.into()));
    }

    // Adding 2 way link
    for (tx_hash, transaction) in transaction_map.clone() {
        let previous_tx_hash = transaction.previous_key();
        if previous_tx_hash.is_none() {
            // Assuming there is only one tx with previous has as none which will be root node
            root_node = tx_hash;
        } else if let Some(parent_node) = transaction_map.get_mut(&previous_tx_hash.unwrap()) {
            parent_node.children.push(transaction.current_key());
        }
    }
    (root_node, transaction_map)
}

fn iterate_transactions_tree(root_node: TxHash, transaction_tree: TransactionsMap) {
    let next_node_hash = root_node;
    let transaction = transaction_tree.get(&next_node_hash);
    if let Some(transaction) = transaction {
        // Validate Data
        println!("Validating Data: {}", transaction.current_key());
        println!("{}", transaction.amount);
        for tx in transaction.children.clone().into_iter() {
            iterate_transactions_tree(tx , transaction_tree.clone());
        }
    }
}


fn main() -> Result<()> {
    let json_data = r#"
    [
        {
          "previous_tx_hash": null,
          "amount": 10000,
          "from_did": "0x6469643a737369643a636861726c696500000000000000000000000000000000",
          "to_did": "0x6469643a737369643a616c696365000000000000000000000000000000000000",
          "issued_at": "2023-08-18T08:23:25.135715279Z",
          "expiry_at": "2024-08-17T08:23:25.135717653Z",
          "tx_hash": "0xf697f777b5fe9ac8b405c683f46b0ba650ee63b63a4f2913aaf973dea77dc6a8ac1ce2bad13b836b151f949f83d3e211c3f4a6fb2b95f267e27578eb12bc6587",
          "signed_tx_hash": "0x2056f624118345afdbdaacd63cd82f51d4086d6493339d9626b2f03baf2e5027848c11a9107810ea25c642804fc5eed00f3bb11ba12f3187e785ed8d22cc1f84"
        },
        {
          "previous_tx_hash": "0xf697f777b5fe9ac8b405c683f46b0ba650ee63b63a4f2913aaf973dea77dc6a8ac1ce2bad13b836b151f949f83d3e211c3f4a6fb2b95f267e27578eb12bc6587",
          "amount": 5000,
          "from_did": "0x6469643a737369643a616c696365000000000000000000000000000000000000",
          "to_did": "0x6469643a737369643a626f620000000000000000000000000000000000000000",
          "issued_at": "2023-08-18T08:24:18.143468881Z",
          "expiry_at": "2023-08-15T08:24:18.143469914Z",
          "tx_hash": "0x3691d3e458c48b41b25e929917491885c1455dd762125a1fead0104a042fd857fb86254057778dadc48d50e61aecfeff4b1d9dd2d6e8b63883cc2680b17ec2ab",
          "signed_tx_hash": "0xd0eecd71aff0f28e68159f3ce18134c63c320a519642e9938612fcf7c6a0ce1d8bbda0aca0111ef29d3c1403186f7619534e529e8c5f7d86a52f5a15511e5a8c"
        },
        {
          "previous_tx_hash": "0xf697f777b5fe9ac8b405c683f46b0ba650ee63b63a4f2913aaf973dea77dc6a8ac1ce2bad13b836b151f949f83d3e211c3f4a6fb2b95f267e27578eb12bc6587",
          "amount": 2000,
          "from_did": "0x6469643a737369643a616c696365000000000000000000000000000000000000",
          "to_did": "0x6469643a737369643a626f620000000000000000000000000000000000000000",
          "issued_at": "2023-08-18T08:24:18.143468881Z",
          "expiry_at": "2023-08-15T08:24:18.143469914Z",
          "tx_hash": "0x3691d3e458c48b41b25e929917491885c1455dd762125a1fead0104a042fd857fb86254057778dadc48d50e61aecfeff4b1d9dd2d6e8b63883cc2680000",
          "signed_tx_hash": "0xd0eecd71aff0f28e68159f3ce18134c63c320a519642e9938612fcf7c6a0ce1d8bbda0aca0111ef29d3c1403186f7619534e529e8c5f7d86a52f5a15511e5a8c"
        },
        {
          "previous_tx_hash": "0x3691d3e458c48b41b25e929917491885c1455dd762125a1fead0104a042fd857fb86254057778dadc48d50e61aecfeff4b1d9dd2d6e8b63883cc2680b17ec2ab",
          "amount": 3000,
          "from_did": "0x6469643a737369643a626f620000000000000000000000000000000000000000",
          "to_did": "0x6469643a737369643a6461766500000000000000000000000000000000000000",
          "issued_at": "2023-08-18T08:24:31.057493587Z",
          "expiry_at": "2024-08-17T08:24:31.057494651Z",
          "tx_hash": "0xee37f66c7b07ba9a367331bb3b6dec9cbe336cde279015c571936eb9e26dbe5c04b3f7b2cbcd961611b6aa6d70d4601517468af68ca75852b0e78c5e80c2ec01",
          "signed_tx_hash": "0xec24d14de0041e2c5c2a152624dd7c9aa86910728bddcbe381ac7a0f3d87e306b3a692b8963da9e1b37d66cfb0dacd248d0f719620caa01451245f4cc794888b"
        },
        {
          "previous_tx_hash": "0xee37f66c7b07ba9a367331bb3b6dec9cbe336cde279015c571936eb9e26dbe5c04b3f7b2cbcd961611b6aa6d70d4601517468af68ca75852b0e78c5e80c2ec01",
          "amount": 3000,
          "from_did": "0x6469643a737369643a6461766500000000000000000000000000000000000000",
          "to_did": "0x6469643a737369643a636861726c696500000000000000000000000000000000",
          "issued_at": "2023-08-18T08:25:28.546290228Z",
          "expiry_at": "2024-08-17T08:25:28.546364091Z",
          "tx_hash": "0x28db989c0cf802a6c96f852216f0abc18f57724b3ecef26c4ce9728341770acb42666b8bca0497f7674a3f180abe17a71604552307e8d80f355d10198b9f47d7",
          "signed_tx_hash": "0xd821173157b36c5fb105aa7557b91d73608f763617ae41ac5eafc9c30636700174522a423c4bc2f4ba4924db5f204a0826b210a0af5ffc4311a6ecdd54c46182"
        }
      ]
    "#;

    let transactions: Vec<Transaction> = serde_json::from_str(json_data)?;
    let (root_node, transaction_tree) = build_transaction_tree(transactions.clone());
    println!("{:#?}", transaction_tree);

    iterate_transactions_tree(root_node, transaction_tree);
    Ok(())
}