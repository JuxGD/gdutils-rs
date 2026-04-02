{
  stdenv
, lib
, rustPlatform
, fetchFromGitHub
, openssl
, pkg-config
}:

rustPlatform.buildRustPackage (finalAttrs: {
  pname = "rider";
  version = "0.0.0";
  src = "./src";
})
