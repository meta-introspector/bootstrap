{ description = "Bootstrap Layer 5: Builds on Layer 3";

  inputs = {
    layer3.url = "../layer_3";
  };

  outputs = { self, layer3, nixpkgs }: {
    packages.x86_64-linux.hello = layer3.packages.x86_64-linux.hello;
    packages.aarch64-linux.hello = layer3.packages.aarch64-linux.hello;
    defaultPackage.x86_64-linux = self.packages.x86_64-linux.hello;
    defaultPackage.aarch64-linux = self.packages.aarch64-linux.hello;
  };
}
