# Highfleet-Modloader
This is a simple modloader for the game Highfleet

## Using
Simply go to the [latest release](https://github.com/logdot/Highfleet-Modloader/releases/latest)
and download Artifacts.zip, then simply extract into your highfleet folder. Now
run `Highfleet Modloader.exe` to launch your game with mods.

## Adding parts
Adding parts is really simple, simply add the m_oid of your part into the
Shipworks.json file. You can even edit this file while the game is running and
it will be applied the next time you open the Shipworks.
Here is an example Shipworks.json:

```json
{
	"LoadVanilla": true,
	"Hulls": [
		"HULL2x2_1",
		"HULL3x3_1"	
	],
	"Armors": [

	],
	"Fuels": [

	],
	"Ammos": [

	],
	"Legs": [

	],
	"Engines": [
		"MDL_ENGINE_5"
	],
	"Guns": [

	],
	"Sensors": [

	],
	"Expendables": [

	],
	"Crafts": [
		"Custom_Craft_01",
		"Custom_Craft_02",
		"Custom_Craft_03"
	],
	"Defences": [

	],
	"Generators": [

	],
	"Quarters": [

	]
}
```

## Caveats
Please make that all parts that you define inside of Shipworks.json exist in
OL.seria and parts.seria, otherwise your game will crash upon entering the
Shipworks
