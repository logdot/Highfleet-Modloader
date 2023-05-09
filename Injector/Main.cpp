#include <Windows.h>
#include <iostream>
#include <filesystem>
#include <fstream>
#include <string>
#include "Inject.h"

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

namespace fs = std::filesystem;

std::string GetWindowNameFromFile(std::string path) {
    std::ifstream infile(path);

    std::string windowName;
    std::getline(infile, windowName);

    return windowName;
}

void VerifyModFolders()
{
	if (!fs::exists("Modloader"))
	{
		std::cout << "Modloader folder does not exist. Creating it at `Modloader`" << std::endl;

		fs::create_directories("Modloader");
	}

	if (!fs::exists("Modloader/mods"))
	{
		std::cout << "Mod folder does not exist. Creating it at `Modloader/mods`" << std::endl;

		fs::create_directories("Modloader/mods");
	}

	if (!fs::exists("Modloader/config"))
	{
		std::cout << "Config folder does not exist. Creating it at `Modloader/config`" << std::endl;

		fs::create_directories("Modloader/config");
	}
}

int main(int argc, char *argv[])
{
	VerifyModFolders();

	const char *injectLibraryPath = "ShipworksExtended.dll";

	std::cout << "Attempt to launch game" << std::endl;
	bool launchSuccess = (argc == 2) ? LaunchGame(argv[1]) : LaunchGame();

	if (!launchSuccess)
	{
		std::cout << "Launching game failed, error: " << GetLastError() << std::endl;
		Sleep(EXIT_TIME);
		return -1;
	}

	// Sleep so that the binary can unpack itself
	std::cout << "Waiting " << WAIT_TIME / 1000 << " seconds" << std::endl;
	Sleep(WAIT_TIME);

    std::string windowName = GetWindowNameFromFile("Modloader\\window_name.txt");
    if (windowName.empty())
    {
        std::cout << "Failed to find window name. Does the file '.\\Modloader\\window_name.txt' with a proper window name exist?";
        Sleep(EXIT_TIME);
        return -1;
    }

	std::cout << "Trying to get game process of " << windowName << std::endl;
	HANDLE ph = GetGameProcess(windowName.c_str());
	if (ph == NULL)
	{
		std::cout << "Failed to get game process, error: " << GetLastError() << std::endl;
		Sleep(EXIT_TIME);
		return -1;
	}

    std::cout << "Trying to inject modloader." << std::endl;
    bool injectSuccess = InjectLibrary(".\\Modloader.dll", ph);
    if (!injectSuccess)
        std::cout << "Failed to inject modloader." << std::endl;
    else
        std::cout << "Injected modloader." << std::endl;

    std::cout << "Sleeping" << std::endl;
    Sleep(500);

    HANDLE hMapFile = CreateFileMappingA(INVALID_HANDLE_VALUE, nullptr, PAGE_READWRITE, 0, SHMEMSIZE, SHMEMNAME);
    if (hMapFile == nullptr) {
        std::cout << "Failed to create file mapping, error: " << GetLastError() << std::endl;
        Sleep(EXIT_TIME);
        return -1;
    }

    LPVOID lpMemFile = MapViewOfFile(hMapFile, FILE_MAP_ALL_ACCESS, 0, 0, 0);
    if (lpMemFile == nullptr) {
        std::cout << "Failed to map shared memory, error: " << GetLastError() << std::endl;
        Sleep(EXIT_TIME);
        return -1;
    }

    SharedData data;
    memcpy(&data, lpMemFile, SHMEMSIZE);

    UnmapViewOfFile(lpMemFile);
    CloseHandle(hMapFile);

    std::cout << "Calling init on loader." << std::endl;

    DWORD dwThreadId = 0;
    auto handle = CreateRemoteThread(ph, nullptr, 0, LPTHREAD_START_ROUTINE(data.lpInit), nullptr, 0, &dwThreadId);

    if (handle == nullptr)
        std::cout << "Failed to call init on loader." << std::endl;
    else
        std::cout << "Successfully called init on loader." << std::endl;

	std::cout << "Finished injecting loader :)" << std::endl;

	Sleep(EXIT_TIME);
	return 0;
}