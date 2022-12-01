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

$current=$args[0]
$desired=$args[1]

switch ("$current $desired")
{
    'not_exists exists' {
        # state
        Write-Information 'creation_required'

        # display string
        Write-StdErr '`test_file` will be created'
    }
    'exists exists' {
        # state
        Write-Information 'exists_sync'

        # display string
        Write-StdErr 'Nothing to do'
    }
}
