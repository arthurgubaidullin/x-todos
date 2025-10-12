import { defineConfig } from "tsdown";

export default defineConfig({
  entry: ["./src/index.ts", "./src/web-client.ts", "./src/vue-client.ts"],
  dts: { build: true, resolve: [/^@x-todos\//] },
  external: ["vue"],
  noExternal: [/^@x-todos\//],
  format: "esm",
});
