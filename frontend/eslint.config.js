import js from "@eslint/js";
import globals from "globals";
import reactHooks from "eslint-plugin-react-hooks";
import reactRefresh from "eslint-plugin-react-refresh";
import tseslint from "typescript-eslint";
import plugin from "eslint-plugin-react";
import { globalIgnores } from "eslint/config";

export default tseslint.config([
  globalIgnores(["dist"]),
  {
    files: ["src/**/*.{ts,tsx}"],

    extends: [
      js.configs.recommended,
      tseslint.configs.recommended,
      tseslint.configs.recommendedTypeChecked,
      tseslint.configs.stylisticTypeChecked,
      reactHooks.configs["recommended-latest"],
      reactRefresh.configs.vite,
      plugin.configs.flat.recommended,
    ],

    languageOptions: {
      ecmaVersion: 2020,
      globals: globals.browser,
      parserOptions: {
        projectService: true,
        tsconfigRootDir: import.meta.dirname,
        ecmaFeatures: {
          jsx: true,
        },
      },
    },

    settings: {
      react: {
        version: "detect",
      },
    },

    rules: {
      "prefer-const": "error",
      "no-unassigned-vars": "error",
      "no-self-compare": "error",
      "no-useless-assignment": "error",
      "arrow-body-style": ["error", "as-needed"],
      "block-scoped-var": "error",
      camelcase: ["error", { properties: "always" }],
      curly: "error",
      eqeqeq: "error",
      "no-console": ["error", { allow: ["warn", "error"] }],
      "no-undefined": "error",
      "no-var": "error",
      "@typescript-eslint/array-type": ["error", { default: "generic" }],

      "react/jsx-uses-react": "off",
      "react/react-in-jsx-scope": "off",
    },
  },
]);
