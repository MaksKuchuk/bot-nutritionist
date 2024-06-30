-- Your SQL goes here
CREATE TABLE IF NOT EXISTS "Users" (
	"id"	TEXT NOT NULL UNIQUE,
	"gender"	TEXT NOT NULL,
	"age"	INTEGER NOT NULL,
	"height"	INTEGER NOT NULL,
	"weight"	INTEGER NOT NULL,
	"physical_activity_level"	TEXT NOT NULL,
	"goal"	TEXT NOT NULL,
	PRIMARY KEY("id")
);
CREATE TABLE IF NOT EXISTS "Foods" (
	"id"	INTEGER NOT NULL UNIQUE,
	"name"	TEXT NOT NULL,
	"kcal"	INTEGER NOT NULL,
	"protein"	INTEGER NOT NULL,
	"fat"	INTEGER NOT NULL,
	"carbohydrate"	INTEGER NOT NULL,
	"category"	TEXT NOT NULL,
	PRIMARY KEY("id")
);
CREATE TABLE IF NOT EXISTS "ChoosenDiets" (
	"userid"	TEXT NOT NULL UNIQUE,
	"dietid"	INTEGER NOT NULL UNIQUE,
	"state"	INTEGER NOT NULL,
	PRIMARY KEY("userid")
);
CREATE TABLE IF NOT EXISTS "UserDiets" (
	"id"	INTEGER NOT NULL UNIQUE,
	"userid"	TEXT NOT NULL,
	"name"	TEXT NOT NULL,
	"diet"	TEXT NOT NULL,
	PRIMARY KEY("id" AUTOINCREMENT)
);
CREATE TABLE IF NOT EXISTS "DietExamples" (
	"id"	INTEGER NOT NULL UNIQUE,
	"products"	TEXT NOT NULL,
	"weights"	TEXT NOT NULL,
	"proteins"	INTEGER NOT NULL,
	"fats"	INTEGER NOT NULL,
	"carbohydrates"	INTEGER NOT NULL,
	PRIMARY KEY("id" AUTOINCREMENT)
);
INSERT INTO "Users" VALUES ('1114726232','Мужской',21,177,72,'Выше среднего','Поддержание веса');
INSERT INTO "Foods" VALUES (1,'Баранина',203,16,15,0,'Мясопродукты');
INSERT INTO "Foods" VALUES (2,'Свинина',316,16,28,1,'Мясопродукты');
INSERT INTO "Foods" VALUES (3,'Курица',165,21,9,1,'Мясопродукты');
INSERT INTO "Foods" VALUES (4,'Говядина',187,19,12,0,'Мясопродукты');
INSERT INTO "Foods" VALUES (5,'Утка',346,17,61,0,'Мясопродукты');
INSERT INTO "Foods" VALUES (6,'Карп',96,16,4,0,'Рыбопродукты');
INSERT INTO "Foods" VALUES (7,'Кета',138,22,6,0,'Рыбопродукты');
INSERT INTO "Foods" VALUES (8,'Корюшка',91,16,3,0,'Рыбопродукты');
INSERT INTO "Foods" VALUES (9,'Семга',219,21,15,0,'Рыбопродукты');
INSERT INTO "Foods" VALUES (10,'Минтай',70,16,1,0,'Рыбопродукты');
INSERT INTO "Foods" VALUES (11,'Брынза',260,18,20,0,'Молочные продукты');
INSERT INTO "Foods" VALUES (12,'Кефир жирный',59,3,3,4,'Молочные продукты');
INSERT INTO "Foods" VALUES (13,'Молоко',58,3,3,5,'Молочные продукты');
INSERT INTO "Foods" VALUES (14,'Ряженка',85,3,6,4,'Молочные продукты');
INSERT INTO "Foods" VALUES (15,'Творог жирный',226,14,18,2,'Молочные продукты');
INSERT INTO "Foods" VALUES (16,'Багет',262,8,3,51,'Хлебобулочные изделия');
INSERT INTO "Foods" VALUES (17,'Брускетта',315,5,23,22,'Хлебобулочные изделия');
INSERT INTO "Foods" VALUES (18,'Бублик',336,16,1,70,'Хлебобулочные изделия');
INSERT INTO "Foods" VALUES (19,'Булочки сдобные',339,8,9,56,'Хлебобулочные изделия');
INSERT INTO "Foods" VALUES (20,'Гренки пшеничные',390,11,9,66,'Хлебобулочные изделия');
INSERT INTO "Foods" VALUES (21,'Рис',330,7,1,71,'Крупы, макаронные изделия');
INSERT INTO "Foods" VALUES (22,'Пшено',348,12,3,67,'Крупы, макаронные изделия');
INSERT INTO "Foods" VALUES (23,'Овсные хлопья',366,12,7,69,'Крупы, макаронные изделия');
INSERT INTO "Foods" VALUES (24,'Мука овсяная',369,13,7,65,'Крупы, макаронные изделия');
INSERT INTO "Foods" VALUES (25,'Макароны высший сорт',337,10,1,70,'Крупы, макаронные изделия');
INSERT INTO "Foods" VALUES (26,'Капуста цветная',29,3,0,5,'Фрукты, ягоды, овощи');
INSERT INTO "Foods" VALUES (27,'Морковь',33,1,0,7,'Фрукты, ягоды, овощи');
INSERT INTO "Foods" VALUES (28,'Яблоки',44,0,0,10,'Фрукты, ягоды, овощи');
INSERT INTO "Foods" VALUES (29,'Земляника',34,1,0,6,'Фрукты, ягоды, овощи');
INSERT INTO "Foods" VALUES (30,'Арбуз',40,1,0,9,'Фрукты, ягоды, овощи');
INSERT INTO "Foods" VALUES (31,'Ананасовый сок',52,0,0,12,'Соки');
INSERT INTO "Foods" VALUES (32,'Апельсиновый сок',45,1,0,10,'Соки');
INSERT INTO "Foods" VALUES (33,'Грейпфрутовый сок',38,0,0,8,'Соки');
INSERT INTO "Foods" VALUES (34,'Морковный сок',56,1,0,13,'Соки');
INSERT INTO "Foods" VALUES (35,'Яблочный сок',46,1,0,10,'Соки');
INSERT INTO "UserDiets" VALUES (1,'1114726232','рац','8:20
рис, 150, (2.9 25.2 0.4)
треска отварная, 50, (17.8 0 0.7)
белый хлеб, 50, (11 48 4)

13:00
гречка на воде, 150, (3.38 19.94 0.62)
яйцо куриное вареное, 30, (13 1.12 10.61)

19:10
яблоко красное, 100, (0.4 17 0)
банан, 100, (1.2 22 0.2)');
INSERT INTO "UserDiets" VALUES (2,'1114726232','рац2','9:20
рис, 150, (2.9 25.2 0.4)
треска отварная, 50, (17.8 0 0.7)
белый хлеб, 50, (11 48 4)

14:00
гречка на воде, 150, (3.38 19.94 0.62)
яйцо куриное вареное, 30, (13 1.12 10.61)

17:10
яблоко красное, 100, (0.4 17 0)
банан, 100, (1.2 22 0.2)');
INSERT INTO "DietExample" VALUES (1,'макароны, кета \ рис, курица, сыр брынза \ молоко, яблоко, минтай, апельсиновый сок','50, 100 \ 50, 50, 50 \ 200, 100, 100,  50',72,29,95);
CREATE INDEX IF NOT EXISTS "UserId" ON "Users" (
	"id"	ASC
);
