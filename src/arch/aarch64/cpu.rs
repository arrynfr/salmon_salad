use core::arch::asm;

const _FEAT_AES: u64 = 0b1111 << 4;
const _FEAT_SHA1: u64 = 0b1111 << 8;
const _FEAT_SHA2: u64 = 0b1111 << 12;
const _FEAT_CRC32: u64 = 0b1111 << 16;
const _FEAT_LSE: u64 = 0b1111 << 20;
const _FEAT_TME: u64 = 0b1111 << 24;
const _FEAT_RDM: u64 = 0b1111 << 28;
const _FEAT_SHA3: u64 = 0b1111 << 32;
const _FEAT_SM3: u64 = 0b1111 << 36;
const _FEAT_SM4: u64 = 0b1111 << 40;
const _FEAT_DP: u64 = 0b1111 << 44;
const _FEAT_FHM: u64 = 0b1111 << 48;
const _FEAT_TS: u64 = 0b1111 << 52;
const _FEAT_TLB: u64 = 0b1111 << 56;
const _FEAT_RNDR: u64 = 0b1111 << 60;

#[derive(Debug)]
struct CpuFeatures {
    feat_aes: u8,
    feat_sha1: u8,
    feat_sha2: u8,
    feat_crc32: u8,
    feat_lse: u8,
    feat_tme: u8,
    feat_rdm: u8,
    feat_sha3: u8,
    feat_sm3: u8,
    feat_sm4: u8,
    feat_dp: u8,
    feat_fhm: u8,
    feat_ts: u8,
    feat_tlb: u8,
    feat_rndr: u8
}

#[derive(Debug)]
struct DFR0 {
    cop_dbg: u8,
    cop_s_dbg: u8,
    m_map_bg: u8,
    cop_trc: u8,
    m_map_trc: u8,
    m_prof_dbg: u8,
    perf_mon: u8,
    trace_filt: u8
}

pub fn get_cpu_features() {
    let mut aa64isar0: u64;
    let mut feats: [u8; 15] = [0; 15];
    unsafe { asm!("mrs {}, ID_AA64ISAR0_EL1", out(reg) aa64isar0); }
    for x in 0..feats.len() {
        aa64isar0 = aa64isar0 >> 4;
        feats[x] = (aa64isar0 & 0xF) as u8;
    }
    let s =
    unsafe { core::mem::transmute::<[u8; 15], CpuFeatures>(feats) };
    println!("Detected CPU caps: \r\n{s:#?}");
}

pub fn get_cpu_features2() {
    let mut aa64dfr0: u64;
    let mut feats: [u8; 8] = [0; 8];
    unsafe { asm!("mrs {}, ID_DFR0_EL1", out(reg) aa64dfr0); }
    for x in 0..feats.len() {
        aa64dfr0 = aa64dfr0 >> 4;
        feats[x] = (aa64dfr0 & 0xF) as u8;
    }
    let s =
    unsafe { core::mem::transmute::<[u8; 8], DFR0>(feats) };
    println!("Detected CPU caps2: \r\n{s:#?}");
}

pub fn pmu() {
    unimplemented!("QEMU does not support PMU, so support will be developed on hardware");
}