import { defineConfig } from "vite"

export default defineConfig(({ mode }) => ({
    build: {
        sourcemap: mode == "development" ? "inline" : "hidden",
        rollupOptions: {
            output: {
                minifyInternalExports: mode == "development" ? false : true,
            },
        },
    },
}))
