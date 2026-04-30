import js from "@eslint/js";
import reactHooks from "eslint-plugin-react-hooks";
import tseslint from "typescript-eslint";

const browserGlobals = {
  document: "readonly",
  HTMLElement: "readonly",
  window: "readonly",
};

const nodeGlobals = {
  process: "readonly",
};

export default tseslint.config(
  {
    ignores: [
      "dist/**",
      "node_modules/**",
      "playwright-report/**",
      "src-tauri/gen/**",
      "src-tauri/target/**",
      "test-results/**",
    ],
  },
  js.configs.recommended,
  ...tseslint.configs.recommended,
  {
    files: ["src/**/*.{ts,tsx}", "tests-e2e/**/*.ts", "*.config.ts"],
    languageOptions: {
      ecmaVersion: "latest",
      globals: {
        ...browserGlobals,
        ...nodeGlobals,
      },
    },
    plugins: {
      "react-hooks": reactHooks,
    },
    rules: {
      ...reactHooks.configs.recommended.rules,
    },
  },
  {
    files: ["*.config.js", "eslint.config.js"],
    languageOptions: {
      ecmaVersion: "latest",
      sourceType: "module",
      globals: nodeGlobals,
    },
  },
);
