{ description = "Bootstrap Layer 1: Builds on Layer 0";

  inputs = {
    layer0.url = "../layer_0";
  };

  outputs = { self, layer0, nixpkgs }: {
    packages.x86_64-linux.hello = layer0.packages.x86_64-linux.hello;
    packages.aarch64-linux.hello = layer0.packages.aarch64-linux.hello;
    defaultPackage.x86_64-linux = self.packages.x86_64-linux.hello;
    defaultPackage.aarch64-linux = self.packages.aarch64-linux.hello;
  };
}
