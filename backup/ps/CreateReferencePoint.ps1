[CmdletBinding()]
param(
    [Parameter(Mandatory)]
    [string] $VmName,

    [string] $ComputerName = $env:COMPUTERNAME
)

$ErrorActionPreference = "Stop"
$namespace = "root\virtualization\v2"

# Hyper-V enum values:
# ConsistencyLevel Crash = 1
# ReferencePointType RCTBased = 2
$referencePointSettings = @'
<INSTANCE CLASSNAME="Msvm_VirtualSystemReferencePointSettingData">
  <PROPERTY NAME="ConsistencyLevel" TYPE="uint8">
    <VALUE>1</VALUE>
  </PROPERTY>
</INSTANCE>
'@

$sessionOption = New-CimSessionOption -Protocol Dcom
$cimSession = New-CimSession -ComputerName $ComputerName -SessionOption $sessionOption

try {
    Write-Host "Connecting to Hyper-V WMI namespace '$namespace' on '$ComputerName'..."

    $vm = Get-CimInstance `
    -CimSession $cimSession `
    -Namespace $namespace `
    -ClassName Msvm_ComputerSystem `
    -Filter "ElementName='$VmName' AND Caption='Virtual Machine'"

    if (-not $vm) {
        throw "Hyper-V VM '$VmName' was not found."
    }

    Write-Host "Found VM: $($vm.ElementName) [$($vm.Name)]"

    $service = Get-CimInstance `
    -CimSession $cimSession `
    -Namespace $namespace `
    -ClassName Msvm_VirtualSystemReferencePointService

    if (-not $service) {
        throw "Msvm_VirtualSystemReferencePointService was not found."
    }

    Write-Host "Calling CreateReferencePoint (Crash, RCTBased)..."
    $result = Invoke-CimMethod `
    -CimSession $cimSession `
    -InputObject $service `
    -MethodName CreateReferencePoint `
    -Arguments @{
        AffectedSystem = $vm
        ReferencePointSettings = $referencePointSettings
        ReferencePointType = [uint16]2
        ResultingReferencePoint = $null
    }

    Write-Host "CreateReferencePoint response:"
    $result | Format-List *

if ($result.ReturnValue -eq 0) {
    Write-Host "Reference point created synchronously."
}
elseif ($result.ReturnValue -eq 4096) {
    Write-Host "Reference point creation started asynchronously."

        $job = $result.Job

        if ($job -is [string]) {
            $jobPath = $job.Replace("'", "''")
            $job = Get-CimInstance -CimSession $cimSession -Namespace $namespace -Query `
                "SELECT * FROM Msvm_ConcreteJob WHERE __PATH='$jobPath'"
        }

        do {
            Start-Sleep -Seconds 1
            $job = Get-CimInstance -CimSession $cimSession -Namespace $namespace -ClassName Msvm_ConcreteJob `
                -Filter "InstanceID='$($job.InstanceID)'"
            Write-Host "JobState=$($job.JobState) PercentComplete=$($job.PercentComplete)"
        } while ($job.JobState -in 2, 3, 4, 6)

        if ($job.JobState -ne 7) {
            Write-Host "Complete CreateReferencePoint job details:"
            $job | Format-List *

            $currentVm = Get-CimInstance `
                -CimSession $cimSession `
                -Namespace $namespace `
                -ClassName Msvm_ComputerSystem `
                -Filter "Name='$($vm.Name)'"

            Write-Host "Current VM state:"
            $currentVm |
                Select-Object ElementName,
                    EnabledState,
                    OperationalStatus,
                    HealthState,
                    Status,
                    StatusDescriptions |
                Format-List

            throw "Reference point creation failed. JobState=$($job.JobState), ErrorCode=$($job.ErrorCode), ErrorDescription=$($job.ErrorDescription)"
        }
    }
    else {
        throw "CreateReferencePoint failed. ReturnValue=$($result.ReturnValue)"
    }

    $referencePoints = Get-CimInstance `
    -CimSession $cimSession `
    -Namespace $namespace `
    -ClassName Msvm_VirtualSystemReferencePoint

    $createdReferencePoints = $referencePoints |
    Where-Object {
        $_.VirtualSystemIdentifier -eq $vm.Name -and
        $_.ReferencePointType -eq 2
    } |
    Select-Object @{Name = "Path"; Expression = { $_.CimSystemProperties.Path }}, InstanceID, ReferencePointType,
        VirtualSystemIdentifier,
        ResilientChangeTrackingIdentifiers,
        VirtualDiskIdentifiers,
        HasAssociatedData

    Write-Host "RCT reference points for '$VmName':"
    $createdReferencePoints | Format-List
}
finally {
    if ($cimSession) {
        Remove-CimSession -CimSession $cimSession
    }
}