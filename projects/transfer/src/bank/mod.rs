use std::sync::Arc;
use std::sync::Mutex;

mod private;

use private::*;

pub trait Acc: private::PAcc {}

pub fn transfer<A: Acc, B: 'static + Acc>(
    from: &Arc<Mutex<A>>,
    to: &Arc<Mutex<B>>,
    amt: u32,
) -> Option<()> {
    
    let mut money = PAcc::remove_money(from, amt)?;
    money.change_owner(to.clone());
    Some(())
}

#[derive(Debug)]
pub struct Account {
    money: i64,
}

impl Account {
    pub fn new(amt: i64) -> Self {
        Account { money: amt }
    }
}

impl PAcc for Account {
    fn remove_money(acc: &Arc<Mutex<Account>>, amt: u32) -> Option<TransMoney> {
        let acc_clone = acc.clone();
        let mut acc_locked = acc.lock().ok()?;
        if acc_locked.money >= amt as i64 {
            acc_locked.money -= amt as i64;
            return Some(TransMoney {
                amt: amt,
                owner: acc_clone,
            });
        }
        None
    }
    fn add_money(&mut self, amt: u32) {
        self.money += amt as i64;
    }
}

impl Acc for Account {}

pub struct Cash;

impl Cash {
    pub fn new() -> Self {
        Cash
    }
}

impl PAcc for Cash {
    fn remove_money(acc: &Arc<Mutex<Cash>>, amt: u32) -> Option<TransMoney> {
        Some(TransMoney {
            amt: amt,
            owner: acc.clone(),
        })
    }
    fn add_money(&mut self, _amt: u32) {}
}
impl Acc for Cash {}

pub struct TransMoney {
    amt: u32,
    owner: Arc<Mutex<dyn Acc>>,
}

impl TransMoney {
    fn change_owner(&mut self, new_owner: Arc<Mutex<dyn Acc>>) {
        self.owner = new_owner;
    }
}

impl Drop for TransMoney {
    fn drop(&mut self) {
        let mut acc = self.owner.lock().ok().unwrap();
        acc.add_money(self.amt);
    }
}
