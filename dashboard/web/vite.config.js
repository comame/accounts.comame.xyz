import { defineConfig } from 'vite'

export default defineConfig(({ mode }) => ({
    base: '/dash',
    build: {
        sourcemap: mode == 'development' ? 'inline' : 'hidden'
    }
}))
