drop table if exists title_basics;
create table title_basics(
    tconst VARCHAR(255) NOT NULL PRIMARY KEY,
    titleType VARCHAR(255),
    primaryTitle text,
    originalTitle text,
    isAdult BOOLEAN,
    startYear INTEGER CHECK (startYear >= 1000 AND startYear <= 9999),
    endYear INTEGER CHECK (endYear >= 1000 AND endYear <= 9999),
    runtimeMinutes INTEGER,
    genres TEXT
);
copy title_basics from program 'gzip -dc /data_tsv/title.basics.tsv.gz' with delimiter E'\t' HEADER;


drop table if exists name_basics;
CREATE TABLE name_basics (
    nconst VARCHAR(255) NOT NULL PRIMARY KEY,
    primaryName text,
    birthYear INTEGER,
    deathYear INTEGER,
    primaryProfession TEXT,
    knownForTitles text
);
copy name_basics from program 'gzip -dc /data_tsv/name.basics.tsv.gz' with delimiter E'\t' HEADER;


drop table if exists title_principals;
create table title_principals(
	tconst varchar(10),
	orderingIndex varchar(20),
	nconst varchar(10),
	category varchar(50),
	job text,
	characters text
);
copy title_principals from program 'gzip -dc /data_tsv/title.principals.tsv.gz' with delimiter E'\t' HEADER;

drop table if exists title_ratings;
create table title_ratings(
	tconst varchar(10) primary key,
	averageRating real,
	numVotes integer
);
copy title_ratings from program 'gzip -dc /data_tsv/title.ratings.tsv.gz' with delimiter E'\t' HEADER;


CREATE INDEX idx_tconst ON title_basics (tconst);
CREATE INDEX idx_nconst ON title_principals (nconst);
CREATE INDEX idx_primaryName ON name_basics (primaryName);
CREATE INDEX idx_nconst_nb ON name_basics (nconst);







