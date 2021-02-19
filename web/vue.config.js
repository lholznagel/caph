module.exports = {
  "transpileDependencies": [
    "vuetify"
  ],
  devServer: {
    port: 1337,
    disableHostCheck: true,
    proxy: {
      '/api': {
        target: 'http://localhost:10101'
      }
    }
  }
}
