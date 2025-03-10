// Migrations are an early feature. Currently, they're nothing more than this
// single deploy script that's invoked from the CLI, injecting a provider
// configured from the workspace's Anchor.toml.

const anchor = require("@coral-xyz/anchor");

module.exports = async function () {
  // Configure client to use the provider.
  const provider = anchor.AnchorProvider.local(('https://devnet.helius-rpc.com/?api-key=c9b1e260-a8ab-479e-be6b-a9989f1b6df6'));
  anchor.setProvider(provider);

  // Add your deploy script here.
};
