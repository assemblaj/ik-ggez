use crate::bytecode::*;
use crate::compiler::*;
use crate::ini::*;

// State controller definition file.
// This file contains the parsing code for the function in ZSS and CNS, also called State Controllers.

impl Compiler {
    fn hitBytSub(&mut self, is: &mut IniSection, sc: &StateControllerBase) -> Result<(), String> {
        let mut attr = -1;
        let mut two = false;

        Compiler::state_param(is, "value", |data| -> Result<(), String> {
            attr = Compiler::attr(data, false)?;
            Ok(())
        })?;

        if attr == -1 {
            Compiler::state_param(is, "value2", |data| -> Result<(), String> {
                two = true;
                attr = Compiler::attr(data, false)?;
                Ok(())
            })?;
        }

        if attr == -1 {
            return Err("value parameter not specified".to_string());
        }

        
        Ok(())
    }
}
