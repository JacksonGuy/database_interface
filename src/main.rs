use mysql::*;
use mysql::prelude::*;

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let url = "mysql://root:password@localhost:3307/cobalt_mines";
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
    let result: Vec<Row> = conn.query(
        r"SELECT * FROM test"
    )?;

    for row in result.iter() {
        println!("{}", row.unwrap());
    }

    Ok(())
}
