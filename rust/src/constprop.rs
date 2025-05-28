/*
********************************************************************************
  CONSTPROP.RS : IMPLEMENT THE DOWNSTREAM CONSTANT PROPOGATION OPTIMIZATION HERE
*********************************************************************************
*/
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
extern crate libc;
use crate::expression::*;
use lazy_static::lazy_static;
use std::sync::atomic::{AtomicBool, Ordering};

/*
*************************************************************************************
   YOUR CODE IS TO BE FILLED IN THE GIVEN TODO BLANKS. YOU CAN CHOOSE TO USE ALL
   UTILITY FUNCTIONS OR NONE. YOU CAN ADD NEW FUNCTIONS. BUT DO NOT FORGET TO
   DECLARE THEM IN THE HEADER FILE.
**************************************************************************************
*/
#[derive(Clone)]
struct refConst {
    pub name: String,
    pub value: i64,
    pub next: LinkConst,
}

type LinkConst = Option<Box<refConst>>;

impl Default for refConst {
    fn default() -> refConst {
        refConst {
            name: "".to_string(),
            value: 0,
            next: None,
        }
    }
}

struct constList {
    pub head: LinkConst,
}

impl constList {
    fn new() -> Self {
        constList { head: None }
    }

    fn free_list(&mut self) {
        while !self.head.is_none() {
            self.head.take().map(|node| {
                self.head = node.next;
            });
        }
    }

    /*
    *************************************************************************
      FUNCTION TO ADD A CONSTANT VALUE AND THE ASSOCIATED VARIABLE TO THE LIST
    **************************************************************************
    */
    fn update_list(&mut self, name: String, value: i64) {
        let new_last = Box::new(refConst {
            name: name.clone(),
            value,
            next: None,
        });

        if !self.head.is_none() {
            let mut current = &mut self.head;
            loop {
                if let Some(node) = current.as_mut() {
                    if node.next.is_none() {
                        node.next = Some(new_last);
                        break;
                    }
                    current = &mut node.next;
                } else {
                    break;
                }
            }
        } else {
            self.head = Some(new_last);
        }
    }
    /*
    ****************************************************************************
      FUNCTION TO PRINT OUT THE LIST TO SEE ALL CONSTANTS THAT ARE USED/REFERRED
      AFTER THEIR ASSIGNMENT. YOU CAN USE THIS FOR DEBUGGING PURPOSES OR TO CHECK
      IF YOUR LIST IS GETTING UPDATED CORRECTLY
    ******************************************************************************
    */
    fn print_list(&self) {
        let mut current = &self.head;

        if current.is_none() {
            println!("List is empty");
        }

        loop {
            if let Some(node) = current.as_ref() {
                print!("{}:{} -> ", (node.name), node.value);

                current = &node.next;
            } else {
                break;
            }
        }
        println!();
    }

    /*
    *****************************************************************************
      FUNCTION TO LOOKUP IF A CONSTANT ASSOCIATED VARIABLE IS ALREADY IN THE LIST
    ******************************************************************************
    */
    fn search_list(&self, name: String) -> Option<i64> {
        let mut current = &self.head;
        loop {
            if let Some(node) = current.as_ref() {
                if node.name == name {
                    return Some(node.value);
                }

                current = &node.next;
            } else {
                break;
            }
        }
        return None;
    }
}

lazy_static! {
    static ref madeChange: AtomicBool = AtomicBool::new(false);
}

/*
**********************************************************************************************************************************
 YOU CAN MAKE CHANGES/ADD AUXILLIARY FUNCTIONS BELOW THIS LINE
**********************************************************************************************************************************
*/

