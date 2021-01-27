
global_asm!(".section .drectve");

macro_rules! forwarded_export_with_ordinal {
    ($exp_name:expr, $target_name:expr, $ordinal:expr) => {
        global_asm!(concat!(".ascii \" -export:", $exp_name, "=", $target_name, "\""));
    };
}

// The linker gets very unhappy if code is after this.
// It's probably due to generated code after weird inline asm sections

// Ordinals here refer to the imports.
// Windows defender gets huffy if we leave them off.

// Functions we hook
forwarded_export_with_ordinal!("GetTickCount", "get_tick_count", 469);

// Other functions in kernel32 that mech3 uses.
forwarded_export_with_ordinal!("ReadFile", "KERNEL32.ReadFile", 679);
forwarded_export_with_ordinal!("CreateFileA", "KERNEL32.CreateFileA", 80);
forwarded_export_with_ordinal!("FindNextFileA", "KERNEL32.FindNextFileA", 218);
forwarded_export_with_ordinal!("CloseHandle", "KERNEL32.CloseHandle", 50);
forwarded_export_with_ordinal!("DeleteFileA", "KERNEL32.DeleteFileA", 130);
forwarded_export_with_ordinal!("SetFilePointer", "KERNEL32.SetFilePointer", 778);
forwarded_export_with_ordinal!("FindFirstFileA", "KERNEL32.FindFirstFileA", 209);
forwarded_export_with_ordinal!("ExitProcess", "KERNEL32.ExitProcess", 183);
forwarded_export_with_ordinal!("MoveFileA", "KERNEL32.MoveFileA", 609);
forwarded_export_with_ordinal!("FormatMessageA", "KERNEL32.FormatMessageA", 236);
forwarded_export_with_ordinal!("Sleep", "KERNEL32.Sleep", 836);
forwarded_export_with_ordinal!("LocalFree", "KERNEL32.LocalFree", 591);
forwarded_export_with_ordinal!("GetCommandLineA", "KERNEL32.GetCommandLineA", 266);
forwarded_export_with_ordinal!("SetThreadPriority", "KERNEL32.SetThreadPriority", 819);
forwarded_export_with_ordinal!("SetCurrentDirectoryA", "KERNEL32.SetCurrentDirectoryA", 763);
forwarded_export_with_ordinal!("ResetEvent", "KERNEL32.ResetEvent", 704);
forwarded_export_with_ordinal!("CreateEventA", "KERNEL32.CreateEventA", 76);
forwarded_export_with_ordinal!("CreateThread", "KERNEL32.CreateThread", 109);
forwarded_export_with_ordinal!("WaitForSingleObject", "KERNEL32.WaitForSingleObject", 896);
forwarded_export_with_ordinal!("OutputDebugStringA", "KERNEL32.OutputDebugStringA", 639);
forwarded_export_with_ordinal!("SetEvent", "KERNEL32.SetEvent", 773);
forwarded_export_with_ordinal!("GetDriveTypeA", "KERNEL32.GetDriveTypeA", 332);
forwarded_export_with_ordinal!("GetTempPathA", "KERNEL32.GetTempPathA", 460);
forwarded_export_with_ordinal!("FreeLibrary", "KERNEL32.FreeLibrary", 241);
forwarded_export_with_ordinal!("GetProcAddress", "KERNEL32.GetProcAddress", 409);
forwarded_export_with_ordinal!("GetLogicalDriveStringsA", "KERNEL32.GetLogicalDriveStringsA", 366);
forwarded_export_with_ordinal!("GetFileSize", "KERNEL32.GetFileSize", 348);
forwarded_export_with_ordinal!("GetFileTime", "KERNEL32.GetFileTime", 350);
forwarded_export_with_ordinal!("LoadLibraryA", "KERNEL32.LoadLibraryA", 581);
forwarded_export_with_ordinal!("lstrlenA", "KERNEL32.lstrlenA", 953);
forwarded_export_with_ordinal!("GetVersionExA", "KERNEL32.GetVersionExA", 479);
forwarded_export_with_ordinal!("GlobalMemoryStatus", "KERNEL32.GlobalMemoryStatus", 506);
forwarded_export_with_ordinal!("QueryPerformanceFrequency", "KERNEL32.QueryPerformanceFrequency", 662);
forwarded_export_with_ordinal!("GetThreadPriority", "KERNEL32.GetThreadPriority", 465);
forwarded_export_with_ordinal!("QueryPerformanceCounter", "KERNEL32.QueryPerformanceCounter", 661);
forwarded_export_with_ordinal!("InitializeCriticalSection", "KERNEL32.InitializeCriticalSection", 537);
forwarded_export_with_ordinal!("DeleteCriticalSection", "KERNEL32.DeleteCriticalSection", 128);
forwarded_export_with_ordinal!("GetCurrentThread", "KERNEL32.GetCurrentThread", 318);
forwarded_export_with_ordinal!("EnterCriticalSection", "KERNEL32.EnterCriticalSection", 151);
forwarded_export_with_ordinal!("ReleaseMutex", "KERNEL32.ReleaseMutex", 692);
forwarded_export_with_ordinal!("LeaveCriticalSection", "KERNEL32.LeaveCriticalSection", 580);
forwarded_export_with_ordinal!("GetModuleHandleA", "KERNEL32.GetModuleHandleA", 375);
forwarded_export_with_ordinal!("GetStartupInfoA", "KERNEL32.GetStartupInfoA", 431);
forwarded_export_with_ordinal!("CreateMutexA", "KERNEL32.CreateMutexA", 93);
forwarded_export_with_ordinal!("GetLastError", "KERNEL32.GetLastError", 361);
forwarded_export_with_ordinal!("WriteFile", "KERNEL32.WriteFile", 913);

// Extra functions that recoil uses
forwarded_export_with_ordinal!("InterlockedDecrement", "KERNEL32.InterlockedDecrement", 541);
forwarded_export_with_ordinal!("InterlockedIncrement", "KERNEL32.InterlockedIncrement", 545);
forwarded_export_with_ordinal!("CompareFileTime", "KERNEL32.CompareFileTime", 55);
forwarded_export_with_ordinal!("WaitForMultipleObjects", "KERNEL32.WaitForMultipleObjects", 894);
forwarded_export_with_ordinal!("GetSystemDefaultLangID", "KERNEL32.GetSystemDefaultLangID", 440);
forwarded_export_with_ordinal!("CreateDirectoryA", "KERNEL32.CreateDirectoryA", 72);
forwarded_export_with_ordinal!("GetCurrentDirectoryA", "KERNEL32.GetCurrentDirectoryA", 314);
