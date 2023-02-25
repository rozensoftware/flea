#Installs The Flea Server on the computer
#Enable running PowerShell scripts by typing Set-ExecutionPolicy and selecting Unrestricted

#Script variables

$CurrentDirectory = Get-Location
$UserDirectory = $env:APPDATA
$Flea = $UserDirectory + "\flea\flea.exe"

#---- FUNCTIONS ----

function CopyFiles
{
    New-Item -Path $UserDirectory"\flea" -ItemType Directory -Force
    Get-ChildItem -Path $CurrentDirectory | Copy-Item -Destination $UserDirectory"\flea\" -Force -Exclude *.ps1
    "`nAll files have been copied.`n"
}

function RegisterFleaAutoRun
{
    # Name of the task
    $TaskName = "FleaRule"

    $UserName= $env:USERNAME

    # Set task action
    $TaskAction = New-ScheduledTaskAction -Execute $Flea

    # Create a trigger that will fire the task at log on
    $TaskTrigger = New-ScheduledTaskTrigger -AtLogOn -User $UserName

    # Register the task
    Register-ScheduledTask -Action $TaskAction -Trigger $TaskTrigger -User $UserName -RunLevel Highest -TaskName $TaskName -Force
    "`nThe FleaRule autorun task registered."

    #Unregister-ScheduledTask -TaskName "TheEyeRule" -Confirm:$false
}

function RunFlea
{
    & $Flea
    "`nThe Flea Server installed."
}

function AddFleaToFirewall
{
    & Netsh.exe advfirewall firewall add rule name="Flea TCP inbound" program=$Flea protocol=tcp dir=in enable=yes action=allow profile=Private
    & Netsh.exe advfirewall firewall add rule name="Flea UDP inbound" program=$Flea protocol=udp dir=in enable=yes action=allow profile=Private
    & Netsh.exe advfirewall firewall add rule name="Flea TCP outbound" program=$Flea protocol=tcp dir=out enable=yes action=allow profile=Private
    & Netsh.exe advfirewall firewall add rule name="Flea UDP outbound" program=$Flea protocol=udp dir=out enable=yes action=allow profile=Private
    "`nThe Flea Server has been added to firewall."
}

#---- START ----

Write-Verbose -Message "Starting The Flea Server installer..." -Verbose

# Verify that user running the script is an administrator
$IsAdmin=[Security.Principal.WindowsIdentity]::GetCurrent()
If ((New-Object Security.Principal.WindowsPrincipal $IsAdmin).IsInRole([Security.Principal.WindowsBuiltinRole]::Administrator) -eq $FALSE)
{
    Write-Host "`nSTOP: You are NOT a local administrator. Run this script after logging on with a local administrator account." -ForegroundColor Yellow
    exit
}

CopyFiles
AddFleaToFirewall
RegisterFleaAutoRun
#RunFlea
