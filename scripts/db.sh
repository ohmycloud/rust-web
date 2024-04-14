psql -h localhost -p 5432 -U postgres
create database rustwebdev;
\c rustwebdev;
# create
sqlx migrate add -r question_table
sqlx migrate run --database-url postgresql://postgres:password@localhost:5432/rustwebdev
sqlx migrate add -r answer_table
sqlx migrate run --database-url postgresql://postgres:password@localhost:5432/rustwebdev

# revert
sqlx migrate revert --database-url postgresql://postgres:password@localhost:5432/rustwebdev
sqlx migrate revert --database-url postgresql://postgres:password@localhost:5432/rustwebdev