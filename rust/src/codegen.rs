/*
***********************************************************************
  CODEGEN.C : IMPLEMENT CODE GENERATION HERE
************************************************************************
*/
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
extern crate libc;
use crate::expression::*;
use std::fs::File;
use std::io::prelude::*;

pub const INVAL: i64 = -999;

/*
*************************************************************************************
 USE A STRUCTURE TO STORE GLOBAL VARIABLES
*************************************************************************************
*/
struct globals {
    // The last used offset will help determine the last used stack location
    pub last_used_offset: i64,
    pub last_offset_used: String,
    // The arg counter is used to iterate through the arg list.
    pub arg_counter: i64,
}

impl globals {
    fn new() -> Self {
        globals {
            last_used_offset: 0,
            last_offset_used: "".to_string(),
            arg_counter: 0,
        }
    }
}
/*
*************************************************************************************
     THE REGINFO LIST TRACKS IF REGISTERS ARE AVAILABLE FOR USE
**************************************************************************************
*/

#[derive(Clone)]
struct regInfo {
    name: String,
    avail: i8,
    next: LinkInfo,
}

type LinkInfo = Option<Box<regInfo>>;

impl Default for regInfo {
    fn default() -> regInfo {
        regInfo {
            name: "".to_string(),
            avail: 0,
            next: None,
        }
    }
}

struct regList {
    pub head: LinkInfo,
}

impl regList {
    fn new() -> Self {
        regList { head: None }
    }

    /*
    ***********************************************************************
      FUNCTION TO ADD NEW REGISTER INFORMATION TO THE REGISTER INFO LIST
    ************************************************************************
    */
    fn add_reg(&mut self, name: &str, avail: i8) {
        let new_node = Box::new(regInfo {
            name: name.to_string(),
            avail: avail,
            next: None,
        });

        if !self.head.is_none() {
            let mut current = &mut self.head;
            loop {
                if let Some(node) = current.as_mut() {
                    if node.next.is_none() {
                        node.next = Some(new_node);
                        break;
                    }
                    current = &mut node.next;
                } else {
                    break;
                }
            }
        } else {
            self.head = Some(new_node);
        }
    }

    /*
    ***********************************************************************
      FUNCTION TO UPDATE THE AVAILIBILITY OF REGISTERS IN THE REG INFO LIST
    ************************************************************************
    */
    fn update_reg_info(&mut self, name: String, avail: i8) {
        let mut current = &mut self.head;
        loop {
            if let Some(node) = current.as_mut() {
                if node.name == name {
                    node.avail = avail;
                }
                current = &mut node.next;
            } else {
                break;
            }
        }
    }

    /*
    ***********************************************************************
      FUNCTION TO RETURN THE NEXT AVAILABLE REGISTER
    ************************************************************************
    */
    fn get_next_avail_reg(&self, noAcc: bool) -> String {
        let mut current = &self.head;

        if current.is_none() {
            println!("List is empty");
        }

        loop {
            if let Some(node) = current.as_ref() {
                if node.avail == 1 {
                    if !noAcc {
                        return node.name.clone();
                    }
                    // if not rax and dont return accumulator set to true, return the other reg
                    // if rax and noAcc == true, skip to next avail
                    if noAcc && !(node.name == "%rax") {
                        return node.name.clone();
                    }
                }
                current = &node.next;
            } else {
                break;
            }
        }
        return "NoReg".to_string();
    }

    /*
    ***********************************************************************
      FUNCTION TO DETERMINE IF ANY REGISTER APART FROM OR INCLUDING
      THE ACCUMULATOR(RAX) IS AVAILABLE
    ************************************************************************
    */
    fn if_avail_reg(&self, noAcc: bool) -> usize {
        let mut current = &self.head;

        if current.is_none() {
            println!("Empty reglist");
        }

        loop {
            if let Some(node) = current.as_ref() {
                if node.avail == 1 {
                    // registers available
                    if !noAcc {
                        return 1;
                    } else if noAcc && !(node.name == "%rax") {
                        return 1;
                    }
                }
                current = &node.next;
            } else {
                break;
            }
        }
        return 0;
    }

