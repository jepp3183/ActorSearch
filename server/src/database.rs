use std::collections::HashSet;

use sqlx::postgres::PgPoolOptions;

#[derive(serde::Serialize)]
pub struct Movie {
    primarytitle: Option<String>,
    startyear: Option<i32>,
    averagerating: Option<String>,
    tconst: String,
}

pub struct DB {
    pool: sqlx::PgPool,
}

impl DB {
    pub async fn new(db_url: String) -> Self {
        println!("Connecting to {}", db_url);
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&db_url)
            .await
            .expect("Failed to connect to Postgres.");
        DB { pool }
    }

    pub async fn search_movies(&self, actors: &Vec<String>) -> Vec<Movie> {
        sqlx::query_as!(
            Movie,
            "
            SELECT tb.primaryTitle, tb.startYear, tr.averagerating::varchar(10), tb.tconst
            FROM title_basics tb
            join title_ratings tr on tb.tconst  = tr.tconst
            WHERE tb.tconst IN (
                SELECT tp.tconst
                FROM title_principals tp
                WHERE tp.nconst IN (
                    SELECT nb.nconst
                    FROM name_basics nb
                    WHERE nb.primaryName = ANY($1)
                )
                GROUP BY tp.tconst
                HAVING COUNT(DISTINCT tp.nconst) = $2
            )
            and tb.titleType in ('movie', 'tvSeries', 'tvMovie', 'tvMiniSeries', 'tvShort','short', 'video', 'videoGame', 'tvSpecial')
            order by tr.numVotes desc;
            ",
            &actors,
            actors.len() as i32
        )
        .fetch_all(&self.pool)
        .await
        .unwrap()
    }

    pub async fn get_actor_suggestions(&self, query: String) -> HashSet<String> {
        sqlx::query!(
            "
            SELECT nb.primaryName
            FROM name_basics nb
            WHERE nb.primaryName LIKE $1 or nb.primaryName LIKE $2
            LIMIT 10;
            ",
            format!("{}%", &query),
            format!("% {}%", &query)
        )
        .fetch_all(&self.pool)
        .await
        .unwrap()
        .iter()
        .map(|r| r.primaryname.clone().unwrap())
        .collect::<HashSet<String>>()
    }
}
