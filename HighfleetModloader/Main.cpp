#include <Windows.h>
#include <iostream>

DWORD GetProcessByWindowName(LPCSTR proc) {
	DWORD PID;
	GetWindowThreadProcessId(FindWindow(NULL, proc), &PID); // getting the PID from a window handle
	return PID;
}

int main(int argc, char* argv[]) {
	STARTUPINFO si;
	PROCESS_INFORMATION pi;

	const char* gameName = "Highfleet.exe";
	const char* injectLibraryPath = "DLL.dll";
	const char* windowName = "HIGHFLEET v.1.151";

	const DWORD exitTime = 10000;
	const DWORD waitTime = 10000;

	ZeroMemory(&si, sizeof(si));
	si.cb = sizeof(si);
	ZeroMemory(&pi, sizeof(pi));

	bool createSuccess = false;
	if (argc == 2) {
		createSuccess = CreateProcess(argv[1], NULL, NULL, NULL, FALSE, NULL, NULL, NULL, &si, &pi);
	}
	else {
		createSuccess = CreateProcess(gameName, NULL, NULL, NULL, FALSE, NULL, NULL, NULL, &si, &pi);
	}

	std::cout << "Try and create process" << std::endl;
	if (!createSuccess) {
		std::cout << "Process creation failed, error: " << GetLastError() << std::endl;
		Sleep(exitTime);
		return -1;
	}

	// Sleep for 30 sec so that the binary can unpack itself
	std::cout << "Waiting " << waitTime / 1000 << " seconds" << std::endl;
	Sleep(waitTime);

	DWORD PID = GetProcessByWindowName(windowName);
	std::cout << "Trying to get process handle with PID: " << PID << std::endl;
	HANDLE ph = OpenProcess(PROCESS_CREATE_THREAD | PROCESS_VM_OPERATION | PROCESS_VM_WRITE | PROCESS_VM_READ, false, PID);
	if (ph == NULL) {
		std::cout << "Failed to get process handle" << std::endl;
		Sleep(exitTime);
		return -1;
	}

	std::cout << "Trying to open process" << std::endl;

	std::cout << "Trying to create remote buffer" << std::endl;
	LPVOID remoteBuffer = (LPVOID)VirtualAllocEx(ph, NULL, strlen(injectLibraryPath), MEM_COMMIT | MEM_RESERVE, PAGE_READWRITE);
	if (remoteBuffer == NULL) {
		std::cout << "Failed to create remote buffer, error: " << GetLastError() << std::endl;
		Sleep(exitTime);
		return -1;
	}

	std::cout << "Trying to write into the process memory" << std::endl;
	BOOL writeError = WriteProcessMemory(ph, remoteBuffer, injectLibraryPath, strlen(injectLibraryPath), NULL);
	if (writeError == 0) {
		std::cout << "Failed to write into process memory, error: " << GetLastError() << std::endl;
		Sleep(exitTime);
		return -1;
	}


	LPTHREAD_START_ROUTINE addr = (LPTHREAD_START_ROUTINE)GetProcAddress(GetModuleHandleW(L"Kernel32.dll"), "LoadLibraryA");
	std::cout << "Trying to create a remote thread" << std::endl;
	HANDLE remoteThreadHandle = CreateRemoteThread(ph, NULL, 0, addr, remoteBuffer, 0, NULL);
	if (remoteThreadHandle == NULL)
	{
		std::cout << "Remote thread creation failed" << std::endl;
	}
	else {
		std::cout << "Remote thread creation succeded" << std::endl;
	}
	Sleep(exitTime);
	return 0;
}