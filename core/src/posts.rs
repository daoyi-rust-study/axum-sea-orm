use anyhow;
use common::entity::posts;
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};

pub async fn create_first_posts(
    db: &DatabaseConnection,
    title: String,
) -> anyhow::Result<posts::Model> {
    let posts_model = posts::ActiveModel {
        title: Set(title),
        text: Set("wuhaha asldk.".to_owned()),
        ..Default::default() // all other attributes are `NotSet`
    };

    let posts_model: posts::Model = posts_model.insert(db).await?;

    Ok(posts_model)
}
