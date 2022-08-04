// dllmain.cpp : Defines the entry point for the DLL application.
#include "pch.h"
#include "mem.h"
#include "shipworks.h"
#include <windows.h>
#include <iostream>
#include <string>
#include <TlHelp32.h>
#include <fstream>
#include "json.hpp"

using json = nlohmann::json;

void LoadConfig(json data, const char* name, EditorLibraries library) {
    for (std::string moid : data[name]) {
        // TODO: specify which library the part was loaded into
        std::cout << "Loading part: " << moid << std::endl;
        DefinePart(EditorParts, moid.c_str(), library);
    }
}

void Inject() {
    CreateLibraries();

    // FIX: This will crash if shipyard.json does not exist
    std::ifstream f("Shipyard.json");
    json data = json::parse(f);

    // TODO: Make it so that each category can individually disable it's vanilla parts
    // e.g. Have all vanilla parts except for Fuels.
    if (data["LoadVanilla"] == true) {
        DefineVanillaParts();
    }

    LoadConfig(data, "Hulls", HullLibrary);
    LoadConfig(data, "Armors", ArmorLibrary);
    LoadConfig(data, "Fuels", FuelLibrary);
    LoadConfig(data, "Ammos", AmmoLibrary);
    LoadConfig(data, "Legs", LegLibrary);
    LoadConfig(data, "Engines", EngineLibrary);
    LoadConfig(data, "Guns", GunLibrary);
    LoadConfig(data, "Sensors", SensorLibrary);
    LoadConfig(data, "Expendables", ExpendablesLibrary);
    LoadConfig(data, "Crafts", CraftLibrary);
    LoadConfig(data, "Defences", DefencesLibrary);
    LoadConfig(data, "Generators", GeneratorLibrary);
    LoadConfig(data, "Quarters", QuartersLibrary);
}

DWORD WINAPI HackThread(HMODULE hModule) {
    AllocConsole();
    FILE* f;
    freopen_s(&f, "CONOUT$", "w", stdout);

    std::cout << "------------------------------------------------" << std::endl;
    std::cout << "/\\/\\/ Welcome to Shipyard part loader \\/\\/\\" << std::endl;
    std::cout << "------------------------------------------------" << std::endl;
    std::cout << std::endl;
    std::cout << "If the game crashes during loading the shipworks, you probably wrongly defined" << std::endl;
    std::cout << "an MOID inside the config file." << std::endl;
    std::cout << "Please check your log file to see if this is the case" << std::endl;
    std::cout << std::endl;
    std::cout << "If you have any problems please visit the mod's repository and leave an issue" << std::endl;
    std::cout << "https://github.com/logdot/Highfleet-Modloader" << std::endl;
    std::cout << std::endl;
    std::cout << std::endl;
    std::cout << std::endl;

    uintptr_t gameBase = (uintptr_t)GetModuleHandle(NULL);

    std::cout << "Game Base: " << gameBase << std::endl;
    std::cout << "Injected Function Base: " << Inject << std::endl;

    BYTE* injectAddr = (BYTE*)0x1400cbbcd;
    mem::Patch(injectAddr, (BYTE*)"\xff\x15\x02\x00\x00\x00\xeb\x08", 8); // Patch the absolute call bytes
    void* temp = &Inject;
    mem::Patch(injectAddr + 8, (BYTE*)(&temp), 8); // Patch the address of the call (Inject)
    mem::Patch(injectAddr + 16, (BYTE*)"\x48\x83\xc4\x20\x5b\xc3", 6); // Patch the epilogue

    std::cout << "Injected Address: " << std::hex << (uintptr_t)injectAddr << std::endl;
    std::cout << "Game path: " << std::filesystem::current_path() << std::endl;

    while (true)
    {
        // Eject the DLL if the end key is pressed.
        // Note that it will eject no matter where end is pressed, i.e. it is globally searching for keypresses.
        if ((GetAsyncKeyState(VK_END) & 1)) {
            break;
        }

        Sleep(5);
    }

    // Unpatch
    mem::Patch(injectAddr, (BYTE*)"\x74\x06\x48\x8b\x01\xff\x50\x48\xb9\x28\x01\x00\x00\xe8\x99\xd4\x23\x00\x48\x89\x44\x24", 22); // Replace the overriden bytes back to normal

    fclose(f);
    FreeConsole();
    FreeLibraryAndExitThread(hModule, 0);
    return 0;
}

BOOL APIENTRY DllMain( HMODULE hModule,
                       DWORD  ul_reason_for_call,
                       LPVOID lpReserved
                     )
{
    switch (ul_reason_for_call)
    {
    case DLL_PROCESS_ATTACH: {
        CloseHandle(CreateThread(nullptr, 0, (LPTHREAD_START_ROUTINE)HackThread, hModule, 0, nullptr));
    }
    case DLL_THREAD_ATTACH:
    case DLL_THREAD_DETACH:
    case DLL_PROCESS_DETACH:
        break;
    }
    return TRUE;
}