{
  lib,
  buildGoModule,
  mpv,
  makeWrapper,
  installShellFiles,
  nix-update-script,
  testers,
  nix-gitignore,
  radioboat,
}:
buildGoModule rec {
  pname = "radioboat";
  version = "0.3.0";

  src = nix-gitignore.gitignoreSource [] ./.;

  vendorHash = "sha256-PYO1ZhbNpzN5AJMNo4odSJWufVeRTpUC6i9ZAJryRJo=";

  buildInputs = [mpv];

  ldflags = [
    "-s"
    "-w"
    "-X github.com/slashformotion/radioboat/internal/buildinfo.Version=${version}"
  ];

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
      command = "radioboat version";
    };
  };

  meta = with lib; {
    description = "Terminal web radio client";
    mainProgram = "radioboat";
    homepage = "https://github.com/slashformotion/radioboat";
    license = licenses.asl20;
    platforms = platforms.linux;
  };
}
