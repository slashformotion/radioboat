{
  lib,
  stdenv,
  fetchFromGitHub,
  installShellFiles,
  makeWrapper,
  mpv,
  nix-update-script,
  radioboat,
  rustPlatform,
  testers,
  versionCheckHook,
}:

rustPlatform.buildRustPackage (finalAttrs: {
  pname = "radioboat";
  version = "0.5.0";

  src = fetchFromGitHub {
    owner = "slashformotion";
    repo = "radioboat";
    tag = "v${finalAttrs.version}";
    hash = "sha256-mPktliuWyrXuNzMCdMFZk5Q7lIkRk+y4nX3IBnCc5Mc=";
  };

  __structuredAttrs = true;

  strictDeps = true;

  cargoHash = "sha256-fRF1FvwtvVJSTCK8DcZib6wMLpo73YtV7j+kjt4nVTo=";

  nativeBuildInputs = [
    makeWrapper
    installShellFiles
  ];

  nativeInstallCheckInputs = [ versionCheckHook ];

  preFixup = ''
    wrapProgram $out/bin/radioboat --prefix PATH ":" "${lib.makeBinPath [ mpv ]}";
  '';

  postInstall = lib.optionalString (stdenv.buildPlatform.canExecute stdenv.hostPlatform) ''
    installShellCompletion --cmd radioboat \
      --bash <($out/bin/radioboat completion bash) \
      --fish <($out/bin/radioboat completion fish) \
      --zsh <($out/bin/radioboat completion zsh)
  '';

  doInstallCheck = true;

  meta = {
    description = "Terminal web radio client";
    homepage = "https://github.com/slashformotion/radioboat";
    changelog = "https://github.com/slashformotion/radioboat/releases/tag/${finalAttrs.src.tag}";
    license = lib.licenses.asl20;
    maintainers = with lib.maintainers; [ zendo ];
    mainProgram = "radioboat";
    platforms = lib.platforms.linux;
  };
})
