# ActorSearch
![image](https://github.com/jepp3183/ActorSearch/assets/23053972/a253a536-da6e-4718-bb89-4fd277c90093)
Simple app mainly developed to experiment with a few different libraries and how they work together:
- [Rust](https://www.rust-lang.org) and [Axum](https://github.com/tokio-rs/axum) as the web framework for the back-end
- [SQLx](https://github.com/launchbadge/sqlx) to connect a postgres database with compile-time checked queries!
- [Redis](https://redis.io/) for caching poster-url responses from the simple (but slow) [OMDB Api](https://www.omdbapi.com/)
- [Tera](https://github.com/Keats/tera), [HTMX](https://www.htmx.org), and [Tailwind](tailwindcss.com) for the front-end


## Running
1. First, the imdb data must be downloaded from [here](https://datasets.imdbws.com/). The files name.basics.tsv.gz, title.basics.tsv.gz, title.principals.tsv.gz, title.ratings.tsv.gz must be placed in postgres/data. Postgres will then import the data when the docker container starts.
2. Create a .env file in the root directory of the following form:
```
POSTGRES_PW=password
POSTGRES_USER=postgres
POSTGRES_HOST=postgres
POSTGRES_PORT=5432
DATABASE_URL=postgres://${POSTGRES_USER}:${POSTGRES_PW}@${POSTGRES_HOST}:${POSTGRES_PORT}/postgres

OMDB_KEY=???

REDIS_URL=redis://redis:6379

SERVER_PORT=5000
```
3. Next, simply run `docker-compose up` from the root directory.
   - It might take a little while to compile the rust-app the first time.
   - It will take a while for postgres to import the data, and the site will not be available before the process has completed.
