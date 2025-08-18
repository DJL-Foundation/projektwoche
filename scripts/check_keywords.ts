#!/usr/bin/env bun

// check_keywords.ts
// This script checks a log file for a list of required keywords.
// It returns an exit code of 0 if all keywords are found, and 1 otherwise.

import { readFileSync, existsSync } from "fs";

const logFile = process.argv[2];
const keywordsJson = process.argv[3];

// Check if the log file exists
if (!logFile) {
  console.error("❌ Usage: bun check_keywords.ts <LOG_FILE> <KEYWORDS_JSON>");
  process.exit(1);
}

if (!existsSync(logFile)) {
  console.error(`❌ Log file not found: ${logFile}`);
  process.exit(1);
}

if (!keywordsJson) {
  console.error("❌ Keywords JSON argument is required");
  process.exit(1);
}

console.log(`Checking for expected keywords in ${logFile}...`);

let missingKeywords = 0;

try {
  // Parse the keywords JSON array
  const keywords: string[] = JSON.parse(keywordsJson);

  // Read the log file content
  const logContent = readFileSync(logFile, "utf-8");

  // Iterate over the keywords
  for (const keyword of keywords) {
    if (!logContent.includes(keyword)) {
      console.log(`❌ Missing keyword: ${keyword}`);
      missingKeywords++;
    } else {
      console.log(`✅ Found keyword: ${keyword}`);
    }
  }
} catch (error) {
  console.error(`❌ Error parsing keywords JSON: ${error}`);
  process.exit(1);
}

// If any keywords were missing, exit with an error code
if (missingKeywords > 0) {
  console.log("❌ Missing keywords found. Full log content:");
  console.log(readFileSync(logFile, "utf-8"));
  process.exit(1);
} else {
  console.log("✅ All expected keywords found in log!");
  process.exit(0);
}
