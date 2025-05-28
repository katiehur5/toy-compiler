#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

extern crate libc;
use std::fmt;
use std::ptr;

/* Structs and Enums */
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum NodeType {
    N_NONE,
    FUNCTIONDECL,
    STATEMENT,
    EXPRESSION,
}

impl fmt::Display for NodeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            NodeType::N_NONE => "N_NONE",
            NodeType::FUNCTIONDECL => "FUNCTIONDECL",
            NodeType::STATEMENT => "STATEMENT",
            NodeType::EXPRESSION => "EXPRESSION",
        };
        write!(f, "{}", s)
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum StmtType {
    S_NONE,
    ASSIGN,
    RETURN,
}

impl fmt::Display for StmtType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            StmtType::S_NONE => "S_NONE",
            StmtType::ASSIGN => "ASSIGN",
            StmtType::RETURN => "RETURN",
        };
        write!(f, "{}", s)
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ExprType {
    E_NONE,
    VARIABLE,
    CONSTANT,
    PARAMETER,
    OPERATION,
}

impl fmt::Display for ExprType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            ExprType::E_NONE => "E_NONE",
            ExprType::VARIABLE => "VARIABLE",
            ExprType::CONSTANT => "CONSTANT",
            ExprType::PARAMETER => "PARAMETER",
            ExprType::OPERATION => "OPERATION",
        };
        write!(f, "{}", s)
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum OpType {
    O_NONE,
    FUNCTIONCALL,
    MULTIPLY,
    DIVIDE,
    ADD,
    SUBTRACT,
    NEGATE,
    BOR,
    BAND,
    BXOR,
    BSHR,
    BSHL,
}

impl fmt::Display for OpType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            OpType::O_NONE => "O_NONE",
            OpType::FUNCTIONCALL => "FUNCTIONCALL",
            OpType::MULTIPLY => "MULTIPLY",
            OpType::DIVIDE => "DIVIDE",
            OpType::ADD => "ADD",
            OpType::SUBTRACT => "SUBTRACT",
            OpType::NEGATE => "NEGATE",
            OpType::BOR => "BOR",
            OpType::BAND => "BAND",
            OpType::BXOR => "BXOR",
            OpType::BSHR => "BSHR",
            OpType::BSHL => "BSHL",
        };
        write!(f, "{}", s)
    }
}

/*
 ALL IR Nodes are expressed with Node. In cases we have to keep a list of nodes, then we use NodeList.
 Use type, exprCode, opCode, and stmtCode to identify exactly what this node represents.
 More information on each IR node in "Helper functions to create IR nodes"
*/
#[repr(C)]
#[derive(Debug, Clone)]
pub struct Node {
    pub name: *mut libc::c_char,
    pub value: libc::c_long,

    pub type_: NodeType,
    pub exprCode: ExprType,
    pub opCode: OpType,
    pub stmtCode: StmtType,

    pub left: *mut Node,
    pub right: *mut Node,

    pub arguments: *mut NodeList,
    pub statements: *mut NodeList,
}

impl Node {
    pub fn name(&self) -> String {
        char_ptr_to_string(self.name)
    }

    pub fn value(&self) -> i64 {
        self.value as i64
    }

    pub fn node_stats(&self) -> String {
        format!(
            "Node: name: {}, value: {}, type: {}, exprCode: {}, opCode: {}, stmtCode: {}",
            self.name(),
            self.value(),
            self.type_,
            self.exprCode,
            self.opCode,
            self.stmtCode
        )
    }
}

