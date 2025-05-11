Write-Host "Starting vsixHarvester tests..."
Write-Host "================================="

# Ensure the project is built
Write-Host "Building the project..."
cargo build
if ($LASTEXITCODE -ne 0) {
    Write-Error "Build failed!"
    exit 1
}
Write-Host "Build successful."
Write-Host ""

# --- Info Command Tests ---
Write-Host "--- Testing 'info' command ---"

# Test 1: Info for a single, valid extension
Write-Host "Test 1: Info for rust-lang.rust-analyzer"
Write-Host "Command: cargo run -- info -s rust-lang.rust-analyzer"
cargo run -- info -s rust-lang.rust-analyzer
# Expected: Shows version info for rust-lang.rust-analyzer
Write-Host "--------------------"
Read-Host -Prompt "Press Enter to continue to next test"
Write-Host ""

# Test 2: Info for a single, valid extension (verbose)
Write-Host "Test 2: Info for rust-lang.rust-analyzer (verbose)"
Write-Host "Command: cargo run -- info -s rust-lang.rust-analyzer -v"
cargo run -- info -s rust-lang.rust-analyzer -v
# Expected: Shows version info and verbose logs
Write-Host "--------------------"
Read-Host -Prompt "Press Enter to continue to next test"
Write-Host ""

# Test 3: Info for a non-existent extension
Write-Host "Test 3: Info for a non-existent extension"
Write-Host "Command: cargo run -- info -s publisher.nonexistentextension"
cargo run -- info -s publisher.nonexistentextension
# Expected: Error message indicating failure to get info
Write-Host "--------------------"
Read-Host -Prompt "Press Enter to continue to next test"
Write-Host ""

# Test 4: Info using default extensions.json
Write-Host "Test 4: Info using ./.vscode/extensions.json"
if (Test-Path ./.vscode/extensions.json) {
    Write-Host "Command: cargo run -- info"
    cargo run -- info
    # Expected: Shows info for all extensions in ./.vscode/extensions.json
} else {
    Write-Warning ".vscode/extensions.json not found. Skipping this test."
}
Write-Host "--------------------"
Read-Host -Prompt "Press Enter to continue to next test"
Write-Host ""

# Test 5: Info using default extensions.json (verbose)
Write-Host "Test 5: Info using ./.vscode/extensions.json (verbose)"
if (Test-Path ./.vscode/extensions.json) {
    Write-Host "Command: cargo run -- info -v"
    cargo run -- info -v
    # Expected: Shows info and verbose logs for all extensions in ./.vscode/extensions.json
} else {
    Write-Warning ".vscode/extensions.json not found. Skipping this test."
}
Write-Host "--------------------"
Read-Host -Prompt "Press Enter to continue to next test"
Write-Host ""

# Test 6: Info using a non-existent input file
Write-Host "Test 6: Info using a non-existent input file"
Write-Host "Command: cargo run -- info --input nonexistents.json"
cargo run -- info --input nonexistents.json
# Expected: Error message indicating file not found or parse error
Write-Host "--------------------"
Read-Host -Prompt "Press Enter to continue to next test"
Write-Host ""


# --- Download Command Tests ---
Write-Host "--- Testing 'download' command ---"

# Test 7: Download a single extension (force, verbose)
Write-Host "Test 7: Download single extension (rust-lang.rust-analyzer, force, verbose)"
Write-Host "Command: cargo run -- download -s rust-lang.rust-analyzer -f -v"
cargo run -- download -s rust-lang.rust-analyzer -f -v
# Expected: Downloads rust-lang.rust-analyzer, shows verbose logs, forces re-download
Write-Host "--------------------"
Read-Host -Prompt "Press Enter to continue to next test"
Write-Host ""

# Test 8: Download using default extensions.json (force, verbose)
Write-Host "Test 8: Download using ./.vscode/extensions.json (force, verbose)"
if (Test-Path ./.vscode/extensions.json) {
    Write-Host "Command: cargo run -- download -f -v"
    cargo run -- download -f -v
    # Expected: Downloads all extensions from ./.vscode/extensions.json, shows verbose logs, forces re-download
} else {
    Write-Warning ".vscode/extensions.json not found. Skipping this test."
}
Write-Host "--------------------"
Read-Host -Prompt "Press Enter to continue to next test"
Write-Host ""

Write-Host "================================="
Write-Host "All tests prompted. Please review the output for correctness."
