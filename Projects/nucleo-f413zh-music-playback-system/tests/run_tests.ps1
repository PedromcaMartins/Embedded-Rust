# PowerShell script to run the cargo test command for each file in the test directory and its subdirectories

$testDirectory = "."  # Set the path to the test directory

# Recursively get all the files in the test directory and its subdirectories
$files = Get-ChildItem -Recurse -File -Path $testDirectory

foreach ($file in $files) {
    # Get the filename without the extension
    $test_name = [System.IO.Path]::GetFileNameWithoutExtension($file.FullName)

    # Construct the cargo test command with the current test name
    $command = "cargo test --package arm --no-fail-fast --test $test_name --features='io_mapping_test' -- --show-output --nocapture"

    # Run the command
    Write-Host "Running: $command"
    Invoke-Expression $command
}
