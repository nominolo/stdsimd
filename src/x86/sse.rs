use simd_llvm::simd_shuffle4;
use v128::*;
use v64::f32x2;
use std::os::raw::c_void;
use std::mem;
use std::ptr;

#[cfg(test)]
use stdsimd_test::assert_instr;

/// Adds the first component of `a` and `b`, the other components are copied
/// from `a`.
#[inline(always)]
#[target_feature = "+sse"]
#[cfg_attr(test, assert_instr(addss))]
pub unsafe fn _mm_add_ss(a: f32x4, b: f32x4) -> f32x4 {
    addss(a, b)
}

/// Adds f32x4 vectors.
#[inline(always)]
#[target_feature = "+sse"]
#[cfg_attr(test, assert_instr(addps))]
pub unsafe fn _mm_add_ps(a: f32x4, b: f32x4) -> f32x4 {
    a + b
}

/// Subtracts the first component of `b` from `a`, the other components are
/// copied from `a`.
#[inline(always)]
#[target_feature = "+sse"]
#[cfg_attr(test, assert_instr(subss))]
pub unsafe fn _mm_sub_ss(a: f32x4, b: f32x4) -> f32x4 {
    subss(a, b)
}

/// Subtracts f32x4 vectors.
#[inline(always)]
#[target_feature = "+sse"]
#[cfg_attr(test, assert_instr(subps))]
pub unsafe fn _mm_sub_ps(a: f32x4, b: f32x4) -> f32x4 {
    a - b
}

/// Multiplies the first component of `a` and `b`, the other components are
/// copied from `a`.
#[inline(always)]
#[target_feature = "+sse"]
#[cfg_attr(test, assert_instr(mulss))]
pub unsafe fn _mm_mul_ss(a: f32x4, b: f32x4) -> f32x4 {
    mulss(a, b)
}

/// Multiplies f32x4 vectors.
#[inline(always)]
#[target_feature = "+sse"]
#[cfg_attr(test, assert_instr(mulps))]
pub unsafe fn _mm_mul_ps(a: f32x4, b: f32x4) -> f32x4 {
    a * b
}

/// Divides the first component of `b` by `a`, the other components are
/// copied from `a`.
#[inline(always)]
#[target_feature = "+sse"]
#[cfg_attr(test, assert_instr(divss))]
pub unsafe fn _mm_div_ss(a: f32x4, b: f32x4) -> f32x4 {
    divss(a, b)
}

/// Divides f32x4 vectors.
#[inline(always)]
#[target_feature = "+sse"]
#[cfg_attr(test, assert_instr(divps))]
pub unsafe fn _mm_div_ps(a: f32x4, b: f32x4) -> f32x4 {
    a / b
}

/// Return the square root of the first single-precision (32-bit)
/// floating-point element in `a`, the other elements are unchanged.
#[inline(always)]
#[target_feature = "+sse"]
#[cfg_attr(test, assert_instr(sqrtss))]
pub unsafe fn _mm_sqrt_ss(a: f32x4) -> f32x4 {
    sqrtss(a)
}

/// Return the square root of packed single-precision (32-bit) floating-point
/// elements in `a`.
#[inline(always)]
#[target_feature = "+sse"]
#[cfg_attr(test, assert_instr(sqrtps))]
pub unsafe fn _mm_sqrt_ps(a: f32x4) -> f32x4 {
    sqrtps(a)
}

/// Return the approximate reciprocal of the first single-precision
/// (32-bit) floating-point element in `a`, the other elements are unchanged.
#[inline(always)]
#[target_feature = "+sse"]
#[cfg_attr(test, assert_instr(rcpss))]
pub unsafe fn _mm_rcp_ss(a: f32x4) -> f32x4 {
    rcpss(a)
}

/// Return the approximate reciprocal of packed single-precision (32-bit)
/// floating-point elements in `a`.
#[inline(always)]
#[target_feature = "+sse"]
#[cfg_attr(test, assert_instr(rcpps))]
pub unsafe fn _mm_rcp_ps(a: f32x4) -> f32x4 {
    rcpps(a)
}

/// Return the approximate reciprocal square root of the fist single-precision
/// (32-bit) floating-point elements in `a`, the other elements are unchanged.
#[inline(always)]
#[target_feature = "+sse"]
#[cfg_attr(test, assert_instr(rsqrtss))]
pub unsafe fn _mm_rsqrt_ss(a: f32x4) -> f32x4 {
    rsqrtss(a)
}

/// Return the approximate reciprocal square root of packed single-precision
/// (32-bit) floating-point elements in `a`.
#[inline(always)]
#[target_feature = "+sse"]
#[cfg_attr(test, assert_instr(rsqrtps))]
pub unsafe fn _mm_rsqrt_ps(a: f32x4) -> f32x4 {
    rsqrtps(a)
}

/// Compare the first single-precision (32-bit) floating-point element of `a`
/// and `b`, and return the minimum value in the first element of the return
/// value, the other elements are copied from `a`.
#[inline(always)]
#[target_feature = "+sse"]
#[cfg_attr(test, assert_instr(minss))]
pub unsafe fn _mm_min_ss(a: f32x4, b: f32x4) -> f32x4 {
    minss(a, b)
}

/// Compare packed single-precision (32-bit) floating-point elements in `a` and
/// `b`, and return the corresponding minimum values.
#[inline(always)]
#[target_feature = "+sse"]
#[cfg_attr(test, assert_instr(minps))]
pub unsafe fn _mm_min_ps(a: f32x4, b: f32x4) -> f32x4 {
    minps(a, b)
}

/// Compare the first single-precision (32-bit) floating-point element of `a`
/// and `b`, and return the maximum value in the first element of the return
/// value, the other elements are copied from `a`.
#[inline(always)]
#[target_feature = "+sse"]
#[cfg_attr(test, assert_instr(maxss))]
pub unsafe fn _mm_max_ss(a: f32x4, b: f32x4) -> f32x4 {
    maxss(a, b)
}

/// Compare packed single-precision (32-bit) floating-point elements in `a` and
/// `b`, and return the corresponding maximum values.
#[inline(always)]
#[target_feature = "+sse"]
#[cfg_attr(test, assert_instr(maxps))]
pub unsafe fn _mm_max_ps(a: f32x4, b: f32x4) -> f32x4 {
    maxps(a, b)
}

/// Bitwise AND of packed single-precision (32-bit) floating-point elements.
#[inline(always)]
#[target_feature = "+sse"]
// i586 only seems to generate plain `and` instructions, so ignore it.
#[cfg_attr(all(test, any(target_arch = "x86_64", target_feature = "sse2")),
    assert_instr(andps))]
pub unsafe fn _mm_and_ps(a: f32x4, b: f32x4) -> f32x4 {
    let aa: i32x4 = mem::transmute(a);
    let bb: i32x4 = mem::transmute(b);
    mem::transmute(aa & bb)
}

/// Bitwise AND-NOT of packed single-precision (32-bit) floating-point elements.
///
/// Computes `!a & b` for each bit in `a` and `b`.
#[inline(always)]
#[target_feature = "+sse"]
// i586 only seems to generate plain `not` and `and` instructions, so ignore it.
#[cfg_attr(all(test, any(target_arch = "x86_64", target_feature = "sse2")),
    assert_instr(andnps))]
pub unsafe fn _mm_andnot_ps(a: f32x4, b: f32x4) -> f32x4 {
    let aa: i32x4 = mem::transmute(a);
    let bb: i32x4 = mem::transmute(b);
    mem::transmute(!aa & bb)
}

/// Bitwise OR of packed single-precision (32-bit) floating-point elements.
#[inline(always)]
#[target_feature = "+sse"]
// i586 only seems to generate plain `or` instructions, so we ignore it.
#[cfg_attr(all(test, any(target_arch = "x86_64", target_feature = "sse2")),
    assert_instr(orps))]
pub unsafe fn _mm_or_ps(a: f32x4, b: f32x4) -> f32x4 {
    let aa: i32x4 = mem::transmute(a);
    let bb: i32x4 = mem::transmute(b);
    mem::transmute(aa | bb)
}

/// Bitwise exclusive OR of packed single-precision (32-bit) floating-point
/// elements.
#[inline(always)]
#[target_feature = "+sse"]
// i586 only seems to generate plain `xor` instructions, so we ignore it.
#[cfg_attr(all(test, any(target_arch = "x86_64", target_feature = "sse2")),
    assert_instr(xorps))]
pub unsafe fn _mm_xor_ps(a: f32x4, b: f32x4) -> f32x4 {
    let aa: i32x4 = mem::transmute(a);
    let bb: i32x4 = mem::transmute(b);
    mem::transmute(aa ^ bb)
}

/// Compare the lowest `f32` of both inputs for equality. The lowest 32 bits of
/// the result will be `0xffffffff` if the two inputs are equal, or `0`
/// otherwise. The upper 96 bits of the result are the upper 96 bits of `a`.
#[inline(always)]
#[target_feature = "+sse"]
#[cfg_attr(test, assert_instr(cmpeqss))]
pub unsafe fn _mm_cmpeq_ss(a: f32x4, b: f32x4) -> f32x4 {
    cmpss(a, b, 0)
}

/// Compare the lowest `f32` of both inputs for less than. The lowest 32 bits of
/// the result will be `0xffffffff` if `a.extract(0)` is less than
/// `b.extract(0)`, or `0` otherwise. The upper 96 bits of the result are the
/// upper 96 bits of `a`.
#[inline(always)]
#[target_feature = "+sse"]
#[cfg_attr(test, assert_instr(cmpltss))]
pub unsafe fn _mm_cmplt_ss(a: f32x4, b: f32x4) -> f32x4 {
    cmpss(a, b, 1)
}

/// Compare the lowest `f32` of both inputs for less than or equal. The lowest
/// 32 bits of the result will be `0xffffffff` if `a.extract(0)` is less than or
/// equal `b.extract(0)`, or `0` otherwise. The upper 96 bits of the result are
/// the upper 96 bits of `a`.
#[inline(always)]
#[target_feature = "+sse"]
#[cfg_attr(test, assert_instr(cmpless))]
pub unsafe fn _mm_cmple_ss(a: f32x4, b: f32x4) -> f32x4 {
    cmpss(a, b, 2)
}

/// Compare the lowest `f32` of both inputs for greater than. The lowest 32 bits of
/// the result will be `0xffffffff` if `a.extract(0)` is greater than
/// `b.extract(0)`, or `0` otherwise. The upper 96 bits of the result are the
/// upper 96 bits of `a`.
#[inline(always)]
#[target_feature = "+sse"]
#[cfg_attr(test, assert_instr(cmpltss))]
pub unsafe fn _mm_cmpgt_ss(a: f32x4, b: f32x4) -> f32x4 {
    simd_shuffle4(a, cmpss(b, a, 1), [4, 1, 2, 3])
}

/// Compare the lowest `f32` of both inputs for greater than or equal. The
/// lowest 32 bits of the result will be `0xffffffff` if `a.extract(0)` is
/// greater than or equal `b.extract(0)`, or `0` otherwise. The upper 96 bits of
/// the result are the upper 96 bits of `a`.
#[inline(always)]
#[target_feature = "+sse"]
#[cfg_attr(test, assert_instr(cmpless))]
pub unsafe fn _mm_cmpge_ss(a: f32x4, b: f32x4) -> f32x4 {
    simd_shuffle4(a, cmpss(b, a, 2), [4, 1, 2, 3])
}

/// Compare the lowest `f32` of both inputs for inequality. The lowest 32 bits
/// of the result will be `0xffffffff` if `a.extract(0)` is not equal to
/// `b.extract(0)`, or `0` otherwise. The upper 96 bits of the result are the
/// upper 96 bits of `a`.
#[inline(always)]
#[target_feature = "+sse"]
#[cfg_attr(test, assert_instr(cmpneqss))]
pub unsafe fn _mm_cmpneq_ss(a: f32x4, b: f32x4) -> f32x4 {
    cmpss(a, b, 4)
}

/// Compare the lowest `f32` of both inputs for not-less-than. The lowest 32
/// bits of the result will be `0xffffffff` if `a.extract(0)` is not less than
/// `b.extract(0)`, or `0` otherwise. The upper 96 bits of the result are the
/// upper 96 bits of `a`.
#[inline(always)]
#[target_feature = "+sse"]
#[cfg_attr(test, assert_instr(cmpnltss))]
pub unsafe fn _mm_cmpnlt_ss(a: f32x4, b: f32x4) -> f32x4 {
    cmpss(a, b, 5)
}

/// Compare the lowest `f32` of both inputs for not-less-than-or-equal. The
/// lowest 32 bits of the result will be `0xffffffff` if `a.extract(0)` is not
/// less than or equal to `b.extract(0)`, or `0` otherwise. The upper 96 bits of
/// the result are the upper 96 bits of `a`.
#[inline(always)]
#[target_feature = "+sse"]
#[cfg_attr(test, assert_instr(cmpnless))]
pub unsafe fn _mm_cmpnle_ss(a: f32x4, b: f32x4) -> f32x4 {
    cmpss(a, b, 6)
}

/// Compare the lowest `f32` of both inputs for not-greater-than. The lowest 32
/// bits of the result will be `0xffffffff` if `a.extract(0)` is not greater
/// than `b.extract(0)`, or `0` otherwise. The upper 96 bits of the result are
/// the upper 96 bits of `a`.
#[inline(always)]
#[target_feature = "+sse"]
#[cfg_attr(test, assert_instr(cmpnltss))]
pub unsafe fn _mm_cmpngt_ss(a: f32x4, b: f32x4) -> f32x4 {
    simd_shuffle4(a, cmpss(b, a, 5), [4, 1, 2, 3])
}

/// Compare the lowest `f32` of both inputs for not-greater-than-or-equal. The
/// lowest 32 bits of the result will be `0xffffffff` if `a.extract(0)` is not
/// greater than or equal to `b.extract(0)`, or `0` otherwise. The upper 96 bits
/// of the result are the upper 96 bits of `a`.
#[inline(always)]
#[target_feature = "+sse"]
#[cfg_attr(test, assert_instr(cmpnless))]
pub unsafe fn _mm_cmpnge_ss(a: f32x4, b: f32x4) -> f32x4 {
    simd_shuffle4(a, cmpss(b, a, 6), [4, 1, 2, 3])
}

