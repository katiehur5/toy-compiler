/*
***********************************************************************
  DEADASSIGN.RS : IMPLEMENT THE DEAD CODE ELIMINATION OPTIMIZATION HERE
************************************************************************
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
struct refVar {
    pub name: String,
    pub next: LinkVar,
}

type LinkVar = Option<Box<refVar>>;

impl Default for refVar {
    fn default() -> refVar {
        refVar {
            name: "".to_string(),
            next: None,
        }
    }
}

struct varList {
    pub head: LinkVar,
}

impl varList {
    fn new() -> Self {
        varList { head: None }
    }

    fn free_list(&mut self) {
        while !self.head.is_none() {
            self.head.take().map(|node| {
                self.head = node.next;
            });
        }
    }

    /*
    ***********************************************************************
    FUNCTION TO ADD A REFERENCE TO THE REFERENCE LIST
    ************************************************************************
    */
    fn add_ref(&mut self, name: String) {
        let new_last = Box::new(refVar {
            name: name,
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
    ***********************************************************************
    FUNCTION TO UPDATE THE REFERENCE LIST WHEN A VARIABLE IS REFERENCED
    IF NOT DONE SO ALREADY.
    ************************************************************************
    */
    fn update_list(&mut self, node: &mut RNode) {
        if let Some(right) = node.right.as_mut() {
            if right.exprCode == ExprType::VARIABLE {
                if !self.var_exists(right.name.clone()) {
                    self.add_ref(right.name.clone())
                }
            }
        }

        if let Some(left) = node.left.as_mut() {
            if left.exprCode == ExprType::VARIABLE {
                if !self.var_exists(left.name.clone()) {
                    self.add_ref(left.name.clone())
                }
            }
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
                print!("{}-> ", node.name);
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
    fn var_exists(&self, name: String) -> bool {
        let mut current = &self.head;
        loop {
            if let Some(node) = current.as_ref() {
                if node.name == name {
                    return true;
                }

                current = &node.next;
            } else {
                break;
            }
        }
        return false;
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
********************************************************************
  THIS FUNCTION IS MEANT TO TRACK THE REFERENCES OF EACH VARIABLE
  TO HELP DETERMINE IF IT WAS USED OR NOT LATER
********************************************************************
*/
#[no_mangle]
fn TrackRef(funcNode: &mut RNode, vlist: &mut varList) {
    if let Some(mut statements) = funcNode.statements.as_mut() {
        loop {
            if let Some(stmtNode) = statements.node.as_mut() {
                /*
                ****************************************
                        TODO : YOUR CODE HERE
                ****************************************
                */
                if stmtNode.stmtCode == StmtType::ASSIGN {
                    if let Some(stmtNodeRight) = stmtNode.right.as_mut() {
                        if stmtNodeRight.opCode == OpType::FUNCTIONCALL {
                            if let Some(mut args) = stmtNodeRight.arguments.as_mut() {
                                loop {
                                    if let Some(arg) = args.node.as_mut() {
                                        if arg.exprCode == ExprType::VARIABLE {
                                            if !vlist.var_exists(arg.name.clone()) {
                                                vlist.add_ref(arg.name.clone());
                                            }
                                        }
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
                        }
                        else if stmtNodeRight.exprCode == ExprType::OPERATION {
                            vlist.update_list(stmtNodeRight);
                        }
                        else if stmtNodeRight.exprCode == ExprType::VARIABLE {
                            if !vlist.var_exists(stmtNodeRight.name.clone()) {
                                vlist.add_ref(stmtNodeRight.name.clone());
                            }
                        }
                    }
                }
                else if stmtNode.stmtCode == StmtType::RETURN {
                    if let Some(stmtNodeLeft) = stmtNode.left.as_mut() {
                        if stmtNodeLeft.exprCode == ExprType::VARIABLE {
                            if !vlist.var_exists(stmtNodeLeft.name.clone()) {
                                vlist.add_ref(stmtNodeLeft.name.clone());
                            }
                        }
                    }
                }

                //unimplemented!();
            } else {
                break;
            }
            if let Some(next) = statements.next.as_mut() {
                statements = next;
            } else {
                break;
            }
        }
    } else {
        return;
    }
}

/*
***************************************************************
  THIS FUNCTION IS MEANT TO DO THE ACTUAL DEADCODE REMOVAL
  BASED ON THE INFORMATION OF TRACKED REFERENCES
****************************************************************
*/
#[no_mangle]
fn RemoveDead(mut statements: &mut RList, vlist: &mut varList) {
    while let Some(stmtNode) = statements.node.as_mut() {
        /*
          *************************************************************************************
        TODO: YOUR CODE HERE
          **************************************************************************************
          */
        if stmtNode.stmtCode == StmtType::ASSIGN {
            if !vlist.var_exists(stmtNode.name.clone()) {
                if let Some(next_stmt) = statements.next.take() {
                    *statements = *next_stmt;
                }
                else {
                    statements.node = None;
                }
                madeChange.store(true, Ordering::Relaxed);
                continue;
            }

        }

        if let Some(next) = statements.next.as_mut() {
            statements = next;
        } else {
            break;
        }
    }
}

#[no_mangle]
pub fn DeadAssign(mut worklist: &mut RList) -> bool {
    let mut vlist = varList::new();
    madeChange.store(false, Ordering::Relaxed);
    loop {
        /*
        ****************************************
                TODO : YOUR CODE HERE
        ****************************************
        */
        if let Some(mut node) = worklist.node.as_mut() {
            TrackRef(&mut node,&mut vlist);
            if let Some(mut stmts) = node.statements.as_mut() {
                RemoveDead(&mut stmts, &mut vlist);
            }
        }
        else {
            break;
        }

        if let Some(next_node) = worklist.next.as_mut() {
            worklist = next_node;
            vlist.free_list();
        }
        else {
            break;
        }
    }
    let res: bool = madeChange.load(Ordering::Relaxed);
    return res;
}
