@echo off
@REM Expected to be run from the scripts folder

copy commit-msg ..\.git\hooks\

if %ERRORLEVEL% NEQ 0 echo [error] The copy seems to have failed. This script expects to be run from inside the scripts folder