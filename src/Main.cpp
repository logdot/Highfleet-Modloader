#include <Windows.h>
#include <iostream>
#include "Inject.h"

int main(int argc, char *argv[])
{
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

	std::cout << "Attempting to load mod" << std::endl;
	bool injectSuccess = InjectLibrary(injectLibraryPath, ph);

	if (!injectSuccess)
	{
		std::cout << "Failed to load mod, error: " << GetLastError() << std::endl;
		Sleep(EXIT_TIME);
		return -1;
	}

	Sleep(EXIT_TIME);
	return 0;
}