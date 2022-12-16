use std::{collections::HashMap, fmt::Display, ptr::null_mut};

use crate::{
    gc::{Gc, GcHeader, GcRef},
    objects::{Closure, Object, Pair, Procedure, Symbol},
    op::Op,
    procs::scm_write,
};

const STACK_SIZE: usize = 256;

pub struct Vm {
    pub gc: Box<Gc>,
    ac: Object, // accumulator register.
    dc: Object, // display closure register.
    stack: [Object; STACK_SIZE],
    sp: *mut Object,
    fp: *mut Object,
    globals: HashMap<GcRef<Symbol>, Object>,
    ops: Vec<Op>, // Keep running ops so that they are not garbage collected.
}

impl Vm {
    pub fn new() -> Self {
        Self {
            ac: Object::Undef,
            dc: Object::Undef,
            gc: Box::new(Gc::new()),
            stack: [Object::Undef; STACK_SIZE],
            sp: null_mut(),
            fp: null_mut(),
            globals: HashMap::new(),
            ops: vec![],
        }
    }

    fn initialize_free_vars(&mut self) {
        let free_vars = vec![Object::Procedure(self.gc.alloc(Procedure::new(scm_write)))];
        let mut display = self.gc.alloc(Closure::new(0, 0, false, free_vars));
        display.prev = self.dc;
        self.dc = Object::Closure(display);
    }

