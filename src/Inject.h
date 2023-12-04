#pragma once

#include <Windows.h>

const DWORD EXIT_TIME = 50000;
const DWORD WAIT_TIME = 50000;

DWORD GetPidByWindowName(LPCSTR windowName);
HANDLE GetGameProcess(LPCSTR windowName);
DWORD LaunchGame();
DWORD LaunchGame(LPCSTR gameName);
DWORD InjectLibrary(LPCSTR libraryPath, HANDLE processHandle);