/// Check if the lowest `f32` of both inputs are ordered. The lowest 32 bits of
/// the result will be `0xffffffff` if neither of `a.extract(0)` or
/// `b.extract(0)` is a NaN, or `0` otherwise. The upper 96 bits of the result
/// are the upper 96 bits of `a`.
#[inline(always)]
#[target_feature = "+sse"]
#[cfg_attr(test, assert_instr(cmpordss))]
pub unsafe fn _mm_cmpord_ss(a: f32x4, b: f32x4) -> f32x4 {
    cmpss(a, b, 7)
}

/// Check if the lowest `f32` of both inputs are unordered. The lowest 32 bits
/// of the result will be `0xffffffff` if any of `a.extract(0)` or
/// `b.extract(0)` is a NaN, or `0` otherwise. The upper 96 bits of the result
/// are the upper 96 bits of `a`.
#[inline(always)]
#[target_feature = "+sse"]
#[cfg_attr(test, assert_instr(cmpunordss))]
pub unsafe fn _mm_cmpunord_ss(a: f32x4, b: f32x4) -> f32x4 {
    cmpss(a, b, 3)
}

/// Compare each of the four floats in `a` to the corresponding element in `b`.
/// The result in the output vector will be `0xffffffff` if the input elements
/// were equal, or `0` otherwise.
#[inline(always)]
#[target_feature = "+sse"]
#[cfg_attr(test, assert_instr(cmpeqps))]
pub unsafe fn _mm_cmpeq_ps(a: f32x4, b: f32x4) -> f32x4 {
    cmpps(a, b, 0)
}

/// Compare each of the four floats in `a` to the corresponding element in `b`.
/// The result in the output vector will be `0xffffffff` if the input element in
/// `a` is less than the corresponding element in `b`, or `0` otherwise.
#[inline(always)]
#[target_feature = "+sse"]
#[cfg_attr(test, assert_instr(cmpltps))]
pub unsafe fn _mm_cmplt_ps(a: f32x4, b: f32x4) -> f32x4 {
    cmpps(a, b, 1)
}

/// Compare each of the four floats in `a` to the corresponding element in `b`.
/// The result in the output vector will be `0xffffffff` if the input element in
/// `a` is less than or equal to the corresponding element in `b`, or `0`
/// otherwise.
#[inline(always)]
#[target_feature = "+sse"]
#[cfg_attr(test, assert_instr(cmpleps))]
pub unsafe fn _mm_cmple_ps(a: f32x4, b: f32x4) -> f32x4 {
    cmpps(a, b, 2)
}

/// Compare each of the four floats in `a` to the corresponding element in `b`.
/// The result in the output vector will be `0xffffffff` if the input element in
/// `a` is greater than the corresponding element in `b`, or `0` otherwise.
#[inline(always)]
#[target_feature = "+sse"]
#[cfg_attr(test, assert_instr(cmpltps))]
pub unsafe fn _mm_cmpgt_ps(a: f32x4, b: f32x4) -> f32x4 {
    cmpps(b, a, 1)
}

/// Compare each of the four floats in `a` to the corresponding element in `b`.
/// The result in the output vector will be `0xffffffff` if the input element in
/// `a` is greater than or equal to the corresponding element in `b`, or `0`
/// otherwise.
#[inline(always)]
#[target_feature = "+sse"]
#[cfg_attr(test, assert_instr(cmpleps))]
pub unsafe fn _mm_cmpge_ps(a: f32x4, b: f32x4) -> f32x4 {
    cmpps(b, a, 2)
}

/// Compare each of the four floats in `a` to the corresponding element in `b`.
/// The result in the output vector will be `0xffffffff` if the input elements
/// are *not* equal, or `0` otherwise.
#[inline(always)]
#[target_feature = "+sse"]
#[cfg_attr(test, assert_instr(cmpneqps))]
pub unsafe fn _mm_cmpneq_ps(a: f32x4, b: f32x4) -> f32x4 {
    cmpps(a, b, 4)
}

/// Compare each of the four floats in `a` to the corresponding element in `b`.
/// The result in the output vector will be `0xffffffff` if the input element in
/// `a` is *not* less than the corresponding element in `b`, or `0` otherwise.
#[inline(always)]
#[target_feature = "+sse"]
#[cfg_attr(test, assert_instr(cmpnltps))]
pub unsafe fn _mm_cmpnlt_ps(a: f32x4, b: f32x4) -> f32x4 {
    cmpps(a, b, 5)
}

/// Compare each of the four floats in `a` to the corresponding element in `b`.
/// The result in the output vector will be `0xffffffff` if the input element in
/// `a` is *not* less than or equal to the corresponding element in `b`, or `0`
/// otherwise.
#[inline(always)]
#[target_feature = "+sse"]
#[cfg_attr(test, assert_instr(cmpnleps))]
pub unsafe fn _mm_cmpnle_ps(a: f32x4, b: f32x4) -> f32x4 {
    cmpps(a, b, 6)
}

/// Compare each of the four floats in `a` to the corresponding element in `b`.
/// The result in the output vector will be `0xffffffff` if the input element in
/// `a` is *not* greater than the corresponding element in `b`, or `0`
/// otherwise.
#[inline(always)]
#[target_feature = "+sse"]
#[cfg_attr(test, assert_instr(cmpnltps))]
pub unsafe fn _mm_cmpngt_ps(a: f32x4, b: f32x4) -> f32x4 {
    cmpps(b, a, 5)
}

/// Compare each of the four floats in `a` to the corresponding element in `b`.
/// The result in the output vector will be `0xffffffff` if the input element in
/// `a` is *not* greater than or equal to the corresponding element in `b`, or
/// `0` otherwise.
#[inline(always)]
#[target_feature = "+sse"]
#[cfg_attr(test, assert_instr(cmpnleps))]
pub unsafe fn _mm_cmpnge_ps(a: f32x4, b: f32x4) -> f32x4 {
    cmpps(b, a, 6)
}

/// Compare each of the four floats in `a` to the corresponding element in `b`.
/// Returns four floats that have one of two possible bit patterns. The element
/// in the output vector will be `0xffffffff` if the input elements in `a` and
/// `b` are ordered (i.e., neither of them is a NaN), or 0 otherwise.
#[inline(always)]
#[target_feature = "+sse"]
#[cfg_attr(test, assert_instr(cmpordps))]
pub unsafe fn _mm_cmpord_ps(a: f32x4, b: f32x4) -> f32x4 {
    cmpps(b, a, 7)
}

/// Compare each of the four floats in `a` to the corresponding element in `b`.
/// Returns four floats that have one of two possible bit patterns. The element
/// in the output vector will be `0xffffffff` if the input elements in `a` and
/// `b` are unordered (i.e., at least on of them is a NaN), or 0 otherwise.
#[inline(always)]
#[target_feature = "+sse"]
#[cfg_attr(test, assert_instr(cmpunordps))]
pub unsafe fn _mm_cmpunord_ps(a: f32x4, b: f32x4) -> f32x4 {
    cmpps(b, a, 3)
}

/// Compare two 32-bit floats from the low-order bits of `a` and `b`. Returns
/// `1` if they are equal, or `0` otherwise.
#[inline(always)]
#[target_feature = "+sse"]
#[cfg_attr(test, assert_instr(comiss))]
pub unsafe fn _mm_comieq_ss(a: f32x4, b: f32x4) -> i32 {
    comieq_ss(a, b)
}

/// Compare two 32-bit floats from the low-order bits of `a` and `b`. Returns
/// `1` if the value from `a` is less than the one from `b`, or `0` otherwise.
#[inline(always)]
#[target_feature = "+sse"]
#[cfg_attr(test, assert_instr(comiss))]
pub unsafe fn _mm_comilt_ss(a: f32x4, b: f32x4) -> i32 {
    comilt_ss(a, b)
}

/// Compare two 32-bit floats from the low-order bits of `a` and `b`. Returns
/// `1` if the value from `a` is less than or equal to the one from `b`, or `0`
/// otherwise.
#[inline(always)]
#[target_feature = "+sse"]
#[cfg_attr(test, assert_instr(comiss))]
pub unsafe fn _mm_comile_ss(a: f32x4, b: f32x4) -> i32 {
    comile_ss(a, b)
}

/// Compare two 32-bit floats from the low-order bits of `a` and `b`. Returns
/// `1` if the value from `a` is greater than the one from `b`, or `0`
/// otherwise.
#[inline(always)]
#[target_feature = "+sse"]
#[cfg_attr(test, assert_instr(comiss))]
pub unsafe fn _mm_comigt_ss(a: f32x4, b: f32x4) -> i32 {
    comigt_ss(a, b)
}

/// Compare two 32-bit floats from the low-order bits of `a` and `b`. Returns
/// `1` if the value from `a` is greater than or equal to the one from `b`, or
/// `0` otherwise.
#[inline(always)]
#[target_feature = "+sse"]
#[cfg_attr(test, assert_instr(comiss))]
pub unsafe fn _mm_comige_ss(a: f32x4, b: f32x4) -> i32 {
    comige_ss(a, b)
}

/// Compare two 32-bit floats from the low-order bits of `a` and `b`. Returns
/// `1` if they are *not* equal, or `0` otherwise.
#[inline(always)]
#[target_feature = "+sse"]
#[cfg_attr(test, assert_instr(comiss))]
pub unsafe fn _mm_comineq_ss(a: f32x4, b: f32x4) -> i32 {
    comineq_ss(a, b)
}

/// Compare two 32-bit floats from the low-order bits of `a` and `b`. Returns
/// `1` if they are equal, or `0` otherwise. This instruction will not signal
/// an exception if either argument is a quiet NaN.
#[inline(always)]
#[target_feature = "+sse"]
#[cfg_attr(test, assert_instr(ucomiss))]
pub unsafe fn _mm_ucomieq_ss(a: f32x4, b: f32x4) -> i32 {
    ucomieq_ss(a, b)
}

/// Compare two 32-bit floats from the low-order bits of `a` and `b`. Returns
/// `1` if the value from `a` is less than the one from `b`, or `0` otherwise.
/// This instruction will not signal an exception if either argument is a quiet
/// NaN.
#[inline(always)]
#[target_feature = "+sse"]
#[cfg_attr(test, assert_instr(ucomiss))]
pub unsafe fn _mm_ucomilt_ss(a: f32x4, b: f32x4) -> i32 {
    ucomilt_ss(a, b)
}

/// Compare two 32-bit floats from the low-order bits of `a` and `b`. Returns
/// `1` if the value from `a` is less than or equal to the one from `b`, or `0`
/// otherwise. This instruction will not signal an exception if either argument
/// is a quiet NaN.
#[inline(always)]
#[target_feature = "+sse"]
#[cfg_attr(test, assert_instr(ucomiss))]
pub unsafe fn _mm_ucomile_ss(a: f32x4, b: f32x4) -> i32 {
    ucomile_ss(a, b)
}

/// Compare two 32-bit floats from the low-order bits of `a` and `b`. Returns
/// `1` if the value from `a` is greater than the one from `b`, or `0`
/// otherwise. This instruction will not signal an exception if either argument
/// is a quiet NaN.
#[inline(always)]
#[target_feature = "+sse"]
#[cfg_attr(test, assert_instr(ucomiss))]
pub unsafe fn _mm_ucomigt_ss(a: f32x4, b: f32x4) -> i32 {
    ucomigt_ss(a, b)
}

/// Compare two 32-bit floats from the low-order bits of `a` and `b`. Returns
/// `1` if the value from `a` is greater than or equal to the one from `b`, or
/// `0` otherwise. This instruction will not signal an exception if either
/// argument is a quiet NaN.
#[inline(always)]
#[target_feature = "+sse"]
#[cfg_attr(test, assert_instr(ucomiss))]
pub unsafe fn _mm_ucomige_ss(a: f32x4, b: f32x4) -> i32 {
    ucomige_ss(a, b)
}

/// Compare two 32-bit floats from the low-order bits of `a` and `b`. Returns
/// `1` if they are *not* equal, or `0` otherwise. This instruction will not
/// signal an exception if either argument is a quiet NaN.
#[inline(always)]
#[target_feature = "+sse"]
#[cfg_attr(test, assert_instr(ucomiss))]
pub unsafe fn _mm_ucomineq_ss(a: f32x4, b: f32x4) -> i32 {
    ucomineq_ss(a, b)
}

/// Construct a `f32x4` with the lowest element set to `a` and the rest set to
/// zero.
#[inline(always)]
#[target_feature = "+sse"]
#[cfg_attr(test, assert_instr(movss))]
pub unsafe fn _mm_set_ss(a: f32) -> f32x4 {
    f32x4::new(a, 0.0, 0.0, 0.0)
}

/// Construct a `f32x4` with all element set to `a`.
#[inline(always)]
#[target_feature = "+sse"]
#[cfg_attr(test, assert_instr(shufps))]
pub unsafe fn _mm_set1_ps(a: f32) -> f32x4 {
    f32x4::new(a, a, a, a)
}

/// Alias for [`_mm_set1_ps`](fn._mm_set1_ps.html)
#[inline(always)]
#[target_feature = "+sse"]
#[cfg_attr(test, assert_instr(shufps))]
pub unsafe fn _mm_set_ps1(a: f32) -> f32x4 {
    _mm_set1_ps(a)
}

/// Construct a `f32x4` from four floating point values highest to lowest.
///
/// Note that `a` will be the highest 32 bits of the result, and `d` the lowest.
/// This matches the standard way of writing bit patterns on x86:
///
/// ```text
///  bit    127 .. 96  95 .. 64  63 .. 32  31 .. 0
///        +---------+---------+---------+---------+
///        |    a    |    b    |    c    |    d    |   result
///        +---------+---------+---------+---------+
/// ```
///
/// Alternatively:
///
/// ```text
/// assert_eq!(f32x4::new(a, b, c, d), _mm_set_ps(d, c, b, a));
/// ```
#[inline(always)]
#[target_feature = "+sse"]
#[cfg_attr(test, assert_instr(unpcklps))]
pub unsafe fn _mm_set_ps(a: f32, b: f32, c: f32, d: f32) -> f32x4 {
    f32x4::new(d, c, b, a)
}

