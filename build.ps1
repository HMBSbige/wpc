param([string]$rid = 'win-x64')
$ErrorActionPreference = 'Stop'

Write-Host 'dotnet SDK info'
dotnet --info

$net_tfm = 'net7.0'
$configuration = 'Release'
$output_dir = "$PSScriptRoot\wpc\bin\$configuration"
$proj_path = "$PSScriptRoot\wpc\wpc.csproj"

function New-App
{
    param([string]$rid)
    Write-Host 'Building'

    $outdir = "$output_dir\$net_tfm"
    $publishDir = "$outdir\publish"

    Remove-Item $publishDir -Recurse -Force -Confirm:$false -ErrorAction Ignore

    dotnet publish "$proj_path" -c $configuration -f $net_tfm -r $rid
    if ($LASTEXITCODE) { exit $LASTEXITCODE }
}

New-App $rid