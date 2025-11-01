use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize, ToSchema)]
#[sea_orm(table_name = "expenses")]
pub struct Model {
    #[sea_orm(primary_key)]
    /// Unique identifier for the expense
    pub id: Uuid,
    /// User ID who owns this expense
    pub user_id: Uuid,
    /// Description of the expense
    pub description: String,
    /// Amount of the expense
    pub amount: Decimal,
    /// Date and time of the expense
    pub date: DateTime,
    /// Category of the expense
    pub category: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id"
    )]
    User,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
