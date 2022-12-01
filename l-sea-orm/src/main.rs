mod entity;


use sea_orm::Database;
use sea_orm::DatabaseConnection;
use entity::post;
use sea_orm::EntityTrait;
use sea_orm::ActiveModelTrait;
use sea_orm::ActiveValue::Set;
use sea_orm::ModelTrait;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_test_writer()
        .init();

    let db: DatabaseConnection = Database::connect("postgres://postgres:postgres@192.168.106.2:30490/test").await?;

    // insert
    let post = post::ActiveModel {
        title: Set(String::from("Amazing title 1")),
        text: Set(String::from("Lorem ipsum dolor sit amet.")),
        ..Default::default()
    };

    let post: post::Model = post.insert(&db).await?;

    println!("Post created with ID: {}, TITLE: {}", post.id, post.title);


    // find all
    let posts = post::Entity::find().all(&db).await?;

    println!("post {:#?}", posts);


    // UPDATE titel of Post by ID
    let post = post::Entity::find_by_id(1).one(&db).await?;
    let mut post: post::ActiveModel = post.unwrap().into();
    post.title = Set("Updated title".to_owned());
    let post: post::Model = post.update(&db).await?;

    println!("Post updated for ID: {} with TITLE: {}", post.id, post.title);

    /*
    // delete
    let post = post::Entity::find_by_id(1).one(&db).await?;
    let post: post::Model = post.unwrap();

    let res: DeleteResult = post.delete(&db).await?;
    assert_eq!(res.rows_affected, 1);

    println!("{:?}", res);
    */

    Ok(())
}
