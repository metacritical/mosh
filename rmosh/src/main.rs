// GC implementation based on Loxido written by Manuel Cerón.
// See https://github.com/ceronman/loxido.
use std::alloc;
use std::fmt;
use std::fmt::Display;
use std::mem;
use std::ptr::NonNull;
use std::{ops::Deref, sync::atomic::AtomicUsize, usize};

struct GlobalAllocator {
    bytes_allocated: AtomicUsize,
}

impl GlobalAllocator {
    fn bytes_allocated(&self) -> usize {
        self.bytes_allocated
            .load(std::sync::atomic::Ordering::Relaxed)
    }
}

unsafe impl alloc::GlobalAlloc for GlobalAllocator {
    unsafe fn alloc(&self, layout: alloc::Layout) -> *mut u8 {
        self.bytes_allocated
            .fetch_add(layout.size(), std::sync::atomic::Ordering::Relaxed);
        mimalloc::MiMalloc.alloc(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: alloc::Layout) {
        mimalloc::MiMalloc.dealloc(ptr, layout);
        self.bytes_allocated
            .fetch_sub(layout.size(), std::sync::atomic::Ordering::Relaxed);
    }
}

#[global_allocator]
static GLOBAL: GlobalAllocator = GlobalAllocator {
    bytes_allocated: AtomicUsize::new(0),
};

pub struct Pair {
    pub header: GcHeader,
    pub first: Value,
    pub second: Value,
}

impl Pair {
    pub fn new(first: Value, second: Value) -> Self {
        Pair {
            header: GcHeader::new(ObjectType::Pair),
            first: first,
            second: second,
        }
    }
}

impl Display for Pair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "pair")
    }
}

pub struct Symbol {
    pub header: GcHeader,
    pub string: String,
}

impl Symbol {
    pub fn new(s: String) -> Self {
        Symbol {
            header: GcHeader::new(ObjectType::Symbol),
            string: s,
        }
    }
}

impl Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.string)
    }
}

pub struct GcRef<T> {
    pointer: NonNull<T>,
}

impl<T> Copy for GcRef<T> {}

impl<T> Clone for GcRef<T> {
    fn clone(&self) -> GcRef<T> {
        *self
    }
}

impl<T> Deref for GcRef<T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { self.pointer.as_ref() }
    }
}

pub struct Gc {
    next_gc: usize,
    first: Option<NonNull<GcHeader>>,
    grey_stack: Vec<NonNull<GcHeader>>,
}

impl Gc {
    const HEAP_GROW_FACTOR: usize = 2;

    pub fn new() -> Self {
        Gc {
            next_gc: 1024 * 1024,
            first: None,
            grey_stack: Vec::new(),
        }
    }

