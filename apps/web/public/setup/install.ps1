#!/usr/bin/env pwsh
param(
  [String]$Version = "latest",
  # Skips adding the prowo-setup.exe directory to the user's %PATH%
  [Switch]$NoPathUpdate = $false,
  # Skips adding the prowo-setup to the list of installed programs
  [Switch]$NoRegisterInstallation = $false,

  # Debugging: Always download with 'Invoke-RestMethod' instead of 'curl.exe'
  [Switch]$DownloadWithoutCurl = $false
);

# filter out 32 bit + ARM
if (-not ((Get-CimInstance Win32_ComputerSystem)).SystemType -match "x64-based") {
  Write-Output "Install Failed:"
  Write-Output "prowo-setup for Windows is currently only available for x86 64-bit Windows.`n"
  return 1
}

# Check for Visual C++ Redistributable
function Test-VCRedist {
  try {
    # Check for VC++ 2015-2022 redistributable (x64)
    $vcRedist = Get-ItemProperty "HKLM:\SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall\*" | 
      Where-Object { $_.DisplayName -like "*Visual C++ 2015-2022 Redistributable*" -and $_.DisplayName -like "*x64*" }
    
    if ($vcRedist) {
      return $true
    }
    
    # Alternative check - look for specific VC++ runtime DLL
    $systemRoot = [Environment]::GetFolderPath("System")
    $vcDll = Join-Path $systemRoot "vcruntime140.dll"
    return Test-Path $vcDll
  } catch {
    return $false
  }
}

if (-not (Test-VCRedist)) {
  Write-Warning "Visual C++ Redistributable (x64) is not installed or not detected."
  Write-Output "prowo-setup requires the Visual C++ Redistributable to run properly."
  Write-Output "Please install it from: https://aka.ms/vs/17/release/vc_redist.x64.exe"
  Write-Output ""
  Write-Output "The installation will continue, but prowo-setup may not work until you install the redistributable.`n"
}

# This corresponds to Windows 10 1809 / Windows Server 2019
$MinBuild = 17763;
$MinBuildName = "Windows 10 1809 / Windows Server 2019"

$WinVer = [System.Environment]::OSVersion.Version
if ($WinVer.Major -lt 10 -or ($WinVer.Major -eq 10 -and $WinVer.Build -lt $MinBuild)) {
  Write-Warning "prowo-setup requires at ${MinBuildName} or newer.`n`nThe install will still continue but it may not work.`n"
  return 1
}

$ErrorActionPreference = "Stop"

# These three environment functions are roughly copied from https://github.com/prefix-dev/pixi/pull/692
# They are used instead of `SetEnvironmentVariable` because of unwanted variable expansions.
function Publish-Env {
  if (-not ("Win32.NativeMethods" -as [Type])) {
    Add-Type -Namespace Win32 -Name NativeMethods -MemberDefinition @"
[DllImport("user32.dll", SetLastError = true, CharSet = CharSet.Auto)]
public static extern IntPtr SendMessageTimeout(
    IntPtr hWnd, uint Msg, UIntPtr wParam, string lParam,
    uint fuFlags, uint uTimeout, out UIntPtr lpdwResult);
"@
  }
  $HWND_BROADCAST = [IntPtr] 0xffff
  $WM_SETTINGCHANGE = 0x1a
  $result = [UIntPtr]::Zero
  [Win32.NativeMethods]::SendMessageTimeout($HWND_BROADCAST,
    $WM_SETTINGCHANGE,
    [UIntPtr]::Zero,
    "Environment",
    2,
    5000,
    [ref] $result
  ) | Out-Null
}

function Write-Env {
  param([String]$Key, [String]$Value)

  $RegisterKey = Get-Item -Path 'HKCU:'

  $EnvRegisterKey = $RegisterKey.OpenSubKey('Environment', $true)
  if ($null -eq $Value) {
    $EnvRegisterKey.DeleteValue($Key)
  } else {
    $RegistryValueKind = if ($Value.Contains('%')) {
      [Microsoft.Win32.RegistryValueKind]::ExpandString
    } elseif ($EnvRegisterKey.GetValue($Key)) {
      $EnvRegisterKey.GetValueKind($Key)
    } else {
      [Microsoft.Win32.RegistryValueKind]::String
    }
    $EnvRegisterKey.SetValue($Key, $Value, $RegistryValueKind)
  }

  Publish-Env
}

