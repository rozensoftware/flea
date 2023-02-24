#include <Windows.h>
#include <stdio.h>
#include <shlwapi.h>
#include <TlHelp32.h>

#pragma comment(lib, "Shlwapi.lib")

UINT32 GetPidByName(WCHAR * name)
{
	HANDLE snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);

	if (snapshot == INVALID_HANDLE_VALUE)
	{
		return -1;
	}

	PROCESSENTRY32W entry;
	entry.dwSize = sizeof(PROCESSENTRY32W);

	if (!Process32FirstW(snapshot, &entry))
	{
		return -1;
	}

	do
	{
		if (lstrcmpW(entry.szExeFile, name) == 0)
		{
			CloseHandle(snapshot);
			return entry.th32ProcessID;
		}
	} while (Process32NextW(snapshot, &entry));

	CloseHandle(snapshot);

	return 0;
}

int32_t Inject()
{
	LPVOID loadlibrary = (LPVOID)GetProcAddress(GetModuleHandleW(L"kernel32.dll"), "LoadLibraryW");

	if (!PathFileExistsW(L"HideProcessHook.dll"))
	{
		return -1;
	}

	WCHAR buffer[MAX_PATH];
	BOOL result = GetFullPathNameW(L"HideProcessHook.dll", MAX_PATH, buffer, NULL);

	UINT32 pid = GetPidByName(L"Taskmgr.exe");

	if (!pid)
	{
		return -1;
	}
	
	HANDLE handle = OpenProcess(PROCESS_ALL_ACCESS, FALSE, pid);

	if (!handle)
	{
		return -1;
	}

	LPVOID address = (LPVOID)VirtualAllocEx(handle, NULL, (wcslen(buffer) + 1) * sizeof(WCHAR), MEM_RESERVE | MEM_COMMIT, PAGE_READWRITE);

	if (!address)
	{
		return -1;
	}

	if (!WriteProcessMemory(handle, address, buffer, (wcslen(buffer) + 1) * sizeof(WCHAR), NULL))
	{
		return -1;
	}

	HANDLE thread = CreateRemoteThread(handle, NULL, 0, (LPTHREAD_START_ROUTINE)loadlibrary, address, 0, NULL);

	if (!thread)
	{
		return -1;
	}

	if (WaitForSingleObject(thread, INFINITE) == WAIT_FAILED)
	{
		return -1;
	}
		
	DWORD ret = 0;
	if (!GetExitCodeThread(thread, &ret)) 
	{
		return -1;
	}

	if (!ret)
	{
		return -1;
	}
		
	CloseHandle(thread);
	CloseHandle(handle);

	return 0;
}