# Highfleet-Modloader
This is a simple modloader for the game Highfleet.
It can load any DLL into the game, making it very flexible.

## Instalation
The installation process is very simple:
- Go to the [latest release](https://github.com/logdot/Highfleet-Modloader/releases/latest)
- Download Artifacts.zip
- Run `HighfleetModloader.exe` once to have it create it's necessary folders

## Adding Mods
To add a mod all you have to do is drag and drop it into the `Modloader/mods` folder
and any of the mod's configuration files into `Modloader/config`.

## Developing mods
Thanks to the nature of the modloader you can develop almost any DLL and it will be injected
into the game. This means you have almost absolute control over the game.

Currently there isn't much to guide you into developing your own mod, apart from looking
at the source code of another existing mod. Eventually a guide for creating your own
mods is going to be created, but it's not currently available.
Additionally I plan on making a library for mod development that should greatly
ease the development process.

## Available mods
As of right now there is only
- [ShipworksExtended](https://github.com/logdot/ShipworksExtended)
developed by me.

If you create a mod message me and I'll add you to this list