/// Construct a `f32x4` from four floating point values lowest to highest.
///
/// This matches the memory order of `f32x4`, i.e., `a` will be the lowest 32
/// bits of the result, and `d` the highest.
///
/// ```text
/// assert_eq!(f32x4::new(a, b, c, d), _mm_setr_ps(a, b, c, d));
/// ```
#[inline(always)]
#[target_feature = "+sse"]
#[cfg_attr(all(test, target_arch = "x86_64"), assert_instr(unpcklps))]
// On a 32-bit architecture it just copies the operands from the stack.
#[cfg_attr(all(test, target_arch = "x86"), assert_instr(movaps))]
pub unsafe fn _mm_setr_ps(a: f32, b: f32, c: f32, d: f32) -> f32x4 {
    f32x4::new(a, b, c, d)
}

/// Construct a `f32x4` with all elements initialized to zero.
#[inline(always)]
#[target_feature = "+sse"]
#[cfg_attr(test, assert_instr(xorps))]
pub unsafe fn _mm_setzero_ps() -> f32x4 {
    f32x4::new(0.0, 0.0, 0.0, 0.0)
}

/// Shuffle packed single-precision (32-bit) floating-point elements in `a` and
/// `b` using `mask`.
///
/// The lower half of result takes values from `a` and the higher half from
/// `b`. Mask is split to 2 control bits each to index the element from inputs.
#[inline(always)]
#[target_feature = "+sse"]
#[cfg_attr(test, assert_instr(shufps, mask = 3))]
pub unsafe fn _mm_shuffle_ps(a: f32x4, b: f32x4, mask: i32) -> f32x4 {
    let mask = (mask & 0xFF) as u8;

    macro_rules! shuffle_done {
        ($x01:expr, $x23:expr, $x45:expr, $x67:expr) => {
            simd_shuffle4(a, b, [$x01, $x23, $x45, $x67])
        }
    }
    macro_rules! shuffle_x67 {
        ($x01:expr, $x23:expr, $x45:expr) => {
            match (mask >> 6) & 0b11 {
                0b00 => shuffle_done!($x01, $x23, $x45, 4),
                0b01 => shuffle_done!($x01, $x23, $x45, 5),
                0b10 => shuffle_done!($x01, $x23, $x45, 6),
                _ => shuffle_done!($x01, $x23, $x45, 7),
            }
        }
    }
    macro_rules! shuffle_x45 {
        ($x01:expr, $x23:expr) => {
            match (mask >> 4) & 0b11 {
                0b00 => shuffle_x67!($x01, $x23, 4),
                0b01 => shuffle_x67!($x01, $x23, 5),
                0b10 => shuffle_x67!($x01, $x23, 6),
                _ => shuffle_x67!($x01, $x23, 7),
            }
        }
    }
    macro_rules! shuffle_x23 {
        ($x01:expr) => {
            match (mask >> 2) & 0b11 {
                0b00 => shuffle_x45!($x01, 0),
                0b01 => shuffle_x45!($x01, 1),
                0b10 => shuffle_x45!($x01, 2),
                _ => shuffle_x45!($x01, 3),
            }
        }
    }
    match mask & 0b11 {
        0b00 => shuffle_x23!(0),
        0b01 => shuffle_x23!(1),
        0b10 => shuffle_x23!(2),
        _ => shuffle_x23!(3),
    }
}

/// Unpack and interleave single-precision (32-bit) floating-point elements
/// from the higher half of `a` and `b`.
#[inline(always)]
#[target_feature = "+sse"]
#[cfg_attr(test, assert_instr(unpckhps))]
pub unsafe fn _mm_unpackhi_ps(a: f32x4, b: f32x4) -> f32x4 {
    simd_shuffle4(a, b, [2, 6, 3, 7])
}

/// Unpack and interleave single-precision (32-bit) floating-point elements
/// from the lower half of `a` and `b`.
#[inline(always)]
#[target_feature = "+sse"]
#[cfg_attr(test, assert_instr(unpcklps))]
pub unsafe fn _mm_unpacklo_ps(a: f32x4, b: f32x4) -> f32x4 {
    simd_shuffle4(a, b, [0, 4, 1, 5])
}

/// Combine higher half of `a` and `b`. The highwe half of `b` occupies the lower
/// half of result.
#[inline(always)]
#[target_feature = "+sse"]
#[cfg_attr(all(test, not(windows)), assert_instr(movhlps))]
#[cfg_attr(all(test, windows), assert_instr(unpckhpd))]
pub unsafe fn _mm_movehl_ps(a: f32x4, b: f32x4) -> f32x4 {
    // TODO; figure why this is a different instruction on Windows?
    simd_shuffle4(a, b, [6, 7, 2, 3])
}

/// Combine lower half of `a` and `b`. The lower half of `b` occupies the higher
/// half of result.
#[inline(always)]
#[target_feature = "+sse"]
#[cfg_attr(all(test, target_feature = "sse2"), assert_instr(unpcklpd))]
#[cfg_attr(all(test, not(target_feature = "sse2")), assert_instr(movlhps))]
pub unsafe fn _mm_movelh_ps(a: f32x4, b: f32x4) -> f32x4 {
    simd_shuffle4(a, b, [0, 1, 4, 5])
}

/// Return a mask of the most significant bit of each element in `a`.
///
/// The mask is stored in the 4 least significant bits of the return value.
/// All other bits are set to `0`.
#[inline(always)]
#[target_feature = "+sse"]
#[cfg_attr(test, assert_instr(movmskps))]
pub unsafe fn _mm_movemask_ps(a: f32x4) -> i32 {
    movmskps(a)
}

/// Set the upper two single-precision floating-point values with 64 bits of
/// data loaded from the address `p`; the lower two values are passed through
/// from `a`.
///
/// This corresponds to the `MOVHPS` / `MOVHPD` / `VMOVHPD` instructions.
///
/// ```rust
/// # #![feature(cfg_target_feature)]
/// # #![feature(target_feature)]
/// #
/// # #[macro_use] extern crate stdsimd;
/// #
/// # // The real main function
/// # fn main() {
/// #     if cfg_feature_enabled!("sse") {
/// #         #[target_feature = "+sse"]
/// #         fn worker() {
/// #
/// #   use stdsimd::simd::f32x4;
/// #   use stdsimd::vendor::_mm_loadh_pi;
/// #
/// let a = f32x4::new(1.0, 2.0, 3.0, 4.0);
/// let data: [f32; 4] = [5.0, 6.0, 7.0, 8.0];
///
/// let r = unsafe { _mm_loadh_pi(a, data[..].as_ptr()) };
///
/// assert_eq!(r, f32x4::new(1.0, 2.0, 5.0, 6.0));
/// #
/// #         }
/// #         worker();
/// #     }
/// # }
/// ```
#[inline(always)]
#[target_feature = "+sse"]
// TODO: generates MOVHPD if the CPU supports SSE2.
// #[cfg_attr(test, assert_instr(movhps))]
#[cfg_attr(all(test, target_arch = "x86_64"), assert_instr(movhpd))]
// 32-bit codegen does not generate `movhps` or `movhpd`, but instead
// `movsd` followed by `unpcklpd` (or `movss'/`unpcklps` if there's no SSE2).
#[cfg_attr(all(test, target_arch = "x86", target_feature = "sse2"),
    assert_instr(unpcklpd))]
#[cfg_attr(all(test, target_arch = "x86", not(target_feature = "sse2")),
    assert_instr(unpcklps))]
// TODO: This function is actually not limited to floats, but that's what
// what matches the C type most closely: (__m128, *const __m64) -> __m128
pub unsafe fn _mm_loadh_pi(a: f32x4, p: *const f32) -> f32x4 {
    let q = p as *const f32x2;
    let b: f32x2 = *q;
    let bb = simd_shuffle4(b, b, [0, 1, 0, 1]);
    simd_shuffle4(a, bb, [0, 1, 4, 5])
}

/// Load two floats from `p` into the lower half of a `f32x4`. The upper half
/// is copied from the upper half of `a`.
///
/// This corresponds to the `MOVLPS` / `MOVLDP` / `VMOVLDP` instructions.
///
/// ```rust
/// # #![feature(cfg_target_feature)]
/// # #![feature(target_feature)]
/// #
/// # #[macro_use] extern crate stdsimd;
/// #
/// # // The real main function
/// # fn main() {
/// #     if cfg_feature_enabled!("sse") {
/// #         #[target_feature = "+sse"]
/// #         fn worker() {
/// #
/// #   use stdsimd::simd::f32x4;
/// #   use stdsimd::vendor::_mm_loadl_pi;
/// #
/// let a = f32x4::new(1.0, 2.0, 3.0, 4.0);
/// let data: [f32; 4] = [5.0, 6.0, 7.0, 8.0];
///
/// let r = unsafe { _mm_loadl_pi(a, data[..].as_ptr()) };
///
/// assert_eq!(r, f32x4::new(5.0, 6.0, 3.0, 4.0));
/// #
/// #         }
/// #         worker();
/// #     }
/// # }
/// ```
#[inline(always)]
#[target_feature = "+sse"]
// TODO: generates MOVLPD if the CPU supports SSE2.
// #[cfg_attr(test, assert_instr(movlps))]
#[cfg_attr(all(test, target_arch = "x86_64"), assert_instr(movlpd))]
// On 32-bit targets with SSE2, it just generates two `movsd`.
#[cfg_attr(all(test, target_arch = "x86", target_feature = "sse2"),
    assert_instr(movsd))]
// It should really generate "movlps", but oh well...
#[cfg_attr(all(test, target_arch = "x86", not(target_feature = "sse2")),
    assert_instr(movss))]
// TODO: Like _mm_loadh_pi, this also isn't limited to floats.
pub unsafe fn _mm_loadl_pi(a: f32x4, p: *const f32) -> f32x4 {
    let q = p as *const f32x2;
    let b: f32x2 = *q;
    let bb = simd_shuffle4(b, b, [0, 1, 0, 1]);
    simd_shuffle4(a, bb, [4, 5, 2, 3])
}

/// Construct a `f32x4` with the lowest element read from `p` and the other
/// elements set to zero.
///
/// This corresponds to instructions `VMOVSS` / `MOVSS`.
#[inline(always)]
#[target_feature = "+sse"]
#[cfg_attr(test, assert_instr(movss))]
pub unsafe fn _mm_load_ss(p: *const f32) -> f32x4 {
    f32x4::new(*p, 0.0, 0.0, 0.0)
}

/// Construct a `f32x4` by duplicating the value read from `p` into all
/// elements.
///
/// This corresponds to instructions `VMOVSS` / `MOVSS` followed by some
/// shuffling.
#[inline(always)]
#[target_feature = "+sse"]
#[cfg_attr(test, assert_instr(movss))]
pub unsafe fn _mm_load1_ps(p: *const f32) -> f32x4 {
    let a = *p;
    f32x4::new(a, a, a, a)
}

/// Alias for [`_mm_load1_ps`](fn._mm_load1_ps.html)
#[inline(always)]
#[target_feature = "+sse"]
#[cfg_attr(test, assert_instr(movss))]
pub unsafe fn _mm_load_ps1(p: *const f32) -> f32x4 {
    _mm_load1_ps(p)
}

/// Load four `f32` values from *aligned* memory into a `f32x4`. If the pointer
/// is not aligned to a 128-bit boundary (16 bytes) a general protection fault
/// will be triggered (fatal program crash).
///
/// Use [`_mm_loadu_ps`](fn._mm_loadu_ps.html) for potentially unaligned memory.
///
/// This corresponds to instructions `VMOVAPS` / `MOVAPS`.
#[inline(always)]
#[target_feature = "+sse"]
#[cfg_attr(test, assert_instr(movaps))]
pub unsafe fn _mm_load_ps(p: *const f32) -> f32x4 {
    *(p as *const f32x4)
}

/// Load four `f32` values from memory into a `f32x4`. There are no restrictions
/// on memory alignment. For aligned memory [`_mm_load_ps`](fn._mm_load_ps.html)
/// may be faster.
///
/// This corresponds to instructions `VMOVUPS` / `MOVUPS`.
#[inline(always)]
#[target_feature = "+sse"]
#[cfg_attr(test, assert_instr(movups))]
pub unsafe fn _mm_loadu_ps(p: *const f32) -> f32x4 {
    // Note: Using `*p` would require `f32` alignment, but `movups` has no
    // alignment restrictions.
    let mut dst = f32x4::splat(mem::uninitialized());
    ptr::copy_nonoverlapping(
        p as *const u8,
        &mut dst as *mut f32x4 as *mut u8,
        mem::size_of::<f32x4>());
    dst
}

/// Load four `f32` values from aligned memory into a `f32x4` in reverse order.
///
/// If the pointer is not aligned to a 128-bit boundary (16 bytes) a general
/// protection fault will be triggered (fatal program crash).
///
/// Functionally equivalent to the following code sequence (assuming `p`
/// satisfies the alignment restrictions):
///
/// ```text
/// let a0 = *p;
/// let a1 = *p.offset(1);
/// let a2 = *p.offset(2);
/// let a3 = *p.offset(3);
/// f32x4::new(a3, a2, a1, a0)
/// ```
///
/// This corresponds to instructions `VMOVAPS` / `MOVAPS` followed by some
/// shuffling.
#[inline(always)]
#[target_feature = "+sse"]
#[cfg_attr(test, assert_instr(movaps))]
pub unsafe fn _mm_loadr_ps(p: *const f32) -> f32x4 {
    let a = _mm_load_ps(p);
    simd_shuffle4(a, a, [3, 2, 1, 0])
}

/// Store four 32-bit floats in aligned memory. To minimize cache pollution, the
/// data is flagged as non-temporal (unlikely to be used again soon).
///
/// The target address must be aligned to a 16-byte boundary. Unaligned memory
/// will cause a general protection fault (hard program crash).
///
/// This intrinsic corresponds to the `MOVNTPS` instruction.
#[inline(always)]
#[target_feature = "+sse"]
#[cfg_attr(test, assert_instr(movntps))]
pub unsafe fn _mm_stream_ps(dst: *mut f32, src: f32x4) {
    // debug_assert!(false, "Rust does not yet support non-temporary stores");
    *(dst as *mut f32x4) = src;
}

/// Perform a serializing operation on all store-to-memory instructions that
/// were issued prior to this instruction.
///
/// Guarantees that every store instruction that precedes, in program order, is
/// globally visible before any store instruction which follows the fence in
/// program order.
#[inline(always)]
#[target_feature = "+sse"]
#[cfg_attr(test, assert_instr(sfence))]
pub unsafe fn _mm_sfence() {
    sfence()
}

