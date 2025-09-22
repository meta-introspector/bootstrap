{ description = "Bootstrap Layer 3: Builds on Layer 2";

  inputs = {
    layer2.url = "../layer_2";
  };

  outputs = { self, layer2, nixpkgs }: {
    packages.x86_64-linux.hello = layer2.packages.x86_64-linux.hello;
    packages.aarch64-linux.hello = layer2.packages.aarch64-linux.hello;
    defaultPackage.x86_64-linux = self.packages.x86_64-linux.hello;
    defaultPackage.aarch64-linux = self.packages.aarch64-linux.hello;
  };
}
