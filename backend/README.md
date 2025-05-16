### Setup diesel
- 1: Create file .env with DATABASE_URL={your url} (DATABASE_URL=mysql://username:password@host:port/{database_name})
- 2: diesel setup 
- 3: diesel migration generate {name_of_migration}
- 4: Write:

  in file up.sql
  ```sql
    CREATE TABLE posts (
    id INTEGER AUTO_INCREMENT PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    body TEXT NOT NULL,
    published BOOLEAN NOT NULL DEFAULT FALSE
    );
  ```
  in file down.sql
  ```
    DROP TABLE posts;
  ```
- 5: diesel migration run / diesel migration redo


### (TODO) Save token to Cache Database(Redis)
