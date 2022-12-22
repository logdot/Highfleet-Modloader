#pragma once

#include <Windows.h>

const DWORD EXIT_TIME = 10000;
const DWORD WAIT_TIME = 10000;

DWORD GetProcessByWindowName(LPCSTR windowName);
HANDLE GetGameProcess(LPCSTR windowName);
DWORD LaunchGame();
DWORD LaunchGame(LPCSTR gameName);
DWORD InjectLibrary(LPCSTR libraryPath, HANDLE processHandle);