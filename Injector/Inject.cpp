#include "Inject.h"

DWORD GetPidByWindowName(LPCSTR windowName)
{
    DWORD PID;
    GetWindowThreadProcessId(FindWindow(NULL, windowName), &PID); // getting the PID from a window handle
    return PID;
}

bool LaunchGame()
{
    return LaunchGame("Highfleet.exe");
}

bool LaunchGame(LPCSTR gameName)
{
    STARTUPINFO si;
    PROCESS_INFORMATION pi;

    ZeroMemory(&si, sizeof(si));
    si.cb = sizeof(si);
    ZeroMemory(&pi, sizeof(pi));

    DWORD createSuccess = CreateProcess(gameName, NULL, NULL, NULL, FALSE, NULL, NULL, NULL, &si, &pi);
    if (createSuccess == 0)
        return false;
    else
        return true;
}

HANDLE GetGameProcess(LPCSTR windowName)
{
    DWORD PID = GetPidByWindowName(windowName);
    HANDLE ph = OpenProcess(PROCESS_CREATE_THREAD | PROCESS_VM_OPERATION | PROCESS_VM_WRITE | PROCESS_VM_READ, false, PID);

    return ph;
}

bool InjectLibrary(LPCSTR libraryPath, HANDLE processHandle)
{
    LPVOID remoteBuffer = (LPVOID)VirtualAllocEx(processHandle, NULL, strlen(libraryPath), MEM_COMMIT | MEM_RESERVE, PAGE_READWRITE);
    if (remoteBuffer == NULL)
        return false;

    BOOL writeError = WriteProcessMemory(processHandle, remoteBuffer, libraryPath, strlen(libraryPath), NULL);
    if (writeError == 0)
        return false;

    LPTHREAD_START_ROUTINE addr = (LPTHREAD_START_ROUTINE)GetProcAddress(GetModuleHandleW(L"Kernel32.dll"), "LoadLibraryA");
    HANDLE remoteThreadHandle = CreateRemoteThread(processHandle, NULL, 0, addr, remoteBuffer, 0, NULL);
    if (remoteThreadHandle == NULL)
        return false;

    return true;
}