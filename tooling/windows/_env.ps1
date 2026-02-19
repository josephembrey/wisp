# Shared environment setup for Windows tooling scripts.
# Dot-source this from other scripts: . "$PSScriptRoot\_env.ps1"

$script:root = Resolve-Path "$PSScriptRoot\..\.."

# Short target dir to avoid MSBuild FileTracker MAX_PATH failures
# (whisper.cpp Vulkan shader build creates deeply nested paths)
if (-not $env:CARGO_TARGET_DIR) {
    $env:CARGO_TARGET_DIR = 'C:\wisp-target'
}
