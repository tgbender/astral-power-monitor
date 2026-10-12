# Install this user's tray monitor, always disarmed. No elevation or scheduled task.
[CmdletBinding()]
param()
$ErrorActionPreference = 'Stop'
$taskSource = Join-Path $PSScriptRoot '..\dist\AstralPowerMonitor.exe'
if (-not (Test-Path -LiteralPath $taskSource -PathType Leaf)) {
    throw 'Build and copy the release executable to dist\AstralPowerMonitor.exe first.'
}
$taskInstallFolder = Join-Path ([Environment]::GetFolderPath('LocalApplicationData')) 'AstralPowerMonitor'
$taskTarget = Join-Path $taskInstallFolder 'AstralPowerMonitor.exe'
$taskDesktopFolder = [Environment]::GetFolderPath('Desktop')
$taskDesktopShortcutPath = Join-Path $taskDesktopFolder 'Astral Power Monitor.lnk'
$taskStartupFolder = [Environment]::GetFolderPath('Startup')
if ([string]::IsNullOrWhiteSpace($taskStartupFolder)) { throw 'Windows Startup folder unavailable.' }
$taskShortcutPath = Join-Path $taskStartupFolder 'Astral Power Monitor.lnk'
$taskShell = New-Object -ComObject WScript.Shell
$taskKnownTargets = @($taskTarget)
foreach ($taskPackage in @(Get-ChildItem -LiteralPath (Join-Path $env:USERPROFILE 'AppData\Local\Packages') -Directory -Filter 'OpenAI.Codex_*' -ErrorAction SilentlyContinue)) {
    $taskKnownTargets += Join-Path $taskPackage.FullName 'LocalCache\Local\AstralPowerMonitor\AstralPowerMonitor.exe'
}
foreach ($taskPath in @($taskShortcutPath, $taskDesktopShortcutPath)) {
    if (Test-Path -LiteralPath $taskPath) {
        $taskExisting = $taskShell.CreateShortcut($taskPath)
        if ($taskExisting.TargetPath -notin $taskKnownTargets -or $taskExisting.Arguments -notin @('', '--tray')) {
            throw "A different shortcut already uses this name: $taskPath. It has been left unchanged."
        }
    }
}
New-Item -ItemType Directory -Path $taskInstallFolder -Force | Out-Null
# Packaged parent processes can redirect AppData despite reporting the normal path.
# Refuse that environment before copying anything or changing shortcuts.
$taskProbePath = Join-Path $taskInstallFolder ('.install-path-probe-' + [guid]::NewGuid().ToString('N'))
try {
    $taskProbe = [IO.File]::Open($taskProbePath, [IO.FileMode]::CreateNew, [IO.FileAccess]::Write, [IO.FileShare]::None)
    $taskProbe.Close()
    # NTFS reports the actual on-disk name; Win32 final-path APIs can retain the
    # virtual name. This zero-byte probe is removed in both success/failure cases.
    $taskPhysicalNames = @(& "$env:SystemRoot\System32\fsutil.exe" hardlink list $taskProbePath 2>&1)
    if ($LASTEXITCODE -ne 0) { throw 'Cannot verify the physical installation folder.' }
    $taskExpectedName = [IO.Path]::GetFullPath((Join-Path $env:USERPROFILE ('AppData\Local\AstralPowerMonitor\' + [IO.Path]::GetFileName($taskProbePath)))).Substring(2)
    if ($taskExpectedName -notin $taskPhysicalNames) {
        throw 'Windows is redirecting this installation into a packaged app folder. Run this script from ordinary Windows PowerShell; no executable or shortcuts were changed.'
    }
} finally {
    if (Test-Path -LiteralPath $taskProbePath) { Remove-Item -LiteralPath $taskProbePath }
}
# Refuse to overwrite a running image: quit the monitor before installing an update.
Copy-Item -LiteralPath $taskSource -Destination $taskTarget -Force
foreach ($taskPath in @($taskShortcutPath, $taskDesktopShortcutPath)) {
    $taskShortcut = $taskShell.CreateShortcut($taskPath)
    $taskShortcut.TargetPath = $taskTarget
    $taskShortcut.Arguments = if ($taskPath -eq $taskShortcutPath) { '--tray' } else { '' }
    $taskShortcut.WorkingDirectory = $taskInstallFolder
    $taskShortcut.Description = 'Per-pin monitoring and audio alerts; protection starts disarmed.'
    $taskShortcut.IconLocation = "$taskTarget,0"
    $taskShortcut.WindowStyle = 7
    $taskShortcut.Save()
    $taskVerified = $taskShell.CreateShortcut($taskPath)
    if ($taskVerified.TargetPath -ne $taskTarget -or $taskVerified.Arguments -ne $taskShortcut.Arguments -or $taskVerified.IconLocation -ne "$taskTarget,0") {
        throw "Shortcut verification failed: $taskPath"
    }
}
Write-Output "Installed: $taskTarget"
Write-Output "Startup: $taskShortcutPath"
Write-Output "Desktop: $taskDesktopShortcutPath"
Write-Output 'Starts disarmed with audio warnings enabled. No automatic arming or restart task.'
