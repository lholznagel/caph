module.exports = {
  devServer: {
    allowedHosts: 'all',
    hot: false,
    liveReload: false,
    port: 1337,
    proxy: {
      '/api': {
        target: 'http://0.0.0.0:10101'
      },
      '/img': {
        target: 'https://images.evetech.net/types',
        pathRewrite: { '^/img': '' }
      }
    },
    webSocketServer: false
  }
}
