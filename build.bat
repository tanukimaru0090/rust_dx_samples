cargo build --release
COPY "./DxLib_x64.dll" "./target/release/"
@ping 127.0.0.1 -n 6 > nul