/*
************************************************************************************
  THIS FUNCTION IS MEANT TO UPDATE THE CONSTANT LIST WITH THE ASSOCIATED VARIABLE
  AND CONSTANT VALUE WHEN ONE IS SEEN. IT SHOULD ALSO PROPOGATE THE CONSTANTS WHEN
  WHEN APPLICABLE. YOU CAN ADD A NEW FUNCTION IF YOU WISH TO MODULARIZE BETTER.
*************************************************************************************
*/
#[no_mangle]
fn TrackConst(mut statements: &mut RList, clist: &mut constList) {
    loop {
        if let Some(node) = statements.node.as_mut() {
            /*
            ****************************************
                    TODO : YOUR CODE HERE
            ****************************************
            */
            if node.stmtCode == StmtType::ASSIGN {
                if let Some(right_node) = node.right.as_mut() {
                    if right_node.type_ == NodeType::EXPRESSION {
                        if right_node.exprCode == ExprType::CONSTANT {
                            clist.update_list(node.name.clone(), right_node.value);
                        }
                        else if right_node.exprCode == ExprType::VARIABLE || right_node.exprCode == ExprType::PARAMETER {
                            if let Some(val) = clist.search_list(right_node.name.clone()).as_mut() {
                                right_node.name = "".to_string();
                                right_node.value = *val;

                                right_node.type_ = NodeType::EXPRESSION;
                                right_node.exprCode = ExprType::CONSTANT;
                                right_node.opCode = OpType::O_NONE;
                                right_node.stmtCode = StmtType::S_NONE;

                                right_node.left = None;
                                right_node.right = None;
                                right_node.arguments = None;
                                right_node.statements = None;

                                madeChange.store(true, Ordering::Relaxed);
                            }
                        }
                        else if right_node.exprCode == ExprType::OPERATION {
                            if right_node.opCode == OpType::NEGATE {
                                if let Some(inner_exp) = right_node.left.as_mut() {
                                    if let Some(val) = clist.search_list(inner_exp.name.clone()).as_mut() {
                                        inner_exp.name = "".to_string();
                                        inner_exp.value = *val;

                                        inner_exp.type_ = NodeType::EXPRESSION;
                                        inner_exp.exprCode = ExprType::CONSTANT;
                                        inner_exp.opCode = OpType::O_NONE;
                                        inner_exp.stmtCode = StmtType::S_NONE;

                                        inner_exp.left = None;
                                        inner_exp.right = None;
                                        inner_exp.arguments = None;
                                        inner_exp.statements = None;

                                        madeChange.store(true, Ordering::Relaxed);
                                    }
                                }
                            }
                            
                            else if right_node.opCode == OpType::FUNCTIONCALL {
                                if let Some(mut args) = right_node.arguments.as_mut() {
                                    'arg_counter: loop {
                                        if let Some(arg) = args.node.as_mut() {
                                            if let Some(val) = clist.search_list(arg.name.clone()).as_mut() {
                                                arg.name = "".to_string();
                                                arg.value = *val;

                                                arg.type_ = NodeType::EXPRESSION;
                                                arg.exprCode = ExprType::CONSTANT;
                                                arg.opCode = OpType::O_NONE;
                                                arg.stmtCode = StmtType::S_NONE;

                                                arg.left = None;
                                                arg.right = None;
                                                arg.arguments = None;
                                                arg.statements = None;
                                                madeChange.store(true, Ordering::Relaxed);
                                            }
                                        }
                                        else {
                                            break 'arg_counter;
                                        }

                                        if let Some(next_arg) = args.next.as_mut() {
                                            args = next_arg;
                                        }
                                        else {
                                            break 'arg_counter;
                                        }
                                    }
                                }
                            }

                            else {
                                if let Some(left_operand) = right_node.left.as_mut() {
                                    if left_operand.exprCode == ExprType::VARIABLE || left_operand.exprCode == ExprType::PARAMETER {
                                        if let Some(val) = clist.search_list(left_operand.name.clone()).as_mut() {
                                            left_operand.name = "".to_string();
                                            left_operand.value = *val;

                                            left_operand.type_ = NodeType::EXPRESSION;
                                            left_operand.exprCode = ExprType::CONSTANT;
                                            left_operand.opCode = OpType::O_NONE;
                                            left_operand.stmtCode = StmtType::S_NONE;

                                            left_operand.left = None;
                                            left_operand.right = None;
                                            left_operand.arguments = None;
                                            left_operand.statements = None;
                                            madeChange.store(true, Ordering::Relaxed);
                                        }
                                    }
                                }
                                if let Some(right_operand) = right_node.right.as_mut() {
                                    if right_operand.exprCode == ExprType::VARIABLE || right_operand.exprCode == ExprType::PARAMETER {
                                        if let Some(val) = clist.search_list(right_operand.name.clone()).as_mut(){
                                            right_operand.name = "".to_string();
                                            right_operand.value = *val;

                                            right_operand.type_ = NodeType::EXPRESSION;
                                            right_operand.exprCode = ExprType::CONSTANT;
                                            right_operand.opCode = OpType::O_NONE;
                                            right_operand.stmtCode = StmtType::S_NONE;

                                            right_operand.left = None;
                                            right_operand.right = None;
                                            right_operand.arguments = None;
                                            right_operand.statements = None;

                                            madeChange.store(true, Ordering::Relaxed);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                
            }
            else if node.stmtCode == StmtType::RETURN {
                if let Some(inner_exp) = node.left.as_mut() {
                    if let Some(val) = clist.search_list(inner_exp.name.clone()).as_mut() {
                        inner_exp.name = "".to_string();
                        inner_exp.value = *val;

                        inner_exp.type_ = NodeType::EXPRESSION;
                        inner_exp.exprCode = ExprType::CONSTANT;
                        inner_exp.opCode = OpType::O_NONE;
                        inner_exp.stmtCode = StmtType::S_NONE;

                        inner_exp.left = None;
                        inner_exp.right = None;
                        inner_exp.arguments = None;
                        inner_exp.statements = None;
                        madeChange.store(true, Ordering::Relaxed);
                    }
                }
            }
        } else {
            break;
        }

        if let Some(next) = statements.next.as_mut() {
            statements = next;
        } else {
            break;
        }
    }
}

#[no_mangle]
pub fn ConstProp(mut worklist: &mut RList) -> bool {
    let mut clist = constList::new();

    madeChange.store(false, Ordering::Relaxed);
    loop {
        /*
        ****************************************
                TODO : YOUR CODE HERE
        ****************************************
        */

        if let Some(node) = worklist.node.as_mut() {
            if let Some(statements) = node.statements.as_mut() {
                TrackConst(statements, &mut clist);
            }
        }
        else{
            break;
        }

        if let Some(next_node) = worklist.next.as_mut() {
            worklist = next_node;
            clist.free_list();
        }
        else{
            break;
        }
    }
    let res: bool = madeChange.load(Ordering::Relaxed);
    return res;
}

#[cfg(test)]
mod tests {

    use super::constList;

    #[test]
    fn create_list() {
        let mut clist = constList::new();

        clist.print_list();
        clist.update_list("A".to_string(), 1);
        clist.update_list("B".to_string(), 2);
        clist.update_list("C".to_string(), 3);

        clist.print_list();

        assert_eq!(clist.search_list("B".to_string()), Some(2));
        clist.free_list();
        clist.print_list();
    }
}
