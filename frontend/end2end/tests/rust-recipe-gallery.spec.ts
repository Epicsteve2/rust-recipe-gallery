import { test, expect, Page } from "@playwright/test";

test("homepage has title", async ({ page }) => {
  await page.goto("http://localhost:3000/");

  await expect(page).toHaveTitle("Rust Recipe Gallery");
});

test('add recipe', async ({ page }) => {
  await page.goto('http://localhost:3000/');
  await page.getByRole('link', { name: 'Add Recipe' }).click();
  await page.getByPlaceholder('Title').fill('Roasted Sweet Potato');
  await page.getByPlaceholder('Write your ingredients here...').fill(`Sweet Potato
Olive Oil
Salt`);
  await page.getByPlaceholder('Write your steps here...').fill(`1. Bake
2. EAT!!`);
  // This is needed because the button stays inactive if automatically filled out. should fix
  await page.getByRole('main').click();
  await page.getByRole('button', { name: 'Create Recipe' }).click();
  await expect(page.getByRole('link', { name: /^Success! Recipe ID: [0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/ })).toBeVisible();
});