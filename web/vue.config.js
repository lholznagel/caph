module.exports = {
  "transpileDependencies": [
    "vuetify"
  ],
  devServer: {
    hot: false,
    liveReload: false,
    port: 1337,
    disableHostCheck: true,
    proxy: {
      '/api': {
        target: 'http://0.0.0.0:10101'
      }
    }
  }
}