function Get-Env {
  param([String] $Key)

  $RegisterKey = Get-Item -Path 'HKCU:'
  $EnvRegisterKey = $RegisterKey.OpenSubKey('Environment')
  $EnvRegisterKey.GetValue($Key, $null, [Microsoft.Win32.RegistryValueOptions]::DoNotExpandEnvironmentNames)
}

function Install-ProwoSetup {
  param(
    [string]$Version
  );

  # if a semver is given, we need to adjust it to this format: v0.0.0
  if ($Version -match "^\d+\.\d+\.\d+$") {
    $Version = "v$Version"
  }

  $Arch = "x64"
  $Target = "x86_64-pc-windows-msvc"

  $ProwoSetupRoot = if ($env:PROWO_INSTALL) { $env:PROWO_INSTALL } else { "${Home}\.prowo-setup" }
  $ProwoSetupBin = mkdir -Force "${ProwoSetupRoot}\bin"

  try {
    Remove-Item "${ProwoSetupBin}\prowo-setup.exe" -Force
  } catch [System.Management.Automation.ItemNotFoundException] {
    # ignore
  } catch [System.UnauthorizedAccessException] {
    $openProcesses = Get-Process -Name prowo-setup | Where-Object { $_.Path -eq "${ProwoSetupBin}\prowo-setup.exe" }
    if ($openProcesses.Count -gt 0) {
      Write-Output "Install Failed - An older installation exists and is open. Please close open prowo-setup processes and try again."
      return 1
    }
    Write-Output "Install Failed - An unknown error occurred while trying to remove the existing installation"
    Write-Output $_
    return 1
  } catch {
    Write-Output "Install Failed - An unknown error occurred while trying to remove the existing installation"
    Write-Output $_
    return 1
  }

  $BaseURL = "https://github.com/djl-foundation/projektwoche/releases"
  $URL = "$BaseURL/$(if ($Version -eq "latest") { "latest/download" } else { "download/$Version" })/prowo-setup-$Target.zip"

  $ZipPath = "${ProwoSetupBin}\prowo-setup-$Target.zip"

  $DisplayVersion = $(
    if ($Version -eq "latest") { "prowo-setup" }
    elseif ($Version -match "^v\d+\.\d+\.\d+$") { "prowo-setup $Version" }
    else { "prowo-setup tag='${Version}'" }
  )

  $null = mkdir -Force $ProwoSetupBin
  Remove-Item -Force $ZipPath -ErrorAction SilentlyContinue

  # curl.exe is faster than PowerShell 5's 'Invoke-WebRequest'
  # note: 'curl' is an alias to 'Invoke-WebRequest'. so the exe suffix is required
  if (-not $DownloadWithoutCurl) {
    curl.exe "-#SfLo" "$ZipPath" "$URL" 
  }
  if ($DownloadWithoutCurl -or ($LASTEXITCODE -ne 0)) {
    Write-Warning "The command 'curl.exe $URL -o $ZipPath' exited with code ${LASTEXITCODE}`nTrying an alternative download method..."
    try {
      # Use Invoke-RestMethod instead of Invoke-WebRequest because Invoke-WebRequest breaks on
      # some machines
      Invoke-RestMethod -Uri $URL -OutFile $ZipPath
    } catch {
      Write-Output "Install Failed - could not download $URL"
      Write-Output "The command 'Invoke-RestMethod $URL -OutFile $ZipPath' exited with code ${LASTEXITCODE}`n"
      return 1
    }
  }

  if (!(Test-Path $ZipPath)) {
    Write-Output "Install Failed - could not download $URL"
    Write-Output "The file '$ZipPath' does not exist. Did an antivirus delete it?`n"
    return 1
  }

  try {
    $lastProgressPreference = $global:ProgressPreference
    $global:ProgressPreference = 'SilentlyContinue';
    Expand-Archive "$ZipPath" "$ProwoSetupBin" -Force
    $global:ProgressPreference = $lastProgressPreference
    if (!(Test-Path "${ProwoSetupBin}\prowo-setup.exe")) {
      throw "The file '${ProwoSetupBin}\prowo-setup.exe' does not exist. Download is corrupt or intercepted Antivirus?`n"
    }
  } catch {
    Write-Output "Install Failed - could not unzip $ZipPath"
    Write-Error $_
    return 1
  }

  Remove-Item $ZipPath -Force

  $ProwoSetupVersion = "$(& "${ProwoSetupBin}\prowo-setup.exe" --version 2>&1)"
  if ($LASTEXITCODE -eq 3221225781 -or $LASTEXITCODE -eq -1073741515) { # STATUS_DLL_NOT_FOUND
    Write-Output "Install Failed - You are missing the Visual C++ Redistributable required to run prowo-setup.exe"
    Write-Output "Please install the Visual C++ Redistributable from Microsoft:"
    Write-Output "See https://learn.microsoft.com/cpp/windows/latest-supported-vc-redist"
    Write-Output "Direct Download -> https://aka.ms/vs/17/release/vc_redist.x64.exe`n"
    Write-Output "After installing the redistributable, try running the installer again.`n"
    return 1
  }
  if ($LASTEXITCODE -ne 0) {
    Write-Output "Install Failed - could not verify prowo-setup.exe"
    Write-Output "The command '${ProwoSetupBin}\prowo-setup.exe --version' exited with code ${LASTEXITCODE}`n"
    return 1
  }

  $C_RESET = [char]27 + "[0m"
  $C_GREEN = [char]27 + "[1;32m"

  Write-Output "${C_GREEN}prowo-setup ${ProwoSetupVersion} was installed successfully!${C_RESET}"
  Write-Output "The binary is located at ${ProwoSetupBin}\prowo-setup.exe`n"

  $hasExistingOther = $false;
  try {
    $existing = Get-Command prowo-setup -ErrorAction SilentlyContinue
    if ($existing -and $existing.Source -ne "${ProwoSetupBin}\prowo-setup.exe") {
      Write-Warning "Note: Another prowo-setup.exe is already in %PATH% at $($existing.Source)`nTyping 'prowo-setup' in your terminal will not use what was just installed.`n"
      $hasExistingOther = $true;
    }
  } catch {}

  if (-not $NoRegisterInstallation) {
    $rootKey = $null
    try {
      $RegistryKey = "HKCU:\Software\Microsoft\Windows\CurrentVersion\Uninstall\ProwoSetup"  
      $rootKey = New-Item -Path $RegistryKey -Force
      New-ItemProperty -Path $RegistryKey -Name "DisplayName" -Value "prowo-setup" -PropertyType String -Force | Out-Null
      New-ItemProperty -Path $RegistryKey -Name "InstallLocation" -Value "${ProwoSetupRoot}" -PropertyType String -Force | Out-Null
      New-ItemProperty -Path $RegistryKey -Name "DisplayIcon" -Value $ProwoSetupBin\prowo-setup.exe -PropertyType String -Force | Out-Null
      New-ItemProperty -Path $RegistryKey -Name "UninstallString" -Value "powershell -c `"Remove-Item -Recurse -Force `'$ProwoSetupRoot`'`"" -PropertyType String -Force | Out-Null
    } catch {
      if ($rootKey -ne $null) {
        Remove-Item -Path $RegistryKey -Force
      }
    }
  }

  if(!$hasExistingOther) {
    # Only try adding to path if there isn't already a prowo-setup.exe in the path
    $Path = (Get-Env -Key "Path") -split ';'
    if ($Path -notcontains $ProwoSetupBin) {
      if (-not $NoPathUpdate) {
        $Path += $ProwoSetupBin
        Write-Env -Key 'Path' -Value ($Path -join ';')
        $env:PATH = $Path -join ';'
      } else {
        Write-Output "Skipping adding '${ProwoSetupBin}' to the user's %PATH%`n"
      }
    }

    Write-Output "To get started, restart your terminal/editor, then type `"prowo-setup`"`n"
  }

  $LASTEXITCODE = 0;
}

Install-ProwoSetup -Version $Version