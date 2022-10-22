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
        filename: '[name].js'
    },
    resolve: {
        extensions: [ '.js', '.ts', '.tsx', '.json' ]
    },
    module: {
        rules: [{
            test: /\.(tsx|ts)$/,
            use: 'ts-loader'
        }, {
            test: /\.html$/,
            use: [{
                loader: 'file-loader',
                options: {
                    name: '[name].html'
                }
            }]
        }, {
            test: /\.scss$/,
            use: [{
                loader: 'style-loader'
            }, {
                loader: 'css-loader',
                options: {
                    sourceMap: true,
                    url: false
                }
            }, {
                loader: 'sass-loader',
                options: {
                    sourceMap: true
                }
            }]
        }, {
            test: /assets\//,
            use: [{
                loader: 'file-loader',
                options: {
                    name: '[name].[ext]'
                }
            }]
        }]
    }
}
