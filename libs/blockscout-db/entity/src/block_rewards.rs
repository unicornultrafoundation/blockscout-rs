//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.4

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "block_rewards")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub address_hash: Vec<u8>,
    #[sea_orm(primary_key, auto_increment = false)]
    pub address_type: String,
    #[sea_orm(primary_key, auto_increment = false)]
    pub block_hash: Vec<u8>,
    #[sea_orm(column_type = "Decimal(Some((100, 0)))", nullable)]
    pub reward: Option<Decimal>,
    pub inserted_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::addresses::Entity",
        from = "Column::AddressHash",
        to = "super::addresses::Column::Hash",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Addresses,
    #[sea_orm(
        belongs_to = "super::blocks::Entity",
        from = "Column::BlockHash",
        to = "super::blocks::Column::Hash",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Blocks,
}

impl Related<super::addresses::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Addresses.def()
    }
}

impl Related<super::blocks::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Blocks.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
