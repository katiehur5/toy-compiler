/*
   CONSTFOLDING.RS : THIS FILE IMPLEMENTS THE CONSTANT FOLDING OPTIMIZATION
*/
#![allow(non_snake_case)]
use lazy_static::lazy_static;

use std::sync::atomic::{AtomicBool, Ordering};

lazy_static! {
    static ref madeChange: AtomicBool = AtomicBool::new(false);
}

use crate::expression::*;

/*
*************************************************************************************
   YOUR CODE IS TO BE FILLED IN THE GIVEN TODO BLANKS. YOU CAN CHOOSE TO USE ALL
   UTILITY FUNCTIONS OR NONE. YOU CAN ADD NEW FUNCTIONS. BUT DO NOT FORGET TO
   DECLARE THEM IN THE HEADER FILE.
**************************************************************************************
*/

/*
******************************************************************************************
FUNCTION TO CALCULATE THE CONSTANT EXPRESSION VALUE
OBSERVE THAT THIS IMPLEMENTATION CONSIDERS ADDITIONAL OPTIMIZATIONS SUCH AS:
1.  IDENTITY MULTIPLY = 1 * ANY_VALUE = ANY_VALUE - AVOID MULTIPLICATION CYCLE IN THIS CASE
2.  ZERO MULTIPLY = 0 * ANY_VALUE = 0 - AVOID MULTIPLICATION CYCLE
3.  DIVIDE BY ONE = ORIGINAL_VALUE - AVOID DIVISION CYCLE
4.  SUBTRACT BY ZERO = ORIGINAL_VALUE - AVOID SUBTRACTION
5.  MULTIPLICATION BY 2 = ADDITION BY SAME VALUE [STRENGTH REDUCTION]
******************************************************************************************
*/

#[no_mangle]
fn CalcExprValue(node: &mut RNode) -> i64 {
    if let Some(leftNode) = node.left.as_mut() {
        match node.opCode {
            OpType::MULTIPLY => {
                if let Some(rightNode) = node.right.as_mut() {
                    if leftNode.value == 1 {
                        return rightNode.value;
                    } else if rightNode.value == 1 {
                        return leftNode.value;
                    } else if leftNode.value == 0 || rightNode.value == 0 {
                        return 0;
                    } else if leftNode.value == 2 {
                        return rightNode.value + rightNode.value;
                    } else if rightNode.value == 2 {
                        return leftNode.value + leftNode.value;
                    } else {
                        return leftNode.value * rightNode.value;
                    }
                } else {
                    return -1;
                }
            }
            OpType::DIVIDE => {
                if let Some(rightNode) = node.right.as_mut() {
                    if rightNode.value == 1 {
                        return leftNode.value;
                    } else {
                        return leftNode.value / rightNode.value;
                    }
                } else {
                    return -1;
                }
            }
            OpType::ADD => {
                if let Some(rightNode) = node.right.as_mut() {
                    return leftNode.value + rightNode.value;
                } else {
                    return -1;
                }
            }
            OpType::SUBTRACT => {
                if let Some(rightNode) = node.right.as_mut() {
                    return leftNode.value - rightNode.value;
                } else {
                    return -1;
                }
            } 
            OpType::NEGATE => {
                return -leftNode.value;
            }
            OpType::BSHR => {
                if let Some(rightNode) = node.right.as_mut() {
                    return leftNode.value >> rightNode.value;
                } else {
                    return -1;
                }
            }
            OpType::BSHL => {
                if let Some(rightNode) = node.right.as_mut() {
                    return leftNode.value << rightNode.value;
                } else {
                    return -1;
                }
            }
            OpType::BAND => {
                if let Some(rightNode) = node.right.as_mut() {
                    return leftNode.value & rightNode.value;
                } else {
                    return -1;
                }
            }
            OpType::BOR => {
                if let Some(rightNode) = node.right.as_mut() {
                    return leftNode.value | rightNode.value;
                } else {
                    return -1;
                }
            }
            OpType::BXOR => {
                if let Some(rightNode) = node.right.as_mut() {
                    return leftNode.value ^ rightNode.value;
                } else {
                    return -1;
                }
            }
            _ => {
                return 0;
            }
        }
    } else {
        return -1;
    }
}

/*
**********************************************************************************************************************************
// YOU CAN MAKE CHANGES/ADD AUXILLIARY FUNCTIONS BELOW THIS LINE
**********************************************************************************************************************************
*/

