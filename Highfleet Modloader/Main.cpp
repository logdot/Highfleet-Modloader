#include <Windows.h>
#include <iostream>

void main(int argc, char* argv[]) {
	STARTUPINFO si;
	PROCESS_INFORMATION pi;

	const char* gameName = "Highfleet.exe";
	const char* injectLibraryPath = "DLL.dll";

	ZeroMemory(&si, sizeof(si));
	si.cb = sizeof(si);
	ZeroMemory(&pi, sizeof(pi));

	bool createSuccess = false;
	if (argc == 2) {
		createSuccess = CreateProcess(argv[1], NULL, NULL, NULL, FALSE, 0, NULL, NULL, &si, &pi);
	}
	else {
		createSuccess = CreateProcess(gameName, NULL, NULL, NULL, FALSE, 0, NULL, NULL, &si, &pi);
	}


	std::cout << "Try and create process" << std::endl;
	if (!createSuccess) {
		std::cout << "Process creation failed" << std::endl;
		Sleep(5000);
		return;
	}

	// Sleep for 30 sec so that the binary can unpack itself
	std::cout << "Waiting 1 second" << std::endl;
	Sleep(1000);

	//HANDLE ph = OpenProcess(PROCESS_CREATE_THREAD | PROCESS_VM_OPERATION | PROCESS_VM_WRITE, false, pi.dwProcessId);
	HANDLE ph = pi.hProcess;


	LPVOID remoteBuffer = (LPVOID)VirtualAllocEx(ph, NULL, strlen(injectLibraryPath), MEM_COMMIT | MEM_RESERVE, PAGE_READWRITE);
	WriteProcessMemory(ph, remoteBuffer, injectLibraryPath, strlen(injectLibraryPath), NULL);
	LPTHREAD_START_ROUTINE addr = (LPTHREAD_START_ROUTINE)GetProcAddress(GetModuleHandleW(L"Kernel32.dll"), "LoadLibraryA");
	//PTHREAD_START_ROUTINE loadLibraryFunction = (PTHREAD_START_ROUTINE)(GetProcAddress(GetModuleHandleW(L"Kernel32"), "LoadLibraryA"));
	HANDLE remoteThreadHandle = CreateRemoteThread(ph, NULL, 0, addr, remoteBuffer, 0, NULL);
}