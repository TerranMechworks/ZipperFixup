use object::read::pe::{ExportTarget, PeFile32};

const EXPECTED_EXPORTS: &[&str] = &["DllMain", "GetTickCount"];
const EXPECTED_FORWARDS: &[&str] = &[
    "CloseHandle",
    "CompareFileTime",
    "CreateDirectoryA",
    "CreateEventA",
    "CreateFileA",
    "CreateMutexA",
    "CreateThread",
    "DeleteCriticalSection",
    "DeleteFileA",
    "EnterCriticalSection",
    "EnumResourceNamesA",
    "ExitProcess",
    "FindClose",
    "FindFirstFileA",
    "FindNextFileA",
    "FindResourceExA",
    "FormatMessageA",
    "FreeLibrary",
    "GetCommandLineA",
    "GetCurrentDirectoryA",
    "GetCurrentThread",
    "GetDriveTypeA",
    "GetFileSize",
    "GetFileTime",
    "GetLastError",
    "GetLogicalDriveStringsA",
    "GetModuleHandleA",
    "GetProcAddress",
    "GetStartupInfoA",
    "GetSystemDefaultLangID",
    "GetTempPathA",
    "GetThreadPriority",
    "GetVersionExA",
    "GlobalMemoryStatus",
    "InitializeCriticalSection",
    "InterlockedDecrement",
    "InterlockedIncrement",
    "LeaveCriticalSection",
    "LoadLibraryA",
    "LoadResource",
    "LocalFree",
    "LockResource",
    "lstrlenA",
    "MoveFileA",
    "OutputDebugStringA",
    "OutputDebugStringW",
    "QueryPerformanceCounter",
    "QueryPerformanceFrequency",
    "ReadFile",
    "ReleaseMutex",
    "ResetEvent",
    "SetCurrentDirectoryA",
    "SetEvent",
    "SetFilePointer",
    "SetThreadPriority",
    "SizeofResource",
    "Sleep",
    "WaitForMultipleObjects",
    "WaitForSingleObject",
    "WriteFile",
];

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let contents = std::fs::read("target/i686-pc-windows-gnu/release/zipfixup.dll")?;
    let data = contents.as_slice();

    let pe = PeFile32::parse(data)?;
    let table = pe.export_table()?.ok_or("no export table")?;
    let exported = table.exports()?;

    let mut exports = Vec::new();
    let mut forwards = Vec::new();
    for export in &exported {
        let name = export
            .name
            .ok_or_else(|| format!("export has no name: {:?}", export))?;
        let name = str::from_utf8(name)?;

        match export.target {
            ExportTarget::ForwardByName(module, forward) => {
                if module != b"KERNEL32" {
                    return Err(format!("expected forward to KERNEL32: {:?}", export).into());
                }
                if name.as_bytes() != forward {
                    return Err(format!(
                        "forward name mismatch `{}` != `{}`",
                        name,
                        forward.escape_ascii()
                    )
                    .into());
                }
                forwards.push(name);
            }
            ExportTarget::Address(_addr) => {
                exports.push(name);
            }
            ExportTarget::ForwardByOrdinal(module, _ordinal) => {
                if module != b"KERNEL32" {
                    return Err(format!("expected forward to KERNEL32: {:?}", export).into());
                }
                return Err(format!("unexpected forward by ordinal: {:?}", export).into());
            }
        }
    }
    exports.sort();
    forwards.sort();

    for name in exports.iter().copied() {
        println!("{}", name);
    }

    for name in forwards.iter().copied() {
        println!("KERNEL32.{}", name);
    }

    for expected in EXPECTED_EXPORTS {
        if !exports.contains(expected) {
            return Err(format!("missing export `{}`", expected).into());
        }
    }

    for expected in EXPECTED_FORWARDS {
        if !forwards.contains(expected) {
            return Err(format!("missing forward `{}`", expected).into());
        }
    }

    Ok(())
}
