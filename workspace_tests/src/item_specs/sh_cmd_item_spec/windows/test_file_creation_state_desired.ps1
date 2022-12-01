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

# state
Write-Information 'exists'

# display string
Write-StdErr '`test_file` exists'
