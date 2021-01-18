#include "mech3fixup.h"

#include "unused.h"

#include <stdint.h>
#include <stdio.h>
#include <string.h>
typedef void(__fastcall* drawLinePtr_t)(void* screenBuffer, int32_t x1, int32_t y1, int32_t x2, int32_t y2, int16_t color);
drawLinePtr_t OrigDrawLineFunc = (drawLinePtr_t)0x00563b80;
drawLinePtr_t* g_drawLinePtr = (drawLinePtr_t*)0x008008c0;

// sections =1 is the same as a regular line
typedef void(__fastcall* drawLineDottedPtr_t)(void* screenBuffer, int32_t x1, int32_t y1, int32_t x2, int32_t y2, int16_t color, uint32_t sections);
drawLineDottedPtr_t OrigDrawLineDottedFunc = (drawLineDottedPtr_t)0x00563c50;
drawLineDottedPtr_t* g_drawLineDottedPtr = (drawLineDottedPtr_t*)0x008008c4;

int32_t* g_screenBufferWidthPx = (int32_t*)0x8007cc;
int32_t* g_screenBufferHeightPx = (int32_t*)0x8007d0;
int32_t* g_screenBufferWidthBytes = (int32_t*)0x8007d4;
int32_t* g_screenBufferColDepthBytes = (int32_t*)0x8007d8;

int32_t* g_unk1 = (int32_t*)0x8007dc;
int32_t* g_unk2 = (int32_t*)0x8007e0;

__fastcall void DrawLine(void* screenBuffer, int32_t x1, int32_t y1, int32_t x2, int32_t y2, int16_t color)
{
    int32_t width = *g_screenBufferWidthPx;
    int32_t height = *g_screenBufferHeightPx;

    // bounds check drawing operation as target boxes can go outside the screen
    x1 = min(x1, width - 1);
    x1 = max(0, x1);
    y1 = min(y1, height - 1);
    y1 = max(0, y1);
    x2 = min(x2, width - 1);
    x2 = max(0, x2);
    y2 = min(y2, height - 1);
    y2 = max(0, y2);

    OrigDrawLineFunc(screenBuffer, x1, y1, x2, y2, color);
}

__fastcall void DrawLineDottedFunc(void* screenBuffer, int32_t x1, int32_t y1, int32_t x2, int32_t y2, int16_t color, uint32_t unk)
{
    OrigDrawLineDottedFunc(screenBuffer, x1, y1, x2, y2, color, unk);
}

static int64_t frequency = 0;
static int64_t startTime = 0;
int64_t GetClockFrequency()
{
    LARGE_INTEGER freq;
    QueryPerformanceFrequency(&freq);
    return freq.QuadPart;
}

int64_t GetClockTicks()
{
    LARGE_INTEGER ticks;
    QueryPerformanceCounter(&ticks);
    return ticks.QuadPart;
}

DWORD WINAPI InstallFuncs(LPVOID unused)
{
    UNUSED(unused);

    //Wait for the original functions to be loaded then overwrite them
    Sleep(5000);
    OutputDebugStringA("Installing funcs");
    (*g_drawLinePtr) = DrawLine;
    return 0;
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
        CreateThread(NULL, 0, InstallFuncs, NULL, 0, NULL);
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

size_t findStr(const char* haystack, size_t haystackLen, const char* needle, size_t needleLen)
{
    for (size_t i = 0; i < haystackLen - needleLen; ++i) {
        int rc = memcmp(haystack + i, needle, needleLen);
        if (rc == 0) {
            return i;
        }
    }
    return 0;
}

// Helper function to patch mech3.exe. Run using
// `rundll mech3fix.dll PatchMech3`
KERNEL32_API void PatchMech3()
{
    const char oldDll[] = "KERNEL32.dll";
    const char newDll[] = "MECH3FIX.dll";
    const char exeName[] = "Mech3.exe";
    const char newExeName[] = "Mech3fixed.exe";
    size_t bufferSize = 3 * 1024 * 1024;
    char* buffer = malloc(bufferSize);

    printf("Reading %s\n", exeName);
    FILE* fp = fopen(exeName, "rb");
    if (fp == NULL) {
        printf("failed to open %s\n", exeName);
        return;
    }

    size_t count = fread(buffer, 1, bufferSize, fp);
    printf("Read %d bytes\n", count);
    fclose(fp);

    size_t offset = findStr(buffer, count, oldDll, sizeof(oldDll) - 1);
    printf("Patching offset %d\n", offset);
    if (offset == 0) {
        printf("Failed to find patch location\n");
        return;
    }

    memcpy(buffer + offset, newDll, sizeof(newDll) - 1);

    printf("Writing %s\n", newExeName);
    fp = fopen(newExeName, "wb");
    if (fp == NULL) {
        printf("Failed to open %s\n", newExeName);
        return;
    }

    size_t writeCount = fwrite(buffer, 1, count, fp);
    printf("written %d bytes\n", writeCount);
    fclose(fp);
}
