#pragma once

#include <Windows.h>

const DWORD EXIT_TIME = 10000;
const DWORD WAIT_TIME = 10000;
const LPCSTR WINDOW_NAME = "HIGHFLEET v.1.151";

DWORD GetProcessByWindowName(LPCSTR windowName);
HANDLE GetGameProcess();
DWORD LaunchGame();
DWORD LaunchGame(LPCSTR gameName);
DWORD InjectLibrary(LPCSTR libraryPath, HANDLE processHandle);