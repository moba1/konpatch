
use std::io;
use std::ptr;
use std::error;
use std::fmt;

#[derive(Debug)]
pub enum Symbol {
    ValueIncrement
}

#[derive(Debug)]
pub struct OneDirectionNode {
    symbol: Symbol,
    next: Option<ptr::NonNull<OneDirectionNode>>,
}

#[derive(Debug)]
pub struct ControlFlowGraph {
    head: Option<ptr::NonNull<OneDirectionNode>>,
    tail: Option<ptr::NonNull<OneDirectionNode>>,
}

impl ControlFlowGraph {
    unsafe fn push(&mut self, node: ptr::NonNull<OneDirectionNode>) {
        unsafe {
            (*node.as_ptr()).next = None;
            let node = Some(node);
            match self.tail {
                None => self.head = node,
                Some(tail) => (*tail.as_ptr()).next = node,
            }
            self.tail = node;
        }
    }
}

#[derive(Debug)]
pub struct UnknownSymbolError(u8);

impl fmt::Display for UnknownSymbolError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Unknown symbol (0x{:02x})", self.0)
    }
}

impl error::Error for UnknownSymbolError {}

pub fn parse<R: io::Read>(reader: R) -> io::Result<ControlFlowGraph> {
    let mut control_flow_graph = ControlFlowGraph {
        head: None,
        tail: None,
    };

    for byte in reader.bytes() {
        let symbol = match byte? {
            // `+` in US ASCII
            0x2B => Symbol::ValueIncrement,
            // `\n` in US ASCII
            0x0A => continue,
            unknown_symbol => return Err(io::Error::new(
                io::ErrorKind::Other, UnknownSymbolError(unknown_symbol)
            )),
        };

        let node = Box::new(OneDirectionNode {
            symbol,
            next: None,
        });
        let node_ptr = ptr::NonNull::from(Box::leak(node));
        unsafe {
            control_flow_graph.push(node_ptr);
        }
    }

    Ok(control_flow_graph)
}