    /*
    ***********************************************************************
      FUNCTION TO DETERMINE IF A SPECIFIC REGISTER IS AVAILABLE
    ************************************************************************
    */
    fn is_avail_reg(&self, name: String) -> bool {
        let mut current = &self.head;

        if current.is_none() {
            println!("Empty reglist");
        }

        loop {
            if let Some(node) = current.as_ref() {
                if node.name == name {
                    if node.avail == 1 {
                        return true;
                    }
                }
                current = &node.next;
            } else {
                break;
            }
        }
        return false;
    }

    /*
    ***********************************************************************
      FUNCTION TO FREE REGISTER INFORMATION LIST
    ************************************************************************
    */
    fn free_list(&mut self) {
        while !self.head.is_none() {
            self.head.take().map(|node| {
                self.head = node.next;
            });
        }
    }

    fn print_list(&self) {
        let mut current = &self.head;

        if current.is_none() {
            println!("Empty reglist");
        }

        loop {
            if let Some(node) = current.as_ref() {
                print!("\t {} : {} -> ", node.name, node.avail);

                current = &node.next;
            } else {
                break;
            }
        }
        println!();
    }
}

/*
*************************************************************************************
     THE VARSTOREINFO LIST TRACKS A VARIABLE NAME, VALUE AND WHERE IT IS STORED
**************************************************************************************
*/

#[derive(Clone)]
struct varStoreInfo {
    name: String,
    // FLAG TO IDENTIFY IF A VARIABLE IS A CONSTANT OR NOT.
    is_const: bool,
    value: i64,
    // LOCATION COULD BE A STACK LOCATION OR A REGISTER
    // eg: -8(%rbp) or %rcx
    location: String,
    next: LinkVar,
}

type LinkVar = Option<Box<varStoreInfo>>;

impl Default for varStoreInfo {
    fn default() -> varStoreInfo {
        varStoreInfo {
            name: "".to_string(),
            is_const: false,
            value: 0,
            location: "".to_string(),
            next: None,
        }
    }
}

struct varStList {
    head: LinkVar,
}

impl varStList {
    fn new() -> Self {
        varStList { head: None }
    }

    /*
    ***********************************************************************
      FUNCTION TO ADD VARIABLE INFORMATION TO THE VARIABLE INFO LIST
    ************************************************************************
    */
    fn add_var_info(&mut self, varname: String, location: String, val: i64, is_const: bool) {
        // push front like a stack
        let new_node = Box::new(varStoreInfo {
            name: varname,
            is_const: is_const,
            value: val,
            location: location.clone(),
            next: self.head.take(),
        });

        self.head = Some(new_node);
    }

    /*
    ***********************************************************************
      FUNCTION TO LOOKUP VARIABLE INFORMATION FROM THE VARINFO LIST
    ************************************************************************
    */
    fn lookup_var_info(&self, name: String, val: i64) -> String {
        let mut current = &self.head;

        if current.is_none() {
            println!("Empty varlist");
        }

        loop {
            if let Some(node) = current.as_ref() {
                if node.is_const == true {
                    if node.value == val {
                        return node.location.clone();
                    }
                } else {
                    if node.name == name {
                        return node.location.clone();
                    }
                }
                current = &node.next;
            } else {
                break;
            }
        }
        return "".to_string();
    }

    /*
    ***********************************************************************
      FUNCTION TO UPDATE VARIABLE INFORMATION
    ************************************************************************
    */
    fn update_var_info(&mut self, varName: String, location: String, val: i64, is_const: bool) {
        if self.lookup_var_info(varName.clone(), val) == "".to_string() {
            self.add_var_info(varName.clone(), location, val, is_const);
        } else {
            let mut current = &mut self.head;

            if current.is_none() {
                println!("Empty varlist");
            }

            loop {
                if let Some(node) = current.as_mut() {
                    if node.name == varName {
                        node.value = val;
                        node.location = location;
                        node.is_const = is_const;
                        break;
                    }
                    current = &mut node.next;
                } else {
                    break;
                }
            }
        }
    }

    /*
    ***********************************************************************
      FUNCTION TO FREE THE VARIABLE INFORMATION LIST
    ************************************************************************
    */
    fn free_list(&mut self) {
        while !self.head.is_none() {
            self.head.take().map(|node| {
                self.head = node.next;
            });
        }
    }

