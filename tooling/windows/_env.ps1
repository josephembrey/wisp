# Shared environment setup for Windows tooling scripts.
# Dot-source this from other scripts: . "$PSScriptRoot\_env.ps1"

$script:root = Resolve-Path "$PSScriptRoot\..\.."

# Short target dir to avoid MAX_PATH failures
# (whisper.cpp Vulkan shader builds create deeply nested paths)
$env:CARGO_TARGET_DIR = 'C:\wt'

# Refresh PATH from registry so recently-installed tools are available
# without requiring a terminal restart.
$env:Path = [System.Environment]::GetEnvironmentVariable('Path', 'Machine') + ';' +
            [System.Environment]::GetEnvironmentVariable('Path', 'User')
