use crate::balance::Amount;
use crate::id::Id;

#[derive(Debug, Clone)]
pub struct BondAddresses {
    pub source: Id,
    pub target: Id,
}

#[derive(Debug, Clone)]
pub struct Bond {
    pub source: Id,
    pub target: Id,
    pub amount: Amount,
}

impl Bond {
    pub fn fake(validator_address: Id) -> Self {
        let source_address =
            namada_core::address::gen_established_address("namada-indexer");

        Self {
            source: Id::Account(source_address.to_string()),
            target: validator_address,
            amount: Amount::fake(),
        }
    }
}

pub type Bonds = Vec<Bond>;