    fn print_list(&self) {
        let mut current = &self.head;

        if current.is_none() {
            println!("Empty varlist");
        }

        loop {
            if let Some(node) = current.as_ref() {
                if !node.is_const {
                    print!("\t {} : {} -> ", node.name, node.location);
                } else {
                    print!("\t {} : {} -> ", node.value, node.location);
                }

                current = &node.next;
            } else {
                break;
            }
        }
        println!();
    }
}

/*
*************************************************************************************
   YOUR CODE IS TO BE FILLED IN THE GIVEN TODO BLANKS. YOU CAN CHOOSE TO USE ALL
   UTILITY FUNCTIONS OR NONE. YOU CAN ALSO ADD NEW FUNCTIONS
**************************************************************************************
*/
#[no_mangle]
fn init_asm(fileptr: &mut File, funcName: String) {
    fileptr
        .write_all(format!("\n.globl {}", funcName).as_bytes())
        .expect("Unable to write data");
    fileptr
        .write_all(format!("\n{}:", funcName).as_bytes())
        .expect("Unable to write data");

    // Initialize the stack and base pointer
    fileptr
        .write_all("\npushq %rbp".as_bytes())
        .expect("Unable to write data");
    fileptr
        .write_all("\nmovq %rsp, %rbp".as_bytes())
        .expect("Unable to write data");
}

/*
***************************************************************************
   FUNCTION TO WRITE THE RETURNING CODE OF A FUNCTION IN THE ASSEMBLY FILE
****************************************************************************
*/
#[no_mangle]
fn ret_asm(fileptr: &mut File) {
    fileptr
        .write_all("\npopq %rbp".as_bytes())
        .expect("Unable to write data");
    fileptr
        .write_all("\nretq\n".as_bytes())
        .expect("Unable to write data");
}

/*
***************************************************************************
  FUNCTION TO CONVERT OFFSET FROM LONG TO CHAR STRING
****************************************************************************
*/
#[no_mangle]
fn long_to_char_offset(glb: &mut globals) {
    glb.last_used_offset -= 8;

    glb.last_offset_used = format!("{}", glb.last_used_offset);

    // ensure no more than 100 characters are used
    if glb.last_offset_used.len() > 100 {
        glb.last_offset_used.truncate(100);
    }

    glb.last_offset_used.push_str("(%rbp)");
}

/*
***************************************************************************
  FUNCTION TO SAVE VALUE IN ACCUMULATOR (RAX)
****************************************************************************
*/
#[no_mangle]
fn save_val_rax(
    fileptr: &mut File,
    name: String,
    glb: &mut globals,
    var_list: &mut varStList,
    reg_list: &mut regList,
) {
    let temp_reg = reg_list.get_next_avail_reg(true);

    if temp_reg == "NoReg" {
        long_to_char_offset(glb);

        fileptr
            .write_all(format!("\n movq %rax, {}", glb.last_offset_used).as_bytes())
            .expect("Unable to write data");

        var_list.update_var_info(name, glb.last_offset_used.clone(), INVAL, false);
        reg_list.update_reg_info("%rax".to_string(), 1);
    } else {
        fileptr
            .write_all(format!("\nmovq %rax, {}", temp_reg).as_bytes())
            .expect("Unable to write data");

        reg_list.update_reg_info(temp_reg.clone(), 0);
        var_list.update_var_info(name, temp_reg, INVAL, false);
        reg_list.update_reg_info("%rax".to_string(), 1);
    }
}
#[no_mangle]
fn create_reg_list(reg_list: &mut regList) {
    // Create the initial reglist which can be used to store variables.
    // 4 general purpose registers : AX, BX, CX, DX
    // 4 special purpose : SP, BP, SI , DI.
    // Other registers: r8, r9
    // You need to decide which registers you will add in the register list
    // use. Can you use all of the above registers?
    /*
     ****************************************
              TODO : YOUR CODE HERE
     ***************************************
    */
    reg_list.add_reg("%rax",1);
    reg_list.add_reg("%rbx",1);
    reg_list.add_reg("%r9",1);
    reg_list.add_reg("%r8",1);
    reg_list.add_reg("%rcx",1);
    reg_list.add_reg("%rdx",1);
    reg_list.add_reg("%rsi",1);
    reg_list.add_reg("%rdi",1);
}

