{ description = "Bootstrap Layer 13: Builds on Layer 11";

  inputs = {
    layer11.url = "../layer_11";
  };

  outputs = { self, layer11, nixpkgs }: {
    packages.x86_64-linux.hello = layer11.packages.x86_64-linux.hello;
    packages.aarch64-linux.hello = layer11.packages.aarch64-linux.hello;
    defaultPackage.x86_64-linux = self.packages.x86_64-linux.hello;
    defaultPackage.aarch64-linux = self.packages.aarch64-linux.hello;
  };
}
