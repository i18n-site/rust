module.exports = {
  plugins: [
    'mergePaths',
    {
    name: 'preset-default',
    params: {
      overrides: {
        removeViewBox:false,
      },
    },
  }],
};