/*
***********************************************************************
  THIS FUNCTION IS MEANT TO PUT THE FUNCTION ARGUMENTS ON STACK
************************************************************************
*/
#[no_mangle]
fn push_arg_on_stack(
    fileptr: &mut File,
    arguments: &RList,
    glb: &mut globals,
    var_list: &mut varStList,
    reg_list: &mut regList,
) {
    /*
     ****************************************
              TODO : YOUR CODE HERE
     ****************************************
    */

    let mut args = arguments;
    let mut counter: i64 = 0;
    let mut src: String = "%rdi".to_string();
    loop {
        /*
        ***********************************************************************
                 TODO : YOUR CODE HERE
         THINK ABOUT WHERE EACH ARGUMENT COMES FROM. EXAMPLE WHERE IS THE
         FIRST ARGUMENT OF A FUNCTION STORED.
        ************************************************************************
        */
        if let Some(node) = args.node.as_ref() {
            counter += 1;
            if counter == 2 {
                src = "%rsi".to_string();
            }
            else if counter == 3 {
                src = "%rdx".to_string();
            }
            else if counter == 4 {
                src = "%rcx".to_string();
            }
            else if counter == 5 {
                src = "%r8".to_string();
            }
            else if counter == 6 {
                src = "%r9".to_string();
            }
            else {
                // nothing happens
            }
            long_to_char_offset(glb);
            reg_list.add_reg(&glb.last_offset_used.clone(), 0);
            var_list.add_var_info(node.name.clone(), glb.last_offset_used.clone(), INVAL, false);
            fileptr
                .write_all(format!("\nmovq {}, {}", src.clone(), glb.last_offset_used).as_bytes())
                .expect("Unable to write data");
        } else {
            break;
        }

        if let Some(next) = args.next.as_ref() {
            args = next;
        } else {
            break;
        }
    }
}

/*
*************************************************************************
  THIS FUNCTION IS MEANT TO GET THE FUNCTION ARGUMENTS FROM THE  STACK
**************************************************************************
*/
#[no_mangle]
fn pop_arg_from_stack(
    fileptr: &mut File,
    arguments: &RList,
    glb: &mut globals,
    var_list: &mut varStList,
    reg_list: &mut regList,
) {
    let mut args = arguments;
    /*
     ****************************************
              TODO : YOUR CODE HERE
     ****************************************
    */

    loop {
        /*
        ***********************************************************************
                 TODO : YOUR CODE HERE
         THINK ABOUT WHERE EACH ARGUMENT COMES FROM. EXAMPLE WHERE IS THE
         FIRST ARGUMENT OF A FUNCTION STORED AND WHERE SHOULD IT BE EXTRACTED
         FROM AND STORED TO..
        ************************************************************************
        */
        if let Some(node) = args.node.as_ref() {
            // unimplemented!();
        } else {
            break;
        }

        if let Some(next) = args.next.as_ref() {
            args = next;
        } else {
            break;
        }
    }
}
/*
***************************************************************************
  FUNCTION TO CONVERT CONSTANT VALUE TO CHAR STRING
****************************************************************************
*/
#[no_mangle]
fn process_constant(
    fileptr: &mut File,
    op_node: &RNode,
    glb: &mut globals,
    var_list: &mut varStList,
) {
    long_to_char_offset(glb);

    let mut value = format!("{}", op_node.value);
    if value.len() > 10 {
        value.truncate(10);
    }

    let mut offset = format!("{}", glb.last_used_offset);
    if offset.len() > 100 {
        offset.truncate(100);
    }
    offset.push_str("(%rbp)");

    var_list.add_var_info("".to_string(), offset.clone(), op_node.value, true);

    fileptr
        .write_all(format!("\nmovq ${}, {}", value, offset).as_bytes())
        .expect("Unable to write data");
}

