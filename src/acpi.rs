#[derive(Debug)]
#[repr(C, packed)]
pub struct _Rsdp {
	pub signature: [u8; 8],
	pub checksum: u8,
	pub oemid: [u8; 6],
	pub revision: u8,
	pub rsdt_addr: u32,
	pub len: u32,
	pub xsdt_addr: *const usize,
	pub xchksum: u8,
	pub reserved: [u8; 3]
}

//pub fn get_acpi_table() -> Rsdp {}
