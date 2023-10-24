use crate::bytecode::{
    BytecodeExp, BytecodeValue, StateBlock, StateController, StateControllerBase,
};
use crate::ini::IniSection;
use crate::types::opcode::OC_blor;
use crate::types::{attack_type::*, opcode::*, state_type::*, value_type::*};
use crate::utils;
use std::collections::HashMap;
use std::str::FromStr;

type ScFunc =
    fn(&Compiler, IniSection, &StateControllerBase, i8) -> Result<Box<dyn StateController>, String>;
type ExpFunc = fn(&mut Compiler, &BytecodeExp, &str) -> Result<BytecodeValue, String>;


pub(crate) struct Compiler {
    token: String,
    previous_operator: String,
    reverse_order: bool,
    no_range: bool,
    player_no: i64,
    sc_map: HashMap<&'static str, ScFunc>,
    i: usize,
    block: Option<StateBlock>,
}

impl Compiler {
    const SPECIAL_SYMBOLS: &str = " !=<>()|&+-*/%,[]^:;{}#\"\t\r\n";

    fn tokenizer<'a> (&mut self, input: &'a str) -> (&'a str, String) {
        let (input, output) = self.tokenizer_cs(input);
        (input, output.to_lowercase())
    }

    fn tokenizer_cs<'a, 'b> (&mut self, input: &'a str) -> (&'a str, &'a str) {
        let input = input.trim();
        if input.len() == 0 {
            return (input, "");
        }
        match input.chars().nth(0).unwrap() {
            '=' => {
                return (&input[1..], "=");
            }
            ':' => {
                if (input.len()) >= 2 && input.chars().nth(1).unwrap() == '=' {
                    return (&input[2..], ":=");
                }
                return (&input[1..], ":");
            }
            ';' => {
                return (&input[1..], ";");
            }
            '!' => {
                if (input.len()) >= 2 && input.chars().nth(1).unwrap() == '=' {
                    return (&input[2..], "!=");
                }
                return (&input[1..], "!");
            }
            '>' => {
                if input.len() >= 2 && input.chars().nth(1).unwrap() == '=' {
                    return (&input[2..], ">=");
                }
                return (&input[1..], ">");
            }
            '<' => {
                if input.len() >= 2 && input.chars().nth(1).unwrap() == '=' {
                    return (&input[2..], "<=");
                }
                return (&input[1..], "<");
            }
            '~' => {
                return (&input[1..], "~");
            }
            '&' => {
                if input.len() >= 2 && input.chars().nth(1).unwrap() == '&' {
                    return (&input[2..], "&&");
                }
                return (&input[1..], "&");
            }
            '^' => {
                if input.len() >= 2 && input.chars().nth(1).unwrap() == '^' {
                    return (&input[2..], "^^");
                }
                return (&input[1..], "^");
            }
            '|' => {
                if input.len() >= 2 && input.chars().nth(1).unwrap() == '|' {
                    return (&input[2..], "||");
                }
                return (&input[1..], "|");
            }
            '+' => {
                return (&input[1..], "+");
            }
            '-' => {
                return (&input[1..], "-");
            }
            '*' => {
                if input.len() >= 2 && input.chars().nth(1).unwrap() == '*' {
                    return (&input[2..], "**");
                }
                return (&input[1..], "*");
            }
            '/' => {
                return (&input[1..], "/");
            }
            '%' => {
                return (&input[1..], "%");
            }
            ',' => {
                return (&input[1..], ",");
            }
            '(' => {
                return (&input[1..], "(");
            }
            ')' => {
                return (&input[1..], ")");
            }
            '[' => {
                return (&input[1..], "[");
            }
            ']' => {
                return (&input[1..], "]");
            }
            '"' => {
                return (&input[1..], "\"");
            }
            '{' => {
                return (&input[1..], "{");
            }
            '}' => {
                return (&input[1..], "}");
            }
            _ => {}
        }

        let mut i = 0;
        let mut ten = false;
        while i < input.len() {
            let cur = input.chars().nth(i).unwrap();
            if cur == '.' {
                if ten {
                    break;
                }
                ten = true;
            } else if cur < '0' || cur > '9' {
                break;
            }
            i += 1;
        }

        if i > 0 && i < input.len() && input.chars().nth(i).unwrap() == 'e'
            || input.chars().nth(i).unwrap() == 'E'
        {
            let j = i + 1;
            i += 1;
            while i < input.len() {
                let cur = input.chars().nth(i).unwrap();
                if cur < '0' || cur > '9' && (i != j || (cur != '-' && cur != '+')) {
                    break;
                }
                i += 1;
            }
        }

        if i == 0 {
            if let Some(index) = utils::index_any(input, Compiler::SPECIAL_SYMBOLS) {
                i = index;
            } else {
                i = input.len();
            }
        }

        (&input[i..], &input[..i])
    }

    fn is_operator(token: &str) -> i64 {
        match token {
            "" | "," | ")" | "]" => -1,
            "||" => 1,
            "^^" => 2,
            "&&" => 3,
            "|" => 4,
            "^" => 5,
            "&" => 6,
            "=" | "!=" => 7,
            ">" | ">=" | "<" | "<=" => 8,
            "+" | "-" => 9,
            "*" | "/" | "%" => 10,
            "**" => 11,
            _ => 0,
        }
    }

    fn operator(&mut self, input: &str) -> Result<&str, String> {
        if self.previous_operator.len() > 0 {
            let opp = Compiler::is_operator(self.token.as_str());
            if opp <= Compiler::is_operator(self.previous_operator.as_str()) {
                let token_first = self.token.chars().nth(0).unwrap();
                if opp < 0
                    || ((!self.reverse_order || token_first != '(')
                        && (token_first < 'A' || token_first > 'Z')
                        && (token_first < 'a' || token_first > 'z'))
                {
                    return Err(format!("Invalid data: {:?}", self.previous_operator));
                }

                self.token = self.previous_operator;
                self.previous_operator = "".to_string();
                self.no_range = true;
                return Ok(format!("{:?} {:?}", self.token, input).as_str());
            }
        }
        Ok(input)
    }

    fn integer2(&mut self, input: &str) -> Result<(&str, i32), String> {
        let mut istr = self.token;
        let mut input: &str;
        (input, self.token) = self.tokenizer(input);
        let minus = istr == "-";
        if minus {
            istr = self.token;
            (input, self.token) = self.tokenizer(input);
        }
        for c in istr.chars() {
            if c < '0' || c > '9' {
                return Err(format!("{:?} is not an integer", istr));
            }
        }
        let mut i: i32 = utils::atoi(istr.as_str());
        if minus {
            i = i * -1;
        }
        Ok((input, i))
    }

    fn number(&mut self, token: &str) -> BytecodeValue {
        match f64::from_str(token) {
            Ok(f) => {
                if f == 0.0 {
                    return BytecodeValue::bv_none();
                }

                if token.contains('.') {
                    self.reverse_order = false;
                    return BytecodeValue::bytecode_float(f as f32);
                }

                if token.contains('E') || token.contains('e') {
                    return BytecodeValue::bv_none();
                }

                self.reverse_order = false;

                if (f as f64) > f64::from(i32::MAX) {
                    return BytecodeValue::bytecode_int(i32::MAX);
                }

                if (f as f64) < f64::from(i32::MIN) {
                    return BytecodeValue::bytecode_int(i32::MIN);
                }

                return BytecodeValue::bytecode_int(f as i32);
            }
            Err(_) => BytecodeValue::bv_none(),
        }
    }

    pub(crate) fn state_param(
        is: &mut IniSection,
        name: &str,
        f: impl FnOnce(&str) -> Result<(), String>,
    ) -> Result<(), String> {
        if let Some(data) = is.remove(name) {
            if let Err(e) = f(&data) {
                return Err(e);
            }
        }
        Ok(())
    }

    pub(crate) fn param_value(
        &mut self,
        is: &mut IniSection,
        sc: &mut StateControllerBase,
        param_name: &str,
        id: u8,
        vt: ValueType,
        num_arg: i32,
        mandatory: bool,
    ) -> Result<(), String> {
        let mut f = false;
        Compiler::state_param(is, param_name, |data| -> Result<(), String> {
            f = true;
            self.sc_add(sc, id, data, vt, num_arg, None)
        })?;

        if mandatory && !f {
            return Err(format!("{:?} not specified", param_name));
        }
        Ok(())
    }

    pub(crate) fn exprs(
        &mut self,
        input: &str,
        vt: ValueType,
        num_arg: i32,
    ) -> Result<Vec<BytecodeExp>, String> {
        let mut bes: Vec<BytecodeExp> = Vec::new();
        for n in 1..=num_arg {
            let (input, be) = if n < num_arg {
                self.arg_expression(input, vt)?
            } else {
                self.full_expression(input, vt)?
            };
            bes.push(be);
            if self.token != "," {
                break;
            }
        }
        Ok(bes)
    }

    pub(crate) fn arg_expression(
        &mut self,
        input: &str,
        vt: ValueType,
    ) -> Result<(&str, BytecodeExp), String> {
        let (input, be) = self.typed_exp(Compiler::exp_bool_or, input, vt)?;
        if self.token.len() > 0 {
            if self.token != "," {
                return Err(format!("Invalid data: {:?}", self.token));
            }
            let old_data = input.clone();
            let (input, result) = self.tokenizer(input);
            if result == "" {
                self.token = "".to_string();
                return Ok((input, be));
            } else {
                return Ok((old_data, be));
            }
        }
        Ok((input, be))
    }

    fn full_expression(
        &mut self,
        input: &str,
        vt: ValueType,
    ) -> Result<(&str, BytecodeExp), String> {
        let (input, be) = self.typed_exp(Compiler::exp_bool_or, input, vt)?;
        if self.token.len() > 0 {
            return Err(format!("Invalid data: {:?}", self.token));
        }
        Ok((input, be))
    }

    fn exp_bool_or(
        &mut self,
        output: &mut BytecodeExp,
        input: &str,
    ) -> Result<(&str, BytecodeValue), (String, BytecodeValue)> {
        if self.block.is_some() {
            return self.exp_one_op(
                output,
                input,
                Compiler::exp_bool_xor,
                "||",
                BytecodeExp::blor,
                OC_blor,
            );
        }
        let (input, mut bv) = match self.exp_bool_xor(output, input) {
            Ok((input, bv)) => (input, bv),
            Err((e, _)) => {
                return Err((e, BytecodeValue::bv_none()));
            }
        };
        loop {
            let mut input = match self.operator(input) {
                Ok(string) => string,
                Err(e) => {
                    return Err((e, BytecodeValue::bv_none()));
                }
            };
            if self.token == "||" {
                (input, self.token) = self.tokenizer(input);
                let mut be = BytecodeExp::new();
                let (input, bv2) = match self.exp_bool_or(&mut be, input) {
                    Ok((input, bv2)) => (input, bv2),
                    Err((e, bv2)) => {
                        return Err((e, BytecodeValue::bv_none()));
                    }
                };
                if bv.is_none() || bv2.is_none() {
                    output.append_value(bv);
                    be.append_value(bv2);
                    if be.len() > (u8::MAX - 1) as usize {
                        output.append_i32_p(OC_jnz, (be.len() + 1) as i32);
                    } else {
                        output.append(&[OC_jnz8, (be.len() + 1) as u8]);
                    }
                    output.append(&[OC_pop]);
                    output.append(be.as_bytes().as_ref());
                    bv = BytecodeValue::bv_none();
                } else {
                    BytecodeExp::blor(&mut bv, bv2);
                }
            } else {
                break Ok((input, bv));
            }
        }
    }

    fn exp_bool_xor(
        &mut self,
        output: &mut BytecodeExp,
        input: &str,
    ) -> Result<(&str, BytecodeValue), (String, BytecodeValue)> {
        return self.exp_one_op(
            output,
            input,
            Compiler::exp_bool_and,
            "^^",
            BytecodeExp::blxor,
            OC_blxor,
        );
    }

    fn exp_eq_ne(
        &mut self,
        output: &mut BytecodeExp,
        input: &str,
    ) -> Result<(&str, BytecodeValue), (String, BytecodeValue)> {
        todo!()
    }

    fn exp_and(
        &mut self,
        output: &mut BytecodeExp,
        input: &str,
    ) -> Result<(&str, BytecodeValue), (String, BytecodeValue)> {
        return self.exp_one_op(
            output,
            input,
            Compiler::exp_eq_ne,
            "&",
            BytecodeExp::and,
            OC_and,
        );
    }

    fn exp_xor(
        &mut self,
        output: &mut BytecodeExp,
        input: &str,
    ) -> Result<(&str, BytecodeValue), (String, BytecodeValue)> {
        return self.exp_one_op(
            output,
            input,
            Compiler::exp_and,
            "^",
            BytecodeExp::xor,
            OC_xor,
        );
    }

    fn exp_or(
        &mut self,
        output: &mut BytecodeExp,
        input: &str,
    ) -> Result<(&str, BytecodeValue), (String, BytecodeValue)> {
        return self.exp_one_op(
            output,
            input,
            Compiler::exp_xor,
            "|",
            BytecodeExp::or,
            OC_or,
        );
    }

    fn exp_bool_and(
        &mut self,
        output: &mut BytecodeExp,
        input: &str,
    ) -> Result<(&str, BytecodeValue), (String, BytecodeValue)> {
        if self.block.is_some() {
            return self.exp_one_op(
                output,
                input,
                Compiler::exp_or,
                "&&",
                BytecodeExp::bland,
                OC_bland,
            );
        }
        let (mut input, mut bv) = match self.exp_or(output, input) {
            Ok((input, bv)) => (input, bv),
            Err((e, bv)) => return Err((e, BytecodeValue::bv_none())),
        };
        loop {
            input = match self.operator(input) {
                Ok(input) => input,
                Err(e) => return Err((e, BytecodeValue::bv_none())),
            };
            if self.token == "&&" {
                (input, self.token) = self.tokenizer(input);
                let mut be = BytecodeExp::new();
                let mut bv2 = BytecodeValue::new();
                (input, bv2) = match self.exp_bool_and(&mut be, input) {
                    Ok((input, bv2)) => (input, bv2),
                    Err((e, bv2)) => {
                        return Err((e, BytecodeValue::bv_none()));
                    }
                };
                if bv.is_none() || bv2.is_none() {
                    output.append_value(bv);
                    be.append_value(bv2);
                    if be.len() > u8::MAX as usize - 1 as usize {
                        output.append_i32_p(OC_jz, be.len() as i32 + 1 as i32);
                    } else {
                        output.append(&[OC_jz8, be.len() as u8 + 1 as u8]);
                    }
                    output.append(&[OC_pop]);
                    output.append(be.as_bytes().as_slice());
                    bv = BytecodeValue::bv_none();
                } else {
                    BytecodeExp::bland(&mut bv, bv2);
                }
            } else {
                break;
            }
        }
        Ok((input, bv))
    }

    fn exp_one_op<'a>(
        &mut self,
        output: &mut BytecodeExp,
        input: &'a str,
        ef: impl FnOnce(
            &mut Self,
            &mut BytecodeExp,
            &'a str,
        ) -> Result<(&'a str, BytecodeValue), (String, BytecodeValue)>,
        opt: &str,
        opf: impl FnOnce(&mut BytecodeValue, BytecodeValue),
        opc: OpCode,
    ) -> Result<(&'a str, BytecodeValue), (String, BytecodeValue)> {
        let (input, mut bv) = match ef(self, output, input) {
            Ok((input, bv)) => (input, bv),
            Err((e, bv)) => {
                return Err((e, bv));
            }
        };
        loop {
            let mut input = match self.operator(input) {
                Ok(input) => input,
                Err(e) => {
                    return Err((e, BytecodeValue::bv_none()));
                }
            };
            if self.token == opt {
                (input, self.token) = self.tokenizer(input);
                input = match self.exp_one_op_sub(output, input, &mut bv, ef, opf, opc) {
                    Ok(input) => input,
                    Err(e) => {
                        return Err((e, BytecodeValue::bv_none()));
                    }
                }
            } else {
                return Ok((input, bv));
            }
        }
    }

    fn exp_one_op_sub<'a>(
        &mut self,
        output: &mut BytecodeExp,
        input: &'a str,
        bv: &mut BytecodeValue,
        ef: impl FnOnce(
            &mut Self,
            &mut BytecodeExp,
            &'a str,
        ) -> Result<(&'a str, BytecodeValue), (String, BytecodeValue)>,
        opf: impl FnOnce(&mut BytecodeValue, BytecodeValue),
        opc: OpCode,
    ) -> Result<&'a str, String> {
        let mut be = BytecodeExp::new();
        let (mut input, mut bv2) = match ef(self, &mut be, input) {
            Ok((input, bv)) => (input, bv),
            Err((e, bv)) => {
                return Err(e);
            }
        };
        if bv.is_none() || bv2.is_none() {
            output.append_value(*bv);
            output.append(be.as_bytes().as_slice());
            output.append_value(bv2);
            output.append(&[opc]);
            *bv = BytecodeValue::bv_none();
        } else {
            opf(bv, bv2)
        }
        Ok(input)
    }

    fn typed_exp<'a>(
        &mut self,
        ef: impl FnOnce(
            &mut Self,
            &mut BytecodeExp,
            &'a str,
        ) -> Result<(&'a str, BytecodeValue), (String, BytecodeValue)>,
        input: &'a str,
        vt: ValueType,
    ) -> Result<(&'a str, BytecodeExp), String> {
        let (input, token) = self.tokenizer(input);
        let mut be: BytecodeExp = BytecodeExp::new();
        let (input, mut bv) = match ef(self, &mut be, input) {
            Ok((input, bv)) => (input, bv),
            Err((e, _)) => {
                return Err(e);
            }
        };
        if !bv.is_none() {
            match vt {
                VT_Float => {
                    bv.set_f(bv.to_f());
                }
                VT_Int => {
                    bv.set_i(bv.to_i());
                }
                VT_Bool => {
                    bv.set_b(bv.to_b());
                }
                _ => {}
            }
            be.append_value(bv);
        }
        Ok((input, be))
    }

    pub(crate) fn sc_add(
        &mut self,
        sc: &mut StateControllerBase,
        id: u8,
        input: &str,
        vt: ValueType,
        num_arg: i32,
        top_be: Option<Vec<BytecodeExp>>,
    ) -> Result<(), String> {
        let mut bes = self.exprs(input, vt, num_arg)?;
        let mut top_be = top_be.unwrap_or_default();
        top_be.append(&mut bes);
        sc.add(id, top_be);
        Ok(())
    }

    pub(crate) fn attr(text: &str, hitdef: bool) -> Result<i32, String> {
        let mut flg = 0 as i32;
        let attr = utils::split_and_trim(text, ",");
        for &a in attr.first() {
            match a {
                "S" | "s" => {
                    if hitdef {
                        flg = ST_S as i32;
                    } else {
                        flg |= ST_S as i32;
                    }
                }
                "C" | "c" => {
                    if hitdef {
                        flg = ST_C as i32;
                    } else {
                        flg != ST_C as i32;
                    }
                }
                "A" | "a" => {
                    if hitdef {
                        flg = ST_A as i32;
                    } else {
                        flg != ST_A as i32;
                    }
                }
                _ => {
                    return Err(format!("Invalid value: {:?}", a));
                }
            }
        }

        for &a in &attr[1..] {
            let l = a.len();
            match a.to_lowercase().as_str() {
                "na" => {
                    flg |= AT_NA;
                }
                "nt" => {
                    flg |= AT_NT;
                }
                "np" => {
                    flg |= AT_NP;
                }
                "sa" => {
                    flg |= AT_SA;
                }
                "st" => {
                    flg |= AT_ST;
                }
                "sp" => {
                    flg |= AT_SP;
                }
                "ha" => {
                    flg |= AT_HA;
                }
                "ht" => {
                    flg |= AT_HT;
                }
                "hp" => {
                    flg |= AT_HP;
                }
                "aa" => {
                    flg |= AT_AA;
                }
                "at" => {
                    flg |= AT_AT;
                }
                "ap" => {
                    flg |= AT_AP;
                }
                "n" => {
                    flg |= AT_NA | AT_NT | AT_NP;
                }
                "s" => {
                    flg |= AT_SA | AT_ST | AT_SP;
                }
                "h" | "a" => {
                    flg |= AT_HA | AT_HT | AT_HP;
                }
                _ => {
                    return Err(format!("Invalid value: {:?}", a));
                }
            }

            if l > 2 {
                break;
            }
        }
        Ok(flg)
    }
}
