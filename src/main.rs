// use core::fmt;
use std::fmt;

use mysql::*;
use mysql::prelude::*;

mod tables;
mod database;
mod csv;

#[derive(Debug, PartialEq, Eq)]
struct Payment {
    customer_id: i32,
    amount: i32,
    account_name: String
}

impl fmt::Display for Payment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.customer_id, self.amount, self.account_name)
    }
}

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let url = "mysql://root:password@localhost:3306/cobalt_mines";
    let pool = Pool::new(url)?;

    let mut conn = pool.get_conn()?;

    // Create Table
    conn.query_drop(
        r"CREATE TEMPORARY TABLE test (
            customer_id INT NOT NULL,
            amount INT NOT NULL,
            account_name TEXT
        )"
    )?;

    // Insert Payments
    conn.query_drop(
        r"INSERT INTO test
          VALUES (1, 100, 'Jackson')"
    )?;

    // Get Payments
    let result = conn.query_map(
        r"SELECT * FROM test",
        |(customer_id, amount, account_name)| {
            Payment { customer_id, amount, account_name }
        },
    )?;

    for payment in result {
        println!("{}", payment);
    }

    Ok(())
}
