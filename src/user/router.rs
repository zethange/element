use axum::{extract::Path, Extension, Json};
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::entities::user::{ActiveModel, Entity, Model};

pub fn get_router() -> OpenApiRouter {
    let app = OpenApiRouter::new()
        .routes(routes!(get_users))
        .routes(routes!(get_user_by_id))
        .routes(routes!(delete_user_by_id))
        .routes(routes!(create_user));

    app
}

#[utoipa::path(get, path = "", responses((status = OK, body = Vec<Model>)), tag = "users")]
async fn get_users(Extension(db): Extension<DatabaseConnection>) -> Json<Vec<Model>> {
    let users = Entity::find().all(&db).await.unwrap();

    Json(users)
}

#[utoipa::path(get, path = "/{id}", tag = "users")]
async fn get_user_by_id(
    Path(id): Path<i32>,
    Extension(db): Extension<DatabaseConnection>,
) -> Json<Option<Model>> {
    let user = Entity::find_by_id(id).one(&db).await.unwrap();

    Json(user)
}

#[utoipa::path(delete, path = "/{id}", tag = "users")]
async fn delete_user_by_id(
    Path(id): Path<i32>,
    Extension(db): Extension<DatabaseConnection>,
) -> Json<bool> {
    let res = Entity::delete_by_id(id).exec(&db).await.unwrap();

    Json(res.rows_affected > 0)
}

#[utoipa::path(post, path = "", responses((status = OK, body = Model)), tag = "users")]
async fn create_user(
    Extension(db): Extension<DatabaseConnection>,
    Json(dto): Json<Model>,
) -> Json<Model> {
    let user = ActiveModel {
        email: Set(dto.email),
        first_name: Set(dto.first_name),
        last_name: Set(dto.last_name),
        password: Set(dto.password),
        ..Default::default()
    };

    let result = user.insert(&db).await.unwrap();

    Json(result)
}
