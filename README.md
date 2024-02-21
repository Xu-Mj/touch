# TOUCH ON WINDOWS
A command line tool to simulate touch on Windows.
## Installation
download the latest release from [here](https://github.com/0x0000007f/touch/releases/latest).
set the path to the downloaded file in your PATH variable.

**Example:**
The downloaded file path is `D:\touch\touch.exe`
execute the command in power shell to set the path(replace 'D:\touch\' to your installation):

```
$addPath='D:\touch\'; $target='User'; $path = [Environment]::GetEnvironmentVariable('Path', $target); if($path -match ";$"){ $newPath = $path + $addPath; } else { $newPath = $path + ';' + $addPath; } [Environment]::SetEnvironmentVariable('Path', $newPath, $target)
```

## Usage
```

Usage: touch.exe [OPTIONS] [filename]...

Arguments:
  [filename]...

Options:
  -f, --force    force overwrite
  -h, --help     Print help
  -V, --version  Print version

```
