{ description = "Bootstrap Layer 19: Advanced Self-Replicating Quasi-Meta-Meme (Builds on Layer 17)";

  inputs = {
    layer17.url = "../layer_17";
  };

  outputs = { self, layer17, nixpkgs }: {
    packages.x86_64-linux.hello = layer17.packages.x86_64-linux.hello;
    packages.aarch64-linux.hello = layer17.packages.aarch64-linux.hello;
    defaultPackage.x86_64-linux = self.packages.x86_64-linux.hello;
    defaultPackage.aarch64-linux = self.packages.aarch64-linux.hello;
  };
}
