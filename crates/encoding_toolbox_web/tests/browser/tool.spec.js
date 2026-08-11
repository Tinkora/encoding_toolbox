import { expect, test } from "@playwright/test";

async function openTool(page) {
  await page.goto("/static/");
  await expect(page.getByRole("button", { name: "Run" })).toBeEnabled();
}

const inputBox = (page) => page.getByRole("textbox", { name: "Input" });
const outputBox = (page) => page.getByRole("textbox", { name: "Output" });

test("encodes and decodes text through the real WASM boundary", async ({ page }) => {
  await openTool(page);
  await inputBox(page).fill("hello");
  await page.getByRole("button", { name: "Run" }).click();
  await expect(outputBox(page)).toHaveValue("aGVsbG8=");

  await page.getByText("Decode", { exact: true }).click();
  await inputBox(page).fill("aGVsbG8=");
  await page.getByRole("button", { name: "Run" }).click();
  await expect(outputBox(page)).toHaveValue("hello");
  await expect(page.getByRole("status")).toContainText("Decoded 5 bytes");
});

test("calculates digests and visibly warns when MD5 is selected", async ({ page }) => {
  await openTool(page);
  await page.getByText("Digest", { exact: true }).click();
  await page.getByLabel("Algorithm").selectOption("md5");
  await expect(page.getByRole("note")).toContainText("not collision resistant");
  await inputBox(page).fill("abc");
  await page.getByRole("button", { name: "Run" }).click();
  await expect(outputBox(page)).toHaveValue("900150983cd24fb0d6963f7d28e17f72");
});

test("calculates HMAC without persisting the key", async ({ page }) => {
  await openTool(page);
  await page.getByText("HMAC", { exact: true }).click();
  await page.getByLabel("HMAC key").fill("key");
  await inputBox(page).fill("The quick brown fox jumps over the lazy dog");
  await page.getByRole("button", { name: "Run" }).click();
  await expect(outputBox(page)).toHaveValue(
    "f7bc83f430538424b13298e6aa6fb143ef4d59a14946175997479dbc2d1a3cd8",
  );
  expect(await page.evaluate(() => ({ ...localStorage }))).toEqual({});
});

test("digests a bounded local file without uploading it", async ({ page }) => {
  await openTool(page);
  await page.getByText("Digest", { exact: true }).click();
  await page.getByText("File", { exact: true }).click();
  await page.getByLabel("Local file").setInputFiles({
    name: "sample.txt",
    mimeType: "text/plain",
    buffer: Buffer.from("abc"),
  });
  await expect(page.getByText("sample.txt · 3 bytes", { exact: true })).toBeVisible();
  await page.getByRole("button", { name: "Run" }).click();
  await expect(outputBox(page)).toHaveValue(
    "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad",
  );
});

test("copies and downloads a completed result", async ({ page, context }) => {
  await context.grantPermissions(["clipboard-read", "clipboard-write"]);
  await openTool(page);
  await inputBox(page).fill("hello");
  await page.getByRole("button", { name: "Run" }).click();

  await page.getByRole("button", { name: "Copy output" }).click();
  expect(await page.evaluate(() => navigator.clipboard.readText())).toBe("aGVsbG8=");
  await expect(page.getByRole("status")).toContainText("Copied output");

  const downloadPromise = page.waitForEvent("download");
  await page.getByRole("button", { name: "Download output" }).click();
  const download = await downloadPromise;
  expect(download.suggestedFilename()).toBe("encoding-toolbox-base64.txt");
});

test("announces invalid input and supports keyboard operation", async ({ page }) => {
  await openTool(page);
  await page.getByText("Decode", { exact: true }).click();
  await page.getByLabel("Algorithm").selectOption("hex");
  await inputBox(page).fill("abc");
  await page.getByRole("button", { name: "Run" }).focus();
  await page.keyboard.press("Enter");
  await expect(page.getByRole("alert")).toContainText("INVALID_ENCODING");
  await expect(page.getByRole("button", { name: "Run" })).toBeFocused();
});

test("has no unexpected traffic, console problems, or page overflow", async ({ page }) => {
  const problems = [];
  const requests = [];
  page.on("console", (message) => {
    if (["error", "warning"].includes(message.type())) {
      problems.push(`${message.type()}: ${message.text()}`);
    }
  });
  page.on("pageerror", (error) => problems.push(`pageerror: ${error.message}`));
  page.on("request", (request) => requests.push(request.url()));

  await openTool(page);
  await inputBox(page).fill("hello");
  await page.getByRole("button", { name: "Run" }).click();
  await page.waitForLoadState("networkidle");

  const origin = new URL(page.url()).origin;
  expect(requests.every((url) => url.startsWith(origin))).toBe(true);
  expect(problems).toEqual([]);
  expect(await page.evaluate(
    () => document.documentElement.scrollWidth <= document.documentElement.clientWidth,
  )).toBe(true);
});

test("honors reduced-motion preferences", async ({ page }) => {
  await page.emulateMedia({ reducedMotion: "reduce" });
  await openTool(page);
  const duration = await page.getByRole("button", { name: "Run" }).evaluate(
    (button) => Number.parseFloat(getComputedStyle(button).transitionDuration),
  );
  expect(duration).toBeLessThan(0.001);
});
