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
		Sleep(10000);
		return;
	}

	// Sleep for 30 sec so that the binary can unpack itself
	std::cout << "Waiting 1 second" << std::endl;
	Sleep(1000);

	//HANDLE ph = OpenProcess(PROCESS_CREATE_THREAD | PROCESS_VM_OPERATION | PROCESS_VM_WRITE, false, pi.dwProcessId);
	HANDLE ph = pi.hProcess;
	std::cout << "Trying to open process " << ph << std::endl;



	LPVOID remoteBuffer = (LPVOID)VirtualAllocEx(ph, NULL, strlen(injectLibraryPath), MEM_COMMIT | MEM_RESERVE, PAGE_READWRITE);
	std::cout << "Trying to write into the process memory" << std::endl;
	BOOL writeError = WriteProcessMemory(ph, remoteBuffer, injectLibraryPath, strlen(injectLibraryPath), NULL);
	std::cout << "Writing into process memory returned " << writeError << std::endl;

	LPTHREAD_START_ROUTINE addr = (LPTHREAD_START_ROUTINE)GetProcAddress(GetModuleHandleW(L"Kernel32.dll"), "LoadLibraryA");
	//PTHREAD_START_ROUTINE loadLibraryFunction = (PTHREAD_START_ROUTINE)(GetProcAddress(GetModuleHandleW(L"Kernel32"), "LoadLibraryA"));
	std::cout << "Trying to create a remote thread" << std::endl;
	HANDLE remoteThreadHandle = CreateRemoteThread(ph, NULL, 0, addr, remoteBuffer, 0, NULL);
	if (remoteThreadHandle == NULL)
	{
		std::cout << "Remote thread creation failed" << std::endl;
	}
	else {
		std::cout << "Remote thread creation succeded" << std::endl;
	}
	Sleep(10000);
}