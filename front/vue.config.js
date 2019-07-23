module.exports = {
  outputDir: '../static',
  devServer: {
    proxy: {
      '/': {
        target: 'http://127.0.0.1:8000/',
        ws: false, // prevent firefox websockets errors 
      }
    }
  }
}