    pub fn alloc<T: Display + 'static>(&mut self, object: T) -> GcRef<T> {
        unsafe {
            let boxed = Box::new(object);
            let pointer = NonNull::new_unchecked(Box::into_raw(boxed));
            let mut header: NonNull<GcHeader> = mem::transmute(pointer.as_ref());
            header.as_mut().next = self.first.take();
            self.first = Some(header);

            GcRef { pointer }
        }
    }

    // Top level mark.
    // This mark root objects only and push them to grey_stack.
    pub fn mark_value(&mut self, value: Value) {
        match value {
            Value::Number(_) => {}
            Value::Symbol(symbol) => {
                self.mark_object(symbol);
            }            
            Value::Pair(pair) => {
                self.mark_object(pair);
            }
        }
    }

    pub fn mark_object<T: 'static>(&mut self, mut reference: GcRef<T>) {
        unsafe {
            let mut header: NonNull<GcHeader> = mem::transmute(reference.pointer.as_mut());
            header.as_mut().marked = true;
            self.grey_stack.push(header);
        }
    }

    fn trace_references(&mut self) {
        while let Some(pointer) = self.grey_stack.pop() {
            self.trace_pointer(pointer);
        }
    }

    fn trace_value(&mut self, value: Value) {
        match value {
            Value::Number(_) => {}
            Value::Symbol(pair) => {
                self.trace_object(pair);
            }            
            Value::Pair(pair) => {
                self.trace_object(pair);
            }
        }
    }

    pub fn trace_object<T: 'static>(&mut self, mut reference: GcRef<T>) {
        unsafe {
            let header: NonNull<GcHeader> = mem::transmute(reference.pointer.as_mut());
            self.trace_pointer(header);
        }
    }

    fn trace_pointer(&mut self, pointer: NonNull<GcHeader>) {
        let object_type = unsafe { &pointer.as_ref().obj_type };
        #[cfg(feature = "debug_log_gc")]
        println!("blacken(adr:{:?})", pointer);

        match object_type {
            ObjectType::Symbol => {}
            ObjectType::Pair => {
                let pair: &Pair = unsafe { mem::transmute(pointer.as_ref()) };
                self.mark_value(pair.first);
                self.mark_value(pair.second);
                self.trace_value(pair.first);
                self.trace_value(pair.second);
            }
        }
    }

    pub fn collect_garbage(&mut self) {
        self.trace_references();
        self.sweep();
        self.next_gc = GLOBAL.bytes_allocated() * Gc::HEAP_GROW_FACTOR;
    }
    fn sweep(&mut self) {
        let mut previous: Option<NonNull<GcHeader>> = None;
        let mut current: Option<NonNull<GcHeader>> = self.first;
        while let Some(mut object) = current {
            unsafe {
                let object_ptr = object.as_mut();
                current = object_ptr.next;
                if object_ptr.marked {
                    object_ptr.marked = false;
                    previous = Some(object);
                } else {
                    if let Some(mut previous) = previous {
                        previous.as_mut().next = object_ptr.next
                    } else {
                        self.first = object_ptr.next
                    }

                    println!("free(adr:{:?})", object_ptr as *mut GcHeader);
                    drop(Box::from_raw(object_ptr))
                }
            }
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ObjectType {
    Pair,
    Symbol,
}

#[repr(C)]
pub struct GcHeader {
    marked: bool,
    next: Option<NonNull<GcHeader>>,
    obj_type: ObjectType,
}

impl GcHeader {
    pub fn new(obj_type: ObjectType) -> Self {
        Self {
            marked: false,
            next: None,
            obj_type,
        }
    }
}

#[derive(Copy, Clone)]
pub enum Op {
    Constant(Value),
    Push,
    Add,
    AddPair,
    Cons,
}

#[derive(Copy, Clone)]
pub enum Value {
    Number(isize),
    Pair(GcRef<Pair>),
    Symbol(GcRef<Symbol>),
}

const STACK_SIZE: usize = 256;

pub struct Vm {
    pub ac: Value,
    pub gc: Box<Gc>,
    pub stack: [Value; STACK_SIZE],
    pub sp: usize,
    ops: Vec<Op>,
}

impl Vm {
    pub fn new() -> Self {
        Self {
            ac: Value::Number(0),
            gc: Box::new(Gc::new()),
            stack: [Value::Number(0); STACK_SIZE],
            sp: 0,
            ops: vec![],
        }
    }

    fn alloc<T: Display + 'static>(&mut self, object: T) -> GcRef<T> {
        self.mark_and_sweep();
        self.gc.alloc(object)
    }

    fn mark_and_sweep(&mut self) {
        println!("-- gc begin");

        self.mark_roots();
        self.gc.collect_garbage();

        println!("-- gc end");
    }

    fn mark_roots(&mut self) {
        for &value in &self.stack[0..self.sp] {
            self.gc.mark_value(value);
        }

        self.gc.mark_value(self.ac);

        for &value in &self.ops {
            match value {
                Op::Constant(v) => {
                    self.gc.mark_value(v);
                }
                Op::Push => (),
                Op::Add => (),
                Op::AddPair => (),
                Op::Cons => (),
            }
        }
    }

    pub fn run_add(&mut self) -> Value {
        // Don't call self.alloc here. It can trigger mark&sweep.
        let ops = vec![
            Op::Constant(Value::Number(99)),
            Op::Push,
            Op::Constant(Value::Number(1)),
            Op::Add,
        ];
        self.ops = ops;
        self.run()
    }

    pub fn run_add_pair(&mut self) -> Value {
        let ops = vec![
            Op::Constant(Value::Number(99)),
            Op::Push,
            Op::Constant(Value::Number(101)),
            Op::Cons,
            Op::AddPair,
        ];
        self.ops = ops;
        let val = self.run();
        self.mark_and_sweep();
        val
    }

    pub fn run(&mut self) -> Value {
        let len = self.ops.len();
        let mut idx = 0;
        while idx < len {
            let op = self.ops[idx];
            idx += 1;
            match op {
                Op::Constant(c) => {
                    self.ac = c;
                }
                Op::Push => {
                    assert!(self.sp < STACK_SIZE);
                    self.stack[self.sp] = self.ac;
                    self.sp += 1;
                }
                Op::Cons => {
                    self.sp -= 1;
                    let first = self.stack[self.sp];
                    let second = self.ac;
                    let pair = self.alloc(Pair::new(first, second));
                    println!("pair alloc(adr:{:?})", &pair.header as *const GcHeader);
                    self.ac = Value::Pair(pair);
                }
                Op::Add => {
                    self.sp -= 1;
                    match (self.stack[self.sp], self.ac) {
                        (Value::Number(a), Value::Number(b)) => {
                            self.ac = Value::Number(a + b);
                        }
                        _ => {
                            panic!("{:?}", "todo");
                        }
                    }
                }
                Op::AddPair => match self.ac {
                    Value::Pair(p) => match (p.first, p.second) {
                        (Value::Number(lhs), Value::Number(rhs)) => {
                            self.ac = Value::Number(lhs + rhs);
                        }
                        _ => {
                            panic!("{:?}", "todo");
                        }
                    },
                    _ => {
                        panic!("{:?}", "todo");
                    }
                },
            }
        }
        self.ac
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_vm_run_add() {
        let mut vm = Vm::new();
        let ret = vm.run_add();
        match ret {
            Value::Number(a) => {
                assert_eq!(a, 100);
            }
            _ => panic!("{:?}", "todo"),
        }
    }

    #[test]
    fn test_vm_run_add_pair() {
        let mut vm = Vm::new();
        let ret = vm.run_add_pair();
        match ret {
            Value::Number(a) => {
                assert_eq!(a, 200);
            }
            _ => panic!("{:?}", "todo"),
        }
    }


    #[test]
    fn test_symbol() {
        let mut gc = Gc::new();
        let symbol = gc.alloc(Symbol::new("define".to_owned()));
        let symbol = Value::Symbol(symbol);
        match symbol {
            Value::Symbol(s) => {
                assert_eq!(s.string, "define");
            }_ => {
                panic!("{:?}", "todo");                
            }
        }
    }    
}
fn main() {}
