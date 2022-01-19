# reportlinkfix

Authentication workaround for N-Central Report Manager. Takes the link that N-Central creates for Report Manager and outputs a working link.

## Usage

1. Copy the Report Manager link that is in N-Central (this link changes each time you login)
2. Run reportlinkfix (this program)
3. Paste the Report Manager link into the terminal and press enter
4. Copy the fixed link and paste it into your web browser

## Info

*.amp is the N-Central Automation for "installing" reportlinkfix via N-Central

set_windows_path.ps1 is an example script for appending to the user path environment variable in Windows. The built-in set environment variable automation in N-Central does not append.

The work around in the caddy folder does not work currently. Added it in case it can be made to work in the future.