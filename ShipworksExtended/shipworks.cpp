#include "pch.h"
#include "shipworks.h"

void CreateLibraries() {
    CreateLibrary(HullLibrary, "EDITOR_LIBRARY_HULL");
    CreateLibrary(ArmorLibrary, "EDITOR_LIBRARY_ARMOR");
    CreateLibrary(FuelLibrary, "EDITOR_LIBRARY_FUEL");
    CreateLibrary(AmmoLibrary, "EDITOR_LIBRARY_AMMO");
    CreateLibrary(LegLibrary, "EDITOR_LIBRARY_LEG");
    CreateLibrary(EngineLibrary, "EDITOR_LIBRARY_ENGINE");
    CreateLibrary(GunLibrary, "EDITOR_LIBRARY_GUNS");
    CreateLibrary(SensorLibrary, "EDITOR_LIBRARY_SENSORS");
    CreateLibrary(ExpendablesLibrary, "EDITOR_LIBRARY_EXPENDABLES");
    CreateLibrary(CraftLibrary, "EDITOR_LIBRARY_CRAFTS");
    CreateLibrary(DefencesLibrary, "EDITOR_LIBRARY_GUNS");
    CreateLibrary(GeneratorLibrary, "EDITOR_LIBRARY_GENERATORS");
    // This is not a mistake. This is the name given in the source code of the game,
    // so for compatability reasons it's given the same one here.
    CreateLibrary(QuartersLibrary, "EDITOR_LIBRARY_GENERATORS");
    CreateLibrary(EmergencyLibrary, "EDITOR_LIBRARY_EMERGENCY");
}

// Please do not change the order of these functions, as this is the default order in the source code
void DefineVanillaHulls() {
    DefinePart(EditorParts, "MDL_DECK_01", HullLibrary);
    DefinePart(EditorParts, "MDL_HARD2X2_04", HullLibrary);
    DefinePart(EditorParts, "MDL_HARD2X2_03", HullLibrary);
    DefinePart(EditorParts, "MDL_HARD2X2_02", HullLibrary);
    DefinePart(EditorParts, "MDL_HARD2X2_01", HullLibrary);
    DefinePart(EditorParts, "MDL_HARD2X1_01", HullLibrary);
    DefinePart(EditorParts, "MDL_HARD1X1_04", HullLibrary);
    DefinePart(EditorParts, "MDL_HARD1X1_01", HullLibrary);
    DefinePart(EditorParts, "MDL_FERMA2X2_04", HullLibrary);
    DefinePart(EditorParts, "MDL_FERMA2X2_03", HullLibrary);
    DefinePart(EditorParts, "MDL_FERMA2X2_02", HullLibrary);    
    DefinePart(EditorParts, "MDL_FERMA2X2_01", HullLibrary);
    DefinePart(EditorParts, "MDL_FERMA2X1_01", HullLibrary);
    DefinePart(EditorParts, "MDL_FERMA1X1_04", HullLibrary);
    DefinePart(EditorParts, "MDL_FERMA1X1_01", HullLibrary);
    DefinePart(EditorParts, "MDL_FERMA4X4_01", HullLibrary);
}

void DefineVanillaArmor() {
    DefinePart(EditorParts, "MDL_ARMOR2X1_01", ArmorLibrary);
    DefinePart(EditorParts, "MDL_ARMOR1X1_04", ArmorLibrary);
    DefinePart(EditorParts, "MDL_ARMOR1X1_03", ArmorLibrary);
    DefinePart(EditorParts, "MDL_ARMOR1X1_02", ArmorLibrary);
    DefinePart(EditorParts, "MDL_ARMOR1X1_01", ArmorLibrary);
}

void DefineVanillaFuel() {
    DefinePart(EditorParts, "MDL_FUEL_02", FuelLibrary);
    DefinePart(EditorParts, "MDL_FUEL_01", FuelLibrary);
}

void DefineVanillaAmmo() {
    DefinePart(EditorParts, "MDL_AMMO_02", AmmoLibrary);
    DefinePart(EditorParts, "MDL_AMMO", AmmoLibrary);
}

void DefineVanillaLegs() {
    DefinePart(EditorParts, "MDL_LEG_04", LegLibrary);
    DefinePart(EditorParts, "MDL_LEG_03", LegLibrary);
    DefinePart(EditorParts, "MDL_LEG_02", LegLibrary);
    DefinePart(EditorParts, "MDL_LEG_01", LegLibrary);
}