/// Get the unsigned 32-bit value of the MXCSR control and status register.
///
/// For more info see [`_mm_setcsr`](fn._mm_setcsr.html)
#[inline(always)]
#[target_feature = "+sse"]
#[cfg_attr(test, assert_instr(stmxcsr))]
pub unsafe fn _mm_getcsr() -> u32 {
    let mut result = 0i32;
    stmxcsr((&mut result) as *mut _ as *mut i8);
    result as u32
}

/// Set the MXCSR register with the 32-bit unsigned integer value.
///
/// This register constrols how SIMD instructions handle floating point
/// operations. Modifying this register only affects the current thread.
///
/// It contains several groups of flags:
///
/// * *Exception flags* report which exceptions occurred since last they were
/// reset.
///
/// * *Masking flags* can be used to mask (ignore) certain exceptions. By default
/// these flags are all set to 1, so all exceptions are masked. When an
/// an exception is masked, the processor simply sets the exception flag and
/// continues the operation. If the exception is unmasked, the flag is also set
/// but additionally an exception handler is invoked.
///
/// * *Rounding mode flags* control the rounding mode of floating point
/// instructions.
///
/// * The *denormals-are-zero mode flag* turns all numbers which would be
/// denormalized (exponent bits are all zeros) into zeros.
///
/// ## Exception Flags
///
/// * `_MM_EXCEPT_INVALID`: An invalid operation was performed (e.g., dividing
///   Infinity by Infinity).
///
/// * `_MM_EXCEPT_DENORM`: An operation attempted to operate on a denormalized
///   number. Mainly this can cause loss of precision.
///
/// * `_MM_EXCEPT_DIV_ZERO`: Division by zero occured.
///
/// * `_MM_EXCEPT_OVERFLOW`: A numeric overflow exception occured, i.e., a
///   result was too large to be represented (e.g., an `f32` with absolute value
///   greater than `2^128`).
///
/// * `_MM_EXCEPT_UNDERFLOW`: A numeric underflow exception occured, i.e., a
///   result was too small to be represented in a normalized way (e.g., an `f32`
///   with absulte value smaller than `2^-126`.)
///
/// * `_MM_EXCEPT_INEXACT`: An inexact-result exception occured (a.k.a.
///   precision exception). This means some precision was lost due to rounding.
///   For example, the fraction `1/3` cannot be represented accurately in a
///   32 or 64 bit float and computing it would cause this exception to be
///   raised. Precision exceptions are very common, so they are usually masked.
///
/// Exception flags can be read and set using the convenience functions
/// `_MM_GET_EXCEPTION_STATE` and `_MM_SET_EXCEPTION_STATE`. For example, to
/// check if an operation caused some overflow:
///
/// ```rust,ignore
/// _MM_SET_EXCEPTION_STATE(0);  // clear all exception flags
/// // perform calculations
/// if _MM_GET_EXCEPTION_STATE() & _MM_EXCEPT_OVERFLOW != 0 {
///     // handle overflow
/// }
/// ```
///
/// ## Masking Flags
///
/// There is one masking flag for each exception flag: `_MM_MASK_INVALID`,
/// `_MM_MASK_DENORM`, `_MM_MASK_DIV_ZERO`, `_MM_MASK_OVERFLOW`,
/// `_MM_MASK_UNDERFLOW`, `_MM_MASK_INEXACT`.
///
/// A single masking bit can be set via
///
/// ```rust,ignore
/// _MM_SET_EXCEPTION_MASK(_MM_MASK_UNDERFLOW);
/// ```
///
/// However, since mask bits are by default all set to 1, it is more common to
/// want to *disable* certain bits. For example, to unmask the underflow
/// exception, use:
///
/// ```rust,ignore
/// _mm_setcsr(_mm_getcsr() & !_MM_MASK_UNDERFLOW);  // unmask underflow exception
/// ```
///
/// Warning: an unmasked exception will cause an exception handler to be called.
/// The standard handler will simply terminate the process. So, in this case
/// any underflow exception would terminate the current process with something
/// like `signal: 8, SIGFPE: erroneous arithmetic operation`.
///
/// ## Rounding Mode
///
/// The rounding mode is describe using two bits. It can be read and set using
/// the convenience wrappers `_MM_GET_ROUNDING_MODE()` and
/// `_MM_SET_ROUNDING_MODE(mode)`.
///
/// The rounding modes are:
///
/// * `_MM_ROUND_NEAREST`: (default) Round to closest to the infinite precision
///   value. If two values are equally close, round to even (i.e., least
///   significant bit will be zero).
///
/// * `_MM_ROUND_DOWN`: Round toward negative Infinity.
///
/// * `_MM_ROUND_UP`: Round toward positive Infinity.
///
/// * `_MM_ROUND_TOWARD_ZERO`: Round towards zero (truncate).
///
/// Example:
///
/// ```rust,ignore
/// _MM_SET_ROUNDING_MODE(_MM_ROUND_DOWN)
/// ```
///
/// ## Denormals-are-zero/Flush-to-zero Mode
///
/// If this bit is set, values that would be denormalized will be set to zero
/// instead. This is turned off by default.
///
/// You can read and enable/disable this mode via the helper functions
/// `_MM_GET_FLUSH_ZERO_MODE()` and `_MM_SET_FLUSH_ZERO_MODE()`:
///
/// ```rust,ignore
/// _MM_SET_FLUSH_ZERO_MODE(_MM_FLUSH_ZERO_OFF);  // turn off (default)
/// _MM_SET_FLUSH_ZERO_MODE(_MM_FLUSH_ZERO_ON);  // turn on
/// ```
///
#[inline(always)]
#[target_feature = "+sse"]
#[cfg_attr(test, assert_instr(ldmxcsr))]
pub unsafe fn _mm_setcsr(val: u32) {
    ldmxcsr(&val as *const _ as *const i8);
}

/// See [`_mm_setcsr`](fn._mm_setcsr.html)
pub const _MM_EXCEPT_INVALID: u32    = 0x0001;
/// See [`_mm_setcsr`](fn._mm_setcsr.html)
pub const _MM_EXCEPT_DENORM: u32     = 0x0002;
/// See [`_mm_setcsr`](fn._mm_setcsr.html)
pub const _MM_EXCEPT_DIV_ZERO: u32   = 0x0004;
/// See [`_mm_setcsr`](fn._mm_setcsr.html)
pub const _MM_EXCEPT_OVERFLOW: u32   = 0x0008;
/// See [`_mm_setcsr`](fn._mm_setcsr.html)
pub const _MM_EXCEPT_UNDERFLOW: u32  = 0x0010;
/// See [`_mm_setcsr`](fn._mm_setcsr.html)
pub const _MM_EXCEPT_INEXACT: u32    = 0x0020;
pub const _MM_EXCEPT_MASK: u32       = 0x003f;

/// See [`_mm_setcsr`](fn._mm_setcsr.html)
pub const _MM_MASK_INVALID: u32      = 0x0080;
/// See [`_mm_setcsr`](fn._mm_setcsr.html)
pub const _MM_MASK_DENORM: u32       = 0x0100;
/// See [`_mm_setcsr`](fn._mm_setcsr.html)
pub const _MM_MASK_DIV_ZERO: u32     = 0x0200;
/// See [`_mm_setcsr`](fn._mm_setcsr.html)
pub const _MM_MASK_OVERFLOW: u32     = 0x0400;
/// See [`_mm_setcsr`](fn._mm_setcsr.html)
pub const _MM_MASK_UNDERFLOW: u32    = 0x0800;
/// See [`_mm_setcsr`](fn._mm_setcsr.html)
pub const _MM_MASK_INEXACT: u32      = 0x1000;
pub const _MM_MASK_MASK: u32         = 0x1f80;

/// See [`_mm_setcsr`](fn._mm_setcsr.html)
pub const _MM_ROUND_NEAREST: u32     = 0x0000;
/// See [`_mm_setcsr`](fn._mm_setcsr.html)
pub const _MM_ROUND_DOWN: u32        = 0x2000;
/// See [`_mm_setcsr`](fn._mm_setcsr.html)
pub const _MM_ROUND_UP: u32          = 0x4000;
/// See [`_mm_setcsr`](fn._mm_setcsr.html)
pub const _MM_ROUND_TOWARD_ZERO: u32 = 0x6000;
pub const _MM_ROUND_MASK: u32        = 0x6000;

pub const _MM_FLUSH_ZERO_MASK: u32   = 0x8000;
/// See [`_mm_setcsr`](fn._mm_setcsr.html)
pub const _MM_FLUSH_ZERO_ON: u32     = 0x8000;
/// See [`_mm_setcsr`](fn._mm_setcsr.html)
pub const _MM_FLUSH_ZERO_OFF: u32    = 0x0000;

#[inline(always)]
#[allow(non_snake_case)]
#[target_feature = "+sse"]
pub unsafe fn _MM_GET_EXCEPTION_MASK() -> u32 {
    _mm_getcsr() & _MM_MASK_MASK
}

#[inline(always)]
#[allow(non_snake_case)]
#[target_feature = "+sse"]
pub unsafe fn _MM_GET_EXCEPTION_STATE() -> u32 {
    _mm_getcsr() & _MM_EXCEPT_MASK
}

#[inline(always)]
#[allow(non_snake_case)]
#[target_feature = "+sse"]
pub unsafe fn _MM_GET_FLUSH_ZERO_MODE() -> u32 {
    _mm_getcsr() & _MM_FLUSH_ZERO_MASK
}

#[inline(always)]
#[allow(non_snake_case)]
#[target_feature = "+sse"]
pub unsafe fn _MM_GET_ROUNDING_MODE() -> u32 {
    _mm_getcsr() & _MM_ROUND_MASK
}

#[inline(always)]
#[allow(non_snake_case)]
#[target_feature = "+sse"]
pub unsafe fn _MM_SET_EXCEPTION_MASK(x: u32) {
    _mm_setcsr((_mm_getcsr() & !_MM_MASK_MASK) | x)
}

#[inline(always)]
#[allow(non_snake_case)]
#[target_feature = "+sse"]
pub unsafe fn _MM_SET_EXCEPTION_STATE(x: u32) {
    _mm_setcsr((_mm_getcsr() & !_MM_EXCEPT_MASK) | x)
}

#[inline(always)]
#[allow(non_snake_case)]
#[target_feature = "+sse"]
pub unsafe fn _MM_SET_FLUSH_ZERO_MODE(x: u32) {
    let val = (_mm_getcsr() & !_MM_FLUSH_ZERO_MASK) | x;
    //println!("setting csr={:x}", val);
    _mm_setcsr(val)
}

#[inline(always)]
#[allow(non_snake_case)]
#[target_feature = "+sse"]
pub unsafe fn _MM_SET_ROUNDING_MODE(x: u32) {
    _mm_setcsr((_mm_getcsr() & !_MM_ROUND_MASK) | x)
}

/// See [`_mm_prefetch`](fn._mm_prefetch.html).
pub const _MM_HINT_T0: i8 = 3;

/// See [`_mm_prefetch`](fn._mm_prefetch.html).
pub const _MM_HINT_T1: i8 = 2;

/// See [`_mm_prefetch`](fn._mm_prefetch.html).
pub const _MM_HINT_T2: i8 = 1;

/// See [`_mm_prefetch`](fn._mm_prefetch.html).
pub const _MM_HINT_NTA: i8 = 0;


/// Fetch the cache line that contains address `p` using the given `strategy`.
///
/// The `strategy` must be one of:
///
/// * [`_MM_HINT_T0`](constant._MM_HINT_T0.html): Fetch into all levels of the
///   cache hierachy.
///
/// * [`_MM_HINT_T1`](constant._MM_HINT_T1.html): Fetch into L2 and higher.
///
/// * [`_MM_HINT_T2`](constant._MM_HINT_T2.html): Fetch into L3 and higher or an
///   implementation-specific choice (e.g., L2 if there is no L3).
///
/// * [`_MM_HINT_NTA`](constant._MM_HINT_NTA.html): Fetch data using the
///   non-temporal access (NTA) hint. It may be a place closer than main memory
///   but outside of the cache hierarchy. This is used to reduce access latency
///   without polluting the cache.
///
/// The actual implementation depends on the particular CPU. This instruction
/// is considered a hint, so the CPU is also free to simply ignore the request.
///
/// The amount of prefetched data depends on the cache line size of the specific
/// CPU, but it will be at least 32 bytes.
///
/// Common caveats:
///
/// * Most modern CPUs already automatically prefetch data based on predicted
///   access patterns.
///
/// * Data is usually not fetched if this would cause a TLB miss or a page
///   fault.
///
/// * Too much prefetching can cause unnecessary cache evictions.
///
/// * Prefetching may also fail if there are not enough memory-subsystem
///   resources (e.g., request buffers).
///
#[inline(always)]
#[target_feature = "+sse"]
#[cfg_attr(test, assert_instr(prefetcht0, strategy = _MM_HINT_T0))]
#[cfg_attr(test, assert_instr(prefetcht1, strategy = _MM_HINT_T1))]
#[cfg_attr(test, assert_instr(prefetcht2, strategy = _MM_HINT_T2))]
#[cfg_attr(test, assert_instr(prefetchnta, strategy = _MM_HINT_NTA))]
pub unsafe fn _mm_prefetch(p: *const c_void, strategy: i8) {
    // The `strategy` must be a compile-time constant, so we use a short form of
    // `constify_imm8!` for now.
    // We use the `llvm.prefetch` instrinsic with `rw` = 0 (read), and
    // `cache type` = 1 (data cache). `locality` is based on our `strategy`.
    macro_rules! pref {
        ($imm8:expr) => {
            match $imm8 {
                0 => prefetch(p, 0, 0, 1),
                1 => prefetch(p, 0, 1, 1),
                2 => prefetch(p, 0, 2, 1),
                _ => prefetch(p, 0, 3, 1),
            }
        }
    }
    pref!(strategy)
}

/// Return vector of type __m128 with undefined elements.
#[inline(always)]
#[target_feature = "+sse"]
pub unsafe fn _mm_undefined_ps() -> f32x4 {
    f32x4::splat(mem::uninitialized())
}

