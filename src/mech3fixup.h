#pragma once
#include <windows.h>
#define KERNEL32_API __declspec(dllexport)
// Make sure functions are exported with C linkage under C++ compilers.
#ifdef __cplusplus
extern "C" {
#endif

BOOL WINAPI DllMain(HINSTANCE hinstDLL, DWORD fdwReason, LPVOID lpvReserved);
KERNEL32_API DWORD FakeGetTickCount();
KERNEL32_API void PatchMech3();

#ifdef __cplusplus
} // __cplusplus defined.
#endif