use crate::types::{opcode::*, value_type::*};
use crate::Char; 

pub(crate) struct StateControllerBase {
    data: Vec<u8>,
}

impl StateControllerBase {
    pub(crate) fn add(&mut self, id: u8, exp: Vec<BytecodeExp>) {
        self.data.push(id); 
        self.data.push(exp.len() as u8); 

        for e in exp {
            let len_bytes: [u8; 4] = (e.len() as i32).to_le_bytes();
            self.extend_from_slice(&len_bytes);
            self.extend_from_slice(e.as_bytes().as_ref());
        }
    }

    fn push(&mut self, value: u8) {
        self.data.push(value);
    }

    fn extend_from_slice(&mut self, slice: &[u8]) {
        self.data.extend(slice.iter().copied());
    }

}

pub(crate) trait StateController {
    fn run(&self, c: &Char, ps: &[i32]) -> bool;
}
pub(crate) struct BytecodeExp {
    data: Vec<OpCode>,
}

impl BytecodeExp {
    pub fn new() -> Self {  Self {data: Vec::new() } } 

    pub fn len(&self) -> usize {
        self.data.len() 
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        self.data.clone()
    }

    pub fn append(&mut self, op: &[OpCode]) {
        self.data.extend_from_slice(op);
    }

    pub fn append_i32_p(&mut self, op: OpCode, addr: i32) {
        self.append(&[op]);
        self.append(addr.to_le_bytes().as_slice()); 
    }

    pub fn append_i64_p(&mut self, op: OpCode, addr: i64) {
        self.append(&[op]);
        self.append(addr.to_le_bytes().as_slice()); 
    }

    pub fn neg(v: &mut BytecodeValue) {
        if v.t == VT_Float {
            v.v *= -1.0; 
        } else {
            v.set_i(-v.to_i()); 
        }
    }

    pub fn not(v: &mut BytecodeValue) {
        v.set_i(!v.to_i());
    }

    pub fn blnot(v: &mut BytecodeValue) {
        v.set_b(!v.to_b());
    }

    pub fn and(v1: &mut BytecodeValue, v2: BytecodeValue) {
        v1.set_i(v1.to_i() & v2.to_i());
    }

    pub fn xor(v1: &mut BytecodeValue, v2: BytecodeValue) {
        v1.set_i(v1.to_i() ^ v2.to_i());
    }

    pub fn or(v1: &mut BytecodeValue, v2: BytecodeValue) {
        v1.set_i(v1.to_i() | v2.to_i());
    }

    pub fn bland(v1: &mut BytecodeValue, v2: BytecodeValue) {
        v1.set_b(v1.to_b() && v2.to_b());
    }

    pub fn blor(v1: &mut BytecodeValue, v2: BytecodeValue) {
        v1.set_b(v1.to_b() || v2.to_b()); 
    }

    pub fn blxor(v1: &mut BytecodeValue, v2: BytecodeValue) {
        v1.set_b(v1.to_b() != v2.to_b());
    }

    pub fn abs(v1: &mut BytecodeValue) {
        if v1.t == VT_Float {
            v1.v = f64::abs(v1.v);
        } else {
            v1.set_i(i32::abs(v1.to_i()));
        }
    }

    pub fn exp(v1: &mut BytecodeValue) {
        v1.set_f(f32::exp(v1.v as f32));
    }

    pub fn ln(v1: &mut BytecodeValue) {
        if v1.v <= 0.0 {
            *v1 = BytecodeValue::bytecode_sf();
        } else {
            v1.set_f(f32::ln(v1.v as f32));
        }
    }

    pub fn log(v1: &mut BytecodeValue, v2: BytecodeValue) {
        if v1.v <= 0.0 || v2.v <= 0.0 {
            *v1 = BytecodeValue::bytecode_sf();
        } else {
            v1.set_f(f32::ln(v2.v as f32) / f32::ln(v1.v as f32));
        }
    }

    pub fn cos(v1: &mut BytecodeValue) {
        v1.set_f(f32::cos(v1.v as f32));
    }

    pub fn sin(v1: &mut BytecodeValue) {
        v1.set_f(f32::sin(v1.v as f32));
    }

    pub fn tan(v1: &mut BytecodeValue) {
        v1.set_f(f32::tan(v1.v as f32));
    }

    pub fn acos(v1: &mut BytecodeValue) {
        v1.set_f(f32::acos(v1.v as f32));
    }

    pub fn asin(v1: &mut BytecodeValue) {
        v1.set_f(f32::asin(v1.v as f32));
    }

    pub fn atan(v1: &mut BytecodeValue) {
        v1.set_f(f32::atan(v1.v as f32));
    }

    pub fn floor(v1: &mut BytecodeValue) {
        if v1.t == VT_Float {
            let f = f32::floor(v1.v as f32);
            if f32::is_nan(f) {
                *v1 = BytecodeValue::bytecode_sf();
            } else {
                v1.set_i(f as i32);
            }
        }
    }

    pub fn ceil(v1: &mut BytecodeValue) {
        if v1.t == VT_Float {
            let f = f32::ceil(v1.v as f32);
            if f32::is_nan(f) {
                *v1 = BytecodeValue::bytecode_sf();
            } else {
                v1.set_i(f as i32);
            }
        }
    }

