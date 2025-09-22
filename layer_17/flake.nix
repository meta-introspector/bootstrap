{ description = "Bootstrap Layer 17: Builds on Layer 13";

  inputs = {
    layer13.url = "../layer_13";
  };

  outputs = { self, layer13, nixpkgs }: {
    packages.x86_64-linux.hello = layer13.packages.x86_64-linux.hello;
    packages.aarch64-linux.hello = layer13.packages.aarch64-linux.hello;
    defaultPackage.x86_64-linux = self.packages.x86_64-linux.hello;
    defaultPackage.aarch64-linux = self.packages.aarch64-linux.hello;
  };
}
