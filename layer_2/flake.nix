{ description = "Bootstrap Layer 2: Builds on Layer 1";

  inputs = {
    layer1.url = "../layer_1";
  };

  outputs = { self, layer1, nixpkgs }: {
    packages.x86_64-linux.hello = layer1.packages.x86_64-linux.hello;
    packages.aarch64-linux.hello = layer1.packages.aarch64-linux.hello;
    defaultPackage.x86_64-linux = self.packages.x86_64-linux.hello;
    defaultPackage.aarch64-linux = self.packages.aarch64-linux.hello;
  };
}
