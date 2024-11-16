# Ensure that we are using powershell and not `sh` distributed by git on windows
set windows-powershell := true

# The local TouchDesigner Python distribution
touchdesigner_python := if os() == "windows" {
    "C:/Program Files/Derivative/TouchDesigner/bin/python.exe"
} else {
    "/Applications/TouchDesigner.app/Contents/Frameworks/Python.framework/Versions/3.11/bin/python3.11"
}


[windows]
venv-init:
    Start-Process -NoNewWindow -Wait -FilePath {{ touchdesigner_python }} -ArgumentList "-m venv venv"
[macos]
venv-init:
    {{ touchdesigner_python }} -m venv venv

[macos]
venv-activate:
    source venv/bin/activate
    ln -sf $PWD/lib/_stubs $PWD/venv/lib/python3.11/site-packages/_stubs
[windows]
venv-activate:
    . venv/Scripts/Activate.ps1

venv: venv-init venv-activate

build PLUGIN:
    cargo xtask build {{PLUGIN}}

install PLUGIN:
    cargo xtask install {{PLUGIN}}

list-plugins:
    cargo xtask list-plugins