mod bank;

use bank::*;
use std::io::{self};

fn read_line() -> io::Result<String> {
    let mut buffer = String::new();
    let stdin = io::stdin();

    match stdin.read_line(&mut buffer) {
	Ok(_) => Ok(buffer.trim().to_string()),
	Err(e) => Err(e),
    }
}

    

fn login(login : Login) {
    println!("You're in Login.");
    loop {
	match read_line() {
	    Ok(buffer) => {
		if buffer == "login" {
		    let sum = Summary::Summary(login);
		    summary(sum);
		    return;
		} else {
		    println!("Unkown Command");
		}
	    },
	    Err(_) => {
		continue;
	    },
	}
    }


}

fn summary(sum : Summary) {
    println!("You're in Summary.");
    loop {
	match read_line() {
	    Ok(buffer) => {
		if buffer == "auth" {
		    let a = Auth::Auth(sum);
		    auth(a);
		    return;
		} else {
		    println!("Unkown Command");
		}
	    },
	    Err(_) => {
		continue;
	    },
	}
    }
}

fn auth(a : Auth) {
    println!("You're in Authentication.");
    loop {
	match read_line() {
	    Ok(buffer) => {
		if buffer == "trans" {
		    let trans = Transaction::Transaction(a);
		    transaction(trans);
		    return;
		} if buffer == "summary" {
		    let sum = SummaryA::SummaryAFromAuth(a);
		    summary_a(sum);
		    return;
		} else {
		    println!("Unkown Command");
		}
	    },
	    Err(_) => {
		continue;
	    },
	}
    }
}

fn transaction(trans : Transaction) {
    println!("You're in Transaction.");
    loop {
	match read_line() {
	    Ok(buffer) => {
		if buffer == "abort" {
		    let sum = Summary::Abort(Box::new(trans));
		    summary(sum);
		    return;
		} if buffer == "summary" {
		    let sum = SummaryA::SummaryAFromTransaction(trans);
		    summary_a(sum);
		    return;
		} if buffer == "complete" {
		    let d = Done::Done(trans);
		    done(d);
		    return;
		} else {
		    println!("Unkown Command");
		}		   
	    },
	    Err(_) => {
		continue;
	    },
	}
    }
}

fn summary_a(sum : SummaryA) {
    println!("You're in Summary A.");
    loop {
	match read_line() {
	    Ok(buffer) => {
		if buffer == "trans" {
		    let trans = Transaction::TransactionFromSummary(Box::new(sum));
		    transaction(trans);
		    return;
		} else {
		    println!("Unkown Command");
		}
	    },
	    Err(_) => {
		continue;
	    },
	}
    }
}

fn done(d : Done) {
    println!("You're in Done.");
    loop {
	match read_line() {
	    Ok(buffer) => {
		if buffer == "logout" {
		    let l = Logout::Logout(d);
		    logout(l);
		    return;
		} if buffer == "back" {
		    let sum = Summary::Back(Box::new(d));
		    summary(sum);
		    return;
		} else {
		    println!("Unkown Command");
		}
	    },
	    Err(_) => {
		continue;
	    },
	}
    }
}

fn logout(_l : Logout) {
    println!("You're in Logout.");
}

fn main() {
    println!("Movement Table");
    println!("Login   ---------- login    -------> Summary");
    println!("Summary ---------- auth     -------> Authentication");
    println!("Authentication --- trans    -------> Transaction");
    println!("Authentication --- summary  -------> Summary with Authentication (Summary A)");
    println!("Summary A -------- trans    -------> Transaction");
    println!("Transaction ------ abort    -------> Summary without Authentication (Summary)");
    println!("Transaction ------ summary  -------> Summary with Authentication (Summary A)");
    println!("Transaction ------ complete -------> Done");
    println!("Done ------------- logout   -------> Logout");
    println!("Done ------------- Back     -------> Summary");
    println!("Logout ----------------------------> Exit");
    let l = Login::Login;
    login(l);

    
}
