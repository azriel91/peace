# https://stackoverflow.com/questions/4998173/how-do-i-write-to-standard-error-in-powershell
function Write-StdErr {
  param ([PSObject] $InputObject)
  $outFunc = if ($Host.Name -eq 'ConsoleHost') {
    [Console]::Error.WriteLine
  } else {
    $host.ui.WriteErrorLine
  }
  if ($InputObject) {
    [void] $outFunc.Invoke($InputObject.ToString())
  } else {
    [string[]] $lines = @()
    $Input | % { $lines += $_.ToString() }
    [void] $outFunc.Invoke($lines -join "`r`n")
  }
}

if (Test-Path -Path 'test_file' -PathType Leaf) {
    # state
    Write-Information 'exists'

    # display string
    Write-StdErr '`test_file` exists'
}
else {
    # state
    Write-Information 'not_exists'

    # display string
    Write-StdErr '`test_file` does not exist'
}