void DefineVanillaEngines() {
    DefinePart(EditorParts, "MDL_ENGINE_03", EngineLibrary);
    DefinePart(EditorParts, "MDL_ENGINE_04", EngineLibrary);
    DefinePart(EditorParts, "MDL_ENGINE_05", EngineLibrary);
    DefinePart(EditorParts, "MDL_ENGINE_01", EngineLibrary);
    DefinePart(EditorParts, "MDL_ENGINE_02", EngineLibrary);
}

void DefineVanillaGuns() {
    DefinePart(EditorParts, "MDL_CANNON_30_6", GunLibrary);
    DefinePart(EditorParts, "MDL_CANNON_57_2", GunLibrary);
    DefinePart(EditorParts, "MDL_CANNON_100_2", GunLibrary);
    DefinePart(EditorParts, "MDL_CANNON_130", GunLibrary);
    DefinePart(EditorParts, "MDL_CANNON_180", GunLibrary);
    DefinePart(EditorParts, "MDL_CANNON_180_2", GunLibrary);
    DefinePart(EditorParts, "MDL_CANNON_305_2", GunLibrary);
    DefinePart(EditorParts, "MDL_RSZO_220", GunLibrary);
}

void DefineVanillaSensors() {
    DefinePart(EditorParts, "MDL_RADAR_01", SensorLibrary);
    DefinePart(EditorParts, "MDL_RADAR_02", SensorLibrary);
    DefinePart(EditorParts, "MDL_SPO_01", SensorLibrary);
    DefinePart(EditorParts, "MDL_SPO_02", SensorLibrary);
    DefinePart(EditorParts, "MDL_FCR_01", SensorLibrary);
    DefinePart(EditorParts, "MDL_FCR_02", SensorLibrary);
    DefinePart(EditorParts, "MDL_IRST_01", SensorLibrary);
    DefinePart(EditorParts, "MDL_JAMMER_01", SensorLibrary);
    DefinePart(EditorParts, "MDL_ANTENNA_01", SensorLibrary);
}

void DefineVanillaExpendables() {
    DefinePart(EditorParts, "MDL_MISSILE_01", ExpendablesLibrary);
    DefinePart(EditorParts, "MDL_MISSILE_03", ExpendablesLibrary);
    DefinePart(EditorParts, "MDL_BOMB_01", ExpendablesLibrary);
    DefinePart(EditorParts, "MDL_NUKE_01_CONV", ExpendablesLibrary);
    DefinePart(EditorParts, "MDL_NUKE_02_CONV", ExpendablesLibrary);
    DefinePart(EditorParts, "MDL_NUKE_03_CONV", ExpendablesLibrary);
    DefinePart(EditorParts, "MDL_NUKE_04_CONV", ExpendablesLibrary);
    DefinePart(EditorParts, "MDL_NUKE_01", ExpendablesLibrary);
    DefinePart(EditorParts, "MDL_NUKE_02", ExpendablesLibrary);
    DefinePart(EditorParts, "MDL_NUKE_03", ExpendablesLibrary);
    DefinePart(EditorParts, "MDL_NUKE_04", ExpendablesLibrary);
}

void DefineVanillaCrafts() {
    DefinePart(EditorParts, "CRAFT_LA29", CraftLibrary);
    DefinePart(EditorParts, "CRAFT_T7", CraftLibrary);
}

void DefineVanillaDefences() {
    DefinePart(EditorParts, "MDL_FLARES", DefencesLibrary);
    DefinePart(EditorParts, "MDL_KAZ", DefencesLibrary);
}

void DefineVanillaGenerators() {
    DefinePart(EditorParts, "MDL_GENERATOR_01", GeneratorLibrary);
    DefinePart(EditorParts, "MDL_GENERATOR_02", GeneratorLibrary);
}

void DefineVanillaQuarters() {
    DefinePart(EditorParts, "MDL_QUARTERS_01", QuartersLibrary);
    DefinePart(EditorParts, "MDL_QUARTERS_02", QuartersLibrary);
}

void DefineVanillaEmergencies() {
    DefinePart(EditorParts, "MDL_FSS_02", EmergencyLibrary);
    DefinePart(EditorParts, "MDL_EVAC", EmergencyLibrary);
}

void DefineVanillaParts() {
    DefineVanillaHulls();
    DefineVanillaArmor();
    DefineVanillaFuel();
    DefineVanillaAmmo();
    DefineVanillaLegs();
    DefineVanillaEngines();
    DefineVanillaGuns();
    DefineVanillaSensors();
    DefineVanillaExpendables();
    DefineVanillaCrafts();
    DefineVanillaDefences();
    DefineVanillaGenerators();
    DefineVanillaQuarters();
    DefineVanillaEmergencies();
}