/*
*****************************************************************************************************
THIS FUNCTION IS MEANT TO PROCESS THE CANDIDATE STATEMENTS AND PERFORM CONSTANT FOLDING
WHEREVER APPLICABLE.
******************************************************************************************************
*/
#[no_mangle]
fn ConstFoldPerStatement(stmtNodeRight: &mut RNode) {
    /*
      *************************************************************************************
            TODO: YOUR CODE HERE
      **************************************************************************************
    */
    if let Some(left_node) = stmtNodeRight.left.as_mut() {
        if let Some(right_node) = stmtNodeRight.right.as_mut() {
            if left_node.exprCode == ExprType::CONSTANT && right_node.exprCode == ExprType::CONSTANT {
                let result: i64 = CalcExprValue(stmtNodeRight);
                if result != -1 {
                    stmtNodeRight.name = "".to_string();
                    stmtNodeRight.value = result;
                    
                    stmtNodeRight.type_ = NodeType::EXPRESSION;
                    stmtNodeRight.exprCode = ExprType::CONSTANT;
                    stmtNodeRight.opCode = OpType::O_NONE;
                    stmtNodeRight.stmtCode = StmtType::S_NONE;

                    stmtNodeRight.left = None;
                    stmtNodeRight.right = None;
                    stmtNodeRight.arguments = None;
                    stmtNodeRight.statements = None;

                    madeChange.store(true, Ordering::Relaxed);
                }
            }
        }
        else {
            if left_node.exprCode == ExprType::CONSTANT {
                let result: i64 = CalcExprValue(stmtNodeRight);
                if result != -1 {
                    stmtNodeRight.name = "".to_string();
                    stmtNodeRight.value = result;
                    
                    stmtNodeRight.type_ = NodeType::EXPRESSION;
                    stmtNodeRight.exprCode = ExprType::CONSTANT;
                    stmtNodeRight.opCode = OpType::O_NONE;
                    stmtNodeRight.stmtCode = StmtType::S_NONE;

                    stmtNodeRight.left = None;
                    stmtNodeRight.right = None;
                    stmtNodeRight.arguments = None;
                    stmtNodeRight.statements = None;

                    madeChange.store(true, Ordering::Relaxed);
                }
            }
        }
    }

}

/*
*****************************************************************************************************
THIS FUNCTION IS MEANT TO IDENTIFY THE STATEMENTS THAT ARE ACTUAL CANDIDATES FOR CONSTANT FOLDING
AND CALL THE APPROPRIATE FUNCTION FOR THE IDENTIFIED CANDIDATE'S CONSTANT FOLDING
******************************************************************************************************
*/
#[no_mangle]
fn ConstFoldPerFunction(funcNode: &mut RNode) {
    if let Some(mut statements) = funcNode.statements.as_mut() {
        loop {
            if let Some(node) = statements.node.as_mut() {
                if node.stmtCode == StmtType::ASSIGN {
                    if let Some(node_right) = node.right.as_mut() {
                        if node_right.type_ == NodeType::EXPRESSION {
                            if node_right.exprCode == ExprType::OPERATION {
                                if node_right.opCode != OpType::FUNCTIONCALL &&
                                node_right.opCode != OpType::O_NONE {
                                    ConstFoldPerStatement(node_right);
                                }
                            }
                        }
                    }
                }
            }
            else {
                break;
            }
            if let Some(next_node) = statements.next.as_mut() {
                statements = next_node;
            }
            else {
                break;
            }
        }

            // unimplemented!();
    } else {
        return;
    }
}

/*
**********************************************************************************************************************************
// YOU CAN MAKE CHANGES/ADD AUXILLIARY FUNCTIONS ABOVE THIS LINE
********************************************************************************************************************************
*/

/*
*****************************************************************************************************
THIS FUNCTION ENSURES THAT THE CONSTANT FOLDING OPTIMIZATION IS DONE FOR EVERY FUNCTION IN THE PROGRAM
******************************************************************************************************
*/
#[no_mangle]
pub fn ConstantFolding(mut worklist: &mut RList) -> bool {
    madeChange.store(false, Ordering::Relaxed);

    loop {
        if let Some(node) = worklist.node.as_mut() {
            ConstFoldPerFunction(node);
        }
        else {
            break;
        }
        if let Some(next_node) = worklist.next.as_mut() {
            worklist = next_node;
        }
        else {
            break;
        }
    }
    
    let res: bool = madeChange.load(Ordering::Relaxed);
    return res;
}

/*
****************************************************************************************************************************
END OF CONSTANT FOLDING
*****************************************************************************************************************************
*/
