#include "generic_fixup.h"

#include "unused.h"

#include <stdint.h>
#include <stdio.h>
#include <string.h>

static int64_t frequency = 0;
static int64_t startTime = 0;

static int64_t GetClockFrequency()
{
    LARGE_INTEGER freq;
    QueryPerformanceFrequency(&freq);
    return freq.QuadPart;
}

static int64_t GetClockTicks()
{
    LARGE_INTEGER ticks;
    QueryPerformanceCounter(&ticks);
    return ticks.QuadPart;
}

BOOL WINAPI DllMain(HINSTANCE hinstDLL, DWORD fdwReason, LPVOID lpvReserved)
{
    UNUSED(hinstDLL);
    UNUSED(lpvReserved);

    switch (fdwReason) {
    case DLL_PROCESS_ATTACH:
        // On load
        frequency = GetClockFrequency();
        startTime = GetClockTicks();
        InstallExtraHooks();
        break;

    case DLL_PROCESS_DETACH:
        // On unload
        break;

    case DLL_THREAD_ATTACH:
        // On thread start
        break;

    case DLL_THREAD_DETACH:
        // On thread end
        break;
    }

    return TRUE;
}

KERNEL32_API DWORD FakeGetTickCount()
{
    int64_t elapsed = GetClockTicks() - startTime;

    // scale to ms
    int64_t elapsed_ms = (elapsed * 1000) / frequency;

    return elapsed_ms;
}

static size_t FindStr(const char* haystack, size_t haystackLen, const char* needle, size_t needleLen)
{
    for (size_t i = 0; i < haystackLen - needleLen; ++i) {
        int rc = memcmp(haystack + i, needle, needleLen);
        if (rc == 0) {
            return i;
        }
    }
    return 0;
}

// Helper function to patch the game exe.
void PatchExeImpl(const char* oldExe, const char* newExe, const char* newDll)
{
    const char oldDll[] = "KERNEL32.dll";
    size_t bufferSize = 10 * 1024 * 1024;
    char* buffer = malloc(bufferSize);

    printf("Fixup dll version " PROJECT_VERSION "\n");

    printf("Reading %s\n", oldExe);
    FILE* fp = fopen(oldExe, "rb");
    if (fp == NULL) {
        printf("failed to open %s\n", oldExe);
        return;
    }

    size_t count = fread(buffer, 1, bufferSize, fp);
    printf("Read %d bytes\n", count);
    fclose(fp);

    size_t offset = FindStr(buffer, count, oldDll, strlen(oldDll));
    printf("Patching offset %d\n", offset);
    if (offset == 0) {
        printf("Failed to find patch location\n");
        return;
    }

    memcpy(buffer + offset, newDll, strlen(newDll));

    printf("Writing %s\n", newExe);
    fp = fopen(newExe, "wb");
    if (fp == NULL) {
        printf("Failed to open %s\n", newExe);
        return;
    }

    size_t writeCount = fwrite(buffer, 1, count, fp);
    printf("written %d bytes\n", writeCount);
    fclose(fp);
}
