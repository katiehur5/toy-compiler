#![allow(unused)]

mod codegen;
mod constfolding;
mod constprop;
mod deadassign;
pub mod expression;

use codegen::*;
use constfolding::*;
use constprop::*;
use deadassign::*;
use expression::*;

#[no_mangle]
pub extern "C" fn rust_mod(funcdecls: *mut NodeList) {
    let mut rlist = from_nodelist(funcdecls);

    println!("Printing the AST BEFORE OPTIMIZATION");
    print_program(&rlist);

    /*
    *************************************
         TODO: Call Optimizer Functions
    *************************************
    */
    let mut made_change: bool = true;
    while made_change {
     /*
     if ConstantFolding(&mut rlist) {
          println!("folding\n");
     }
     else if ConstProp(&mut rlist) {
          println!("prop\n");
     }
     else if DeadAssign(&mut rlist) {
          println!("dead\n");
     }
     else {
          made_change = false;
     }
     */
     
     made_change = ConstantFolding(&mut rlist) || ConstProp(&mut rlist) || DeadAssign(&mut rlist);
    }

    println!("Printing the AST AFTER OPTIMIZATION");
    print_program(&rlist);

    /*
    *************************************
         TODO: Call Codegen
    *************************************
    */
    Codegen(&mut rlist);
}

