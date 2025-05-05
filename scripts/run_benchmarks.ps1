# Script to run benchmarks and generate a report
# Usage: .\scripts\run_benchmarks.ps1

# Create the benchmark directory if it doesn't exist
$benchmarkDir = "benchmark_results"
if (-not (Test-Path $benchmarkDir)) {
    New-Item -ItemType Directory -Path $benchmarkDir
}

# Run the benchmarks and save the results
Write-Host "Running benchmarks..."
cargo bench -- --output-format bencher | Tee-Object -FilePath "$benchmarkDir\benchmark_results.txt"

# Generate a timestamp for this benchmark run
$timestamp = Get-Date -Format "yyyy-MM-dd_HH-mm-ss"
$resultFile = "$benchmarkDir\benchmark_$timestamp.md"

# Extract the benchmark results and format them as a markdown table
Write-Host "Generating markdown report..."
$benchmarkResults = Get-Content "$benchmarkDir\benchmark_results.txt" | Where-Object { $_ -match "test .* ... bench:" }

# Create the markdown file
@"
# Lexx Benchmark Results
Generated on $(Get-Date -Format "yyyy-MM-dd HH:mm:ss")

## Results

| Test Name | Time (ns) | Throughput |
|-----------|-----------|------------|
"@ | Out-File -FilePath $resultFile

# Process each benchmark result and add to the table
foreach ($line in $benchmarkResults) {
    if ($line -match "test (.*) ... bench:\s+([0-9,]+) ns/iter") {
        $testName = $matches[1]
        $timeNs = $matches[2].Replace(",", "")
        
        # Calculate throughput for file benchmarks
        $throughput = ""
        if ($testName -match "file") {
            if ($testName -match "small_file") {
                $fileSizeBytes = 15
                $throughput = [math]::Round(($fileSizeBytes * 1000000000) / $timeNs, 2).ToString() + " MB/s"
            }
            elseif ($testName -match "utf-8-sampler") {
                $fileSizeBytes = 13658
                $throughput = [math]::Round(($fileSizeBytes * 1000000000) / $timeNs, 2).ToString() + " MB/s"
            }
            elseif ($testName -match "varney|large") {
                $fileSizeBytes = 1884747
                $throughput = [math]::Round(($fileSizeBytes * 1000000000) / $timeNs, 2).ToString() + " MB/s"
            }
        }
        
        "| $testName | $timeNs | $throughput |" | Out-File -FilePath $resultFile -Append
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

Write-Host "Benchmark report generated at: $resultFile"
