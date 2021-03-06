const {
  createConfig,
  entryPoint,
  resolve,
  setOutput,
  match,
  typescript,
  css,
  file,
  addPlugins,
  env,
  devServer
} = require('webpack-blocks');

const {
  CleanWebpackPlugin
} = require('clean-webpack-plugin');
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');
const HtmlWebpackPlugin = require('html-webpack-plugin');

const path = require('path');
const distPath = path.resolve(__dirname, "../target/deploy");

module.exports = createConfig([
  entryPoint(['./bootstrap.js', './src/runtime.ts']),
  resolve({
    extensions: ['.wasm']
  }),
  setOutput({
    path: distPath,
    filename: 'coolcats-ui.js',
    webassemblyModuleFilename: 'coolcats-ui.wasm'
  }),
  typescript(),
  css(),
  match(['*.png', '*.jpg', '*.gif'], [
    file({
      name: '[name].[ext]',
    })
  ]),
  addPlugins([
    new CleanWebpackPlugin({
      dry: false,
      verbose: true,
      cleanOnceBeforeBuildPatterns: [distPath],
      dangerouslyAllowCleanPatternsOutsideProject: true,
    }),
    new WasmPackPlugin({
      crateDirectory: ".",
      extraArgs: "--no-typescript",
    }),
    new HtmlWebpackPlugin({
      inject: false,
      template: require('html-webpack-template'),
      filename: 'index.html',
      title: 'Coolcats',
      meta: [{
        name: 'viewport',
        content: 'width=device-width, initial-scale=1, shrink-to-fit=no',
      }],
      links: [{
        rel: 'stylesheet',
        href: 'https://maxcdn.bootstrapcdn.com/bootstrap/3.3.7/css/bootstrap.min.css',
        integrity: 'sha384-BVYiiSIFeK1dGmJRAkycuHAHRg32OmUcww7on3RYdg4Va+PmSTsz/K68vbdEjh4u',
        crossorigin: 'anonymous',
      }],
      favicon: 'src/application/interfaces/images/favicon.png',
      appMountIds: ['holoclient', 'application'],
      scripts: ['coolcats-ui.js'],
      chunks: [],
      minify: false,
    })
  ]),
  env('development', [
    devServer({
      contentBase: distPath,
      port: 8000
    })
  ])
])
