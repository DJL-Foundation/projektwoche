import config from "prowo-lint/react-internal";
import tseslint from "typescript-eslint";

/** @type {import("eslint").Linter.Config} */
export default tseslint.config(
  ...config,
  {
    languageOptions: {
      parserOptions: {
        projectService: true,
        tsconfigRootDir: import.meta.dirname,
      },
    },
  },
  {
    ignores: ["*.config.js", "dist"],
  },
);
