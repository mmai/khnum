with import <nixpkgs> { };

stdenv.mkDerivation rec {
  name = "activue-${version}";
  version = "0.1.0";
  buildInputs = with pkgs; [ 
    # rustup
    openssl pkgconfig # needed for installing various cargo packages
    postgresql mysql sqlite # needed for `cargo install diesel_cli`
    docker_compose 
  ];

  # (DATABASE_URL env variable overrides value in .env file)
  # diesel features -> install is global, use the followning command if you have other projects with other db engines on your machine :
    # cargo install diesel_cli 
  shellHook = ''
    export DATABASE_URL=postgres://dbuser:password@localhost:5432/activue
    which diesel >/dev/null 2>&1 || cargo install diesel_cli --no-default-features --features postgres
  '';

}
