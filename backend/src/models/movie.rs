use diesel::{AsChangeset, Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name = crate::repository::schema::movies)]
pub struct Movie {
    #[serde(default)]
    pub id: i32,
    pub title: String,
    pub release_date: Option<chrono::NaiveDate>,
    pub description: Option<String>,
    pub genre: Option<String>,
    pub icon: Option<String>
}