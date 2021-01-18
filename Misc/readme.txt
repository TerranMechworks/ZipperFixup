Installation
------------
Copy InstallPatch.bat and mech3fix.dll to your MechWarrior3 directory.
Run InstallPatch.bat this will copy Mech3.exe to Mech3fixed.exe and patch it.
Run Mech3fixed.exe.

Implemented fixes
-----------------
Timing issues related GetTickCount having a resolution of 1ms but an accuracy of 16ms
Timing issues related to high system uptime.
Bounds checking on target box lines (prevents crashes at higher resolutions).
