use super::*;
use std::sync::Arc;
use std::sync::Mutex;

pub trait PAcc {
    fn remove_money(acc: &Arc<Mutex<Self>>, amt: u32) -> Option<TransMoney>
    where
        Self: Sized;

    fn add_money(&mut self, amt: u32);
}
