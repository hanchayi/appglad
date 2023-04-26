module.exports = {
  reactStrictMode: true,
  transpilePackages: ["@appgald/ui"],
  webpack: function (config, options) {
		config.experiments = { asyncWebAssembly: true, syncWebAssembly: true };
		return config;
	},
};
