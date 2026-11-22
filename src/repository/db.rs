use chrono::{self, NaiveDateTime, Utc};
use tokio_postgres::tls::NoTlsStream;
use tokio_postgres::{Client, NoTls};

pub struct Database {
    client: Client,
}

pub struct User {
    pub id: String,
    pub name: String,
    pub avatar: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl User {
    pub fn new(id: String, name: String, avatar: String) -> Self {
        Self {
            id,
            name,
            avatar,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        }
    }
}

impl Database {
    pub async fn new(
        db_url: &str,
    ) -> Result<
        (
            Self,
            tokio_postgres::Connection<tokio_postgres::Socket, NoTlsStream>,
        ),
        tokio_postgres::Error,
    > {
        let (client, connection) = tokio_postgres::connect(db_url, NoTls).await?;
        Ok((Self { client }, connection))
    }

    pub async fn init(&self) -> Result<(), tokio_postgres::Error> {
        self.client
            .execute(
                "CREATE TABLE IF NOT EXISTS users (id TEXT PRIMARY KEY, name TEXT, avatar TEXT, created_at TIMESTAMP, updated_at TIMESTAMP)",
                &[],
            )
            .await?;
        Ok(())
    }

    pub async fn get_user(&self, id: String) -> Result<Option<User>, tokio_postgres::Error> {
        let row = self
            .client
            .query_opt("SELECT * FROM users WHERE id = $1", &[&id])
            .await?;
        Ok(row.map(|r| User {
            id: r.get(0),
            name: r.get(1),
            avatar: r.get(2),
            created_at: r.get(3),
            updated_at: r.get(4),
        }))
    }

    pub async fn create_user(&self, user: User) -> Result<(), tokio_postgres::Error> {
        let dt = Utc::now().naive_utc();
        self.client
            .execute(
                "INSERT INTO users (id, name, avatar, created_at, updated_at) VALUES ($1, $2, $3, $4, $5)",
                &[&user.id, &user.name, &user.avatar, &dt, &dt],
            )
            .await?;
        Ok(())
    }

    pub async fn update_user(
        &self,
        id: String,
        name: Option<String>,
        avatar: Option<String>,
    ) -> Result<u64, tokio_postgres::Error> {
        self.client
            .execute(
                "UPDATE users SET name = COALESCE($2, name), avatar = COALESCE($3, avatar), updated_at = $4 WHERE id = $1",
                &[&id, &name, &avatar, &Utc::now().naive_utc()],
            )
            .await
    }
}