    pub fn max(v1: &mut BytecodeValue, v2: BytecodeValue) {
        if v1.v >= v2.v {
            v1.set_f(v1.v as f32);
        } else {
            v1.set_f(v2.v as f32);
        }
    }

    pub fn min(v1: &mut BytecodeValue, v2: BytecodeValue) {
        if v1.v <= v2.v {
            v1.set_f(v1.v as f32);
        } else {
            v1.set_f(v2.v as f32);
        }
    }

/*    pub fn random(v1: &mut BytecodeValue, v2: BytecodeValue) {
        v1.set_i(RandI(v1.to_i(), v2.to_i()));
    }
*/ 
    pub fn round(v1: &mut BytecodeValue, v2: BytecodeValue) {
        let shift = f32::powi(10.0, v2.v as i32);
        v1.set_f(f32::floor((v1.v as f32 * shift + 0.5)  / shift as f32));
    }

    pub fn append_value(&mut self, bv: BytecodeValue) -> bool {
        match bv.t {
            VT_Float => {
                self.append(&[OC_float]);
                let f = bv.v as f32; 
                self.append(f.to_le_bytes().as_slice()); 
            }, 
            VT_Int => {
                if bv.v >= -128_f64 && bv.v <= 127_f64 {
                    self.append(&[OC_int8, bv.v as u8]); 
                } else if bv.v >= i32::MIN as f64 && bv.v <= i32::MAX as f64 {
                    self.append(&[OC_int]); 
                    let i :i32 = bv.v as i32; 
                    self.append(i.to_le_bytes().as_slice());
                } else {
                    self.append(&[OC_int64]);
                    let i: i64 = bv.v as i64; 
                    self.append(i.to_le_bytes().as_slice()); 
                }
            }, 
            VT_Bool => {
                if bv.v != 0_f64 {
                    self.append(&[OC_int8, 1]);
                } else {
                    self.append(&[OC_int8, 0]); 
                }
            }, 
            VT_SFalse => {
                self.append(&[OC_int8, 0]);
            }, 
            _ => { return false; } 
        }
        return true; 
    }

}

#[derive(Copy, Clone)]
pub(crate) struct BytecodeValue {
    t: ValueType,
    v: f64,
}

impl BytecodeValue {
    pub(crate) fn new() -> Self {
        Self {
            t:0, 
            v:0.0, 
        }
    }
    pub(crate) fn is_none(&self) -> bool {
        self.t == VT_None
    }
    pub(crate) fn is_sf(&self) -> bool {
        self.t == VT_SFalse
    }
    pub(crate) fn to_f(&self) -> f32 {
        if self.is_sf() {
            return 0 as f32;
        }
        self.v as f32
    }
    pub(crate) fn to_i(&self) -> i32 {
        if self.is_sf() {
            return 0;
        }
        self.v as i32
    }
    pub(crate) fn to_i64(&self) -> i64 {
        if self.is_sf() {
            return 0;
        }
        self.v as i64
    }
    pub(crate) fn to_b(&self) -> bool {
        if self.is_sf() || self.v == 0 as f64 {
            return false;
        }
        return true;
    }
    pub(crate) fn set_f(&mut self, f: f32) {
        if (f as f64).is_nan() {
            self.set(BytecodeValue::bytecode_sf());
        } else {
            self.set(BytecodeValue {
                t: VT_Float,
                v: f as f64,
            });
        }
    }

    pub(crate) fn set_i(&mut self, i: i32) {
        self.set(BytecodeValue {
            t: VT_Int,
            v: i as f64,
        })
    }

    pub(crate) fn set_i64(&mut self, i: i64) {
        self.set(BytecodeValue {
            t: VT_Int,
            v: i as f64,
        })
    }

    pub(crate) fn set_b(&mut self, b: bool) {
        self.set(BytecodeValue {
            t: VT_Bool,
            v: b as i64 as f64,
        })
    }

    pub(crate) fn set(&mut self, other: BytecodeValue) {
        self.t = other.t;
        self.v = other.v;
    }

    pub(crate) fn bv_none() -> BytecodeValue {
        BytecodeValue { t: VT_None, v: 0.0 }
    }

    pub(crate) fn bytecode_sf() -> BytecodeValue {
        BytecodeValue {
            t: VT_SFalse,
            v: std::f64::NAN,
        }
    }

    pub(crate) fn bytecode_float(f: f32) -> BytecodeValue {
        BytecodeValue {
            t: VT_Float,
            v: f as f64,
        }
    }

    pub(crate) fn  bytecode_int(i: i32) -> BytecodeValue {
        BytecodeValue {
            t: VT_Int,
            v: i as f64,
        }
    }

    pub(crate) fn bytecode_int64(i: i64) -> BytecodeValue {
        BytecodeValue {
            t: VT_Int,
            v: i as f64,
        }
    }

  pub(crate) fn bytecode_bool(b: bool) -> BytecodeValue {
        BytecodeValue {
            t: VT_Bool,
            v: b as i64 as f64,
        }
    }
}

pub(crate) struct StateBlock {

}