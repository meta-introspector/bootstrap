{ description = "Bootstrap Layer 7: Builds on Layer 5";

  inputs = {
    layer5.url = "../layer_5";
  };

  outputs = { self, layer5, nixpkgs }: {
    packages.x86_64-linux.hello = layer5.packages.x86_64-linux.hello;
    packages.aarch64-linux.hello = layer5.packages.aarch64-linux.hello;
    defaultPackage.x86_64-linux = self.packages.x86_64-linux.hello;
    defaultPackage.aarch64-linux = self.packages.aarch64-linux.hello;
  };
}
