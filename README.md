# Highfleet-Modloader
This is a simple modloader for the game Highfleet.
It can load any DLL into the game, making it very flexible.
However it is intended to be used with custom made highfleet mods.

## Instalation
The installation process is very simple:
- Go to the [latest release](https://github.com/logdot/Highfleet-Modloader/releases/latest)
- Download Artifacts.zip
- Extract the zip file into your highfleet folder
- You should end up with a bass.dll and bass.dll_ in your game folder
- Then launch the game as normal. If you get a black terminal window, you have successfully installed the modloader.

## Uninstalling
If you need to uninstall the modloader, simply:
1. Optionally delete the Modloader folder. This will delete your mods and their configurations.
2. Delete `bass.dll`
3. Rename `bass.dll_` to `bass.dll`
4. You're done!

## Adding modloader Mods
Most mods are designed to be drag and drop into your game folder.

If you need to install a mod manually, drag and drop it into the `Modloader/mods` folder.
If the mod comes with configuration files, drag and drop it into the `Modloader/config` folder.
Note that some mods may auto generate their config files.

## Developing mods
Thanks to the nature of the modloader you can develop almost any DLL and it will be injected into the game.
This means you have almost absolute control over the game from within it's own process.

Mods can optionally define any or all of these functions:

#### `version(*c_char) -> bool`
The modloader passes in the detected version of the game in a null terminated c string pointer.
The versions that can be passed in are:
* `Steam 1.151`
* `Steam 1.163`
* `Gog 1.163`

If your mod implements this function, it should return either true or false depending on if your mod supports the given version.
You **may** choose to not initialize your mod if you do not support the game version.

#### `setup_logger(&Log, LevelFilter)`
The modloader passes in a reference to a logger object and the LevelFilter so that the mod can configure the logger from their end.
Logs will be outputed in the main modloader terminal window.

This function can only be used from Rust based mods using the same compiler version as is used with the modloader.
I'm currently working on a FFI safe implementation which will likely require linking to a shared logging library.

#### `init() -> bool`
The modloader calls this function last.
If you return false, the modloader assumes that you have failed to initialize.

This function is techincally optional as you could do your setup on your library being loaded by windows.

## Available mods

### Direct mods
Direct mods are mods (DLLs) designed to run directly with the modloader and allow to more easily modify the game.
These are mostly intended to be used by mod developers to include in their own mods, and not necessarily end users.
These are all currently developed by me.

- [ShipworksExtended](https://github.com/logdot/ShipworksExtended)
- [Ammo Editor](https://github.com/logdot/ammo-extended)
- [Highfleet QOL](https://github.com/logdot/highfleet-qol)

### Indirect mods
Indirect mods are mods that use direct mods in conjunction with game file modifications to change the game behaviour.
These are intended to be used directly by end users.
These are all developed by other mod developers.

- [Hara Vera](https://www.nexusmods.com/highfleet/mods/16)
- [Ohiofleet](https://www.nexusmods.com/highfleet/mods/40)
- [Vanilla 305MM](https://www.nexusmods.com/highfleet/mods/38)
- [Unoffical Parts Pack](https://discord.gg/unofficial-highfleet-community-735016620719603833)
- [Crabs w/ Guns](https://discord.gg/unofficial-highfleet-community-735016620719603833)

If you create a mod or I haven't added you yet, message me or create an issue.
~~Doing a PR is also welcome.~~
