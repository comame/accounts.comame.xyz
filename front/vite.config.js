import { defineConfig } from "vite"
import { resolve } from "node:path"

export default defineConfig(({ mode }) => ({
    base: "/front",
    build: {
        outDir: "../static/front",
        emptyOutDir: true,
        sourcemap: mode == "development" ? "inline" : "hidden",
        rollupOptions: {
            input: {
                signin: resolve(__dirname, "./src/signin.html"),
            },
        },
    },
}))
