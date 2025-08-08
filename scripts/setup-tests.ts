#!/usr/bin/env bun

enum TestBundle {
  Projektwoche = "projektwoche",
}

async function runTest(bundle: TestBundle): Promise<void> {
  switch (bundle) {
    case TestBundle.Projektwoche:
      await import("./setup-tests/projektwoche.ts");
      break;
    default:
      throw new Error(`Unknown bundle: ${bundle}`);
  }
}

async function main(): Promise<void> {
  const args = process.argv.slice(2);

  if (args.length === 0) {
    console.error("Usage: bun setup-tests.ts <bundle>");
    console.error("Available bundles:", Object.values(TestBundle).join(", "));
    process.exit(1);
  }

  const bundleName = args[0];

  if (!Object.values(TestBundle).includes(bundleName as TestBundle)) {
    console.error(`Invalid bundle: ${bundleName}`);
    console.error("Available bundles:", Object.values(TestBundle).join(", "));
    process.exit(1);
  }

  const bundle = bundleName as TestBundle;

  try {
    await runTest(bundle);
  } catch (error) {
    console.error("Test failed:", error);
    process.exit(1);
  }
}

main();
