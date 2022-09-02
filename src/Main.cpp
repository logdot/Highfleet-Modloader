#include <Windows.h>
#include <iostream>
#include <filesystem>
#include "Inject.h"

namespace fs = std::filesystem;

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

	std::cout << "Trying to get game process" << std::endl;
	HANDLE ph = GetGameProcess();
	if (ph == NULL)
	{
		std::cout << "Failed to get game process, error: " << GetLastError() << std::endl;
		Sleep(EXIT_TIME);
		return -1;
	}

	for (const auto &modEntry : fs::directory_iterator("Modloader/mods"))
	{
		std::cout << "Attempting to load mod at: " << modEntry.path() << std::endl;
		std::cout << (modEntry.path().string().c_str()) << std::endl;
		bool injectSuccess = InjectLibrary((modEntry.path().string().c_str()), ph);

		if (!injectSuccess)
		{
			std::cout << "Failed to load mod, error: " << GetLastError() << std::endl;
			Sleep(EXIT_TIME);
			return -1;
		}
	}

	std::cout << "Finished loading all mods :)" << std::endl;

	Sleep(EXIT_TIME);
	return 0;
}