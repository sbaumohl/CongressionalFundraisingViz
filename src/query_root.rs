#[derive(Debug, seaography::macros::QueryRoot)]
#[seaography(entity = "crate::entities::committee_contributions")]
#[seaography(entity = "crate::entities::committees")]
#[seaography(entity = "crate::entities::independent_expenditures")]
#[seaography(entity = "crate::entities::members")]
#[seaography(entity = "crate::entities::seaql_migrations")]
pub struct QueryRoot;