/*
***********************************************************************
 THIS FUNCTION IS MEANT TO PROCESS EACH CODE STATEMENT AND GENERATE
 ASSEMBLY FOR IT.
 TIP: YOU CAN MODULARIZE BETTER AND ADD NEW SMALLER FUNCTIONS IF YOU
 WANT THAT CAN BE CALLED FROM HERE.
************************************************************************
*/
#[no_mangle]
fn process_statements(
    fileptr: &mut File,
    statements: &RList,
    glb: &mut globals,
    var_list: &mut varStList,
    reg_list: &mut regList,
) {
    let mut stmt = statements;
    /*
     ****************************************
              TODO : YOUR CODE HERE
     ****************************************
    */

    loop {
        if let Some(node) = stmt.node.as_ref() {
            /*
             ****************************************
                      TODO : YOUR CODE HERE
             ****************************************
            */
            // if ASSIGN
            if node.stmtCode == StmtType::ASSIGN {
                if let Some(rightNode) = node.right.as_ref() {
                    // if assigning variable to CONSTANT
                    if rightNode.exprCode == ExprType::CONSTANT {
                        let val = rightNode.value;
                        long_to_char_offset(glb);
                        reg_list.add_reg(&glb.last_offset_used.clone(), 0);
                        var_list.add_var_info(node.name.clone(), glb.last_offset_used.clone(), val, true);
                        fileptr
                            .write_all(format!("\nmovq ${}, {}", val, glb.last_offset_used).as_bytes())
                            .expect("Unable to write data");
                    }

                    // if assigning variable to VARIABLE OR PARAMETER
                    else if rightNode.exprCode == ExprType::VARIABLE || rightNode.exprCode == ExprType::PARAMETER {
                        let addy: String = var_list.lookup_var_info(rightNode.name.clone(), INVAL);
                        long_to_char_offset(glb);
                        reg_list.add_reg(&glb.last_offset_used.clone(), 0);
                        var_list.add_var_info(node.name.clone(), glb.last_offset_used.clone(), INVAL, false);
                        fileptr
                            .write_all(format!("\nmovq {}, {}", addy, glb.last_offset_used).as_bytes())
                            .expect("Unable to write data");
                    }

                    // if OPERATION
                    else if rightNode.exprCode == ExprType::OPERATION {
                        // if FUNCTIONCALL
                        if rightNode.opCode == OpType::FUNCTIONCALL {
                            let mut new_arg_counter: i64 = 0;
                            if let Some(mut args) = rightNode.arguments.as_ref() {
                                loop {
                                    if let Some(arg) = args.node.as_ref() {
                                        let mut dest: String = "%rdi".to_string();
                                        new_arg_counter += 1;
                                        if new_arg_counter == 2 {
                                            dest = "%rsi".to_string();
                                        }
                                        else if new_arg_counter == 3 {
                                            dest = "%rdx".to_string();
                                        }
                                        else if new_arg_counter == 4 {
                                            dest = "%rcx".to_string();
                                        }
                                        else if new_arg_counter == 5 {
                                            dest = "%r8".to_string();
                                        }
                                        else if new_arg_counter == 6 {
                                            dest = "%r9".to_string();
                                        }
                                        else {
                                            // this shouldn't happen
                                        }

                                        reg_list.update_reg_info(dest.clone(), 0);

                                        if arg.exprCode == ExprType::CONSTANT {
                                            let val = arg.value;
                                            fileptr
                                                .write_all(format!("\nmovq ${}, {}", val, dest.clone()).as_bytes())
                                                .expect("Unable to write data");
                                        }
                                        else if arg.exprCode == ExprType::VARIABLE || arg.exprCode == ExprType::PARAMETER {
                                            let addy: String = var_list.lookup_var_info(arg.name.clone(), INVAL);
                                            fileptr
                                                .write_all(format!("\nmovq {}, {}", addy, dest.clone()).as_bytes())
                                                .expect("Unable to write data");
                                        }
                                        else {
                                            // this shouldn't happen
                                        }
                                    }
                                    else {
                                        break;
                                    }

                                    if let Some(next_arg) = args.next.as_ref() {
                                        args = next_arg;
                                    }
                                    else {
                                        break;
                                    }
                                }
                                // call the function
                                if let Some(funcDecl) = rightNode.left.as_ref() {
                                    fileptr
                                        .write_all(format!("\ncall {}", funcDecl.name.clone()).as_bytes())
                                        .expect("Unable to write data");
                                }
                                else {
                                    // this shouldn't happen
                                }
                            }
                        }

                        // if not FUNCTION CALL
                        else if let Some(leftOperand) = rightNode.left.as_ref() {
                            if leftOperand.exprCode == ExprType::CONSTANT {
                                let val = leftOperand.value;
                                reg_list.update_reg_info("%rax".to_string(),0);
                                fileptr
                                    .write_all(format!("\nmovq ${}, %rax", val).as_bytes())
                                    .expect("Unable to write data");
                            }
                            else if leftOperand.exprCode == ExprType::VARIABLE || leftOperand.exprCode == ExprType::PARAMETER {
                                let addy: String = var_list.lookup_var_info(leftOperand.name.clone(), INVAL);
                                reg_list.update_reg_info("%rax".to_string(),0);
                                fileptr
                                    .write_all(format!("\nmovq {}, %rax", addy).as_bytes())
                                    .expect("Unable to write data");
                            }
                            else {
                                // this shouldn't happen
                            }

                            if let Some(rightOperand) = rightNode.right.as_ref() {
                                if rightOperand.exprCode == ExprType::CONSTANT {
                                    let val = rightOperand.value;
                                    reg_list.update_reg_info("%rcx".to_string(), 0);
                                    fileptr
                                        .write_all(format!("\nmovq ${}, %rcx", val).as_bytes())
                                        .expect("Unable to write data");
                                }
                                else if rightOperand.exprCode == ExprType::VARIABLE || rightOperand.exprCode == ExprType::PARAMETER {
                                    let addy: String = var_list.lookup_var_info(rightOperand.name.clone(), INVAL);
                                    reg_list.update_reg_info("%rax".to_string(),0);
                                    fileptr
                                        .write_all(format!("\nmovq {}, %rcx", addy).as_bytes())
                                        .expect("Unable to write data");
                                }
                                else {
                                    // this shouldn't happen
                                }
                                
                                // binary operation
                                if rightNode.opCode == OpType::MULTIPLY {
                                    fileptr
                                        .write_all("\nimulq %rcx, %rax".as_bytes())
                                        .expect("Unable to write data");
                                }
                                else if rightNode.opCode == OpType::DIVIDE {
                                    fileptr
                                        .write_all("\ncqto".as_bytes())
                                        .expect("Unable to write data");
                                    fileptr
                                        .write_all("\nidivq %rcx".as_bytes())
                                        .expect("Unable to write data");
                                }
                                else if rightNode.opCode == OpType::ADD {
                                    fileptr
                                        .write_all("\naddq %rcx, %rax".as_bytes())
                                        .expect("Unable to write data");
                                }
                                else if rightNode.opCode == OpType::SUBTRACT {
                                    fileptr
                                        .write_all("\nsubq %rcx, %rax".as_bytes())
                                        .expect("Unable to write data");
                                }
                                else if rightNode.opCode == OpType::BOR {
                                    fileptr
                                        .write_all("\norq %rcx, %rax".as_bytes())
                                        .expect("Unable to write data");
                                }
                                else if rightNode.opCode == OpType::BAND {
                                    fileptr
                                        .write_all("\nandq %rcx, %rax".as_bytes())
                                        .expect("Unable to write data");
                                }
                                else if rightNode.opCode == OpType::BXOR {
                                    fileptr
                                        .write_all("\nxorq %rcx, %rax".as_bytes())
                                        .expect("Unable to write data");
                                }
                                else if rightNode.opCode == OpType::BSHR {
                                    fileptr
                                        .write_all("\nsarq %cl, %rax".as_bytes())
                                        .expect("Unable to write data");
                                }
                                else if rightNode.opCode == OpType::BSHL {
                                    fileptr
                                        .write_all("\nsalq %cl, %rax".as_bytes())
                                        .expect("Unable to write data");
                                }
                                else {
                                    // this shouldn't happen
                                }
                            }

                            // unary operation
                            if rightNode.opCode == OpType::NEGATE {
                                fileptr
                                    .write_all("\nnegq %rax".as_bytes())
                                    .expect("Unable to write data");
                            }
                        }

                        // save val rax
                        long_to_char_offset(glb);
                        reg_list.add_reg(&glb.last_offset_used.clone(), 0);
                        var_list.add_var_info(node.name.clone(), glb.last_offset_used.clone(), INVAL, false);
                        fileptr
                            .write_all(format!("\nmovq %rax, {}",glb.last_offset_used).as_bytes())
                            .expect("Unable to write data");
                    }
                    
                    else {
                        // this shouldn't happen
                    }
                }
                else {
                    // this shouldn't happen
                }
            }
            else if node.stmtCode == StmtType::RETURN {
                if let Some(leftNode) = node.left.as_ref() {
                    if leftNode.exprCode == ExprType::CONSTANT {
                        let val = leftNode.value;
                        fileptr
                            .write_all(format!("\nmovq ${}, %rax", val).as_bytes())
                            .expect("Unable to write data");
                    }

                    else if leftNode.exprCode == ExprType::VARIABLE || leftNode.exprCode == ExprType::PARAMETER {
                        let addy: String = var_list.lookup_var_info(leftNode.name.clone(), INVAL);
                        fileptr
                            .write_all(format!("\nmovq {}, %rax", addy).as_bytes())
                            .expect("Unable to write data");
                    }

                    else {
                        // this shouldn't happen
                    }
                }
            }
        } else {
            break;
        }
        if let Some(next) = stmt.next.as_ref() {
            stmt = next;
        } else {
            break;
        }
    }
}

