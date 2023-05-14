CREATE TABLE "users" (
    "id" SERIAL PRIMARY KEY,
    "name" VARCHAR(50) NOT NULL,
    "email" VARCHAR(50) DEFAULT NULL,
    "phone_number_code" INT NOT NULL,
    "phone_number" VARCHAR(10) NOT NULL,
    "password" VARCHAR(50) NOT NULL,
    "token" VARCHAR(200) DEFAULT NULL
)