    /// GC functions.
    fn alloc<T: Display + 'static>(&mut self, object: T) -> GcRef<T> {
        self.mark_and_sweep();
        self.gc.alloc(object)
    }

    fn mark_and_sweep(&mut self) {
        if self.gc.should_gc() {
            #[cfg(feature = "debug_log_gc")]
            println!("-- gc begin");

            self.mark_roots();
            self.gc.collect_garbage();

            #[cfg(feature = "debug_log_gc")]
            println!("-- gc end");
        }
    }

    fn mark_roots(&mut self) {
        for &value in &self.stack[0..self.stack_len()] {
            self.gc.mark_object(value);
        }

        self.gc.mark_object(self.ac);
        self.gc.mark_object(self.dc);

        for &value in &self.ops {
            match value {
                Op::Closure { .. } => (),
                Op::Constant(v) => {
                    self.gc.mark_object(v);
                }
                Op::DefineGlobal(symbol) => {
                    self.gc.mark_heap_object(symbol);
                }
                Op::ReferGlobal(symbol) => {
                    self.gc.mark_heap_object(symbol);
                }
                Op::Display(_) => (),
                Op::ReferFree(_) => (),
                Op::LetFrame(_) => (),
                Op::Enter(_) => (),
                Op::Halt => (),
                Op::Nop => (),
                Op::Undef => (),
                Op::ReferLocal(_) => (),
                Op::Leave(_) => (),
                Op::Push => (),
                Op::NumberAdd => (),
                Op::AddPair => (),
                Op::Cons => (),
                Op::LocalJmp(_) => (),
                Op::Test(_) => (),
                Op::Call(_) => (),
                Op::Return(_) => (),
                Op::Frame(_) => (),
            }
        }
    }

    pub fn intern(&mut self, s: &str) -> GcRef<Symbol> {
        self.gc.intern(s.to_owned())
    }

    fn pop(&mut self) -> Object {
        unsafe {
            self.sp = self.sp.offset(-1);
            *self.sp
        }
    }

    fn push(&mut self, value: Object) {
        unsafe {
            *self.sp = value;
            self.sp = self.sp.offset(1);
        }
    }

    fn index(&mut self, sp: *mut Object, n: isize) -> Object {
        unsafe { *sp.offset(-n - 1) }
    }

    fn stack_len(&self) -> usize {
        unsafe { self.sp.offset_from(self.stack.as_ptr()) as usize }
    }

    #[cfg(feature = "debug_log_vm")]
    fn print_vm(&mut self, op: Op) {
        println!("-----------------------------------------");
        println!("{:?} executed ac={:?}", op, self.ac);
        println!("-----------------------------------------");
        for &value in &self.stack[0..self.stack_len()] {
            println!("{:?}", value);
        }
        println!("-----------------------------------------<== sp")
    }
    #[cfg(not(feature = "debug_log_vm"))]
    fn print_vm(&mut self, _: Op) {}

    pub fn run(&mut self, ops: Vec<Op>) -> Object {
        // todo move to the initializer
        self.ops = ops;
        self.sp = self.stack.as_mut_ptr();
        self.fp = self.sp;
        let len = self.ops.len();
        self.initialize_free_vars();
        let mut pc = 0;
        while pc < len {
            let op = self.ops[pc];
            match op {
                Op::Constant(c) => {
                    self.ac = c;
                }
                Op::Push => {
                    self.push(self.ac);
                }
                Op::Cons => {
                    let first = self.pop();
                    let second = self.ac;
                    let pair = self.alloc(Pair::new(first, second));
                    println!("pair alloc(adr:{:?})", &pair.header as *const GcHeader);
                    self.ac = Object::Pair(pair);
                }
                Op::NumberAdd => match (self.pop(), self.ac) {
                    (Object::Number(a), Object::Number(b)) => {
                        println!("a={} ac={}", a, b);
                        self.ac = Object::Number(a + b);
                    }
                    (a, b) => {
                        panic!("add: numbers required but got {:?} {:?}", a, b);
                    }
                },
                Op::AddPair => {
                    if let Object::Pair(pair) = self.ac {
                        match (pair.first, pair.second) {
                            (Object::Number(lhs), Object::Number(rhs)) => {
                                self.ac = Object::Number(lhs + rhs);
                            }
                            _ => {
                                panic!(
                                    "add pair: numbers require but got {:?} and {:?}",
                                    pair.first, pair.second
                                );
                            }
                        }
                    } else {
                        panic!("pair required but got {:?}", self.ac);
                    }
                }
                Op::DefineGlobal(symbol) => {
                    self.globals.insert(symbol, self.ac);
                    self.ac = Object::Undef;
                }
                Op::ReferGlobal(symbol) => match self.globals.get(&symbol) {
                    Some(&value) => {
                        self.ac = value;
                    }
                    None => {
                        panic!("identifier {:?} not found", symbol);
                    }
                },
                Op::Enter(n) => unsafe {
                    self.fp = self.sp.offset(-n);
                },
                Op::LetFrame(_) => {
                    // todo: expand stack here.
                    self.push(self.dc);
                    self.push(Object::VMStackPointer(self.fp));
                }
                Op::ReferLocal(n) => unsafe {
                    self.ac = *self.fp.offset(n);
                },
                Op::Leave(n) => unsafe {
                    let sp = self.sp.offset(-n);

                    match self.index(sp, 0) {
                        Object::VMStackPointer(fp) => {
                            self.fp = fp;
                        }
                        x => {
                            panic!("fp expected but got {:?}", x);
                        }
                    }
                    self.dc = self.index(sp, 1);
                    self.sp = sp.offset(-2);
                },
                Op::Display(num_free_vars) => {
                    let mut free_vars = vec![];
                    let start = unsafe { self.sp.offset(-1) };
                    for i in 0..num_free_vars {
                        let var = unsafe { *start.offset(-i) };
                        free_vars.push(var);
                    }
                    let mut display = self.alloc(Closure::new(0, 0, false, free_vars));
                    display.prev = self.dc;

                    let display = Object::Closure(display);
                    self.dc = display;
                    self.sp = unsafe { self.sp.offset(-num_free_vars) };
                }
                Op::ReferFree(n) => match self.dc {
                    Object::Closure(mut closure) => {
                        self.ac = closure.refer_free(n);
                    }
                    _ => {
                        panic!("refer_free: display closure expected but got {:?}", self.dc);
                    }
                },
                Op::Test(skip_size) => {
                    if self.ac.is_false() {
                        pc = pc + skip_size - 1;
                    }
                }
                Op::LocalJmp(jump_size) => {
                    pc = pc + jump_size - 1;
                }
                Op::Closure {
                    size,
                    arg_len,
                    is_optional_arg,
                    num_free_vars,
                } => {
                    let mut free_vars = vec![];
                    let start = unsafe { self.sp.offset(-1) };
                    for i in 0..num_free_vars {
                        let var = unsafe { *start.offset(-i) };
                        free_vars.push(var);
                    }
                    self.ac = Object::Closure(self.alloc(Closure::new(
                        pc,
                        arg_len,
                        is_optional_arg,
                        free_vars,
                    )));

                    self.sp = unsafe { self.sp.offset(-num_free_vars) };
                    pc += size - 1;
                }
                Op::Call(arg_len) => {
                    match self.ac {
                        Object::Closure(closure) => {
                            self.dc = self.ac;
                            // self.cl = self.ac;
                            pc = closure.pc;
                            if closure.is_optional_arg {
                                panic!("not supported yet");
                            } else if arg_len == closure.arg_len {
                                self.fp = unsafe { self.sp.offset(-arg_len) };
                            } else {
                                panic!("wrong arguments");
                            }
                        }
                        Object::Procedure(procedure) => {
                            // self.cl = self.ac
                            assert_eq!(1, arg_len);
                            let arg = unsafe { *self.fp.offset(arg_len) };
                            //self.fp = unsafe { self.sp.offset(-arg_len) };
                            self.ac = (procedure.func)(arg);

                            self.return_n(1, &mut pc);
                        }
                        _ => {
                            panic!("can't call {:?}", self.ac);
                        }
                    }
                }
                Op::Return(n) => {
                    self.return_n(n, &mut pc);
                }
                Op::Frame(skip_size) => {
                    // Call frame in stack.
                    // ======================
                    //          pc*
                    // ======================
                    //          dc
                    // ======================
                    //          cl
                    // ======================
                    //          fp
                    // ======== sp ==========
                    //
                    // where pc* = pc + skip_size -1
                    let next_pc =
                        isize::try_from(pc + skip_size - 1).expect("can't convert to isize");
                    self.push(Object::Number(next_pc));
                    self.push(self.dc);
                    self.push(self.dc); // todo this should be cl.
                    self.push(Object::VMStackPointer(self.fp));
                }
                Op::Halt => return self.ac,
                Op::Undef => self.ac = Object::Undef,
                Op::Nop => {}
            }
            self.print_vm(op);
            pc += 1;
        }
        self.ac
    }

    fn return_n(&mut self, n: isize, pc: &mut usize) {
        let sp = unsafe { self.sp.offset(-n) };
        match self.index(sp, 0) {
            Object::VMStackPointer(fp) => {
                self.fp = fp;
            }
            _ => {
                panic!("not fp pointer")
            }
        }
        // todo We don't have cl yet.
        // self.cl = index(sp, 1);
        self.dc = self.index(sp, 2);
        match self.index(sp, 3) {
            Object::Number(next_pc) => {
                *pc = usize::try_from(next_pc).expect("pc it not a number");
            }
            _ => {
                panic!("not a pc");
            }
        }
        self.sp = unsafe { sp.offset(-4) }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    static SIZE_OF_PAIR: usize = std::mem::size_of::<Pair>();
    static SIZE_OF_CLOSURE: usize = std::mem::size_of::<Closure>();
    static SIZE_OF_PROCEDURE: usize = std::mem::size_of::<Procedure>();
    // Base closure + procedure as free variable
    static SIZE_OF_MIN_VM: usize = SIZE_OF_CLOSURE + SIZE_OF_PROCEDURE;

    fn test_ops_with_size(vm: &mut Vm, ops: Vec<Op>, expected: Object, expected_heap_diff: usize) {
        let before_size = vm.gc.bytes_allocated();
        let ret = vm.run(ops);
        vm.mark_and_sweep();
        let after_size = vm.gc.bytes_allocated();
        assert_eq!(
            after_size - before_size,
            SIZE_OF_MIN_VM + expected_heap_diff
        );
        assert_eq!(ret, expected);
    }

    // Custom hand written tests.
    #[test]
    fn test_symbol_intern() {
        let mut gc = Gc::new();

        let symbol = gc.intern("foo".to_owned());
        let symbol2 = gc.intern("foo".to_owned());
        assert_eq!(symbol.pointer, symbol2.pointer);
    }

    #[test]
    fn test_vm_call_proc() {
        let mut vm = Vm::new();
        // ((lambda (a) (+ a a))
        let ops: Vec<Op> = vec![
            Op::Frame(8),
            Op::Constant(Object::Number(3)),
            Op::Push,
            Op::ReferFree(0),
            Op::Call(1),
        ];
        let before_size = vm.gc.bytes_allocated();
        let ret = vm.run(ops);
        vm.mark_and_sweep();
        let after_size = vm.gc.bytes_allocated();
        assert_eq!(after_size - before_size, SIZE_OF_MIN_VM);
        match ret {
            Object::Undef => {}
            _ => panic!("ac was {:?}", ret),
        }
    }

    #[test]
    fn test_vm_alloc_many_pairs() {
        let mut vm = Vm::new();
        let mut ops = vec![];

        for _ in 0..100 {
            ops.push(Op::Constant(Object::Number(99)));
            ops.push(Op::Push);
            ops.push(Op::Constant(Object::Number(101)));
            ops.push(Op::Cons);
        }
        let before_size = vm.gc.bytes_allocated();
        vm.run(ops);
        vm.mark_and_sweep();
        let after_size = vm.gc.bytes_allocated();
        assert_eq!(after_size - before_size, SIZE_OF_MIN_VM + SIZE_OF_PAIR);
    }

    #[test]
    fn test_vm_define() {
        let mut vm = Vm::new();
        let ops = vec![
            Op::Constant(Object::Number(9)),
            Op::DefineGlobal(vm.gc.intern("a".to_owned())),
            Op::ReferGlobal(vm.gc.intern("a".to_owned())),
        ];
        let before_size = vm.gc.bytes_allocated();
        let ret = vm.run(ops);
        vm.mark_and_sweep();
        let after_size = vm.gc.bytes_allocated();
        assert_eq!(after_size - before_size, SIZE_OF_MIN_VM);
        match ret {
            Object::Number(a) => {
                assert_eq!(a, 9);
            }
            _ => panic!("{:?}", "todo"),
        }
    }

    #[test]
    fn test_vm_run_add_pair() {
        let mut vm = Vm::new();
        let ops = vec![
            Op::Constant(Object::Number(99)),
            Op::Push,
            Op::Constant(Object::Number(101)),
            Op::Cons,
            Op::AddPair,
        ];
        let before_size = vm.gc.bytes_allocated();
        let ret = vm.run(ops);
        vm.mark_and_sweep();
        let after_size = vm.gc.bytes_allocated();
        assert_eq!(after_size - before_size, SIZE_OF_MIN_VM);
        match ret {
            Object::Number(a) => {
                assert_eq!(a, 200);
            }
            _ => panic!("{:?}", "todo"),
        }
    }

    // All ops in the following tests are generated in data/.

    #[test]
    fn test_call0() {
        let mut vm = Vm::new();
        let ops = vec![
            Op::Frame(5),
            Op::Closure {
                size: 3,
                arg_len: 0,
                is_optional_arg: false,
                num_free_vars: 0,
            },
            Op::Constant(Object::Number(3)),
            Op::Return(0),
            Op::Call(0),
            Op::Halt,
            Op::Nop,
            Op::Nop,
        ];
        test_ops_with_size(&mut vm, ops, Object::Number(3), 0);
    }

    #[test]
    fn test_call1() {
        let mut vm = Vm::new();
        let ops = vec![
            Op::Frame(10),
            Op::Constant(Object::Number(1)),
            Op::Push,
            Op::Closure {
                size: 6,
                arg_len: 1,
                is_optional_arg: false,
                num_free_vars: 0,
            },
            Op::ReferLocal(0),
            Op::Push,
            Op::ReferLocal(0),
            Op::NumberAdd,
            Op::Return(1),
            Op::Call(1),
            Op::Halt,
            Op::Nop,
            Op::Nop,
        ];
        test_ops_with_size(&mut vm, ops, Object::Number(2), 0);
    }

    #[test]
    fn test_call2() {
        let mut vm = Vm::new();
        let ops = vec![
            Op::Frame(12),
            Op::Constant(Object::Number(1)),
            Op::Push,
            Op::Constant(Object::Number(2)),
            Op::Push,
            Op::Closure {
                size: 6,
                arg_len: 2,
                is_optional_arg: false,
                num_free_vars: 0,
            },
            Op::ReferLocal(0),
            Op::Push,
            Op::ReferLocal(1),
            Op::NumberAdd,
            Op::Return(2),
            Op::Call(2),
            Op::Halt,
            Op::Nop,
            Op::Nop,
        ];
        test_ops_with_size(&mut vm, ops, Object::Number(3), 0);
    }

    #[test]
    fn test_if0() {
        let mut vm = Vm::new();
        let ops = vec![
            Op::Constant(Object::Number(1)),
            Op::Test(3),
            Op::Constant(Object::Number(2)),
            Op::LocalJmp(2),
            Op::Constant(Object::Number(3)),
            Op::Halt,
            Op::Nop,
            Op::Nop,
        ];
        test_ops_with_size(&mut vm, ops, Object::Number(2), 0);
    }

    #[test]
    fn test_if1() {
        let mut vm = Vm::new();
        let ops = vec![
            Op::Constant(Object::False),
            Op::Test(3),
            Op::Constant(Object::Number(2)),
            Op::LocalJmp(2),
            Op::Constant(Object::Number(3)),
            Op::Halt,
            Op::Nop,
            Op::Nop,
        ];
        test_ops_with_size(&mut vm, ops, Object::Number(3), 0);
    }

    #[test]
    fn test_let0() {
        let mut vm = Vm::new();
        let ops = vec![
            Op::LetFrame(1),
            Op::Constant(Object::Number(0)),
            Op::Push,
            Op::Enter(1),
            Op::ReferLocal(0),
            Op::Leave(1),
            Op::Halt,
        ];
        test_ops_with_size(&mut vm, ops, Object::Number(0), 0);
    }

    #[test]
    fn test_let1() {
        let mut vm = Vm::new();
        let ops = vec![
            Op::LetFrame(2),
            Op::Constant(Object::Number(1)),
            Op::Push,
            Op::Constant(Object::Number(2)),
            Op::Push,
            Op::Enter(2),
            Op::ReferLocal(0),
            Op::Push,
            Op::ReferLocal(1),
            Op::NumberAdd,
            Op::Leave(2),
            Op::Halt,
        ];
        test_ops_with_size(&mut vm, ops, Object::Number(3), 0);
    }

    #[test]
    fn test_define0() {
        let mut vm = Vm::new();
        let ops = vec![
            Op::Constant(Object::Number(3)),
            Op::DefineGlobal(vm.gc.intern("a".to_owned())),
            Op::Halt,
        ];
        test_ops_with_size(&mut vm, ops, Object::Undef, 0);
    }

    #[test]
    fn test_nested_let0() {
        let mut vm = Vm::new();
        let ops = vec![
            Op::LetFrame(3),
            Op::Constant(Object::Number(1)),
            Op::Push,
            Op::Enter(1),
            Op::LetFrame(2),
            Op::ReferLocal(0),
            Op::Push,
            Op::Display(1),
            Op::Constant(Object::Number(2)),
            Op::Push,
            Op::Enter(1),
            Op::ReferFree(0),
            Op::Push,
            Op::ReferLocal(0),
            Op::NumberAdd,
            Op::Leave(1),
            Op::Leave(1),
            Op::Halt,
        ];
        test_ops_with_size(&mut vm, ops, Object::Number(3), 0);
    }

    #[test]
    fn test_nested_let1() {
        let mut vm = Vm::new();
        let ops = vec![
            Op::LetFrame(5),
            Op::Constant(Object::Number(1)),
            Op::Push,
            Op::Enter(1),
            Op::LetFrame(4),
            Op::ReferLocal(0),
            Op::Push,
            Op::Display(1),
            Op::Constant(Object::Number(2)),
            Op::Push,
            Op::Enter(1),
            Op::LetFrame(3),
            Op::ReferFree(0),
            Op::Push,
            Op::ReferLocal(0),
            Op::Push,
            Op::Display(2),
            Op::Constant(Object::Number(3)),
            Op::Push,
            Op::Enter(1),
            Op::ReferFree(1),
            Op::Push,
            Op::ReferFree(0),
            Op::NumberAdd,
            Op::Push,
            Op::ReferLocal(0),
            Op::NumberAdd,
            Op::Leave(1),
            Op::Leave(1),
            Op::Leave(1),
            Op::Halt,
        ];
        test_ops_with_size(&mut vm, ops, Object::Number(6), 0);
    }

    #[test]
    fn test_and() {
        let mut vm = Vm::new();
        let ops = vec![Op::Constant(Object::True), Op::Halt];
        test_ops_with_size(&mut vm, ops, Object::True, 0);
    }

    #[test]
    fn test_if2() {
        let mut vm = Vm::new();
        let ops = vec![
            Op::Constant(Object::False),
            Op::Test(3),
            Op::Constant(Object::False),
            Op::LocalJmp(2),
            Op::Constant(Object::True),
            Op::Halt,
            Op::Nop,
            Op::Nop,
        ];
        test_ops_with_size(&mut vm, ops, Object::True, 0);
    }
}
