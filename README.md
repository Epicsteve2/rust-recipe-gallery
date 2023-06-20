```bash
docker exec --interactive --tty rust-recipe-gallery-postgres bash
psql --username=rust-recipe-gallery --dbname=recipe-gallery

diesel migration redo
diesel migration run
diesel database reset

# Frontend
tailwindcss --input ./input.css --output ./style/tailwind.css --watch
cargo leptos watch
cd end2end
npm install
npx playwright test
```

```sql
SELECT * FROM recipes;
```
