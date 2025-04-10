use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "film_actor")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub actor_id: i32,
    #[sea_orm(primary_key, auto_increment = false)]
    pub film_id: i32,
    pub last_update: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::actor::Entity",
        from = "Column::ActorId",
        to = "super::actor::Column::ActorId",
        on_update = "Cascade",
        on_delete = "NoAction"
    )]
    Actor,
    #[sea_orm(
        belongs_to = "super::film::Entity",
        from = "Column::FilmId",
        to = "super::film::Column::FilmId",
        on_update = "Cascade",
        on_delete = "NoAction"
    )]
    Film,
}

impl Related<super::actor::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Actor.def()
    }
}

impl Related<super::film::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Film.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelatedEntity)]
pub enum RelatedEntity {
    #[sea_orm(entity = "super::actor::Entity")]
    Actor,
    #[sea_orm(entity = "super::film::Entity")]
    Film,
}
