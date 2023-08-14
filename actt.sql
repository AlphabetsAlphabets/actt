CREATE TABLE IF NOT EXISTS "Ranks" (
	"Name"	TEXT,
	PRIMARY KEY("Name")
);

INSERT INTO Ranks (Name)
SELECT 'Unrated' WHERE NOT EXISTS (SELECT 1 FROM Ranks WHERE Name = 'Unrated')
UNION
SELECT 'Gold' WHERE NOT EXISTS (SELECT 1 FROM Ranks WHERE Name = 'Gold')
UNION
SELECT 'Emerald' WHERE NOT EXISTS (SELECT 1 FROM Ranks WHERE Name = 'Emerald')
UNION
SELECT 'Diamond' WHERE NOT EXISTS (SELECT 1 FROM Ranks WHERE Name = 'Diamond');

CREATE TABLE IF NOT EXISTS "Players" (
	"UID"	TEXT,
	"Win"	INTEGER,
	"Loss"	INTEGER,
	"Disqualifications"	INTEGER,
	"Rank"	TEXT,
	"Points"	INTEGER,
	"WinStreak"	INTEGER,
	"LoseStreak"	INTEGER,
	PRIMARY KEY("UID")
);

CREATE TABLE IF NOT EXISTS "History" (
	"Challenger"	TEXT,
	"Challenged"	TEXT,
	"Date"	DATE,
	"Finished"	INTEGER,
	"Winner"	INTEGER,
	FOREIGN KEY("Challenger") REFERENCES "Players"("UID")
	FOREIGN KEY("Challenged") REFERENCES "Players"("UID")
);

