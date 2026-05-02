@echo off
call "C:\Program Files\Microsoft Visual Studio\2022\Enterprise\Common7\Tools\VsDevCmd.bat" -arch=amd64 >nul 2>&1
echo LIB_VAR=%LIB%
echo INCLUDE_VAR=%INCLUDE%
where link.exe