/// Transpose the 4x4 matrix formed by 4 rows of f32x4 in place.
#[inline(always)]
#[allow(non_snake_case)]
#[target_feature = "+sse"]
pub unsafe fn _MM_TRANSPOSE4_PS(row0: &mut f32x4, row1: &mut f32x4, row2: &mut f32x4, row3: &mut f32x4) {
    let tmp0 = _mm_unpacklo_ps(*row0, *row1);
    let tmp2 = _mm_unpacklo_ps(*row2, *row3);
    let tmp1 = _mm_unpackhi_ps(*row0, *row1);
    let tmp3 = _mm_unpackhi_ps(*row2, *row3);

    *row0 = _mm_movelh_ps(tmp0, tmp2);
    *row1 = _mm_movehl_ps(tmp2, tmp0);
    *row2 = _mm_movelh_ps(tmp1, tmp3);
    *row3 = _mm_movehl_ps(tmp3, tmp1);
}

#[allow(improper_ctypes)]
extern {
    #[link_name = "llvm.x86.sse.add.ss"]
    fn addss(a: f32x4, b: f32x4) -> f32x4;
    #[link_name = "llvm.x86.sse.sub.ss"]
    fn subss(a: f32x4, b: f32x4) -> f32x4;
    #[link_name = "llvm.x86.sse.mul.ss"]
    fn mulss(a: f32x4, b: f32x4) -> f32x4;
    #[link_name = "llvm.x86.sse.div.ss"]
    fn divss(a: f32x4, b: f32x4) -> f32x4;
    #[link_name = "llvm.x86.sse.sqrt.ss"]
    fn sqrtss(a: f32x4) -> f32x4;
    #[link_name = "llvm.x86.sse.sqrt.ps"]
    fn sqrtps(a: f32x4) -> f32x4;
    #[link_name = "llvm.x86.sse.rcp.ss"]
    fn rcpss(a: f32x4) -> f32x4;
    #[link_name = "llvm.x86.sse.rcp.ps"]
    fn rcpps(a: f32x4) -> f32x4;
    #[link_name = "llvm.x86.sse.rsqrt.ss"]
    fn rsqrtss(a: f32x4) -> f32x4;
    #[link_name = "llvm.x86.sse.rsqrt.ps"]
    fn rsqrtps(a: f32x4) -> f32x4;
    #[link_name = "llvm.x86.sse.min.ss"]
    fn minss(a: f32x4, b: f32x4) -> f32x4;
    #[link_name = "llvm.x86.sse.min.ps"]
    fn minps(a: f32x4, b: f32x4) -> f32x4;
    #[link_name = "llvm.x86.sse.max.ss"]
    fn maxss(a: f32x4, b: f32x4) -> f32x4;
    #[link_name = "llvm.x86.sse.max.ps"]
    fn maxps(a: f32x4, b: f32x4) -> f32x4;
    #[link_name = "llvm.x86.sse.movmsk.ps"]
    fn movmskps(a: f32x4) -> i32;
    #[link_name = "llvm.x86.sse.cmp.ps"]
    fn cmpps(a: f32x4, b: f32x4, imm8: i8) -> f32x4;
    #[link_name = "llvm.x86.sse.comieq.ss"]
    fn comieq_ss(a: f32x4, b: f32x4) -> i32;
    #[link_name = "llvm.x86.sse.comilt.ss"]
    fn comilt_ss(a: f32x4, b: f32x4) -> i32;
    #[link_name = "llvm.x86.sse.comile.ss"]
    fn comile_ss(a: f32x4, b: f32x4) -> i32;
    #[link_name = "llvm.x86.sse.comigt.ss"]
    fn comigt_ss(a: f32x4, b: f32x4) -> i32;
    #[link_name = "llvm.x86.sse.comige.ss"]
    fn comige_ss(a: f32x4, b: f32x4) -> i32;
    #[link_name = "llvm.x86.sse.comineq.ss"]
    fn comineq_ss(a: f32x4, b: f32x4) -> i32;
    #[link_name = "llvm.x86.sse.ucomieq.ss"]
    fn ucomieq_ss(a: f32x4, b: f32x4) -> i32;
    #[link_name = "llvm.x86.sse.ucomilt.ss"]
    fn ucomilt_ss(a: f32x4, b: f32x4) -> i32;
    #[link_name = "llvm.x86.sse.ucomile.ss"]
    fn ucomile_ss(a: f32x4, b: f32x4) -> i32;
    #[link_name = "llvm.x86.sse.ucomigt.ss"]
    fn ucomigt_ss(a: f32x4, b: f32x4) -> i32;
    #[link_name = "llvm.x86.sse.ucomige.ss"]
    fn ucomige_ss(a: f32x4, b: f32x4) -> i32;
    #[link_name = "llvm.x86.sse.ucomineq.ss"]
    fn ucomineq_ss(a: f32x4, b: f32x4) -> i32;
    #[link_name = "llvm.x86.sse.sfence"]
    fn sfence();
    #[link_name = "llvm.x86.sse.stmxcsr"]
    fn stmxcsr(p: *mut i8);
    #[link_name = "llvm.x86.sse.ldmxcsr"]
    fn ldmxcsr(p: *const i8);
    #[link_name = "llvm.prefetch"]
    fn prefetch(p: *const c_void, rw: i32, loc: i32, ty: i32);
    #[link_name = "llvm.x86.sse.cmp.ss"]
    fn cmpss(a: f32x4, b: f32x4, imm8: i8) -> f32x4;
}

#[cfg(test)]
mod tests {
    use v128::*;
    use x86::sse;
    use stdsimd_test::simd_test;
    use test::black_box;  // Used to inhibit constant-folding.

    #[simd_test = "sse"]
    unsafe fn _mm_add_ps() {
        let a = f32x4::new(-1.0, 5.0, 0.0, -10.0);
        let b = f32x4::new(-100.0, 20.0, 0.0, -5.0);
        let r = sse::_mm_add_ps(a, b);
        assert_eq!(r, f32x4::new(-101.0, 25.0, 0.0, -15.0));
    }

    #[simd_test = "sse"]
    unsafe fn _mm_add_ss() {
        let a = f32x4::new(-1.0, 5.0, 0.0, -10.0);
        let b = f32x4::new(-100.0, 20.0, 0.0, -5.0);
        let r = sse::_mm_add_ss(a, b);
        assert_eq!(r, f32x4::new(-101.0, 5.0, 0.0, -10.0));
    }

    #[simd_test = "sse"]
    unsafe fn _mm_sub_ps() {
        let a = f32x4::new(-1.0, 5.0, 0.0, -10.0);
        let b = f32x4::new(-100.0, 20.0, 0.0, -5.0);
        let r = sse::_mm_sub_ps(a, b);
        assert_eq!(r, f32x4::new(99.0, -15.0, 0.0, -5.0));
    }

    #[simd_test = "sse"]
    unsafe fn _mm_sub_ss() {
        let a = f32x4::new(-1.0, 5.0, 0.0, -10.0);
        let b = f32x4::new(-100.0, 20.0, 0.0, -5.0);
        let r = sse::_mm_sub_ss(a, b);
        assert_eq!(r, f32x4::new(99.0, 5.0, 0.0, -10.0));
    }

    #[simd_test = "sse"]
    unsafe fn _mm_mul_ps() {
        let a = f32x4::new(-1.0, 5.0, 0.0, -10.0);
        let b = f32x4::new(-100.0, 20.0, 0.0, -5.0);
        let r = sse::_mm_mul_ps(a, b);
        assert_eq!(r, f32x4::new(100.0, 100.0, 0.0, 50.0));
    }

    #[simd_test = "sse"]
    unsafe fn _mm_mul_ss() {
        let a = f32x4::new(-1.0, 5.0, 0.0, -10.0);
        let b = f32x4::new(-100.0, 20.0, 0.0, -5.0);
        let r = sse::_mm_mul_ss(a, b);
        assert_eq!(r, f32x4::new(100.0, 5.0, 0.0, -10.0));
    }

    #[simd_test = "sse"]
    unsafe fn _mm_div_ps() {
        let a = f32x4::new(-1.0, 5.0, 2.0, -10.0);
        let b = f32x4::new(-100.0, 20.0, 0.2, -5.0);
        let r = sse::_mm_div_ps(a, b);
        assert_eq!(r, f32x4::new(0.01, 0.25, 10.0, 2.0));
    }

    #[simd_test = "sse"]
    unsafe fn _mm_div_ss() {
        let a = f32x4::new(-1.0, 5.0, 0.0, -10.0);
        let b = f32x4::new(-100.0, 20.0, 0.0, -5.0);
        let r = sse::_mm_div_ss(a, b);
        assert_eq!(r, f32x4::new(0.01, 5.0, 0.0, -10.0));
    }

    #[simd_test = "sse"]
    unsafe fn _mm_sqrt_ss() {
        let a = f32x4::new(4.0, 13.0, 16.0, 100.0);
        let r = sse::_mm_sqrt_ss(a);
        let e = f32x4::new(2.0, 13.0, 16.0, 100.0);
        assert_eq!(r, e);
    }

    #[simd_test = "sse"]
    unsafe fn _mm_sqrt_ps() {
        let a = f32x4::new(4.0, 13.0, 16.0, 100.0);
        let r = sse::_mm_sqrt_ps(a);
        let e = f32x4::new(2.0, 3.6055512, 4.0, 10.0);
        assert_eq!(r, e);
    }

    #[simd_test = "sse"]
    unsafe fn _mm_rcp_ss() {
        let a = f32x4::new(4.0, 13.0, 16.0, 100.0);
        let r = sse::_mm_rcp_ss(a);
        let e = f32x4::new(0.24993896, 13.0, 16.0, 100.0);
        assert_eq!(r, e);
    }

    #[simd_test = "sse"]
    unsafe fn _mm_rcp_ps() {
        let a = f32x4::new(4.0, 13.0, 16.0, 100.0);
        let r = sse::_mm_rcp_ps(a);
        let e = f32x4::new(0.24993896, 0.0769043, 0.06248474, 0.0099983215);
        assert_eq!(r, e);
    }

    #[simd_test = "sse"]
    unsafe fn _mm_rsqrt_ss() {
        let a = f32x4::new(4.0, 13.0, 16.0, 100.0);
        let r = sse::_mm_rsqrt_ss(a);
        let e = f32x4::new(0.49987793, 13.0, 16.0, 100.0);
        assert_eq!(r, e);
    }

    #[simd_test = "sse"]
    unsafe fn _mm_rsqrt_ps() {
        let a = f32x4::new(4.0, 13.0, 16.0, 100.0);
        let r = sse::_mm_rsqrt_ps(a);
        let e = f32x4::new(0.49987793, 0.2772827, 0.24993896, 0.099990845);
        assert_eq!(r, e);
    }

    #[simd_test = "sse"]
    unsafe fn _mm_min_ss() {
        let a = f32x4::new(-1.0, 5.0, 0.0, -10.0);
        let b = f32x4::new(-100.0, 20.0, 0.0, -5.0);
        let r = sse::_mm_min_ss(a, b);
        assert_eq!(r, f32x4::new(-100.0, 5.0, 0.0, -10.0));
    }

    #[simd_test = "sse"]
    unsafe fn _mm_min_ps() {
        let a = f32x4::new(-1.0, 5.0, 0.0, -10.0);
        let b = f32x4::new(-100.0, 20.0, 0.0, -5.0);
        let r = sse::_mm_min_ps(a, b);
        assert_eq!(r, f32x4::new(-100.0, 5.0, 0.0, -10.0));
    }

    #[simd_test = "sse"]
    unsafe fn _mm_max_ss() {
        let a = f32x4::new(-1.0, 5.0, 0.0, -10.0);
        let b = f32x4::new(-100.0, 20.0, 0.0, -5.0);
        let r = sse::_mm_max_ss(a, b);
        assert_eq!(r, f32x4::new(-1.0, 5.0, 0.0, -10.0));
    }

    #[simd_test = "sse"]
    unsafe fn _mm_max_ps() {
        let a = f32x4::new(-1.0, 5.0, 0.0, -10.0);
        let b = f32x4::new(-100.0, 20.0, 0.0, -5.0);
        let r = sse::_mm_max_ps(a, b);
        assert_eq!(r, f32x4::new(-1.0, 20.0, 0.0, -5.0));
    }

    #[simd_test = "sse"]
    unsafe fn _mm_and_ps() {
        use std::mem::transmute;

        let a: f32x4 = transmute(u32x4::splat(0b0011));
        let b: f32x4 = transmute(u32x4::splat(0b0101));
        let r = sse::_mm_and_ps(*black_box(&a), *black_box(&b));
        let e: f32x4 = transmute(u32x4::splat(0b0001));
        assert_eq!(r, e);
    }

    #[simd_test = "sse"]
    unsafe fn _mm_andnot_ps() {
        use std::mem::transmute;

        let a: f32x4 = transmute(u32x4::splat(0b0011));
        let b: f32x4 = transmute(u32x4::splat(0b0101));
        let r = sse::_mm_andnot_ps(*black_box(&a), *black_box(&b));
        let e: f32x4 = transmute(u32x4::splat(0b0100));
        assert_eq!(r, e);
    }

    #[simd_test = "sse"]
    unsafe fn _mm_or_ps() {
        use std::mem::transmute;

        let a: f32x4 = transmute(u32x4::splat(0b0011));
        let b: f32x4 = transmute(u32x4::splat(0b0101));
        let r = sse::_mm_or_ps(*black_box(&a), *black_box(&b));
        let e: f32x4 = transmute(u32x4::splat(0b0111));
        assert_eq!(r, e);
    }

    #[simd_test = "sse"]
    unsafe fn _mm_xor_ps() {
        use std::mem::transmute;

        let a: f32x4 = transmute(u32x4::splat(0b0011));
        let b: f32x4 = transmute(u32x4::splat(0b0101));
        let r = sse::_mm_xor_ps(*black_box(&a), *black_box(&b));
        let e: f32x4 = transmute(u32x4::splat(0b0110));
        assert_eq!(r, e);
    }

    #[simd_test = "sse"]
    unsafe fn _mm_cmpeq_ss() {
        use std::mem::transmute;

        let a = f32x4::new(1.0, 2.0, 3.0, 4.0);
        let b = f32x4::new(-1.0, 5.0, 6.0, 7.0);
        let r: u32x4 = transmute(sse::_mm_cmpeq_ss(a, b));
        let e: u32x4 = transmute(f32x4::new(transmute(0u32), 2.0, 3.0, 4.0));
        assert_eq!(r, e);

        let b2 = f32x4::new(1.0, 5.0, 6.0, 7.0);
        let r2: u32x4 = transmute(sse::_mm_cmpeq_ss(a, b2));
        let e2: u32x4 =
            transmute(f32x4::new(transmute(0xffffffffu32), 2.0, 3.0, 4.0));
        assert_eq!(r2, e2);
    }

