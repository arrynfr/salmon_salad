use core::ascii;

#[derive(Debug,Copy,Clone)]
#[repr(C)]
pub enum EfiMemoryType {
    EfiReservedMemoryType,
    EfiLoaderCode,
    EfiLoaderData,
    EfiBootServicesCode,
    EfiBootServicesData,
    EfiRuntimeServicesCode,
    EfiRuntimeServicesData,
    EfiConventionalMemory,
    EfiUnusableMemory,
    EfiACPIReclaimMemory,
    EfiACPIMemoryNVS,
    EfiMemoryMappedIO,
    EfiMemoryMappedIOPortSpace,
    EfiPalCode,
    EfiPersistentMemory,
    EfiUnacceptedMemoryType,
    EfiMaxMemoryType
}

#[repr(C)]
pub enum EfiAllocateType {
    AllocateAnyPages,
    AllocateMaxAddress,
    AllocateAddress,
    MaxAllocateType
 }

pub type BOOLEAN = bool;
pub type INTN = isize;
pub type UINTN = usize;
pub type INT8 = i8;
pub type UINT8 = u8;
pub type INT16 = i16;
pub type UINT16 = u16;
pub type INT32 = i32;
pub type UINT32 = u32;
pub type INT64 = i64;
pub type UINT64 = u64;
pub type INT128 = i128;
pub type UINT128 = u128;
pub type CHAR8 = ascii::Char;
//pub type CHAR16 = char; this is not correct since UTF-8 != UCS-2
pub type VOID = u8;
//pub type EFI_GUID;
pub type EfiStatus = usize;
 
pub type EfiPhysicalAddress = u64;
pub type EfiVirtualAddress = u64;