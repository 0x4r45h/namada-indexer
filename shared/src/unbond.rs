use crate::{balance::Amount, block::Epoch, id::Id};

//TODO: maybe reuse bond with Option<Amount> instead of Amount
#[derive(Debug, Clone)]
pub struct UnbondAddresses {
    pub source: Id,
    pub validator: Id,
}

#[derive(Debug, Clone)]
pub struct Unbond {
    pub source: Id,
    pub target: Id,
    pub amount: Amount,
    pub withdraw_at: Epoch,
}

#[derive(Debug, Clone)]
pub struct Unbonds {
    pub epoch: Epoch,
    pub values: Vec<Unbond>,
}