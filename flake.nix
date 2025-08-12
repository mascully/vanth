{
    inputs = {
        nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
        crane.url = "github:ipetkov/crane";
        flake-parts.url = "github:hercules-ci/flake-parts";
        rust-overlay.url = "github:oxalica/rust-overlay";
    };

    outputs =
        inputs:
        inputs.flake-parts.lib.mkFlake { inherit inputs; } {
            systems = [
                "aarch64-linux"
                "aarch64-darwin"
                "x86_64-darwin"
                "x86_64-linux"
            ];

            perSystem =
                { system, ... }:
                let
                    pkgs = import inputs.nixpkgs {
                        inherit system;
                        overlays = [ inputs.rust-overlay.overlays.default ];
                    };
                    rustToolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
                    craneLib = (inputs.crane.mkLib pkgs).overrideToolchain rustToolchain;

                    packages =
                        with pkgs;
                        [
                            dbus
                            libssh2
                            makeWrapper
                            openssl
                            openssl.dev
                            pkg-config
                        ]
                        ++ lib.optionals stdenv.isLinux [
                            systemd
                            patchelf
                        ]
                        ++ lib.optionals stdenv.isDarwin [
                            libiconv
                            darwin.apple_sdk.frameworks.Security
                            darwin.apple_sdk.frameworks.SystemConfiguration
                            darwin.apple_sdk.frameworks.AudioUnit
                            darwin.apple_sdk.frameworks.CoreAudio
                        ];

                    commonArgs = {
                        src = ./.;
                        strictDeps = true;
                        nativeBuildInputs = packages;
                        buildInputs = packages;
                        checkPhase = "";

                        CARGO_BUILD_INCREMENTAL = "false";
                        RUST_BACKTRACE = "1";
                        LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath packages;
                    };

                    craneArgs = {
                    };

                    cargoArtifacts = craneLib.buildDepsOnly (
                        commonArgs
                        // craneArgs
                        // {
                            pname = "vanth-deps";
                            version = "0.1.0";
                        }
                    );

                    vanth = craneLib.buildPackage (
                        commonArgs
                        // craneArgs
                        // {
                            inherit cargoArtifacts;
                            pname = "vanth";
                            version = "0.1.0";
                            cargoExtraArgs = "--package vanth_cli";

                            postInstall = ''
                                if [ ! -f "$out/bin/vanth" ]; then
                                  echo "Error: vanth binary not found in $out/bin/"
                                  ls -la $out/bin/
                                  exit 1
                                fi
                            ''
                            + pkgs.lib.optionalString pkgs.stdenv.isLinux ''
                                patchelf $out/bin/vanth --set-rpath ${pkgs.lib.makeLibraryPath packages}
                                wrapProgram $out/bin/vanth --set LD_LIBRARY_PATH "${pkgs.lib.makeLibraryPath packages}"
                            '';
                        }
                    );
                in
                {
                    packages = rec {
                        inherit vanth;
                        default = vanth;
                    };

                    devShells.default = craneLib.devShell {
                        inputsFrom = [ vanth ];
                        packages = packages;
                        LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath packages;
                    };
                };
        };
}
