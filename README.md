```bash
docker exec --interactive --tty rust-recipe-gallery-postgres bash
psql --username=postgres --dbname=recipe-gallery

diesel migration redo
diesel migration run
diesel database reset

# Frontend
cd frontend/
tailwindcss --input ./input.css --output ./style/tailwind.css --watch
cargo leptos watch --hot-reload
# I think this is faster
cd frontend/
cargo leptos watch

trunk -v serve --features hydrate

cd end2end
npm install
npx playwright test
```

```sql
SELECT * FROM recipes;
```

```bash
docker build --tag rust-recipe-gallery-devcontainer-test .
```