    #[simd_test = "sse"]
    unsafe fn _mm_cmplt_ss() {
        use std::mem::transmute;

        let a = f32x4::new(1.0, 2.0, 3.0, 4.0);
        let b = f32x4::new(0.0, 5.0, 6.0, 7.0);
        let c = f32x4::new(1.0, 5.0, 6.0, 7.0);
        let d = f32x4::new(2.0, 5.0, 6.0, 7.0);

        let b1 = 0u32;  // a.extract(0) < b.extract(0)
        let c1 = 0u32;  // a.extract(0) < c.extract(0)
        let d1 = !0u32;  // a.extract(0) < d.extract(0)

        let rb: u32x4 = transmute(sse::_mm_cmplt_ss(a, b));
        let eb: u32x4 = transmute(f32x4::new(transmute(b1), 2.0, 3.0, 4.0));
        assert_eq!(rb, eb);

        let rc: u32x4 = transmute(sse::_mm_cmplt_ss(a, c));
        let ec: u32x4 = transmute(f32x4::new(transmute(c1), 2.0, 3.0, 4.0));
        assert_eq!(rc, ec);

        let rd: u32x4 = transmute(sse::_mm_cmplt_ss(a, d));
        let ed: u32x4 = transmute(f32x4::new(transmute(d1), 2.0, 3.0, 4.0));
        assert_eq!(rd, ed);
    }

    #[simd_test = "sse"]
    unsafe fn _mm_cmple_ss() {
        use std::mem::transmute;

        let a = f32x4::new(1.0, 2.0, 3.0, 4.0);
        let b = f32x4::new(0.0, 5.0, 6.0, 7.0);
        let c = f32x4::new(1.0, 5.0, 6.0, 7.0);
        let d = f32x4::new(2.0, 5.0, 6.0, 7.0);

        let b1 = 0u32;  // a.extract(0) <= b.extract(0)
        let c1 = !0u32;  // a.extract(0) <= c.extract(0)
        let d1 = !0u32;  // a.extract(0) <= d.extract(0)

        let rb: u32x4 = transmute(sse::_mm_cmple_ss(a, b));
        let eb: u32x4 = transmute(f32x4::new(transmute(b1), 2.0, 3.0, 4.0));
        assert_eq!(rb, eb);

        let rc: u32x4 = transmute(sse::_mm_cmple_ss(a, c));
        let ec: u32x4 = transmute(f32x4::new(transmute(c1), 2.0, 3.0, 4.0));
        assert_eq!(rc, ec);

        let rd: u32x4 = transmute(sse::_mm_cmple_ss(a, d));
        let ed: u32x4 = transmute(f32x4::new(transmute(d1), 2.0, 3.0, 4.0));
        assert_eq!(rd, ed);
    }

    #[simd_test = "sse"]
    unsafe fn _mm_cmpgt_ss() {
        use std::mem::transmute;

        let a = f32x4::new(1.0, 2.0, 3.0, 4.0);
        let b = f32x4::new(0.0, 5.0, 6.0, 7.0);
        let c = f32x4::new(1.0, 5.0, 6.0, 7.0);
        let d = f32x4::new(2.0, 5.0, 6.0, 7.0);

        let b1 = !0u32;  // a.extract(0) > b.extract(0)
        let c1 = 0u32;  // a.extract(0) > c.extract(0)
        let d1 = 0u32;  // a.extract(0) > d.extract(0)

        let rb: u32x4 = transmute(sse::_mm_cmpgt_ss(a, b));
        let eb: u32x4 = transmute(f32x4::new(transmute(b1), 2.0, 3.0, 4.0));
        assert_eq!(rb, eb);

        let rc: u32x4 = transmute(sse::_mm_cmpgt_ss(a, c));
        let ec: u32x4 = transmute(f32x4::new(transmute(c1), 2.0, 3.0, 4.0));
        assert_eq!(rc, ec);

        let rd: u32x4 = transmute(sse::_mm_cmpgt_ss(a, d));
        let ed: u32x4 = transmute(f32x4::new(transmute(d1), 2.0, 3.0, 4.0));
        assert_eq!(rd, ed);
    }

    #[simd_test = "sse"]
    unsafe fn _mm_cmpge_ss() {
        use std::mem::transmute;

        let a = f32x4::new(1.0, 2.0, 3.0, 4.0);
        let b = f32x4::new(0.0, 5.0, 6.0, 7.0);
        let c = f32x4::new(1.0, 5.0, 6.0, 7.0);
        let d = f32x4::new(2.0, 5.0, 6.0, 7.0);

        let b1 = !0u32;  // a.extract(0) >= b.extract(0)
        let c1 = !0u32;  // a.extract(0) >= c.extract(0)
        let d1 = 0u32;  // a.extract(0) >= d.extract(0)

        let rb: u32x4 = transmute(sse::_mm_cmpge_ss(a, b));
        let eb: u32x4 = transmute(f32x4::new(transmute(b1), 2.0, 3.0, 4.0));
        assert_eq!(rb, eb);

        let rc: u32x4 = transmute(sse::_mm_cmpge_ss(a, c));
        let ec: u32x4 = transmute(f32x4::new(transmute(c1), 2.0, 3.0, 4.0));
        assert_eq!(rc, ec);

        let rd: u32x4 = transmute(sse::_mm_cmpge_ss(a, d));
        let ed: u32x4 = transmute(f32x4::new(transmute(d1), 2.0, 3.0, 4.0));
        assert_eq!(rd, ed);
    }

    #[simd_test = "sse"]
    unsafe fn _mm_cmpneq_ss() {
        use std::mem::transmute;

        let a = f32x4::new(1.0, 2.0, 3.0, 4.0);
        let b = f32x4::new(0.0, 5.0, 6.0, 7.0);
        let c = f32x4::new(1.0, 5.0, 6.0, 7.0);
        let d = f32x4::new(2.0, 5.0, 6.0, 7.0);

        let b1 = !0u32;  // a.extract(0) != b.extract(0)
        let c1 = 0u32;  // a.extract(0) != c.extract(0)
        let d1 = !0u32;  // a.extract(0) != d.extract(0)

        let rb: u32x4 = transmute(sse::_mm_cmpneq_ss(a, b));
        let eb: u32x4 = transmute(f32x4::new(transmute(b1), 2.0, 3.0, 4.0));
        assert_eq!(rb, eb);

        let rc: u32x4 = transmute(sse::_mm_cmpneq_ss(a, c));
        let ec: u32x4 = transmute(f32x4::new(transmute(c1), 2.0, 3.0, 4.0));
        assert_eq!(rc, ec);

        let rd: u32x4 = transmute(sse::_mm_cmpneq_ss(a, d));
        let ed: u32x4 = transmute(f32x4::new(transmute(d1), 2.0, 3.0, 4.0));
        assert_eq!(rd, ed);
    }

    #[simd_test = "sse"]
    unsafe fn _mm_cmpnlt_ss() {
        // TODO: This test is exactly the same as for _mm_cmpge_ss, but there
        // must be a difference. It may have to do with behavior in the presence
        // of NaNs (signaling or quiet). If so, we should add tests for those.
        use std::mem::transmute;

        let a = f32x4::new(1.0, 2.0, 3.0, 4.0);
        let b = f32x4::new(0.0, 5.0, 6.0, 7.0);
        let c = f32x4::new(1.0, 5.0, 6.0, 7.0);
        let d = f32x4::new(2.0, 5.0, 6.0, 7.0);

        let b1 = !0u32;  // a.extract(0) >= b.extract(0)
        let c1 = !0u32;  // a.extract(0) >= c.extract(0)
        let d1 = 0u32;  // a.extract(0) >= d.extract(0)

        let rb: u32x4 = transmute(sse::_mm_cmpnlt_ss(a, b));
        let eb: u32x4 = transmute(f32x4::new(transmute(b1), 2.0, 3.0, 4.0));
        assert_eq!(rb, eb);

        let rc: u32x4 = transmute(sse::_mm_cmpnlt_ss(a, c));
        let ec: u32x4 = transmute(f32x4::new(transmute(c1), 2.0, 3.0, 4.0));
        assert_eq!(rc, ec);

        let rd: u32x4 = transmute(sse::_mm_cmpnlt_ss(a, d));
        let ed: u32x4 = transmute(f32x4::new(transmute(d1), 2.0, 3.0, 4.0));
        assert_eq!(rd, ed);
    }

    #[simd_test = "sse"]
    unsafe fn _mm_cmpnle_ss() {
        // TODO: This test is exactly the same as for _mm_cmpgt_ss, but there
        // must be a difference. It may have to do with behavior in the presence
        // of NaNs (signaling or quiet). If so, we should add tests for those.
        use std::mem::transmute;

        let a = f32x4::new(1.0, 2.0, 3.0, 4.0);
        let b = f32x4::new(0.0, 5.0, 6.0, 7.0);
        let c = f32x4::new(1.0, 5.0, 6.0, 7.0);
        let d = f32x4::new(2.0, 5.0, 6.0, 7.0);

        let b1 = !0u32;  // a.extract(0) > b.extract(0)
        let c1 = 0u32;  // a.extract(0) > c.extract(0)
        let d1 = 0u32;  // a.extract(0) > d.extract(0)

        let rb: u32x4 = transmute(sse::_mm_cmpnle_ss(a, b));
        let eb: u32x4 = transmute(f32x4::new(transmute(b1), 2.0, 3.0, 4.0));
        assert_eq!(rb, eb);

        let rc: u32x4 = transmute(sse::_mm_cmpnle_ss(a, c));
        let ec: u32x4 = transmute(f32x4::new(transmute(c1), 2.0, 3.0, 4.0));
        assert_eq!(rc, ec);

        let rd: u32x4 = transmute(sse::_mm_cmpnle_ss(a, d));
        let ed: u32x4 = transmute(f32x4::new(transmute(d1), 2.0, 3.0, 4.0));
        assert_eq!(rd, ed);
    }

    #[simd_test = "sse"]
    unsafe fn _mm_cmpngt_ss() {
        // TODO: This test is exactly the same as for _mm_cmple_ss, but there
        // must be a difference. It may have to do with behavior in the presence
        // of NaNs (signaling or quiet). If so, we should add tests for those.
        use std::mem::transmute;

        let a = f32x4::new(1.0, 2.0, 3.0, 4.0);
        let b = f32x4::new(0.0, 5.0, 6.0, 7.0);
        let c = f32x4::new(1.0, 5.0, 6.0, 7.0);
        let d = f32x4::new(2.0, 5.0, 6.0, 7.0);

        let b1 = 0u32;  // a.extract(0) <= b.extract(0)
        let c1 = !0u32;  // a.extract(0) <= c.extract(0)
        let d1 = !0u32;  // a.extract(0) <= d.extract(0)

        let rb: u32x4 = transmute(sse::_mm_cmpngt_ss(a, b));
        let eb: u32x4 = transmute(f32x4::new(transmute(b1), 2.0, 3.0, 4.0));
        assert_eq!(rb, eb);

        let rc: u32x4 = transmute(sse::_mm_cmpngt_ss(a, c));
        let ec: u32x4 = transmute(f32x4::new(transmute(c1), 2.0, 3.0, 4.0));
        assert_eq!(rc, ec);

        let rd: u32x4 = transmute(sse::_mm_cmpngt_ss(a, d));
        let ed: u32x4 = transmute(f32x4::new(transmute(d1), 2.0, 3.0, 4.0));
        assert_eq!(rd, ed);
    }

    #[simd_test = "sse"]
    unsafe fn _mm_cmpnge_ss() {
        // TODO: This test is exactly the same as for _mm_cmplt_ss, but there
        // must be a difference. It may have to do with behavior in the presence
        // of NaNs (signaling or quiet). If so, we should add tests for those.
        use std::mem::transmute;

        let a = f32x4::new(1.0, 2.0, 3.0, 4.0);
        let b = f32x4::new(0.0, 5.0, 6.0, 7.0);
        let c = f32x4::new(1.0, 5.0, 6.0, 7.0);
        let d = f32x4::new(2.0, 5.0, 6.0, 7.0);

        let b1 = 0u32;  // a.extract(0) < b.extract(0)
        let c1 = 0u32;  // a.extract(0) < c.extract(0)
        let d1 = !0u32;  // a.extract(0) < d.extract(0)

        let rb: u32x4 = transmute(sse::_mm_cmpnge_ss(a, b));
        let eb: u32x4 = transmute(f32x4::new(transmute(b1), 2.0, 3.0, 4.0));
        assert_eq!(rb, eb);

        let rc: u32x4 = transmute(sse::_mm_cmpnge_ss(a, c));
        let ec: u32x4 = transmute(f32x4::new(transmute(c1), 2.0, 3.0, 4.0));
        assert_eq!(rc, ec);

        let rd: u32x4 = transmute(sse::_mm_cmpnge_ss(a, d));
        let ed: u32x4 = transmute(f32x4::new(transmute(d1), 2.0, 3.0, 4.0));
        assert_eq!(rd, ed);
    }

    #[simd_test = "sse"]
    unsafe fn _mm_cmpord_ss() {
        use std::mem::transmute;
        use std::f32::NAN;

        let a = f32x4::new(1.0, 2.0, 3.0, 4.0);
        let b = f32x4::new(0.0, 5.0, 6.0, 7.0);
        let c = f32x4::new(NAN, 5.0, 6.0, 7.0);
        let d = f32x4::new(2.0, 5.0, 6.0, 7.0);

        let b1 = !0u32;  // a.extract(0) ord b.extract(0)
        let c1 = 0u32;  // a.extract(0) ord c.extract(0)
        let d1 = !0u32;  // a.extract(0) ord d.extract(0)

        let rb: u32x4 = transmute(sse::_mm_cmpord_ss(a, b));
        let eb: u32x4 = transmute(f32x4::new(transmute(b1), 2.0, 3.0, 4.0));
        assert_eq!(rb, eb);

        let rc: u32x4 = transmute(sse::_mm_cmpord_ss(a, c));
        let ec: u32x4 = transmute(f32x4::new(transmute(c1), 2.0, 3.0, 4.0));
        assert_eq!(rc, ec);

        let rd: u32x4 = transmute(sse::_mm_cmpord_ss(a, d));
        let ed: u32x4 = transmute(f32x4::new(transmute(d1), 2.0, 3.0, 4.0));
        assert_eq!(rd, ed);
    }

