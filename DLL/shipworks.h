#pragma once
#include <basetsd.h>

typedef DWORD64 pointer;

static const DWORD64 EditorParts = 0x143942568;

enum EditorLibraries : DWORD64 {
	HullLibrary = 0x14038d420,
	ArmorLibrary = 0x14038d438,
	FuelLibrary = 0x14038d450,
	AmmoLibrary = 0x14038d468,
	LegLibrary = 0x14038d480,
	EngineLibrary = 0x14038d498,
	GunLibrary = 0x14038d4b0,
	SensorLibrary = 0x14038d4c8,
	ExpendablesLibrary = 0x14038d4e0,
	CraftLibrary = 0x14038d4f8,
	DefencesLibrary = 0x14038d510,
	GeneratorLibrary = 0x14038d528,
	QuartersLibrary = 0x14038d540,
	EmergencyLibrary = 0x14038d558
};

extern "C" {
	void DefinePart(pointer editorParts, const char* moid, pointer editorLibrary);
	void CreateLibrary(EditorLibraries library, const char* libraryName);
}

void CreateLibraries();
void DefineVanillaParts();