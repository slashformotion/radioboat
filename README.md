```
██████   █████  ██████  ██  ██████  ██████   ██████   █████  ████████ 
██   ██ ██   ██ ██   ██ ██ ██    ██ ██   ██ ██    ██ ██   ██    ██    
██████  ███████ ██   ██ ██ ██    ██ ██████  ██    ██ ███████    ██    
██   ██ ██   ██ ██   ██ ██ ██    ██ ██   ██ ██    ██ ██   ██    ██    
██   ██ ██   ██ ██████  ██  ██████  ██████   ██████  ██   ██    ██    
```
[![Open in Visual Studio Code](https://open.vscode.dev/badges/open-in-vscode.svg)](https://open.vscode.dev/slashformotion/radioboat)
![Contributors](https://img.shields.io/github/contributors/slashformotion/radioboat)
![Forks](https://img.shields.io/github/forks/slashformotion/radioboat)
![Stars](https://img.shields.io/github/stars/slashformotion/radioboat)
![Licence](https://img.shields.io/github/license/slashformotion/radioboat)
![Issues](https://img.shields.io/github/issues/slashformotion/radioboat)

**Radioboat is a terminal web radio client, built with simplicity in mind**

## Installation

You need a functional [go setup](https://go.dev/doc/install).

```
git clone https://github.com/slashformotion/radioboat
cd radioboat
go build
go install
```
## How to Use ? 

> please head to the [wiki](https://github.com/slashformotion/radioboat/wiki) for more information  

- Copy the sample [stations.csv](https://github.com/slashformotion/radioboat/blob/master/stations.csv) to ~/.config/radioboat/urls.csv.
- Add the audio stream of your choice and give them a name
- Launch the program:
```bash
radioboat
```
Then, follow the intruction at the bottom of the screen.

![](https://raw.githubusercontent.com/slashformotion/radioboat/master/.github/assets/screencast.gif)

## Dependencies

- [mpv](https://mpv/io) (although this programm could be adapted easily to use other tools capable of reading audio streams)
- Various golang libraries: see [go.mod](https://github.com/slashformotion/radioboat/blob/master/go.mod)


### Contribution Guidelines

The contribution guidelines are as per the guide [HERE](https://github.com/slashformotion/radioboat/blob/master/CONTRIBUTING.md).

## Licence

Copyright 2022 slashformotion

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.