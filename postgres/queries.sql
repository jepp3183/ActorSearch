select primaryTitle, startYear from title_basics where tconst in (
	select tconst from title_principals where nconst=(
		select nconst from name_basics where primaryName='Tom Hanks'
	)
);

select * from title_basics tb 

SELECT tb.primaryTitle, tb.startYear
FROM title_basics tb
JOIN title_principals tp ON tb.tconst = tp.tconst
JOIN name_basics nb ON tp.nconst = nb.nconst
WHERE nb.primaryName = 'Tom Hanks'
order by startYear asc;


SELECT tb.primaryTitle, tb.startYear, tr.averagerating::varchar(10), tb.titleType
FROM title_basics tb
join title_ratings tr on tb.tconst  = tr.tconst
WHERE tb.tconst IN (
    SELECT tp.tconst
    FROM title_principals tp
    WHERE tp.nconst IN (
        SELECT nb.nconst 
        FROM name_basics nb
        WHERE nb.primaryName = 'Cillian Murphy'
    )
    GROUP BY tp.tconst
    HAVING COUNT(DISTINCT tp.nconst) = 1
) and
tb.titleType in ('movie', 'tvSeries', 'tvMovie', 'tvMiniSeries', 'tvShort','short', 'video', 'videoGame', 'tvSpecial')
order by tr.numVotes desc;

SELECT tb.primaryTitle, tb.startYear, tr.numVotes, tb.titleType 
FROM title_basics tb
join title_ratings tr on tb.tconst  = tr.tconst
WHERE tb.tconst IN (
    SELECT tp.tconst
    FROM title_principals tp
    WHERE tp.nconst IN (
        SELECT nb.nconst 
        FROM name_basics nb
        WHERE nb.primaryName = 'Cillian Murphy'
    )
    GROUP BY tp.tconst
    HAVING COUNT(DISTINCT tp.nconst) = 1
)
order by tr.numVotes desc;

select tb.titleType, COUNT(tb.titleType)
from title_basics tb
group by tb.titletype 
order by COUNT(tb.titleType) desc

SELECT tp.tconst
FROM title_principals tp
WHERE tp.nconst IN (
    SELECT nb.nconst 
    FROM name_basics nb
    WHERE nb.primaryName = ANY('{Tom Hanks, Meryl Streep}')
)
GROUP BY tp.tconst
HAVING COUNT(DISTINCT tp.nconst) = 2 

select count(distinct nb.primaryName)
from name_basics nb
where nb.nconst in (
	select tp.nconst
	from title_principals tp
);

select tp.nconst, count(tp.nconst) as c
from title_principals tp
group by tp.nconst
order by c desc;

select COUNT(nb.primaryName)
FROM name_basics nb


DBCC DROPCLEANBUFFERS;


SELECT tb.primaryTitle, tb.startYear, tr.averagerating::varchar(10), tb.tconst
FROM title_basics tb
join title_ratings tr on tb.tconst  = tr.tconst
WHERE tb.tconst IN (
    SELECT tp.tconst
    FROM title_principals tp
    WHERE tp.nconst IN (
        SELECT nb.nconst
        FROM name_basics nb
        WHERE nb.primaryName = ANY('{Meryl Streep}')
    )
    GROUP BY tp.tconst
    HAVING COUNT(DISTINCT tp.nconst) = 1
)
and tb.titleType in ('movie', 'tvSeries', 'tvMovie', 'tvMiniSeries', 'tvShort','short', 'video', 'videoGame', 'tvSpecial')
order by tr.numVotes desc;





