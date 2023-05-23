module.exports = function (context, options) {
  return {
    name: 'wasm-docusuarus-plugin',
    // eslint-disable-next-line
    configureWebpack(config, isServer, utils) {
      return {
        experiments: {
          asyncWebAssembly: true,
        },
        node: {
          __dirname: true
        }      
      };
    },
  };
};
