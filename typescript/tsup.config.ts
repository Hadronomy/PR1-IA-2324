import { defineConfig } from "tsup";

export default defineConfig({
  entry: ["src/cli.ts"],
  sourcemap: true,
  dts: true,
});
