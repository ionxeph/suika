const path = require('path');
const CopyPlugin = require('copy-webpack-plugin');

module.exports = {
  entry: './index.js',
  mode: 'production',
  output: {
    path: path.resolve(__dirname, 'dist'),
    filename: './index.js',
  },
  plugins: [
    new CopyPlugin({
      patterns: [
        { from: './index.html', to: './' },
        // { from: './pkg', to: 'pkg' },
        { from: './assets', to: 'assets' },
      ],
    }),
  ],
  experiments: {
    asyncWebAssembly: true,
    syncWebAssembly: true,
  },
  performance: {
    hints: false,
    maxEntrypointSize: 512000,
    maxAssetSize: 512000,
  },
};