    #[simd_test = "sse"]
    unsafe fn _mm_cmpunord_ss() {
        use std::mem::transmute;
        use std::f32::NAN;

        let a = f32x4::new(1.0, 2.0, 3.0, 4.0);
        let b = f32x4::new(0.0, 5.0, 6.0, 7.0);
        let c = f32x4::new(NAN, 5.0, 6.0, 7.0);
        let d = f32x4::new(2.0, 5.0, 6.0, 7.0);

        let b1 = 0u32;  // a.extract(0) unord b.extract(0)
        let c1 = !0u32;  // a.extract(0) unord c.extract(0)
        let d1 = 0u32;  // a.extract(0) unord d.extract(0)

        let rb: u32x4 = transmute(sse::_mm_cmpunord_ss(a, b));
        let eb: u32x4 = transmute(f32x4::new(transmute(b1), 2.0, 3.0, 4.0));
        assert_eq!(rb, eb);

        let rc: u32x4 = transmute(sse::_mm_cmpunord_ss(a, c));
        let ec: u32x4 = transmute(f32x4::new(transmute(c1), 2.0, 3.0, 4.0));
        assert_eq!(rc, ec);

        let rd: u32x4 = transmute(sse::_mm_cmpunord_ss(a, d));
        let ed: u32x4 = transmute(f32x4::new(transmute(d1), 2.0, 3.0, 4.0));
        assert_eq!(rd, ed);
    }

    unsafe fn _mm_cmpeq_ps() {
        use std::mem::transmute;
        use std::f32::NAN;

        let a = f32x4::new(10.0, 50.0, 1.0, NAN);
        let b = f32x4::new(15.0, 20.0, 1.0, NAN);
        let tru = !0u32;
        let fls = 0u32;

        let e = u32x4::new(fls, fls, tru, fls);
        let r: u32x4 = transmute(sse::_mm_cmpeq_ps(a, b));
        assert_eq!(r, e);
    }

    unsafe fn _mm_cmplt_ps() {
        use std::mem::transmute;
        use std::f32::NAN;

        let a = f32x4::new(10.0, 50.0, 1.0, NAN);
        let b = f32x4::new(15.0, 20.0, 1.0, NAN);
        let tru = !0u32;
        let fls = 0u32;

        let e = u32x4::new(tru, fls, fls, fls);
        let r: u32x4 = transmute(sse::_mm_cmplt_ps(a, b));
        assert_eq!(r, e);
    }

    #[simd_test = "sse"]
    unsafe fn _mm_cmple_ps() {
        use std::mem::transmute;
        use std::f32::NAN;

        let a = f32x4::new(10.0, 50.0, 1.0, 4.0);
        let b = f32x4::new(15.0, 20.0, 1.0, NAN);
        let tru = !0u32;
        let fls = 0u32;

        let e = u32x4::new(tru, fls, tru, fls);
        let r: u32x4 = transmute(sse::_mm_cmple_ps(a, b));
        assert_eq!(r, e);
    }

    #[simd_test = "sse"]
    unsafe fn _mm_cmpgt_ps() {
        use std::mem::transmute;
        use std::f32::NAN;

        let a = f32x4::new(10.0, 50.0, 1.0, NAN);
        let b = f32x4::new(15.0, 20.0, 1.0, 42.0);
        let tru = !0u32;
        let fls = 0u32;

        let e = u32x4::new(fls, tru, fls, fls);
        let r: u32x4 = transmute(sse::_mm_cmpgt_ps(a, b));
        assert_eq!(r, e);
    }

    #[simd_test = "sse"]
    unsafe fn _mm_cmpge_ps() {
        use std::mem::transmute;
        use std::f32::NAN;

        let a = f32x4::new(10.0, 50.0, 1.0, NAN);
        let b = f32x4::new(15.0, 20.0, 1.0, 42.0);
        let tru = !0u32;
        let fls = 0u32;

        let e = u32x4::new(fls, tru, tru, fls);
        let r: u32x4 = transmute(sse::_mm_cmpge_ps(a, b));
        assert_eq!(r, e);
    }

    #[simd_test = "sse"]
    unsafe fn _mm_cmpneq_ps() {
        use std::mem::transmute;
        use std::f32::NAN;

        let a = f32x4::new(10.0, 50.0, 1.0, NAN);
        let b = f32x4::new(15.0, 20.0, 1.0, NAN);
        let tru = !0u32;
        let fls = 0u32;

        let e = u32x4::new(tru, tru, fls, tru);
        let r: u32x4 = transmute(sse::_mm_cmpneq_ps(a, b));
        assert_eq!(r, e);
    }

    #[simd_test = "sse"]
    unsafe fn _mm_cmpnlt_ps() {
        use std::mem::transmute;
        use std::f32::NAN;

        let a = f32x4::new(10.0, 50.0, 1.0, NAN);
        let b = f32x4::new(15.0, 20.0, 1.0, 5.0);
        let tru = !0u32;
        let fls = 0u32;

        let e = u32x4::new(fls, tru, tru, tru);
        let r: u32x4 = transmute(sse::_mm_cmpnlt_ps(a, b));
        assert_eq!(r, e);
    }

    #[simd_test = "sse"]
    unsafe fn _mm_cmpnle_ps() {
        use std::mem::transmute;
        use std::f32::NAN;

        let a = f32x4::new(10.0, 50.0, 1.0, NAN);
        let b = f32x4::new(15.0, 20.0, 1.0, 5.0);
        let tru = !0u32;
        let fls = 0u32;

        let e = u32x4::new(fls, tru, fls, tru);
        let r: u32x4 = transmute(sse::_mm_cmpnle_ps(a, b));
        assert_eq!(r, e);
    }

    #[simd_test = "sse"]
    unsafe fn _mm_cmpngt_ps() {
        use std::mem::transmute;
        use std::f32::NAN;

        let a = f32x4::new(10.0, 50.0, 1.0, NAN);
        let b = f32x4::new(15.0, 20.0, 1.0, 5.0);
        let tru = !0u32;
        let fls = 0u32;

        let e = u32x4::new(tru, fls, tru, tru);
        let r: u32x4 = transmute(sse::_mm_cmpngt_ps(a, b));
        assert_eq!(r, e);
    }

    #[simd_test = "sse"]
    unsafe fn _mm_cmpnge_ps() {
        use std::mem::transmute;
        use std::f32::NAN;

        let a = f32x4::new(10.0, 50.0, 1.0, NAN);
        let b = f32x4::new(15.0, 20.0, 1.0, 5.0);
        let tru = !0u32;
        let fls = 0u32;

        let e = u32x4::new(tru, fls, fls, tru);
        let r: u32x4 = transmute(sse::_mm_cmpnge_ps(a, b));
        assert_eq!(r, e);
    }

    #[simd_test = "sse"]
    unsafe fn _mm_cmpord_ps() {
        use std::mem::transmute;
        use std::f32::NAN;

        let a = f32x4::new(10.0, 50.0, NAN, NAN);
        let b = f32x4::new(15.0, NAN, 1.0, NAN);
        let tru = !0u32;
        let fls = 0u32;

        let e = u32x4::new(tru, fls, fls, fls);
        let r: u32x4 = transmute(sse::_mm_cmpord_ps(a, b));
        assert_eq!(r, e);
    }

    #[simd_test = "sse"]
    unsafe fn _mm_cmpunord_ps() {
        use std::mem::transmute;
        use std::f32::NAN;

        let a = f32x4::new(10.0, 50.0, NAN, NAN);
        let b = f32x4::new(15.0, NAN, 1.0, NAN);
        let tru = !0u32;
        let fls = 0u32;

        let e = u32x4::new(fls, tru, tru, tru);
        let r: u32x4 = transmute(sse::_mm_cmpunord_ps(a, b));
        assert_eq!(r, e);
    }

    #[simd_test = "sse"]
    unsafe fn _mm_comieq_ss() {
        use std::f32::NAN;

        let aa = &[3.0f32, 12.0, 23.0, NAN];
        let bb = &[3.0f32, 47.5, 1.5, NAN];

        let ee = &[1i32, 0, 0, 0];

        for i in 0..4 {
            let a = f32x4::new(aa[i], 1.0, 2.0, 3.0);
            let b = f32x4::new(bb[i], 0.0, 2.0, 4.0);

            let r = sse::_mm_comieq_ss(a, b);

            assert_eq!(ee[i], r,
                "_mm_comieq_ss({:?}, {:?}) = {}, expected: {} (i={})",
                a, b, r, ee[i], i);
        }
    }

    #[simd_test = "sse"]
    unsafe fn _mm_comilt_ss() {
        use std::f32::NAN;

        let aa = &[3.0f32, 12.0, 23.0, NAN];
        let bb = &[3.0f32, 47.5, 1.5, NAN];

        let ee = &[0i32, 1, 0, 0];

        for i in 0..4 {
            let a = f32x4::new(aa[i], 1.0, 2.0, 3.0);
            let b = f32x4::new(bb[i], 0.0, 2.0, 4.0);

            let r = sse::_mm_comilt_ss(a, b);

            assert_eq!(ee[i], r,
                "_mm_comilt_ss({:?}, {:?}) = {}, expected: {} (i={})",
                a, b, r, ee[i], i);
        }
    }

    #[simd_test = "sse"]
    unsafe fn _mm_comile_ss() {
        use std::f32::NAN;

        let aa = &[3.0f32, 12.0, 23.0, NAN];
        let bb = &[3.0f32, 47.5, 1.5, NAN];

        let ee = &[1i32, 1, 0, 0];

        for i in 0..4 {
            let a = f32x4::new(aa[i], 1.0, 2.0, 3.0);
            let b = f32x4::new(bb[i], 0.0, 2.0, 4.0);

            let r = sse::_mm_comile_ss(a, b);

            assert_eq!(ee[i], r,
                "_mm_comile_ss({:?}, {:?}) = {}, expected: {} (i={})",
                a, b, r, ee[i], i);
        }
    }

    #[simd_test = "sse"]
    unsafe fn _mm_comigt_ss() {
        use std::f32::NAN;

        let aa = &[3.0f32, 12.0, 23.0, NAN];
        let bb = &[3.0f32, 47.5, 1.5, NAN];

        let ee = &[0i32, 0, 1, 0];

        for i in 0..4 {
            let a = f32x4::new(aa[i], 1.0, 2.0, 3.0);
            let b = f32x4::new(bb[i], 0.0, 2.0, 4.0);

            let r = sse::_mm_comigt_ss(a, b);

            assert_eq!(ee[i], r,
                "_mm_comigt_ss({:?}, {:?}) = {}, expected: {} (i={})",
                a, b, r, ee[i], i);
        }
    }

    #[simd_test = "sse"]
    unsafe fn _mm_comige_ss() {
        use std::f32::NAN;

        let aa = &[3.0f32, 12.0, 23.0, NAN];
        let bb = &[3.0f32, 47.5, 1.5, NAN];

        let ee = &[1i32, 0, 1, 0];

        for i in 0..4 {
            let a = f32x4::new(aa[i], 1.0, 2.0, 3.0);
            let b = f32x4::new(bb[i], 0.0, 2.0, 4.0);

            let r = sse::_mm_comige_ss(a, b);

            assert_eq!(ee[i], r,
                "_mm_comige_ss({:?}, {:?}) = {}, expected: {} (i={})",
                a, b, r, ee[i], i);
        }
    }

    #[simd_test = "sse"]
    unsafe fn _mm_comineq_ss() {
        use std::f32::NAN;

        let aa = &[3.0f32, 12.0, 23.0, NAN];
        let bb = &[3.0f32, 47.5, 1.5, NAN];

        let ee = &[0i32, 1, 1, 1];

        for i in 0..4 {
            let a = f32x4::new(aa[i], 1.0, 2.0, 3.0);
            let b = f32x4::new(bb[i], 0.0, 2.0, 4.0);

            let r = sse::_mm_comineq_ss(a, b);

            assert_eq!(ee[i], r,
                "_mm_comineq_ss({:?}, {:?}) = {}, expected: {} (i={})",
                a, b, r, ee[i], i);
        }
    }

    #[simd_test = "sse"]
    unsafe fn _mm_ucomieq_ss() {
        use std::f32::NAN;

        let aa = &[3.0f32, 12.0, 23.0, NAN];
        let bb = &[3.0f32, 47.5, 1.5, NAN];

        let ee = &[1i32, 0, 0, 0];

        for i in 0..4 {
            let a = f32x4::new(aa[i], 1.0, 2.0, 3.0);
            let b = f32x4::new(bb[i], 0.0, 2.0, 4.0);

            let r = sse::_mm_ucomieq_ss(a, b);

            assert_eq!(ee[i], r,
                "_mm_ucomieq_ss({:?}, {:?}) = {}, expected: {} (i={})",
                a, b, r, ee[i], i);
        }
    }

    #[simd_test = "sse"]
    unsafe fn _mm_ucomilt_ss() {
        use std::f32::NAN;

        let aa = &[3.0f32, 12.0, 23.0, NAN];
        let bb = &[3.0f32, 47.5, 1.5, NAN];

        let ee = &[0i32, 1, 0, 0];

        for i in 0..4 {
            let a = f32x4::new(aa[i], 1.0, 2.0, 3.0);
            let b = f32x4::new(bb[i], 0.0, 2.0, 4.0);

            let r = sse::_mm_ucomilt_ss(a, b);

            assert_eq!(ee[i], r,
                "_mm_ucomilt_ss({:?}, {:?}) = {}, expected: {} (i={})",
                a, b, r, ee[i], i);
        }
    }

    #[simd_test = "sse"]
    unsafe fn _mm_ucomile_ss() {
        use std::f32::NAN;

        let aa = &[3.0f32, 12.0, 23.0, NAN];
        let bb = &[3.0f32, 47.5, 1.5, NAN];

        let ee = &[1i32, 1, 0, 0];

        for i in 0..4 {
            let a = f32x4::new(aa[i], 1.0, 2.0, 3.0);
            let b = f32x4::new(bb[i], 0.0, 2.0, 4.0);

            let r = sse::_mm_ucomile_ss(a, b);

            assert_eq!(ee[i], r,
                "_mm_ucomile_ss({:?}, {:?}) = {}, expected: {} (i={})",
                a, b, r, ee[i], i);
        }
    }

