use core::arch::global_asm;

global_asm!(".section .drectve");

macro_rules! forwarded_kernel32 {
    ($exp_name:expr) => {
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

// Kernel32 functions for MW and others
forwarded_kernel32!("ReadFile"); // ordinal: 679
forwarded_kernel32!("CreateFileA"); // ordinal: 80
forwarded_kernel32!("FindNextFileA"); // ordinal: 218
forwarded_kernel32!("CloseHandle"); // ordinal: 50
forwarded_kernel32!("DeleteFileA"); // ordinal: 130
forwarded_kernel32!("SetFilePointer"); // ordinal: 778
forwarded_kernel32!("FindFirstFileA"); // ordinal: 209
forwarded_kernel32!("ExitProcess"); // ordinal: 183
forwarded_kernel32!("MoveFileA"); // ordinal: 609
forwarded_kernel32!("FormatMessageA"); // ordinal: 236
forwarded_kernel32!("Sleep"); // ordinal: 836
forwarded_kernel32!("LocalFree"); // ordinal: 591
forwarded_kernel32!("GetCommandLineA"); // ordinal: 266
forwarded_kernel32!("SetThreadPriority"); // ordinal: 819
forwarded_kernel32!("SetCurrentDirectoryA"); // ordinal: 763
forwarded_kernel32!("ResetEvent"); // ordinal: 704
forwarded_kernel32!("CreateEventA"); // ordinal: 76
forwarded_kernel32!("CreateThread"); // ordinal: 109
forwarded_kernel32!("WaitForSingleObject"); // ordinal: 896
forwarded_kernel32!("OutputDebugStringA"); // ordinal: 639
forwarded_kernel32!("SetEvent"); // ordinal: 773
forwarded_kernel32!("GetDriveTypeA"); // ordinal: 332
forwarded_kernel32!("GetTempPathA"); // ordinal: 460
forwarded_kernel32!("FreeLibrary"); // ordinal: 241
forwarded_kernel32!("GetProcAddress"); // ordinal: 409
forwarded_kernel32!("GetLogicalDriveStringsA"); // ordinal: 366
forwarded_kernel32!("GetFileSize"); // ordinal: 348
forwarded_kernel32!("GetFileTime"); // ordinal: 350
forwarded_kernel32!("LoadLibraryA"); // ordinal: 581
forwarded_kernel32!("lstrlenA"); // ordinal: 953
forwarded_kernel32!("GetVersionExA"); // ordinal: 479
forwarded_kernel32!("GlobalMemoryStatus"); // ordinal: 506
forwarded_kernel32!("QueryPerformanceFrequency"); // ordinal: 662
forwarded_kernel32!("GetThreadPriority"); // ordinal: 465
forwarded_kernel32!("QueryPerformanceCounter"); // ordinal: 661
forwarded_kernel32!("InitializeCriticalSection"); // ordinal: 537
forwarded_kernel32!("DeleteCriticalSection"); // ordinal: 128
forwarded_kernel32!("GetCurrentThread"); // ordinal: 318
forwarded_kernel32!("EnterCriticalSection"); // ordinal: 151
forwarded_kernel32!("ReleaseMutex"); // ordinal: 692
forwarded_kernel32!("LeaveCriticalSection"); // ordinal: 580
forwarded_kernel32!("GetModuleHandleA"); // ordinal: 375
forwarded_kernel32!("GetStartupInfoA"); // ordinal: 431
forwarded_kernel32!("CreateMutexA"); // ordinal: 93
forwarded_kernel32!("GetLastError"); // ordinal: 361
forwarded_kernel32!("WriteFile"); // ordinal: 913

// Kernel32 functions for RC
forwarded_kernel32!("InterlockedDecrement"); // ordinal: 541
forwarded_kernel32!("InterlockedIncrement"); // ordinal: 545
forwarded_kernel32!("CompareFileTime"); // ordinal: 55
forwarded_kernel32!("WaitForMultipleObjects"); // ordinal: 894
forwarded_kernel32!("GetSystemDefaultLangID"); // ordinal: 440
forwarded_kernel32!("CreateDirectoryA"); // ordinal: 72
forwarded_kernel32!("GetCurrentDirectoryA"); // ordinal: 314

// Kernel32 functions for PM
forwarded_kernel32!("FindClose"); // ordinal: 205
forwarded_kernel32!("LoadResource"); // ordinal: 586
forwarded_kernel32!("LockResource"); // ordinal: 600
forwarded_kernel32!("SizeofResource"); // ordinal: 835
forwarded_kernel32!("FindResourceExA"); // ordinal: 225
forwarded_kernel32!("EnumResourceNamesA"); // ordinal: 164
