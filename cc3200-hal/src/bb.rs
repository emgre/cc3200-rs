//! Code to handle bit-banding.
//!
//! Bit-banding is where the SoC maps each 8-bit byte to 8 consecutive 32-bit
//! words. Writing a 1 to that word sets the matching bit. Writing a 0 clears
//! the matching bit. It means you can perform atomic bit set/clear; i.e.
//! without a read-modify-write.

use core::ptr::write_volatile;

/// Sets or clears a bit at the given address atomically, using the bit-banding
/// feature. We take a const pointer and mutate it, but that's because the
/// svd2rust crate will only give us const pointers.
pub unsafe fn change_bit<T>(address: *const T, bit: u8, value: bool) {
    let address = address as u32;
    let bit_word = ref_to_bitband(address, bit);
    write_volatile(bit_word, if value { 0x01 } else { 0x00 });
}
