{
  lib,
  rustPlatform,
  mpv,
  makeWrapper,
  installShellFiles,
  nix-update-script,
  testers,
  nix-gitignore,
  radioboat,
}:
rustPlatform.buildRustPackage {
  pname = "radioboat";
  version = "0.4.0";

  src = nix-gitignore.gitignoreSource [] ./.;

  cargoLock.lockFile = ./Cargo.lock;

  buildInputs = [mpv];

  nativeBuildInputs = [makeWrapper installShellFiles];

  preFixup = ''
    wrapProgram $out/bin/radioboat --prefix PATH ":" "${lib.makeBinPath [mpv]}";
  '';

  postInstall = ''
    installShellCompletion --cmd radioboat \
      --bash <($out/bin/radioboat completion bash) \
      --fish <($out/bin/radioboat completion fish) \
      --zsh <($out/bin/radioboat completion zsh)
  '';

  passthru = {
    updateScript = nix-update-script {};
    tests.version = testers.testVersion {
      package = radioboat;
      command = "radioboat --version";
    };
  };

  meta = with lib; {
    description = "Radioboat is a terminal web radio client, built with simplicity in mind.";
    mainProgram = "radioboat";
    homepage = "https://github.com/slashformotion/radioboat";
    license = licenses.asl20;
    platforms = platforms.linux ++ platforms.darwin;
  };
}
