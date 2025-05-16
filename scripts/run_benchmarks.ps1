# Script to run benchmarks and generate a report
# Usage: .\scripts\run_benchmarks.ps1

# Create the benchmark directory if it doesn't exist
$benchmarkDir = "benchmark_results"
if (-not (Test-Path $benchmarkDir)) {
    New-Item -ItemType Directory -Path $benchmarkDir
}

# Run the benchmarks and save the results
Write-Host "Running benchmarks..."
cargo bench | Tee-Object -FilePath "$benchmarkDir\benchmark_results.txt"

# Generate a timestamp for this benchmark run
$timestamp = Get-Date -Format "yyyy-MM-dd_HH-mm-ss"
$resultFile = "$benchmarkDir\benchmark_$timestamp.md"

# Create the markdown file
@"
# Lexxor Benchmark Results
Generated on $(Get-Date -Format "yyyy-MM-dd HH:mm:ss")

## Results

| Benchmark | Time |
|-----------|------|
"@ | Out-File -FilePath $resultFile

# Process the benchmark results
$results = Get-Content "$benchmarkDir\benchmark_results.txt"
foreach ($line in $results) {
    if ($line -match "time:.*\[([\d\.]+ [a-z]+)\]") {
        $time = $matches[1]
        
        # Extract benchmark name from the line or previous lines
        $benchName = ""
        if ($line -match "^([^:]+):") {
            $benchName = $matches[1].Trim()
        }
        
        if ($benchName -ne "") {
            "| $benchName | $time |" | Out-File -FilePath $resultFile -Append
        }
    }
}

# Add a note about the environment
@"

## Environment
- CPU: $(Get-WmiObject -Class Win32_Processor | Select-Object -ExpandProperty Name)
- Memory: $([math]::Round((Get-WmiObject -Class Win32_ComputerSystem).TotalPhysicalMemory / 1GB, 2)) GB
- Rust: $(rustc --version)
- OS: $(Get-WmiObject -Class Win32_OperatingSystem | Select-Object -ExpandProperty Caption)
"@ | Out-File -FilePath $resultFile -Append

# Copy the Criterion HTML reports
$criterionDir = "target\criterion"
if (Test-Path $criterionDir) {
    $targetDir = "$benchmarkDir\criterion_$timestamp"
    New-Item -ItemType Directory -Path $targetDir -Force | Out-Null
    Copy-Item -Path "$criterionDir\*" -Destination $targetDir -Recurse
    Write-Host "Criterion reports copied to: $targetDir"
}

Write-Host "Benchmark report generated at: $resultFile"
