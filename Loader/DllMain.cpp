/// Almost all of this file was ripped straight out of https://stackoverflow.com/questions/13428881/calling-a-function-in-an-injected-dll

#include <windows.h>

struct SharedData
{
    DWORD dwOffset = 0;
    HMODULE hModule = nullptr;
    LPDWORD lpInit = nullptr;
};

#define DLL_REMOTEINIT_FUNCNAME "RemoteInit"
#define SHMEMSIZE sizeof(SharedData)
// Name of the shared file map (NOTE: Global namespaces must have the SeCreateGlobalPrivilege privilege)
#define SHMEMNAME "Global\\InjectedDllName_SHMEM"

static HANDLE hMapFile;
static LPVOID lpMemFile;

extern "C" {
    bool RemoteInit() {
        HINSTANCE getProc = LoadLibraryA(R"(.\Modloader\mods\ShipworksExtended.dll)");

        if (getProc == nullptr)
            return false;
        else
            return true;
    }
}

BOOL APIENTRY DllMain(HMODULE hModule,
                      DWORD ul_reason_for_call,
                      LPVOID lpReserved) {
    SharedData data;

    switch (ul_reason_for_call) {
    case DLL_PROCESS_ATTACH:
        DisableThreadLibraryCalls(hModule);

        hMapFile = CreateFileMappingA(INVALID_HANDLE_VALUE, nullptr, PAGE_READWRITE, 0, SHMEMSIZE, SHMEMNAME);
        if (hMapFile == nullptr) {
            MessageBoxA(nullptr, "Failed to create file mapping!", "DLL_PROCESS_ATTACH", MB_OK | MB_ICONERROR);
            return FALSE;
        }

        lpMemFile = MapViewOfFile(hMapFile, FILE_MAP_ALL_ACCESS, 0, 0, 0);
        if (lpMemFile == nullptr) {
            MessageBoxA(nullptr, "Failed to map shared memory!", "DLL_PROCESS_ATTACH", MB_OK | MB_ICONERROR);
            return FALSE;
        }

        // Set shared memory to hold what our remote process needs
        memset(lpMemFile, 0, SHMEMSIZE);
        data.hModule = hModule;
        data.lpInit = LPDWORD(GetProcAddress(hModule, DLL_REMOTEINIT_FUNCNAME));
        data.dwOffset = (DWORD*)data.lpInit - (DWORD*)data.hModule;
        memcpy(lpMemFile, &data, sizeof(SharedData));
    case DLL_THREAD_ATTACH:
        break;
    case DLL_THREAD_DETACH:
        break;
    case DLL_PROCESS_DETACH:
        // Tie up any loose ends
        UnmapViewOfFile(lpMemFile);
        CloseHandle(hMapFile);
        break;
    }
    return TRUE;
    UNREFERENCED_PARAMETER(lpReserved);
}