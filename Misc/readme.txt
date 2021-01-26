Installation
============
Copy InstallPatch.bat and zipfixup.dll to your MechWarrior3 directory.
Run InstallPatch.bat this will copy Mech3.exe to Mech3fixed.exe and patch it.
Run Mech3fixed.exe.

Change Log
==========
0.1.0
-----
-Try adding support for Recoil and Pirate's moon
-Move all code to rust as it was a pain to get C++ working

0.0.2
-----
-Fix install script

0.0.1
-----
-Timing issues related GetTickCount having a resolution of 1ms but an accuracy of 16ms
-Timing issues related to high system uptime.
-Mech3 Bounds checking on target box lines (prevents crashes at higher resolutions).
