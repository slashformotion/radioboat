# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html)

## Unreleased

### Added

- Added a retry mechanism for the mpv socket connection using an exponential backoff. That way we are sure that mpv is ready to accept connections when creating the ipcclient.

## [v0.2.2] - 04/08/2022

## Added

- A message box: You can now see when you saved the current song name to the [Trackfile](https://github.com/slashformotion/radioboat/wiki/trackfile). If an error happened during that process, it would be logged to the message box.

## Fixed

- Now you can't save the same song/track name multiple times to the [Trackfile](https://github.com/slashformotion/radioboat/wiki/trackfile).

### Changed

- 58065ac chore: update gitignore
- 5c7ed95 feat(cosmetics): add a message box for errors and logs
- 48be3de fix: Fix multiples track names save bug

## [v0.2.1] - 08/07/2022

## Fixed

- #7 (at 8623afc): Now versions are set at build time

## Changed

- b38bc0a Merge pull request #8 from slashformotion/fix/fix-version-flag
- 2d526ce Update issue templates
- b271c69 chore(deps): go mod tidy
- 40905bc chore(goreleaser): update linker args
- 3741078 chore: add makefile
- 8623afc fix(cmd): versions are now set at build time
- a0fd6cc fix(goreleaser): fix build config

## [v0.2.0] - 06/07/2022

## Added

- Added a keybinding `Ctrl+s` to save the current track name to a text file  (Please see [the wiki page](https://github.com/slashformotion/radioboat/wiki/trackfile))

## Changed

- 9cbcb46 Merge pull request #6 from slashformotion/feat-makeover
- 41caeda build: adding new dep: github.com/manifoldco/promptui
- e3f99ec chores: add licence to golang files
- 1fdd119 chores: prepare for v0.2.0
- eb3f1df feat: save trackname to a file
- b4ec3ee refactor(cosmetics): Refactor header

## [v0.1.2] - 02/07/2022

### Fixed

- 3258da8 and  383274f fix errors handling

## Changed

- 10e8d1e Merge branch 'fix-check-url-file-existence'
- d6d53fb chores: update wiki
- 3258da8 fix(cmd): now handle gracefully the error returned by Get_player
- 383274f fix(parsing): parser now handle errors gracefully instead of panicking

## [v0.1.1] - 01/07/2022

### Fixed

- 73adcfa mpv startup time is variable so Radioboat now wait 400ms to be sure that mpv is ready do respond on the pipe. This is a momentary fix.
- 7255852 The track name is now in the header, if the track name is provided by the stream producer. 
- bd1fca7 Prepare the player mechanism in case someone want to add support for  another player. 

### changed

- 12c8f69 add gitmodule radioboat.wiki.git
- cebdb91 chore: add AUR link and fix typos
- cfb74cf chore: change go install intructions
- bd1fca7 feat(cmd): add "player" flag and player provider mechanism
- 7255852 feat(cmd): add track name in header
- a715487 feat: implement fully the RadioPlayer interface in MpvPlayer
- 73adcfa fix: increase waiting time at startup for mpv to get ready
- 4047f16 refactor: Modify the reader interface

## [v0.1.0] - 30/06/2022

### Changed

- 941eca1 add goreleaser config
- 8ac1c2a build: build go.mod
- 9f5d4eb build: upd goreleaser file and .gitignore
- 10475b5 chore: add licence
- fb87ec5 chore: add readme
- 7d219f6 chore: fix broken link in readme
- f39d5cc chore: fix small typo and one image broken link
- 24abdfc chore: update screencast and add wiki link to readme
- 16a6c37 chores: add contributing.md
- 4a8bdea chores: add gitignore
- cfd20b9 chores: move screencast to the to of readme
- 1e47a48 feat(cmd): add new flag "volume" for root command
- e64b10a feat(cosmetics): add a quite good looking header
- 03bea98 feat: First commit
- 0550db9 feat: make the edit command great (see commit description)
- 5b3db75 fix(parsing): fix trailing whitespace issue


[unreleased]: https://github.com/slashformotion/radioboat/blob/master/changelog.md#unreleased
[v0.2.2]: https://github.com/slashformotion/radioboat/blob/v0.2.2/changelog.md#unreleased
[v0.2.1]: https://github.com/slashformotion/radioboat/blob/v0.2.1/changelog.md#unreleased
[v0.2.0]: https://github.com/slashformotion/radioboat/blob/v0.2.0/changelog.md#unreleased
[v0.1.2]: https://github.com/slashformotion/radioboat/blob/v0.1.2/changelog.md#unreleased
[v0.1.1]: https://github.com/slashformotion/radioboat/blob/v0.1.1/changelog.md#unreleased
[v0.1.0]: https://github.com/slashformotion/radioboat/blob/v0.1.0/changelog.md#unreleased