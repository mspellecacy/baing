use crate::AppState;
use common::model::collections::UserCollection;
use std::error;
use uuid::Uuid;

pub async fn get_user_special_collections(
    owner_id: Uuid,
    app: &AppState,
) -> Result<Vec<UserCollection>, Box<dyn error::Error>> {
    let collections = sqlx::query_as!(
        UserCollection,
        r#"SELECT
        id, owner_id,name, created_at, active, collection, locked, tags, special,
        CAST(sharing as text)
        FROM collections
        WHERE
            owner_id = $1
        AND
            special is not null
        "#,
        owner_id
    )
    .fetch_all(&app.db)
    .await;

    Ok(collections?)
}
