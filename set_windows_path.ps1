# https://stackoverflow.com/a/29109007
$PATH = [Environment]::GetEnvironmentVariable("PATH", "User")
$opt_path = "C:\opt"
if( $PATH -notlike "*"+$opt_path+"*" ){
    [Environment]::SetEnvironmentVariable("PATH", "$PATH;$opt_path", "User")
}