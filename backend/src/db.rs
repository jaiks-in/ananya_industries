use sqlx::PgPool;

pub async fn init_db(database_url:&str) ->Result<PgPool,sqlx::Error>{
    let pool=PgPool::connect(&database_url).await?;
    sqlx::query(
        r#"CREATE TABLE IF NOT EXISTS users(
        id SERIAL PRIMARY KEY,
        username TEXT NOT NULL,
        password TEXT NOT NULL,
        email TEXT NOT NULL,
        created_at TIMESTAMP NOT NULL DEFAULT now()
        )"#
    ).execute(&pool).await?;
    sqlx::query(
        r#"CREATE TABLE IF NOT EXISTS contacts(
        id SERIAL PRIMARY KEY,
        name TEXT NOT NULL,
        email TEXT NOT NULL,
        mobile TEXT NOT NULL,
        message TEXT NOT NULL,
        created_at TIMESTAMP NOT NULL DEFAULT now()
        )"#
    ).execute(&pool).await?;
    Ok(pool)
}