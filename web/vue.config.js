module.exports = {
  "transpileDependencies": [
    "vuetify"
  ],
  devServer: {
    port: 1337,
    proxy: {
      '/api/v1': {
        target: 'http://localhost:10101'
      }
    }
  }
}