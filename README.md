```bash
docker exec --interactive --tty rust-recipe-gallery-postgres bash
psql --username=rust-recipe-gallery --dbname=recipe-gallery

diesel migration redo
```

```sql
SELECT * FROM recipes;
```