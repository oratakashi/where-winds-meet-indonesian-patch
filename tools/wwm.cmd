@echo off
rem wwm.cmd - run tools\wwm.sh from PowerShell / cmd (where `sh` is usually not on PATH).
rem   tools\wwm.cmd progress --check
rem Finds the sh.exe of Git for Windows and passes every argument through unchanged.
setlocal
set "SH="
for /f "delims=" %%i in ('where sh 2^>nul') do if not defined SH set "SH=%%i"
if not defined SH for /f "delims=" %%i in ('git --exec-path 2^>nul') do set "GITEXEC=%%i"
if not defined SH if defined GITEXEC if exist "%GITEXEC%/../../../usr/bin/sh.exe" set "SH=%GITEXEC%/../../../usr/bin/sh.exe"
if not defined SH if exist "%ProgramFiles%\Git\usr\bin\sh.exe" set "SH=%ProgramFiles%\Git\usr\bin\sh.exe"
if not defined SH (
  echo wwm.cmd: sh.exe tidak ditemukan - install Git for Windows, atau jalankan python tools\... langsung 1>&2
  exit /b 127
)
rem sh.exe started from cmd has no Git usr\bin on PATH (find, head, ...): add it
for %%d in ("%SH%") do set "PATH=%%~dpd;%PATH%"
set "SCRIPT=%~dp0wwm.sh"
"%SH%" "%SCRIPT:\=/%" %*
exit /b %ERRORLEVEL%
