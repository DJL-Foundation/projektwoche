#!/usr/bin/env bun

import { builder } from "./builder.ts";

await builder([
  { name: "Node.js", assert_available: "node" },
  { name: "Bun", assert_available: "bun" },
  { name: "VSCode", assert_available: "code" },
]);
