```bash
docker exec --interactive --tty rust-recipe-gallery-postgres bash
psql --username=rust-recipe-gallery --dbname=recipe-gallery

diesel migration redo

tailwindcss --input ./input.css --output ./style/tailwind.css --watch
```

```sql
SELECT * FROM recipes;
```