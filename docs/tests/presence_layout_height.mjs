import { chromium } from "playwright";

const url =
  process.env.PRESENCE_DOCS_URL ??
  "http://127.0.0.1:8080/dioxus-motion/docs/presence";

const browser = await chromium.launch({ headless: true });
const page = await browser.newPage({
  viewport: { width: 1600, height: 1000 },
  deviceScaleFactor: 1,
});

const pageErrors = [];
page.on("pageerror", (error) => {
  pageErrors.push(String(error.stack || error.message || error));
});

try {
  await page.goto(url, { waitUntil: "networkidle", timeout: 90_000 });
  await page.waitForTimeout(500);

  const before = await page.evaluate(() => {
    const sections = [...document.querySelectorAll("section")].filter((candidate) => {
      const heading = candidate.querySelector("h3")?.textContent ?? "";
      return (
        heading.includes("Notification Stack With Stable Keys") ||
        heading.includes("Lists Need Stable Keys") ||
        heading.includes("Incident Queue With Stable Keys")
      );
    });
    const section = sections.at(-1);

    if (!section) {
      return { ok: false, reason: "stable-key presence section not found" };
    }

    const layoutRows = () =>
      [...section.querySelectorAll('div[style*="box-sizing: border-box"]')]
        .filter((candidate) => candidate.textContent?.trim())
        .map((candidate) => {
          const rect = candidate.getBoundingClientRect();
          const style = getComputedStyle(candidate);
          return {
            text: (candidate.textContent ?? "").replace(/\s+/g, " ").trim(),
            height: rect.height,
            transition: style.transition,
            inlineStyle: candidate.getAttribute("style") ?? "",
          };
        });

    const beforeRows = layoutRows();
    return {
      ok: true,
      beforeRows,
      beforeTexts: beforeRows.map((row) => row.text),
    };
  });

  if (!before.ok) {
    throw new Error(String(before.reason));
  }

  const pushButton = page
    .getByRole("button", { name: /Push toast|Add item/i })
    .first();
  await pushButton.click({ force: true });

  const result = await page.evaluate(async (beforeTextsList) => {
    const sections = [...document.querySelectorAll("section")].filter((candidate) => {
      const heading = candidate.querySelector("h3")?.textContent ?? "";
      return (
        heading.includes("Notification Stack With Stable Keys") ||
        heading.includes("Lists Need Stable Keys") ||
        heading.includes("Incident Queue With Stable Keys")
      );
    });
    const section = sections.at(-1);

    if (!section) {
      return { ok: false, reason: "stable-key presence section not found" };
    }

    const layoutRows = () =>
      [...section.querySelectorAll('div[style*="box-sizing: border-box"]')]
        .filter((candidate) => candidate.textContent?.trim())
        .map((candidate) => {
          const rect = candidate.getBoundingClientRect();
          const style = getComputedStyle(candidate);
          return {
            text: (candidate.textContent ?? "").replace(/\s+/g, " ").trim(),
            height: rect.height,
            transition: style.transition,
            inlineStyle: candidate.getAttribute("style") ?? "",
          };
        });

    const beforeTexts = new Set(beforeTextsList);

    const startedAt = performance.now();
    const samples = [];
    for (const targetMs of [0, 16, 33, 50, 75, 100, 150, 220, 320, 460]) {
      while (performance.now() - startedAt < targetMs) {
        await new Promise((resolve) => requestAnimationFrame(resolve));
      }

      const rows = layoutRows();
      const inserted =
        rows.find((row) => !beforeTexts.has(row.text)) ??
        rows.find((row) => /height:\s*(0|[1-9]\d*(\.\d+)?)px/.test(row.inlineStyle)) ??
        rows[rows.length - 1];

      if (inserted) {
        samples.push({
          at: Math.round(performance.now() - startedAt),
          height: inserted.height,
          transition: inserted.transition,
          inlineStyle: inserted.inlineStyle,
          text: inserted.text,
        });
      }
    }

    const heights = samples.map((sample) => sample.height);
    const finalHeight = Math.max(...heights);
    const hasCollapsedFrame = heights.some((height) => height <= 2);
    const hasIntermediateFrame = heights.some(
      (height) => height > 2 && height < finalHeight - 2,
    );
    const hasHeightTransition = samples.some((sample) =>
      sample.transition.includes("height"),
    );

    return {
      ok: hasCollapsedFrame && hasIntermediateFrame && hasHeightTransition,
      reason: {
        hasCollapsedFrame,
        hasIntermediateFrame,
        hasHeightTransition,
        beforeCount: beforeTextsList.length,
        afterCount: layoutRows().length,
      },
      samples,
    };
  }, before.beforeTexts);

  if (pageErrors.length > 0) {
    throw new Error(`page errors:\n${pageErrors.join("\n")}`);
  }

  if (!result.ok) {
    throw new Error(
      `layout height animation regression:\n${JSON.stringify(result, null, 2)}`,
    );
  }

  console.log(JSON.stringify(result, null, 2));
} finally {
  await browser.close();
}
