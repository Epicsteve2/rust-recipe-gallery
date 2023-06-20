```bash
docker exec --interactive --tty rust-recipe-gallery-postgres bash
psql --username=rust-recipe-gallery --dbname=recipe-gallery

diesel migration redo

# Frontend
tailwindcss --input ./input.css --output ./style/output.css --watch
cargo leptos watch
```

```sql
SELECT * FROM recipes;
```