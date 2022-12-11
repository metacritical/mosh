// GC implementation based on Loxido written by Manuel Cerón.
// See https://github.com/ceronman/loxido.

use std::collections::HashMap;
use std::fmt::Display;
use std::hash::{Hash, Hasher};
use std::mem;
use std::ptr::NonNull;
use std::{alloc, fmt};
use std::{ops::Deref, ops::DerefMut, sync::atomic::AtomicUsize, usize};

use crate::objects::{Pair, Symbol};
use crate::values::Value;
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

#[derive(Debug)]
pub struct GcRef<T> {
    pub pointer: NonNull<T>,
}

impl<T> Display for GcRef<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "GcRef<T>")
    }
}

impl<T> PartialEq for GcRef<T> {
    fn eq(&self, other: &Self) -> bool {
        self.pointer == other.pointer
    }
}
impl<T> Eq for GcRef<T> {}

impl<T> Hash for GcRef<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.pointer.hash(state);
    }
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

impl<T> DerefMut for GcRef<T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { self.pointer.as_mut() }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ObjectType {
    Pair,
    Procedure,
    Symbol,
    Closure,
}

#[repr(C)]
#[derive(Debug)]
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

pub struct Gc {
    next_gc: usize,
    first: Option<NonNull<GcHeader>>,
    grey_stack: Vec<NonNull<GcHeader>>,
    symbols: HashMap<String, GcRef<Symbol>>,
}

impl Gc {
    const HEAP_GROW_FACTOR: usize = 2;

    pub fn new() -> Self {
        Gc {
            next_gc: 1024 * 1024,
            first: None,
            grey_stack: Vec::new(),
            symbols: HashMap::new(),
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

    pub fn intern(&mut self, s: String) -> GcRef<Symbol> {
        match self.symbols.get(s.as_str()) {
            Some(&symbol) => symbol,
            None => {
                let symbol = self.alloc(Symbol::new(s.to_owned()));
                self.symbols.insert(s, symbol);
                symbol
            }
        }
    }

    // Top level mark.
    // This mark root objects only and push them to grey_stack.
    pub fn mark_value(&mut self, value: Value) {
        match value {
            Value::Number(_) => {}
            Value::VMStackPointer(_) => {}
            Value::False => {}
            Value::Undef => {}
            Value::Procedure(_) => {}
            Value::Closure(closure) => {
                self.mark_object(closure);
            }
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
            Value::False => {}
            Value::Undef => {}
            Value::Procedure(_) => {}
            Value::VMStackPointer(_) => {}
            Value::Closure(closure) => {
                for var in &closure.free_vars {
                    self.trace_value(*var);
                }
                self.trace_value(closure.prev);
            }
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
            ObjectType::Procedure => {}
            ObjectType::Closure => {
                //panic!("TODO");
            }
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
