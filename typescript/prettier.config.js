/**
 * @type {import('prettier').Config & import("@ianvs/prettier-plugin-sort-imports").PluginConfig}
 */
export default {
  plugins: ["@ianvs/prettier-plugin-sort-imports"],
  trailingComma: "all",
  arrowParens: "always",
  jsxSingleQuote: false,
  singleQuote: true,
  importOrderTypeScriptVersion: "4.4.0",
  importOrder: ["<THIRD_PARTY_MODULES>", "", "^[./]"],
};