// implement drop trait for Node to free name memory
impl Drop for Node {
    fn drop(&mut self) {
        unsafe {
            if !self.name.is_null() {
                libc::free(self.name as *mut libc::c_void);
            }
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct NodeList {
    pub node: *mut Node,
    pub next: *mut NodeList,
}

impl AsMut<NodeList> for NodeList {
    fn as_mut(&mut self) -> &mut NodeList {
        self
    }
}

#[derive(PartialEq)]
pub struct RList {
    pub node: LinkNode,
    pub next: LinkList,
}

pub type LinkNode = Option<Box<RNode>>;
pub type LinkList = Option<Box<RList>>;

#[derive(PartialEq)]
pub struct RNode {
    pub name: String,
    pub value: i64,

    pub type_: NodeType,
    pub exprCode: ExprType,
    pub opCode: OpType,
    pub stmtCode: StmtType,

    pub left: LinkNode,
    pub right: LinkNode,

    pub arguments: LinkList,
    pub statements: LinkList,
}

pub fn from_nodelist(worklist: *mut NodeList) -> RList {
    // Initialize an empty RList
    let mut rlist = RList::new();

    // Pointer to loop through NodeList
    let worklist = worklist;

    if !worklist.is_null() {
        unsafe {
            rlist.add_node(RNode::from_raw((*worklist).node));
            rlist.add_next(RList::from_raw((*worklist).next));
        }
    }

    rlist
}

pub fn to_nodelist(rlist: &RList) -> *mut NodeList {
    convert_rlist_to_nodelist(rlist)
}

impl RList {
    pub fn new() -> Self {
        RList {
            node: None,
            next: None,
        }
    }

    pub fn add_node(&mut self, node: LinkNode) {
        self.node = node
    }

    pub fn add_next(&mut self, new_list: LinkList) {
        self.next = new_list
    }

    fn from_raw(raw: *mut NodeList) -> LinkList {
        if raw.is_null() {
            None
        } else {
            Some(Box::new(from_nodelist(raw)))
        }
    }
}

// Implement the conversion from raw pointer to Option<Box<Node>>
impl RNode {
    pub fn new(
        name: String,
        value: i64,
        type_: NodeType,
        exprCode: ExprType,
        opCode: OpType,
        stmtCode: StmtType,
        left: LinkNode,
        right: LinkNode,
        arguments: LinkList,
        statements: LinkList,
    ) -> Self {
        RNode {
            name,
            value,
            type_,
            exprCode,
            opCode,
            stmtCode,
            left,
            right,
            arguments,
            statements,
        }
    }

    fn from_raw(raw: *mut Node) -> LinkNode {
        if raw.is_null() {
            None
        } else {
            unsafe {
                Some(Box::new(RNode::new(
                    (*raw).name(),
                    (*raw).value(),
                    (*raw).type_,
                    (*raw).exprCode,
                    (*raw).opCode,
                    (*raw).stmtCode,
                    RNode::from_raw((*raw).left),
                    RNode::from_raw((*raw).right),
                    RList::from_raw((*raw).arguments),
                    RList::from_raw((*raw).statements),
                )))
            }
        }
    }
}

fn convert_rnode_to_node(rnode: &RNode) -> *mut Node {
    let node = Box::new(Node {
        name: string_to_char_ptr(&rnode.name),
        value: rnode.value as libc::c_long,
        type_: rnode.type_,
        exprCode: rnode.exprCode,
        opCode: rnode.opCode,
        stmtCode: rnode.stmtCode,
        left: rnode
            .left
            .as_ref()
            .map(|n| convert_rnode_to_node(n))
            .unwrap_or(ptr::null_mut()),
        right: rnode
            .right
            .as_ref()
            .map(|n| convert_rnode_to_node(n))
            .unwrap_or(ptr::null_mut()),
        arguments: rnode
            .arguments
            .as_ref()
            .map(|l| convert_rlist_to_nodelist(l))
            .unwrap_or(ptr::null_mut()),
        statements: rnode
            .statements
            .as_ref()
            .map(|l| convert_rlist_to_nodelist(l))
            .unwrap_or(ptr::null_mut()),
    });

    Box::into_raw(node)
}

fn convert_rlist_to_nodelist(rlist: &RList) -> *mut NodeList {
    let node = rlist
        .node
        .as_ref()
        .map(|node| convert_rnode_to_node(node))
        .unwrap_or(ptr::null_mut());

    let next = rlist
        .next
        .as_ref()
        .map(|list| convert_rlist_to_nodelist(list))
        .unwrap_or(ptr::null_mut());

    let nodelist = Box::new(NodeList { node, next });

    Box::into_raw(nodelist)
}

/*********************************************************************************************************
                                        Printing Functions
**********************************************************************************************************/

fn string_to_char_ptr(s: &str) -> *mut libc::c_char {
    let mut bytes = s.to_string().into_bytes();
    // null-terminate the string
    bytes.push(0);
    let ptr = bytes.as_mut_ptr();
    // prevent Rust from deallocating the string
    std::mem::forget(bytes);
    ptr as *mut libc::c_char
}

fn char_ptr_to_string(s: *mut libc::c_char) -> String {
    unsafe {
        if s.is_null() {
            return String::new();
        }
        let c_str = std::ffi::CStr::from_ptr(s);
        c_str.to_string_lossy().into_owned()
    }
}

pub fn print_program(list: &RList) {
    let mut current = list;

    loop {
        if let Some(node) = current.node.as_ref() {
            print_node(node);
        } else {
            break;
        }

        if let Some(next) = current.next.as_ref() {
            current = next;
        } else {
            break;
        }
    }
}

pub fn print_node(node: &RNode) {
    match node.type_ {
        NodeType::FUNCTIONDECL => {
            print_function_decl(node);
        }
        NodeType::STATEMENT => {
            print_statement(node);
        }
        NodeType::EXPRESSION => {
            print_expression(node);
        }
        _ => {}
    }
}

fn print_function_decl(node: &RNode) {
    print!("long {} (", node.name);

    if let Some(arguments) = node.arguments.as_ref() {
        print_parameters(arguments);
    }
    print!(") {{\n");

    if let Some(statements) = node.statements.as_ref() {
        print_statements(statements);
    }
    print!("}}\n");
}

fn print_parameters(nodelist: &RList) {
    let mut nl = nodelist;
    loop {
        if let Some(node) = nl.node.as_ref() {
            print_expression(node);
        } else {
            break;
        }

        if let Some(next) = nl.next.as_ref() {
            nl = next;
            print!(", ");
        } else {
            break;
        }
    }
}

fn print_statements(nodelist: &RList) {
    let mut nl = nodelist;

    loop {
        if let Some(node) = nl.node.as_ref() {
            print!("\t");

            print_statement(node);

            print!(";\n");
        } else {
            break;
        }

        if let Some(next) = nl.next.as_ref() {
            nl = next;
        } else {
            break;
        }
    }
}

fn print_statement(node: &RNode) {
    match node.stmtCode {
        StmtType::ASSIGN => {
            print_assignment(node);
        }
        StmtType::RETURN => {
            print_return(node);
        }
        _ => {}
    }
}

fn print_return(node: &RNode) {
    print!("return ");
    if let Some(left) = node.left.as_ref() {
        print_expression(left);
    }
}

fn print_assignment(node: &RNode) {
    print!("{} = ", node.name);
    if let Some(right) = node.right.as_ref() {
        print_expression(right);
    }
}
 
fn print_expression(node: &RNode) {
    match node.exprCode {
        ExprType::VARIABLE => {
            if let Some(left) = node.left.as_ref() {
                if left.type_ == NodeType::EXPRESSION && left.exprCode == ExprType::PARAMETER {
                    print!("{}", left.name);
                } else if left.type_ == NodeType::STATEMENT && left.stmtCode == StmtType::ASSIGN {
                    print!("{}", left.name);
                }
            }
        }
        ExprType::CONSTANT => {
            print!("{}", node.value);
        }
        ExprType::PARAMETER => {
            print!("long {}", node.name);
        }
        ExprType::OPERATION => {
            print_operation(node);
        }
        _ => {}
    }
}

fn print_operation(node: &RNode) {
    match node.opCode {
        OpType::FUNCTIONCALL => {
            print_function_call(node);
        }
        OpType::MULTIPLY => {
            print_binary_operation(node, "*");
        }
        OpType::DIVIDE => {
            print_binary_operation(node, "/");
        }
        OpType::ADD => {
            print_binary_operation(node, "+");
        }
        OpType::SUBTRACT => {
            print_binary_operation(node, "-");
        }
        OpType::NEGATE => {
            print_unary_operation(node, "-");
        }
        OpType::BOR => {
            print_binary_operation(node, "|");
        }
        OpType::BAND => {
            print_binary_operation(node, "&");
        }
        OpType::BXOR => {
            print_binary_operation(node, "^");
        }
        OpType::BSHR => {
            print_binary_operation(node, ">>");
        }
        OpType::BSHL => {
            print_binary_operation(node, "<<");
        }
        _ => {}
    }
}

fn print_function_call(node: &RNode) {
    if let Some(left) = node.left.as_ref() {
        print!("{}(", left.name);
    }
    if let Some(arguments) = node.arguments.as_ref() {
        print_parameters(arguments);
    }
    print!(")");
}

fn print_binary_operation(node: &RNode, op: &str) {
    print!("(");
    if let Some(left) = node.left.as_ref() {
        print_expression(left);
    }
    print!(" {} ", op);
    if let Some(right) = node.right.as_ref() {
        print_expression(right);
    }
    print!(")");
}

fn print_unary_operation(node: &RNode, op: &str) {
    print!("({}", op);
    if let Some(left) = node.left.as_ref() {
        print_expression(left);
    }
    print!(")");
}