    #[simd_test = "sse"]
    unsafe fn _mm_ucomigt_ss() {
        use std::f32::NAN;

        let aa = &[3.0f32, 12.0, 23.0, NAN];
        let bb = &[3.0f32, 47.5, 1.5, NAN];

        let ee = &[0i32, 0, 1, 0];

        for i in 0..4 {
            let a = f32x4::new(aa[i], 1.0, 2.0, 3.0);
            let b = f32x4::new(bb[i], 0.0, 2.0, 4.0);

            let r = sse::_mm_ucomigt_ss(a, b);

            assert_eq!(ee[i], r,
                "_mm_ucomigt_ss({:?}, {:?}) = {}, expected: {} (i={})",
                a, b, r, ee[i], i);
        }
    }

    #[simd_test = "sse"]
    unsafe fn _mm_ucomige_ss() {
        use std::f32::NAN;

        let aa = &[3.0f32, 12.0, 23.0, NAN];
        let bb = &[3.0f32, 47.5, 1.5, NAN];

        let ee = &[1i32, 0, 1, 0];

        for i in 0..4 {
            let a = f32x4::new(aa[i], 1.0, 2.0, 3.0);
            let b = f32x4::new(bb[i], 0.0, 2.0, 4.0);

            let r = sse::_mm_ucomige_ss(a, b);

            assert_eq!(ee[i], r,
                "_mm_ucomige_ss({:?}, {:?}) = {}, expected: {} (i={})",
                a, b, r, ee[i], i);
        }
    }

    #[simd_test = "sse"]
    unsafe fn _mm_ucomineq_ss() {
        use std::f32::NAN;

        let aa = &[3.0f32, 12.0, 23.0, NAN];
        let bb = &[3.0f32, 47.5, 1.5, NAN];

        let ee = &[0i32, 1, 1, 1];

        for i in 0..4 {
            let a = f32x4::new(aa[i], 1.0, 2.0, 3.0);
            let b = f32x4::new(bb[i], 0.0, 2.0, 4.0);

            let r = sse::_mm_ucomineq_ss(a, b);

            assert_eq!(ee[i], r,
                "_mm_ucomineq_ss({:?}, {:?}) = {}, expected: {} (i={})",
                a, b, r, ee[i], i);
        }
    }

    #[simd_test = "sse"]
    unsafe fn _mm_comieq_ss_vs_ucomieq_ss() {
        // If one of the arguments is a quiet NaN `comieq_ss` should signal an
        // Invalid Operation Exception while `ucomieq_ss` should not.
        use std::f32::NAN;  // This is a quiet NaN.
        let aa = &[3.0f32, NAN, 23.0, NAN];
        let bb = &[3.0f32, 47.5, NAN, NAN];

        let ee = &[1i32, 0, 0, 0];
        let exc = &[0u32, 1, 1, 1];  // Should comieq_ss signal an exception?

        for i in 0..4 {
            let a = f32x4::new(aa[i], 1.0, 2.0, 3.0);
            let b = f32x4::new(bb[i], 0.0, 2.0, 4.0);

            sse::_MM_SET_EXCEPTION_STATE(0);
            let r1 = sse::_mm_comieq_ss(*black_box(&a), b);
            let s1 = sse::_MM_GET_EXCEPTION_STATE();

            sse::_MM_SET_EXCEPTION_STATE(0);
            let r2 = sse::_mm_ucomieq_ss(*black_box(&a), b);
            let s2 = sse::_MM_GET_EXCEPTION_STATE();

            assert_eq!(ee[i], r1,
                "_mm_comeq_ss({:?}, {:?}) = {}, expected: {} (i={})",
                a, b, r1, ee[i], i);
            assert_eq!(ee[i], r2,
                "_mm_ucomeq_ss({:?}, {:?}) = {}, expected: {} (i={})",
                a, b, r2, ee[i], i);
            assert_eq!(s1, exc[i] * sse::_MM_EXCEPT_INVALID,
                "_mm_comieq_ss() set exception flags: {} (i={})", s1, i);
            assert_eq!(s2, 0,  // ucomieq_ss should not signal an exception
                "_mm_ucomieq_ss() set exception flags: {} (i={})", s2, i);
        }
    }

    #[simd_test = "sse"]
    unsafe fn _mm_set_ss() {
        let r = sse::_mm_set_ss(black_box(4.25));
        assert_eq!(r, f32x4::new(4.25, 0.0, 0.0, 0.0));
    }

    #[simd_test = "sse"]
    unsafe fn _mm_set1_ps() {
        let r1 = sse::_mm_set1_ps(black_box(4.25));
        let r2 = sse::_mm_set_ps1(black_box(4.25));
        assert_eq!(r1, f32x4::splat(4.25));
        assert_eq!(r2, f32x4::splat(4.25));
    }

    #[simd_test = "sse"]
    unsafe fn _mm_set_ps() {
        let r = sse::_mm_set_ps(
            black_box(1.0), black_box(2.0), black_box(3.0), black_box(4.0));
        assert_eq!(r, f32x4::new(4.0, 3.0, 2.0, 1.0));
    }

    #[simd_test = "sse"]
    unsafe fn _mm_setr_ps() {
        let r = sse::_mm_setr_ps(
            black_box(1.0), black_box(2.0), black_box(3.0), black_box(4.0));
        assert_eq!(r, f32x4::new(1.0, 2.0, 3.0, 4.0));
    }

    #[simd_test = "sse"]
    unsafe fn _mm_setzero_ps() {
        let r = *black_box(&sse::_mm_setzero_ps());
        assert_eq!(r, f32x4::splat(0.0));
    }

    #[simd_test = "sse"]
    unsafe fn _mm_shuffle_ps() {
        let a = f32x4::new(1.0, 2.0, 3.0, 4.0);
        let b = f32x4::new(5.0, 6.0, 7.0, 8.0);
        let mask = 0b00_01_01_11;
        let r = sse::_mm_shuffle_ps(a, b, mask);
        assert_eq!(r, f32x4::new(4.0, 2.0, 6.0, 5.0));
    }

    #[simd_test = "sse"]
    unsafe fn _mm_unpackhi_ps() {
        let a = f32x4::new(1.0, 2.0, 3.0, 4.0);
        let b = f32x4::new(5.0, 6.0, 7.0, 8.0);
        let r = sse::_mm_unpackhi_ps(a, b);
        assert_eq!(r, f32x4::new(3.0, 7.0, 4.0, 8.0));
    }

    #[simd_test = "sse"]
    unsafe fn _mm_unpacklo_ps() {
        let a = f32x4::new(1.0, 2.0, 3.0, 4.0);
        let b = f32x4::new(5.0, 6.0, 7.0, 8.0);
        let r = sse::_mm_unpacklo_ps(a, b);
        assert_eq!(r, f32x4::new(1.0, 5.0, 2.0, 6.0));
    }

    #[simd_test = "sse"]
    unsafe fn _mm_movehl_ps() {
        let a = f32x4::new(1.0, 2.0, 3.0, 4.0);
        let b = f32x4::new(5.0, 6.0, 7.0, 8.0);
        let r = sse::_mm_movehl_ps(a, b);
        assert_eq!(r, f32x4::new(7.0, 8.0, 3.0, 4.0));
    }

    #[simd_test = "sse"]
    unsafe fn _mm_movelh_ps() {
        let a = f32x4::new(1.0, 2.0, 3.0, 4.0);
        let b = f32x4::new(5.0, 6.0, 7.0, 8.0);
        let r = sse::_mm_movelh_ps(a, b);
        assert_eq!(r, f32x4::new(1.0, 2.0, 5.0, 6.0));
    }

    #[simd_test = "sse"]
    unsafe fn _mm_loadh_pi() {
        let a = f32x4::new(1.0, 2.0, 3.0, 4.0);
        let x: [f32; 4] = [5.0, 6.0, 7.0, 8.0];
        let p = x[..].as_ptr();
        let r = sse::_mm_loadh_pi(a, p);
        assert_eq!(r, f32x4::new(1.0, 2.0, 5.0, 6.0));
    }

    #[simd_test = "sse"]
    unsafe fn _mm_loadl_pi() {
        let a = f32x4::new(1.0, 2.0, 3.0, 4.0);
        let x: [f32; 4] = [5.0, 6.0, 7.0, 8.0];
        let p = x[..].as_ptr();
        let r = sse::_mm_loadl_pi(a, p);
        assert_eq!(r, f32x4::new(5.0, 6.0, 3.0, 4.0));
    }

    #[simd_test = "sse"]
    unsafe fn _mm_load_ss() {
        let a = 42.0f32;
        let r = sse::_mm_load_ss(&a as *const f32);
        assert_eq!(r, f32x4::new(42.0, 0.0, 0.0, 0.0));
    }

    #[simd_test = "sse"]
    unsafe fn _mm_load1_ps() {
        let a = 42.0f32;
        let r = sse::_mm_load1_ps(&a as *const f32);
        assert_eq!(r, f32x4::new(42.0, 42.0, 42.0, 42.0));
    }

    #[simd_test = "sse"]
    unsafe fn _mm_load_ps() {
        let vals = &[1.0f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];

        let mut p = vals.as_ptr();
        let mut fixup = 0.0f32;

        // Make sure p is aligned, otherwise we might get a
        // (signal: 11, SIGSEGV: invalid memory reference)

        let unalignment = (p as usize) & 0xf;
        if unalignment != 0 {
            let delta = ((16 - unalignment) >> 2) as isize;
            fixup = delta as f32;
            p = p.offset(delta);
        }

        let r = sse::_mm_load_ps(p);
        assert_eq!(r, f32x4::new(1.0, 2.0, 3.0, 4.0) + f32x4::splat(fixup));
    }

    #[simd_test = "sse"]
    unsafe fn _mm_loadu_ps() {
        let vals = &[1.0f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
        let p = vals.as_ptr().offset(3);
        let r = sse::_mm_loadu_ps(black_box(p));
        assert_eq!(r, f32x4::new(4.0, 5.0, 6.0, 7.0));
    }

    #[simd_test = "sse"]
    unsafe fn _mm_loadr_ps() {
        let vals = &[1.0f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];

        let mut p = vals.as_ptr();
        let mut fixup = 0.0f32;

        // Make sure p is aligned, otherwise we might get a
        // (signal: 11, SIGSEGV: invalid memory reference)

        let unalignment = (p as usize) & 0xf;
        if unalignment != 0 {
            let delta = ((16 - unalignment) >> 2) as isize;
            fixup = delta as f32;
            p = p.offset(delta);
        }

        let r = sse::_mm_loadr_ps(p);
        assert_eq!(r, f32x4::new(4.0, 3.0, 2.0, 1.0) + f32x4::splat(fixup));
    }

    #[simd_test = "sse"]
    unsafe fn _mm_movemask_ps() {
        let r = sse::_mm_movemask_ps(f32x4::new(-1.0, 5.0, -5.0, 0.0));
        assert_eq!(r, 0b0101);

        let r = sse::_mm_movemask_ps(f32x4::new(-1.0, -5.0, -5.0, 0.0));
        assert_eq!(r, 0b0111);
    }

    #[simd_test = "sse"]
    unsafe fn _mm_sfence() {
        sse::_mm_sfence();
    }

    #[simd_test = "sse"]
    unsafe fn _mm_getcsr_setcsr_1() {
        let saved_csr = sse::_mm_getcsr();

        let a = f32x4::new(1.1e-36, 0.0, 0.0, 1.0);
        let b = f32x4::new(0.001, 0.0, 0.0, 1.0);

        sse::_MM_SET_FLUSH_ZERO_MODE(sse::_MM_FLUSH_ZERO_ON);
        let r = sse::_mm_mul_ps(*black_box(&a), *black_box(&b));

        sse::_mm_setcsr(saved_csr);

        let exp = f32x4::new(0.0, 0.0, 0.0, 1.0);
        assert_eq!(r, exp);  // first component is a denormalized f32
    }

    #[simd_test = "sse"]
    unsafe fn _mm_getcsr_setcsr_2() {
        // Same as _mm_setcsr_1 test, but with opposite flag value.

        let saved_csr = sse::_mm_getcsr();

        let a = f32x4::new(1.1e-36, 0.0, 0.0, 1.0);
        let b = f32x4::new(0.001, 0.0, 0.0, 1.0);

        sse::_MM_SET_FLUSH_ZERO_MODE(sse::_MM_FLUSH_ZERO_OFF);
        let r = sse::_mm_mul_ps(*black_box(&a), *black_box(&b));

        sse::_mm_setcsr(saved_csr);

        let exp = f32x4::new(1.1e-39, 0.0, 0.0, 1.0);
        assert_eq!(r, exp);  // first component is a denormalized f32
    }

    #[simd_test = "sse"]
    unsafe fn _mm_getcsr_setcsr_underflow() {
        sse::_MM_SET_EXCEPTION_STATE(0);

        let a = f32x4::new(1.1e-36, 0.0, 0.0, 1.0);
        let b = f32x4::new(1e-5, 0.0, 0.0, 1.0);

        assert_eq!(sse::_MM_GET_EXCEPTION_STATE(), 0);  // just to be sure

        let r = sse::_mm_mul_ps(*black_box(&a), *black_box(&b));

        let exp = f32x4::new(1.1e-41, 0.0, 0.0, 1.0);
        assert_eq!(r, exp);

        let underflow =
            sse::_MM_GET_EXCEPTION_STATE() & sse::_MM_EXCEPT_UNDERFLOW != 0;
        assert_eq!(underflow, true);
    }

    #[simd_test = "sse"]
    unsafe fn _MM_TRANSPOSE4_PS() {
        let mut a = f32x4::new(1.0, 2.0, 3.0, 4.0);
        let mut b = f32x4::new(5.0, 6.0, 7.0, 8.0);
        let mut c = f32x4::new(9.0, 10.0, 11.0, 12.0);
        let mut d = f32x4::new(13.0, 14.0, 15.0, 16.0);

        sse::_MM_TRANSPOSE4_PS(&mut a, &mut b, &mut c, &mut d);

        assert_eq!(a, f32x4::new(1.0, 5.0, 9.0, 13.0));
        assert_eq!(b, f32x4::new(2.0, 6.0, 10.0, 14.0));
        assert_eq!(c, f32x4::new(3.0, 7.0, 11.0, 15.0));
        assert_eq!(d, f32x4::new(4.0, 8.0, 12.0, 16.0));
    }
}
