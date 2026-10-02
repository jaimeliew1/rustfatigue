@echo off
setlocal enabledelayedexpansion

echo Checking for Rust installation...
where rustc >nul 2>nul
if %errorlevel% neq 0 (
    echo Installing Rust toolchain via rustup...
    curl -sSf https://sh.rustup.rs -o rustup-init.exe
    rustup-init.exe -y
    del rustup-init.exe
    call "%USERPROFILE%\.cargo\env.bat"
) else (
    echo Rust is already installed.
)

:: Change to parent directory of script
cd /d "%~dp0\.."

:: Define Python versions
set PY_VERSIONS=3.8 3.9 3.10 3.11 3.12 3.13 3.14

echo Building and testing wheels for Python versions: %PY_VERSIONS%
for %%V in (%PY_VERSIONS%) do (
    echo Building for python%%V...
    uv build --python python%%V

    echo Testing wheel for python%%V...
    set "WHEEL="
    for /f %%W in ('dir /b /o-d dist\*.whl') do if not defined WHEEL set "WHEEL=dist\%%W"

    set "TEST_VENV=%TEMP%\rf_test_venv_%%V"
    if exist "!TEST_VENV!" rmdir /s /q "!TEST_VENV!"
    uv venv --python python%%V "!TEST_VENV!"
    uv pip install --python "!TEST_VENV!\Scripts\python.exe" "!WHEEL!" pytest

    :: copy the test file to a clean dir so the local rustfatigue\ source
    :: doesn't shadow the installed wheel when imported
    set "TEST_DIR=%TEMP%\rf_test_dir_%%V"
    if exist "!TEST_DIR!" rmdir /s /q "!TEST_DIR!"
    mkdir "!TEST_DIR!"
    copy rustfatigue\tests\rustfatigue_test.py "!TEST_DIR!\" >nul

    pushd "!TEST_DIR!"
    "!TEST_VENV!\Scripts\python.exe" -m pytest . -q
    set "TEST_RESULT=!errorlevel!"
    popd

    rmdir /s /q "!TEST_VENV!"
    rmdir /s /q "!TEST_DIR!"

    if not "!TEST_RESULT!"=="0" (
        echo Tests failed for python%%V
        exit /b 1
    )
)

:: Copy source distributions to wheelhouse
echo Copying source distributions...
if not exist wheelhouse mkdir wheelhouse
copy dist\* wheelhouse\

echo Build complete. Files are in .\wheelhouse\
