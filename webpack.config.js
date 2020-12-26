// webpack.config.js
const path = require('path');

const mode = process.env.NODE_ENV || 'development';
const prod = mode === 'production';

const sveltePreprocess = require('svelte-preprocess');
module.exports = {
    entry: {
        bundle: ['./src/main.js']
    },
    resolve: {
        alias: {
            svelte: path.resolve('node_modules', 'svelte')
        },
        extensions: ['.mjs', '.js', '.svelte', '.wasm'],
        mainFields: ['svelte', 'browser', 'module', 'main']
    },
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
        ]
    },
    devtool: prod ? false : 'source-map'
}