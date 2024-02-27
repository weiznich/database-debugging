use diesel::prelude::*;

#[derive(Insertable, Queryable, Selectable)]
#[diesel(table_name = crate::schema::principals)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Principal {
	pub tconst: i32,
	pub ordering: i32,
	pub nconst: i32,
	pub category: String,
	pub job: Option<String>,
	pub characters: Option<String>,
}
