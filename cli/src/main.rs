use postgres::{Client, NoTls};

fn main() {
    let mut client =
        Client::connect("host=127.0.0.1 user=lipeng dbname=misc_test", NoTls).unwrap();
        //Client::connect("host=127.0.0.1 user=lipeng dbname=userrelations_test", NoTls).unwrap();
        //Client::connect("host=127.0.0.1 user=lipeng dbname=baseinfo_test", NoTls).unwrap();
    println!("Hello, world!");
    // if let Ok(rows) = client.query("select * from dish order by id", &[]) {
    //     for row in rows {
    //         let i: i64 = row.get(0);
    //         println!("row {}", i);
    //     }
    // }

    let mut table_names: Vec<String> = Vec::new();
    if let Ok(rows) = client.query(
        "SELECT table_name FROM information_schema.tables WHERE table_schema = 'public'",
        &[],
    ) {
        for row in rows {
            //let a: () = row.get(0);
            let i: &str = row.get(0);
            //println!("row {}", i);
            table_names.push(i.to_string());
        }
    }
    println!("table names: {:?}\n\n", table_names);

    for table_name in &table_names {
        let mut table_comumns: String = String::new();
        let sql_str:String  = format!("SELECT column_name FROM information_schema.columns WHERE table_schema = 'public' AND table_name = '{}'", table_name);
        let rows = client.query(&sql_str[..], &[]).unwrap();
        for row in &rows {
            //let a: () = row.get(0);
            let i: &str = row.get(0);
            table_comumns.push_str(i);
            table_comumns.push_str(&", ");
            // if table_comumns.len() > 0 {
            //     table_comumns.push_str(",")
            // }
            //println!("row {}", i);
        }
        println!(
            "table:{0} | select {1} from {0}",
            table_name,
            table_comumns.trim().trim_matches(',')
        );
    }

    println!("TRUNCATE {};", table_names.join(","));

    // if let Ok(rows) = {
    //     for row in rows {
    //         //let a: () = row.get(0);
    //         let i: &str = row.get(0);
    //         println!("row {}", i);
    //     }
    // }
}
