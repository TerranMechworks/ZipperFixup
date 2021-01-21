#include "mech3_fixup.h"

#include "generic_fixup.h"

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

__fastcall void DrawLineDotted(void* screenBuffer, int32_t x1, int32_t y1, int32_t x2, int32_t y2, int16_t color, uint32_t segmentCount)
{
    OrigDrawLineDottedFunc(screenBuffer, x1, y1, x2, y2, color, segmentCount);
}

DWORD WINAPI InstallFuncs(LPVOID unused)
{
    UNUSED(unused);
    //Wait for the original functions to be loaded then overwrite them
    Sleep(5000);
    OutputDebugStringA("Installing funcs");
    (*g_drawLinePtr) = DrawLine;
    (*g_drawLineDottedPtr) = DrawLineDotted;

    return 0;
}

void InstallExtraHooks()
{
    OutputDebugStringA("Fixup dll version " PROJECT_VERSION);
    CreateThread(NULL, 0, InstallFuncs, NULL, 0, NULL);
}

KERNEL32_API void PatchExe()
{
    const char oldExe[] = "Mech3.exe";
    const char newExe[] = "Mech3fixed.exe";
    const char newDll[] = "MECH3FIX.dll";
    PatchExeImpl(oldExe, newExe, newDll);
}
