// This is really just using the preprocessor to generate instructions for the linker

#ifdef _MSC_VER
#define FORWARDED_EXPORT_WITH_ORDINAL(exp_name, target_name, ordinal) __pragma(comment(linker, "/export:" #exp_name "=" #target_name ",@" #ordinal))
#define FORWARDED_EXPORT(exp_name, target_name) __pragma(comment(linker, "/export:" #exp_name "=" #target_name))
#endif
#ifdef __GNUC__
#define FORWARDED_EXPORT_WITH_ORDINAL(exp_name, target_name, ordinal) asm(".section .drectve\n\t.ascii \" -export:" #exp_name "=" #target_name "\"");
#define FORWARDED_EXPORT(exp_name, target_name) asm(".section .drectve\n\t.ascii \" -export:" #exp_name "=" #target_name "\"");
#endif

// The linker gets very unhappy if code is after this.
// It's probably due to generated code after weird inline asm sections

// Ordinals here refer to the imports.
// Windows defender gets huffy if we leave them off.

// Functions we hook
FORWARDED_EXPORT_WITH_ORDINAL(GetTickCount, get_tick_count, 469)

// Other functions in kernel32 that mech3 uses.
FORWARDED_EXPORT_WITH_ORDINAL(ReadFile, KERNEL32.ReadFile, 679)
FORWARDED_EXPORT_WITH_ORDINAL(CreateFileA, KERNEL32.CreateFileA, 80)
FORWARDED_EXPORT_WITH_ORDINAL(FindNextFileA, KERNEL32.FindNextFileA, 218)
FORWARDED_EXPORT_WITH_ORDINAL(CloseHandle, KERNEL32.CloseHandle, 50)
FORWARDED_EXPORT_WITH_ORDINAL(DeleteFileA, KERNEL32.DeleteFileA, 130)
FORWARDED_EXPORT_WITH_ORDINAL(SetFilePointer, KERNEL32.SetFilePointer, 778)
FORWARDED_EXPORT_WITH_ORDINAL(FindFirstFileA, KERNEL32.FindFirstFileA, 209)
FORWARDED_EXPORT_WITH_ORDINAL(ExitProcess, KERNEL32.ExitProcess, 183)
FORWARDED_EXPORT_WITH_ORDINAL(MoveFileA, KERNEL32.MoveFileA, 609)
FORWARDED_EXPORT_WITH_ORDINAL(FormatMessageA, KERNEL32.FormatMessageA, 236)
FORWARDED_EXPORT_WITH_ORDINAL(Sleep, KERNEL32.Sleep, 836)
FORWARDED_EXPORT_WITH_ORDINAL(LocalFree, KERNEL32.LocalFree, 591)
FORWARDED_EXPORT_WITH_ORDINAL(GetCommandLineA, KERNEL32.GetCommandLineA, 266)
FORWARDED_EXPORT_WITH_ORDINAL(SetThreadPriority, KERNEL32.SetThreadPriority, 819)
FORWARDED_EXPORT_WITH_ORDINAL(SetCurrentDirectoryA, KERNEL32.SetCurrentDirectoryA, 763)
FORWARDED_EXPORT_WITH_ORDINAL(ResetEvent, KERNEL32.ResetEvent, 704)
FORWARDED_EXPORT_WITH_ORDINAL(CreateEventA, KERNEL32.CreateEventA, 76)
FORWARDED_EXPORT_WITH_ORDINAL(CreateThread, KERNEL32.CreateThread, 109)
FORWARDED_EXPORT_WITH_ORDINAL(WaitForSingleObject, KERNEL32.WaitForSingleObject, 896)
FORWARDED_EXPORT_WITH_ORDINAL(OutputDebugStringA, KERNEL32.OutputDebugStringA, 639)
FORWARDED_EXPORT_WITH_ORDINAL(SetEvent, KERNEL32.SetEvent, 773)
FORWARDED_EXPORT_WITH_ORDINAL(GetDriveTypeA, KERNEL32.GetDriveTypeA, 332)
FORWARDED_EXPORT_WITH_ORDINAL(GetTempPathA, KERNEL32.GetTempPathA, 460)
FORWARDED_EXPORT_WITH_ORDINAL(FreeLibrary, KERNEL32.FreeLibrary, 241)
FORWARDED_EXPORT_WITH_ORDINAL(GetProcAddress, KERNEL32.GetProcAddress, 409)
FORWARDED_EXPORT_WITH_ORDINAL(GetLogicalDriveStringsA, KERNEL32.GetLogicalDriveStringsA, 366)
FORWARDED_EXPORT_WITH_ORDINAL(GetFileSize, KERNEL32.GetFileSize, 348)
FORWARDED_EXPORT_WITH_ORDINAL(GetFileTime, KERNEL32.GetFileTime, 350)
FORWARDED_EXPORT_WITH_ORDINAL(LoadLibraryA, KERNEL32.LoadLibraryA, 581)
FORWARDED_EXPORT_WITH_ORDINAL(lstrlenA, KERNEL32.lstrlenA, 953)
FORWARDED_EXPORT_WITH_ORDINAL(GetVersionExA, KERNEL32.GetVersionExA, 479)
FORWARDED_EXPORT_WITH_ORDINAL(GlobalMemoryStatus, KERNEL32.GlobalMemoryStatus, 506)
FORWARDED_EXPORT_WITH_ORDINAL(QueryPerformanceFrequency, KERNEL32.QueryPerformanceFrequency, 662)
FORWARDED_EXPORT_WITH_ORDINAL(GetThreadPriority, KERNEL32.GetThreadPriority, 465)
FORWARDED_EXPORT_WITH_ORDINAL(QueryPerformanceCounter, KERNEL32.QueryPerformanceCounter, 661)
FORWARDED_EXPORT_WITH_ORDINAL(InitializeCriticalSection, KERNEL32.InitializeCriticalSection, 537)
FORWARDED_EXPORT_WITH_ORDINAL(DeleteCriticalSection, KERNEL32.DeleteCriticalSection, 128)
FORWARDED_EXPORT_WITH_ORDINAL(GetCurrentThread, KERNEL32.GetCurrentThread, 318)
FORWARDED_EXPORT_WITH_ORDINAL(EnterCriticalSection, KERNEL32.EnterCriticalSection, 151)
FORWARDED_EXPORT_WITH_ORDINAL(ReleaseMutex, KERNEL32.ReleaseMutex, 692)
FORWARDED_EXPORT_WITH_ORDINAL(LeaveCriticalSection, KERNEL32.LeaveCriticalSection, 580)
FORWARDED_EXPORT_WITH_ORDINAL(GetModuleHandleA, KERNEL32.GetModuleHandleA, 375)
FORWARDED_EXPORT_WITH_ORDINAL(GetStartupInfoA, KERNEL32.GetStartupInfoA, 431)
FORWARDED_EXPORT_WITH_ORDINAL(CreateMutexA, KERNEL32.CreateMutexA, 93)
FORWARDED_EXPORT_WITH_ORDINAL(GetLastError, KERNEL32.GetLastError, 361)
FORWARDED_EXPORT_WITH_ORDINAL(WriteFile, KERNEL32.WriteFile, 913)

// Extra functions that recoil uses
FORWARDED_EXPORT_WITH_ORDINAL(InterlockedDecrement, KERNEL32.InterlockedDecrement, 541)
FORWARDED_EXPORT_WITH_ORDINAL(InterlockedIncrement, KERNEL32.InterlockedIncrement, 545)
FORWARDED_EXPORT_WITH_ORDINAL(CompareFileTime, KERNEL32.CompareFileTime, 55)
FORWARDED_EXPORT_WITH_ORDINAL(WaitForMultipleObjects, KERNEL32.WaitForMultipleObjects, 894)
FORWARDED_EXPORT_WITH_ORDINAL(GetSystemDefaultLangID, KERNEL32.GetSystemDefaultLangID, 440)
FORWARDED_EXPORT_WITH_ORDINAL(CreateDirectoryA, KERNEL32.CreateDirectoryA, 72)
FORWARDED_EXPORT_WITH_ORDINAL(GetCurrentDirectoryA, KERNEL32.GetCurrentDirectoryA, 314)