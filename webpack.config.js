// webpack.config.js
const path = require('path');
const MiniCssExtractPlugin = require('mini-css-extract-plugin');


const mode = process.env.NODE_ENV || 'development';
const prod = mode === 'production';
const sveltePreprocess = require('svelte-preprocess');

module.exports = {
    mode: mode,
    entry: {
        bundle: ['./src/main.js']
    },
    output: {
        path: __dirname + '/public/build',
        filename: '[name].js',
        chunkFilename: '[name].[id].js'
    },
    resolve: {
        alias: {
            svelte: path.resolve('node_modules', 'svelte')
        },
        extensions: ['.mjs', '.js', '.svelte', '.wasm'],
        mainFields: ['svelte', 'browser', 'module', 'main']
    },
    // webpack 5
    // experiments: {
    //     syncWebAssembly: true,
    //     asyncWebAssembly: true,
    // },
    module: {
        rules: [
            {
                test: /\.svelte$/,
                use: {
                    loader: 'svelte-loader',
                    options: {
                        emitCss: true,
                        hotReload: true,
                        preprocess: sveltePreprocess({
                            // https://github.com/kaisermann/svelte-preprocess/#user-content-options
                            sourceMap: !prod,
                            postcss: {
                                plugins: [
                                    require("tailwindcss"),
                                    require("autoprefixer"),
                                    require("postcss-nesting")
                                ],
                            },
                        }),
                    }
                }
            },
            {
                test: /\.wasm$/,
                loaders: ['base64-loader'],
                type: 'javascript/auto',
            },
            {
                test: /\.css$/,
                use: [
                    /**
                     * MiniCssExtractPlugin doesn't support HMR.
                     * For developing, use 'style-loader' instead.
                     * */
                    prod ? MiniCssExtractPlugin.loader : 'style-loader', {
                        loader: 'file-loader',
                        options: { name: 'bundle.css', publicPath: '/build' }
                    }
                ]
            }
        ]
    },
    plugins: [
        new MiniCssExtractPlugin({
            filename: '[name].css'
        })
    ],
    devServer: {
        hot: true,
        contentBase: path.resolve(__dirname, 'public/build'),
        publicPath: '/build',
        historyApiFallback: true
    },
    devtool: prod ? false : 'source-map'
}