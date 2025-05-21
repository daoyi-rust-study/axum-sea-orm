# axum-sea-orm

https://crates.io/crates/axum

https://docs.rs/axum/0.8.4/axum/

https://www.sea-ql.org/SeaORM/docs/index/

https://github1s.com/lingdu1234/axum_admin

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
```shell
sea-orm-cli migrate init
```

```mysql
CREATE DATABASE `axum-sea-orm` CHARACTER SET 'utf8mb4' COLLATE 'utf8mb4_general_ci';

CREATE USER `axum-sea-orm`@`%` IDENTIFIED WITH mysql_native_password BY '123456';

GRANT Alter, Alter Routine, Create, Create Routine, Create Temporary Tables, Create View, Delete, Drop, Event, Execute, Grant Option, Index, Insert, Lock Tables, References, Select, Show View, Trigger, Update ON `axum-sea-orm`.* TO `axum-sea-orm`@`%`;

```
.env
```dotenv
DATABASE_URL=mysql://root:123456@localhost/axum-sea-orm
```
```shell
sea-orm-cli migrate up
```
```shell
cargo new entity --lib
```
```shell
sea-orm-cli generate entity -o entity/src --with-serde both
```

![img.png](img.png)

```shell
cargo new api --lib
cargo new core --lib
cargo new middleware --lib 
cargo new common --lib
```
```shell
sea-orm-cli generate entity -o common/src/entity --with-serde both
```