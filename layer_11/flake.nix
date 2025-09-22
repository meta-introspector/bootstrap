{ description = "Bootstrap Layer 11: Builds on Layer 7";

  inputs = {
    layer7.url = "../layer_7";
  };

  outputs = { self, layer7, nixpkgs }: {
    packages.x86_64-linux.hello = layer7.packages.x86_64-linux.hello;
    packages.aarch64-linux.hello = layer7.packages.aarch64-linux.hello;
    defaultPackage.x86_64-linux = self.packages.x86_64-linux.hello;
    defaultPackage.aarch64-linux = self.packages.aarch64-linux.hello;
  };
}
