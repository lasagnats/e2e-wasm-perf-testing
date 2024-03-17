const puppeteer = require("puppeteer");

const execCount = 60;

//  Performance measuring script

(async () => {
  await runTestSuite(1000);
  await runTestSuite(10000);
})();

// TODO: check if args are needed
async function runTestSuite(dataSize) {
  const browser = await puppeteer.launch({
    headless: false,
    args: [
      "--disable-web-security",
      "--disable-features=IsolateOrigins",
      "--disable-site-isolation-trials",
    ],
  });
  const page = await browser.newPage();

  let timeStorage = [];

  await page.goto("http://127.0.0.1:8087/");
  await page.waitForSelector(`#entry-count`, { visible: true });

  await page.$eval(`#entry-count`, (input) => (input.value = ""));
  await page.type(`#entry-count`, dataSize + "");

  for (let i = 0; i < execCount; i++) {
    let res = await runTest(page, dataSize);
    console.log("Finished running the test");
    timeStorage[i] = { ...res, ...timeStorage[i] };
  }

  console.log(`Results:`);
  console.log(JSON.stringify(timeStorage));

  await browser.close();
  // TODO: write results to an output file
}

async function runTest(page, dataSize) {
  const initialHeapSize = (await page.metrics()).JSHeapUsedSize;

  await page.click(`#clear`);

  const startTime = performance.now();

  await page.click(`#generate-data`);

  // await page.waitForSelector(`#status-message`, { visible: true });

  await page.evaluate(
    (dataSize) => document.querySelector("ul").childNodes.length === dataSize
  );

  const endTime = performance.now();

  const time = endTime - startTime;

  const performanceMetrics = await page.metrics();
  const heapSize = performanceMetrics.JSHeapUsedSize;
  // In order to convert to megabytes
  // const heapMB = performanceMetrics.JSHeapUsedSize / (1024 * 1024);
  const deltaHeapSize = performanceMetrics.JSHeapUsedSize - initialHeapSize;
  const testRunResult = {
    heapSize,
    deltaHeapSize,
    time,
  };

  return testRunResult;
}
