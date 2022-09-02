#include "Inject.h"

DWORD GetProcessByWindowName(LPCSTR windowName)
{
    DWORD PID;
    GetWindowThreadProcessId(FindWindow(NULL, windowName), &PID); // getting the PID from a window handle
    return PID;
}

DWORD LaunchGame()
{
    return LaunchGame("Highfleet.exe");
}

DWORD LaunchGame(LPCSTR gameName)
{
    STARTUPINFO si;
    PROCESS_INFORMATION pi;

    ZeroMemory(&si, sizeof(si));
    si.cb = sizeof(si);
    ZeroMemory(&pi, sizeof(pi));

    bool createSuccess = false;
    createSuccess = CreateProcess(gameName, NULL, NULL, NULL, FALSE, NULL, NULL, NULL, &si, &pi);

    return createSuccess;
}

HANDLE GetGameProcess()
{
    DWORD PID = GetProcessByWindowName(WINDOW_NAME);
    HANDLE ph = OpenProcess(PROCESS_CREATE_THREAD | PROCESS_VM_OPERATION | PROCESS_VM_WRITE | PROCESS_VM_READ, false, PID);

    return ph;
}

DWORD InjectLibrary(LPCSTR libraryPath, HANDLE processHandle)
{
    LPVOID remoteBuffer = (LPVOID)VirtualAllocEx(processHandle, NULL, strlen(libraryPath), MEM_COMMIT | MEM_RESERVE, PAGE_READWRITE);
    if (remoteBuffer == NULL)
        return -1;

    BOOL writeError = WriteProcessMemory(processHandle, remoteBuffer, libraryPath, strlen(libraryPath), NULL);
    if (writeError == 0)
        return -1;

    LPTHREAD_START_ROUTINE addr = (LPTHREAD_START_ROUTINE)GetProcAddress(GetModuleHandleW(L"Kernel32.dll"), "LoadLibraryA");
    HANDLE remoteThreadHandle = CreateRemoteThread(processHandle, NULL, 0, addr, remoteBuffer, 0, NULL);
    if (remoteThreadHandle == NULL)
        return -1;

    return 1;
}