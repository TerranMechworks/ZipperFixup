use core::arch::global_asm;

global_asm!(".section .drectve");

macro_rules! forwarded_override {
    ($exp_name:expr, $target_name:expr, $ordinal:expr) => {
        global_asm!(concat!(
            ".ascii \" -export:",
            $exp_name,
            "=",
            $target_name,
            "\""
        ));
    };
}

macro_rules! forwarded_kernel32 {
    ($exp_name:expr, $ordinal:expr) => {
        global_asm!(concat!(
            ".ascii \" -export:",
            $exp_name,
            "=KERNEL32.",
            $exp_name,
            "\""
        ));
    };
}

// The linker gets very unhappy if code is after this.
// It's probably due to generated code after weird inline asm sections

// Ordinals here refer to the imports.
// EOP: Windows defender gets huffy if we leave them off.
// Zunder: But we don't use the ordinals in the asm???

// Functions we hook/override
forwarded_override!("GetTickCount", "get_tick_count", 469);

// Kernel32 functions for MW and others
forwarded_kernel32!("ReadFile", 679);
forwarded_kernel32!("CreateFileA", 80);
forwarded_kernel32!("FindNextFileA", 218);
forwarded_kernel32!("CloseHandle", 50);
forwarded_kernel32!("DeleteFileA", 130);
forwarded_kernel32!("SetFilePointer", 778);
forwarded_kernel32!("FindFirstFileA", 209);
forwarded_kernel32!("ExitProcess", 183);
forwarded_kernel32!("MoveFileA", 609);
forwarded_kernel32!("FormatMessageA", 236);
forwarded_kernel32!("Sleep", 836);
forwarded_kernel32!("LocalFree", 591);
forwarded_kernel32!("GetCommandLineA", 266);
forwarded_kernel32!("SetThreadPriority", 819);
forwarded_kernel32!("SetCurrentDirectoryA", 763);
forwarded_kernel32!("ResetEvent", 704);
forwarded_kernel32!("CreateEventA", 76);
forwarded_kernel32!("CreateThread", 109);
forwarded_kernel32!("WaitForSingleObject", 896);
forwarded_kernel32!("OutputDebugStringA", 639);
forwarded_kernel32!("SetEvent", 773);
forwarded_kernel32!("GetDriveTypeA", 332);
forwarded_kernel32!("GetTempPathA", 460);
forwarded_kernel32!("FreeLibrary", 241);
forwarded_kernel32!("GetProcAddress", 409);
forwarded_kernel32!("GetLogicalDriveStringsA", 366);
forwarded_kernel32!("GetFileSize", 348);
forwarded_kernel32!("GetFileTime", 350);
forwarded_kernel32!("LoadLibraryA", 581);
forwarded_kernel32!("lstrlenA", 953);
forwarded_kernel32!("GetVersionExA", 479);
forwarded_kernel32!("GlobalMemoryStatus", 506);
forwarded_kernel32!("QueryPerformanceFrequency", 662);
forwarded_kernel32!("GetThreadPriority", 465);
forwarded_kernel32!("QueryPerformanceCounter", 661);
forwarded_kernel32!("InitializeCriticalSection", 537);
forwarded_kernel32!("DeleteCriticalSection", 128);
forwarded_kernel32!("GetCurrentThread", 318);
forwarded_kernel32!("EnterCriticalSection", 151);
forwarded_kernel32!("ReleaseMutex", 692);
forwarded_kernel32!("LeaveCriticalSection", 580);
forwarded_kernel32!("GetModuleHandleA", 375);
forwarded_kernel32!("GetStartupInfoA", 431);
forwarded_kernel32!("CreateMutexA", 93);
forwarded_kernel32!("GetLastError", 361);
forwarded_kernel32!("WriteFile", 913);

// Kernel32 functions for RC
forwarded_kernel32!("InterlockedDecrement", 541);
forwarded_kernel32!("InterlockedIncrement", 545);
forwarded_kernel32!("CompareFileTime", 55);
forwarded_kernel32!("WaitForMultipleObjects", 894);
forwarded_kernel32!("GetSystemDefaultLangID", 440);
forwarded_kernel32!("CreateDirectoryA", 72);
forwarded_kernel32!("GetCurrentDirectoryA", 314);

// Kernel32 functions for PM
forwarded_kernel32!("FindClose", 205);
forwarded_kernel32!("LoadResource", 586);
forwarded_kernel32!("LockResource", 600);
forwarded_kernel32!("SizeofResource", 835);
forwarded_kernel32!("FindResourceExA", 225);
forwarded_kernel32!("EnumResourceNamesA", 164);
