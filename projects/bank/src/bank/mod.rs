pub enum Login {
    Login,
}

pub enum Summary {
    Summary(Login),
    Abort(Box<Transaction>),
    Back(Box<Done>),
}

pub enum Auth {
    Auth(Summary),
}

pub enum Transaction {
    Transaction(Auth),
    TransactionFromSummary(Box<SummaryA>),
}

pub enum SummaryA {
    SummaryAFromTransaction(Transaction),
    SummaryAFromAuth(Auth),
}

pub enum Done {
    Done(Transaction),
}

pub enum Logout {
    Logout(Done),
}