/*
 ***********************************************************************
  THIS FUNCTION IS MEANT TO DO CODEGEN FOR ALL THE FUNCTIONS IN THE FILE
 ************************************************************************
*/
#[no_mangle]
pub fn Codegen(mut worklist: &mut RList) {
    /*
     ****************************************
              TODO : YOUR CODE HERE
     ****************************************
    */

    let mut fileptr = File::create("assembly.s").expect("Unable to create assembly file");

    // create a register list, set everything to available
    let mut rlist = regList::new();
    create_reg_list(&mut rlist);

    // create an empty variable list
    let mut vlist = varStList::new();
    loop {
        /*
        ****************************************
                 TODO : YOUR CODE HERE
        ****************************************
        */
        // create a globals struct
        let mut glbls = globals::new();
       
        if let Some(mut node) = worklist.node.as_mut() {

            init_asm(&mut fileptr, node.name.clone());
            // subtract from rsp
            let required_space: i64 = calculate_required_space(node);
            // decrement stack pointer, making space for arguments and local variables
            fileptr
                .write_all(format!("\nsubq ${}, %rsp", required_space).as_bytes())
                .expect("Unable to write data");


            // push arguments onto stack
            if let Some(mut arguments) = node.arguments.as_mut() {
                push_arg_on_stack(&mut fileptr, arguments, &mut glbls, &mut vlist, &mut rlist);
            }

            if let Some(mut statements) = node.statements.as_mut() {
                process_statements(&mut fileptr, statements, &mut glbls, &mut vlist, &mut rlist);
            }

            fileptr
                .write_all(format!("\naddq ${}, %rsp", required_space).as_bytes())
                .expect("Unable to write data");
            
            ret_asm(&mut fileptr);

        } else {
            break;
        }

        if let Some(next) = worklist.next.as_mut() {
            worklist = next;
        } else {
            break;
        }
    }
}

