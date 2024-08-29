# 3.0
Full overhaul of the modloader.

The following changes are planned:
[] Changing from C++ to Rust.
[] Using DLL proxying.
[] Better integration with mods
[] Version handling

The most noticable change from a user's perspective is that they do not need to run a separate executable.
They will just need to launch the game, which itself will load the proxied DLL without us having to do anything on our end.
This hopefully will also stop AVs from flagging the modloader.

# Highfleet-Modloader
This is a simple modloader for the game Highfleet.
It can load any DLL into the game, making it very flexible.

It is version independent and is technically not limited on working with just Highfleet.
In essence it is a simple [DLL injector](https://en.wikipedia.org/wiki/DLL_injection).
That means that you might need to run the modloader in administrator mode, or that your AV might flag it.

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

### Direct mods
Direct mods are mods (DLLs) designed to run directly with the modloader and allow to more easily modify the game.
These are intended to be used by mod developers, and not necessarily end users.
These are currently all developed by me.

- [ShipworksExtended](https://github.com/logdot/ShipworksExtended)
- [Ammo Editor](https://github.com/logdot/ammo-extended)
- [Anti Wobble](https://github.com/logdot/anti-wobble)

### Indirect mods
Indirect mods are mods that use the modloader and direct mods in conjunction to change the game behaviour.
These are intended to be used directly by end users.
These are developed by other mod developers

- [Hara Vera](https://www.nexusmods.com/highfleet/mods/16)
- [Ohiofleet](https://www.nexusmods.com/highfleet/mods/40)
- [Vanilla 305MM](https://www.nexusmods.com/highfleet/mods/38)
- [Unoffical Parts Pack](https://discord.gg/unofficial-highfleet-community-735016620719603833)
- [Crabs w/ Guns](https://discord.gg/unofficial-highfleet-community-735016620719603833)

If you create a mod or haven't added you yet, message me or create an issue.
~~Doing a PR is also welcome.~~
