param(
	[Parameter()]
	[switch]$release
)

if($release) {
	cargo build --release
	$binaryPath = "release"
} else {
	cargo build
	$binaryPath = "debug"
}
if(!$LASTEXITCODE) {
	qemu-system-x86_64 `
	-m 4096 `
	-nographic `
	-bios OVMF.fd `
	-device driver=e1000,netdev=n0,bootindex=1 `
	-netdev user,id=n0,tftp=target/x86_64-unknown-uefi/$binaryPath,bootfile=salmon_salad.efi
}
