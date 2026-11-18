{
  description = "The Messenger Nexus research prototype, beginning with its typed core.";

  inputs.base.url = "github:LiGoldragon/build-base";
  inputs.ethos-zero.url = "github:LiGoldragon/ethos-zero";

  outputs = { self, base, ethos-zero }:
    base.lib.workspaceOutputs {
      cargoNix = ./Cargo.nix;
      extraChecks = { pkgs, ... }: {
        ethos-generated = pkgs.runCommand "messenger-ethos-generated" {
          compiler = ethos-zero.packages.${pkgs.stdenv.hostPlatform.system}.ethos-zero;
          source = ./crates/core;
        } ''
          mkdir generated
          for file in "$source"/ethos/*.ethos; do
            "$compiler/bin/ethos-zero" "Check.$file"
            "$compiler/bin/ethos-zero" "Generate.{ $file $PWD/generated }"
            name=$(basename "$file" .ethos)
            cmp "$source/src/generated/$name.rs" "generated/$name.rs"
          done
          touch "$out"
        '';
      };
    };
}
