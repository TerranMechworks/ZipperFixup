#pragma once
#include <windows.h>

// Make sure functions are exported with C linkage under C++ compilers.
#ifdef __cplusplus
extern "C" {
#endif

void InstallExtraHooks();

#ifdef __cplusplus
} // __cplusplus defined.
#endif
