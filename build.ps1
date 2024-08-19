cargo build --locked --release --target x86_64-pc-windows-msvc

$target = "x86_64-pc-windows-msvc"

New-Item -Path "target\$target\release\" -Name "bin" -ItemType "directory" -ErrorAction SilentlyContinue
Copy-Item "C:\gtk-build\gtk\x64\release\bin\*.dll" -Destination "target\$target\release\bin"

# https://wixtoolset.org/docs/v3/overview/heat/
heat dir target/x86_64-pc-windows-msvc/release/bin -cg GtkDependencies -dr Bin -ag -sfrag -srd -sreg -out wix/gtk.wxs

# $version = "0.1.0"
Remove-Item "target\wix\*" -Recurse -ErrorAction SilentlyContinue
candle wix\main.wxs wix\gtk.wxs -dVersion="$version" -dCargoTargetBinDir="target\$target\release\" -o "target\wix\"
Copy-Item "target\$target\release\bin\*.dll" -Destination "target\wix"
light -b "target\wix" -ext WixUIExtension -out "target\wix\hitokage-$version-$target.msi" ".\target\wix\main.wixobj" ".\target\wix\gtk.wixobj"
Compress-Archive .\target\x86_64-pc-windows-msvc\release\*.exe hitokage-nightly-x86_64-pc-windows-msvc.zip -Force
Write-Output "$((Get-FileHash hitokage-nightly-x86_64-pc-windows-msvc.zip).Hash.ToLower())  hitokage-nightly-x86_64-pc-windows-msvc.zip" > checksums.txt