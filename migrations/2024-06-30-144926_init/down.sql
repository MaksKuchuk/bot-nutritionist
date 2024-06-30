-- This file should undo anything in `up.sql`
DROP TABLE IF EXISTS "Users";
DROP TABLE IF EXISTS "Foods";
DROP TABLE IF EXISTS "UserDiets";
DROP TABLE IF EXISTS "ChoosenDiets";
DROP TABLE IF EXISTS "DietExample";
DROP INDEX IF EXISTS "UserId";
