CREATE TABLE "users" (
    "id" SERIAL PRIMARY KEY,
    "username" VARCHAR(50) NOT NULL,
    "password" VARCHAR(50) NOT NULL,
    "token" VARCHAR(200) DEFAULT NULL
)