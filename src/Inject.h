#pragma once

#include <Windows.h>

const DWORD WAIT_TIME = 1000;

DWORD GetPidByWindowName(LPCSTR windowName);
HANDLE GetGameProcess(LPCSTR windowName);
DWORD LaunchGame();
DWORD LaunchGame(LPCSTR gameName);
DWORD InjectLibrary(LPCSTR libraryPath, HANDLE processHandle);
