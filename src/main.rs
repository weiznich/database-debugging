use diesel::{Connection, PgConnection, RunQueryDsl};
use dotenvy::dotenv;
use imdb::{models::Principal, schema::principals};
use rayon::prelude::*;
use std::{
	env,
	fs::File,
	io::{BufRead, BufReader},
	time::Instant,
};

fn main() {
	dotenv().ok();
	let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
	let f = File::open("data/title.principals.tsv").unwrap();
	let mut reader = BufReader::new(f);
	let mut tmp = String::new();
	reader.read_line(&mut tmp).unwrap();
	let mut counter: u128 = 1;
	let mut principals = Vec::new();
	let start = Instant::now();
	loop {
		tmp.clear();
		reader.read_line(&mut tmp).unwrap();
		let row: Vec<&str> = tmp.trim().split('\t').collect();
		principals.push(Principal {
			tconst: (&row[0][2..]).parse().unwrap(),
			ordering: row[1].parse().unwrap(),
			nconst: (&row[2][2..]).parse().unwrap(),
			category: row[3].to_string(),
			job: if row[4] == "\\N" {
				None
			} else {
				Some(row[4].to_string())
			},
			characters: if row[5] == "\\N" {
				None
			} else {
				Some(row[5][2..row[5].len() - 2].to_string())
			},
		});
		counter += 1;
		if reader.fill_buf().unwrap().len() == 0 {
			break;
		}
	}
	principals.par_chunks(10922).for_each_init(
		|| {
			PgConnection::establish(&database_url)
				.unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
		},
		|conn, chunk| {
			diesel::insert_into(principals::table)
				.values(chunk)
				.execute(conn)
				.unwrap();
		},
	);
	println!(
		"Inserted {} rows in {} seconds",
		counter,
		start.elapsed().as_secs()
	);
}
