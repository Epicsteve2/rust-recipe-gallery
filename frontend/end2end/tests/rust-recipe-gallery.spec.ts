import { test, expect, Page } from "@playwright/test";

test("homepage has title", async ({ page }) => {
  await page.goto("http://localhost:3000/");

  await expect(page).toHaveTitle("Rust Recipe Gallery");
});

test("add recipe", async ({page}) => {
    // await page.go
  await page.goto("http://localhost:3000/");
  await page.getByRole('link', { name: 'Add Recipe' }).click();
  await expect(page).toHaveTitle("Rust Recipe Gallery - Add Recipe");

  await page.getByRole('link', { name: 'Add Recipe' }).click();

})