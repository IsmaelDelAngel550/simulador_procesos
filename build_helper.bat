@echo off
set PATH=C:\Program Files\Microsoft Visual Studio\2022\Enterprise\VC\Tools\MSVC\14.41.34120\bin\Hostx64\x64;%USERPROFILE%\.cargo\bin;%PATH%
set LIB=C:\Program Files\Microsoft Visual Studio\2022\Enterprise\VC\Tools\MSVC\14.41.34120\lib\x64;C:\Program Files (x86)\Windows Kits\10\lib\10.0.22621.0\ucrt\x64;C:\Program Files (x86)\Windows Kits\10\lib\10.0.22621.0\um\x64
set INCLUDE=C:\Program Files\Microsoft Visual Studio\2022\Enterprise\VC\Tools\MSVC\14.41.34120\include;C:\Program Files (x86)\Windows Kits\10\include\10.0.22621.0\ucrt;C:\Program Files (x86)\Windows Kits\10\include\10.0.22621.0\um;C:\Program Files (x86)\Windows Kits\10\include\10.0.22621.0\shared
cargo build 2>&1
