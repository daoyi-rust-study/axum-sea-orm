# axum-sea-orm

```shell
cargo new axum-sea-orm 
```
```shell
cd axum-sea-orm
echo "# axum-sea-orm" >> README.md
git add .
git commit -m "first commit"
git branch -M main
git remote add origin git@github.com:daoyi-rust-study/axum-sea-orm.git 
git push -u origin main
```
```shell
cargo add axum
cargo add tokio -F full
```

```mysql
CREATE DATABASE `axum-sea-orm` CHARACTER SET 'utf8mb4' COLLATE 'utf8mb4_general_ci';

CREATE USER `axum-sea-orm`@`%` IDENTIFIED WITH mysql_native_password BY '123456';

GRANT Alter, Alter Routine, Create, Create Routine, Create Temporary Tables, Create View, Delete, Drop, Event, Execute, Grant Option, Index, Insert, Lock Tables, References, Select, Show View, Trigger, Update ON `axum-sea-orm`.* TO `axum-sea-orm`@`%`;

```