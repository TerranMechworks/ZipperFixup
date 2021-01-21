#pragma once
#include <windows.h>

#define KERNEL32_API __declspec(dllexport)

// Make sure functions are exported with C linkage under C++ compilers.
#ifdef __cplusplus
extern "C" {
#endif

// Exported functions
BOOL WINAPI DllMain(HINSTANCE hinstDLL, DWORD fdwReason, LPVOID lpvReserved);
KERNEL32_API DWORD FakeGetTickCount();

// Helper functions
void PatchExeImpl(const char* oldExe, const char* newExe, const char* newDll);

// Implementation specific
extern void InstallExtraHooks();

#ifdef __cplusplus
} // __cplusplus defined.
#endif