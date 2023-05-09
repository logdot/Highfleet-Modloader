#pragma once

#include <Windows.h>

const DWORD EXIT_TIME = 10000;
const DWORD WAIT_TIME = 10000;

DWORD GetPidByWindowName(LPCSTR windowName);
HANDLE GetGameProcess(LPCSTR windowName);
bool LaunchGame();
bool LaunchGame(LPCSTR gameName);
bool InjectLibrary(LPCSTR libraryPath, HANDLE processHandle);