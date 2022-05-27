use super::schema::subs;

#[derive(Queryable)]
pub struct Subs {
    pub id: i32,
    pub name: String,
    pub email: String,
}

#[derive(Insertable)]
#[table_name = "subs"]
pub struct NewSub<'a> {
    pub name: &'a str,
    pub email: &'a str,
}
