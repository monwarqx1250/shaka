Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

function Write-Step {
  param([string]$Message)
  Write-Host "[shaka-install] $Message"
}

function Fail {
  param([string]$Message)
  Write-Error "[shaka-install] $Message"
  exit 1
}

$Repo = "NazmusSayad/shaka"
$ApiUrl = "https://api.github.com/repos/$Repo/releases/latest"
$InstallDir = Join-Path $env:LOCALAPPDATA "shaka\bin"
$BinaryPath = Join-Path $InstallDir "shaka.exe"

Write-Step "Detecting platform and architecture"
$os = [System.Runtime.InteropServices.RuntimeInformation]::OSDescription
$arch = [System.Runtime.InteropServices.RuntimeInformation]::OSArchitecture.ToString().ToLowerInvariant()

$target = switch ($arch) {
  "x64" { "x86_64-pc-windows-msvc" }
  "arm64" { "aarch64-pc-windows-msvc" }
  default { Fail "Unsupported Windows architecture: $arch" }
}

Write-Step "Platform: $os"
Write-Step "Target: $target"

Write-Step "Fetching latest release metadata"
try {
  $release = Invoke-RestMethod -Uri $ApiUrl -Headers @{ "User-Agent" = "shaka-installer" }
} catch {
  Fail "Failed to fetch latest release from GitHub: $($_.Exception.Message)"
}

if (-not $release.tag_name) {
  Fail "Latest release response did not include tag_name"
}

$tag = [string]$release.tag_name
$version = $tag.TrimStart("v")
$assetName = "shaka-$tag-$target.zip"

Write-Step "Latest release tag: $tag"
Write-Step "Selecting asset: $assetName"

$asset = $release.assets | Where-Object { $_.name -eq $assetName } | Select-Object -First 1
if (-not $asset) {
  Fail "Could not find release asset: $assetName"
}

$checksumAsset = $release.assets | Where-Object { $_.name -eq "sha256sums.txt" } | Select-Object -First 1
if (-not $checksumAsset) {
  Fail "Could not find checksum file in release assets: sha256sums.txt"
}

if (Test-Path $BinaryPath) {
  Write-Step "Existing installation found at $BinaryPath"
  try {
    $currentVersionOutput = & $BinaryPath --version 2>$null
    $currentVersion = $null
    if ($currentVersionOutput) {
      $match = [regex]::Match(($currentVersionOutput | Out-String), "(\d+\.\d+\.\d+)")
      if ($match.Success) {
        $currentVersion = $match.Groups[1].Value
      }
    }
    if ($currentVersion -and $currentVersion -eq $version) {
      Write-Step "Installed version ($currentVersion) is already up to date"
      exit 0
    }
    if ($currentVersion) {
      Write-Step "Replacing installed version $currentVersion with $version"
    } else {
      Write-Step "Replacing existing shaka binary with version $version"
    }
  } catch {
    Write-Step "Replacing existing shaka binary with version $version"
  }
}

$tempRoot = Join-Path ([System.IO.Path]::GetTempPath()) ("shaka-install-" + [System.Guid]::NewGuid().ToString("N"))
$archivePath = Join-Path $tempRoot $assetName
$checksumPath = Join-Path $tempRoot "sha256sums.txt"
$extractDir = Join-Path $tempRoot "extract"

New-Item -ItemType Directory -Path $tempRoot -Force | Out-Null
New-Item -ItemType Directory -Path $extractDir -Force | Out-Null

try {
  Write-Step "Downloading release archive"
  Invoke-WebRequest -Uri $asset.browser_download_url -OutFile $archivePath -Headers @{ "User-Agent" = "shaka-installer" }

  Write-Step "Downloading checksum file"
  Invoke-WebRequest -Uri $checksumAsset.browser_download_url -OutFile $checksumPath -Headers @{ "User-Agent" = "shaka-installer" }

  Write-Step "Verifying archive checksum"
  $checksumLines = Get-Content -Path $checksumPath
  $checksumLine = $checksumLines | Where-Object { $_ -match ("\s+" + [regex]::Escape($assetName) + "$") } | Select-Object -First 1
  if (-not $checksumLine) {
    Fail "Could not find checksum entry for $assetName"
  }
  $expectedHash = ($checksumLine -split "\s+")[0].Trim().ToLowerInvariant()
  if (-not $expectedHash) {
    Fail "Checksum entry for $assetName is invalid"
  }
  $actualHash = (Get-FileHash -Path $archivePath -Algorithm SHA256).Hash.Trim().ToLowerInvariant()
  if ($expectedHash -ne $actualHash) {
    Fail "Checksum mismatch for $assetName (expected $expectedHash, got $actualHash)"
  }

  Write-Step "Extracting archive"
  Expand-Archive -Path $archivePath -DestinationPath $extractDir -Force

  $newBinary = Get-ChildItem -Path $extractDir -Recurse -File | Where-Object { $_.Name -eq "shaka.exe" } | Select-Object -First 1
  if (-not $newBinary) {
    Fail "Extracted archive does not contain shaka.exe"
  }

  Write-Step "Installing binary to $BinaryPath"
  New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null
  Move-Item -Path $newBinary.FullName -Destination $BinaryPath -Force

  $userPath = [Environment]::GetEnvironmentVariable("Path", "User")
  $pathEntries = @()
  if ($userPath) {
    $pathEntries = $userPath -split ";"
  }
  $inPath = $pathEntries | Where-Object { $_.TrimEnd("\\") -ieq $InstallDir.TrimEnd("\\") }
  if (-not $inPath) {
    Write-Step "Adding install directory to user PATH"
    $newUserPath = if ([string]::IsNullOrWhiteSpace($userPath)) { $InstallDir } else { "$userPath;$InstallDir" }
    [Environment]::SetEnvironmentVariable("Path", $newUserPath, "User")
  } else {
    Write-Step "Install directory already exists in user PATH"
  }

  if (-not (($env:Path -split ";") | Where-Object { $_.TrimEnd("\\") -ieq $InstallDir.TrimEnd("\\") })) {
    $env:Path = "$InstallDir;$env:Path"
    Write-Step "Updated PATH for current session"
  }

  Write-Step "Installation complete"
  Write-Host "[shaka-install] Installed: $BinaryPath"
  Write-Host "[shaka-install] Verify with: shaka --version"
} finally {
  if (Test-Path $tempRoot) {
    Remove-Item -Path $tempRoot -Recurse -Force -ErrorAction SilentlyContinue
  }
}
