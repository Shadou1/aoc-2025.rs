if ($args.Count -ne 2) {
  Write-Output 'Usage: .\new-day.ps1 {day} {name}'
  Exit 1
}

$Day = '{0:d2}' -f $args[0]
$Name = $args[1]
$Folder = "${Day}-${Name}"

Copy-Item -Recurse -Path ".\00-template\" -Destination ".\${Folder}"

(Get-Content -Path ".\${Folder}\Cargo.toml") -replace "template","${Name}" | Set-Content -Path ".\${Folder}\Cargo.toml" 
(Get-Content -Path ".\${Folder}\src\bin\part1.rs") -replace "template","${Name}" | Set-Content -Path ".\${Folder}\src\bin\part1.rs" 
(Get-Content -Path ".\${Folder}\src\bin\part2.rs") -replace "template","${Name}" | Set-Content -Path ".\${Folder}\src\bin\part2.rs" 
