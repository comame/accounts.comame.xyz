const { config } = require('@charcoal-ui/tailwind-config')

const opt = {
    darkMode: false,
    content: [ './src/**/*.tsx'],
    presets: [config],
    mode: 'jit',
}

module.exports = opt
