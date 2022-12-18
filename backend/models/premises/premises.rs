use crate::diesel::*;
use crate::schema::*;
use diesel::QueryResult;
use serde::{Deserialize, Serialize};

type Connection = create_rust_app::Connection;

#[tsync::tsync]
#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Insertable, AsChangeset, Identifiable)]
#[diesel(table_name=premises, primary_key(id))]
pub struct Premises {
  pub id: i32,
  pub name: String,
  pub address: String,
  pub user_id: i32,
  pub created_at: chrono::DateTime<chrono::Utc>,
  pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[tsync::tsync]
#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name=premises)]
pub struct PremisesInput {
  pub name: String,
  pub address: String,
  pub user_id: i32,
}

impl Premises {
    pub fn create(db: &mut Connection, item: &PremisesInput) -> QueryResult<Premises> {
        use crate::schema::premises::dsl::*;

        insert_into(premises).values(item).get_result::<Premises>(db)
    }
}