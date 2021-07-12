module.exports = {
  devServer: {
    hot: false,
    liveReload: false,
    inline: false,
    port: 1337,
    disableHostCheck: true,
    proxy: {
      '/api': {
        target: 'http://0.0.0.0:10101'
      },
      '/img': {
        target: 'https://images.evetech.net/types',
        pathRewrite: { '^/img': '' }
      }
    }
  }
}
