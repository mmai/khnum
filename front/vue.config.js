module.exports = {
  outputDir: '../static',
  devServer: {
    // historyApiFallback: true, // needed for mode = history in router.js, but I can't make it work
    proxy: {
      '/': {
        target: 'http://127.0.0.1:8000/',
        ws: false, // prevent firefox websockets errors 
      }
    }
  }
}
