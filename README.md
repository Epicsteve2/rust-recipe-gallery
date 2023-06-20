```bash
docker exec --interactive --tty rust-recipe-gallery-postgres bash
psql --username=rust-recipe-gallery --dbname=recipe-gallery

diesel migration redo

# Frontend
tailwindcss --input ./input.css --output ./style/tailwind.css --watch
cargo leptos watch
cd end2end
npm install
npx playwright install
# npx playwright install-deps
cd ..
cargo leptos end-to-end
```

```sql
SELECT * FROM recipes;
```