/*
**********************************************************************************************************************************
 YOU CAN MAKE ADD AUXILLIARY FUNCTIONS BELOW THIS LINE. DO NOT FORGET TO DECLARE THEM IN THE HEADER
**********************************************************************************************************************************
*/
fn calculate_required_space(mut funcNode: &mut RNode) -> i64 {
    // find number of arguments
    let mut argument_counter: i64 = 0;
    if let Some(mut args) = funcNode.arguments.as_mut() {
        loop {
            if let Some(arg) = args.node.as_mut() {
                argument_counter += 1;
            }
            else {
                break;
            }

            if let Some(next_arg) = args.next.as_mut() {
                args = next_arg;
            }
            else {
                break;
            }
        }
    }

    // find number of local variables
    let mut assign_counter: i64 = 0;
    if let Some(mut stmts) = funcNode.statements.as_mut() {
        loop {
            if let Some(stmt) = stmts.node.as_mut() {
                if stmt.stmtCode == StmtType::ASSIGN {
                    assign_counter += 1;
                }
            }
            else {
                break;
            }

            if let Some(next_stmt) = stmts.next.as_mut() {
                stmts = next_stmt;
            }
            else {
                break;
            }
        }
    }

    return (assign_counter + argument_counter) * 8;
}
/*
**********************************************************************************************************************************
 YOU CAN MAKE ADD AUXILLIARY FUNCTIONS ABOVE THIS LINE. DO NOT FORGET TO DECLARE THEM IN THE HEADER
**********************************************************************************************************************************
*/
