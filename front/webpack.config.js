const path = require('path')

module.exports = {
    entry: {
        'signin': './src/signin/signin.tsx',
        'reauthenticate': './src/signin/reauthenticate.tsx',
        'confirm': './src/signin/confirm.tsx',
        'dash': './src/dash/dash.tsx',
    },
    mode: 'development',
    devtool: 'source-map',
    output: {
        path: path.resolve(__dirname, '../static/front/'),
        filename: '[name].js',
        assetModuleFilename: '[name][ext]'
    },
    resolve: {
        extensions: [ '.js', '.ts', '.tsx', '.json' ]
    },
    module: {
        rules: [{
            test: /\.(tsx|ts)$/,
            use: 'ts-loader'
        }, {
            test: /\.svg$/u,
            type: 'asset/resource',
        }]
    }
}
