;; Convert Mosh style op into Rmosh style op.
;; Input: Instructions as Scheme vector.
;;   Example: (CLOSURE 10 0 #f 0 4 (((input string port) 2) a) CONSTANT 3 RETURN 0 DEFINE_GLOBAL a HALT NOP)
;;   Get the instructions by running `gosh vm.scm "compile-file" file-name`.

(import (scheme base) (scheme file) (scheme read) (scheme write) (scheme process-context) (scheme case-lambda
)
        (match) (mosh control) (only (srfi :13) string-delete) (only (mosh) format regexp-replace-all rxmatch) (only (rnrs) string-titlecase))

(define (insn->string insn)
    (string-delete (lambda (c) (equal? c #\_)) (string-titlecase  (symbol->string insn))))

(define (adjust-offset insn* start offset)
  (let loop ([insn* insn*]
             [cur-offset 0]
             [rust-offset 0])
    (format (current-error-port) "cur=~a rust=~a off=~a insn*=~a\n" cur-offset rust-offset offset insn*)
    (cond        
      [(= cur-offset (+ offset 1)) rust-offset]
      [else
        (match insn*
          [((or 'MAKE_CONTINUATION 'CALL 'BRANCH_NOT_EQV 'BRANCH_NOT_GE 'BRANCH_NOT_GT 'BRANCH_NOT_NUMBER_EQUAL 'BRANCH_NOT_LE 'BRANCH_NOT_LT 'BRANCH_NOT_NULL 'BOX 'ASSIGN_LOCAL 'ASSIGN_GLOBAL 'CONSTANT 'DEFINE_GLOBAL 'DISPLAY 'REFER_GLOBAL 'ENTER 'FRAME 'LEAVE 'LET_FRAME 'LOCAL_JMP 'REFER_FREE 'ASSIGN_FREE 'REFER_LOCAL 'RETURN 'TEST) _ . more*)
            (loop more* (+ cur-offset 2) (+ rust-offset 1))]
          [((or 'APPEND2 'MAKE_VECTOR 'VECTOR_LENGTH 'HALT 'SET_CAR 'SET_CDR 'READ_CHAR 'EQ 'PAIR_P 'SYMBOL_P 'NOT 'CAR 'CDR 'CADR 'CONS 'NULL_P 'NUMBER_EQUAL 'NUMBER_GE 'NUMBER_GT 'NUMBER_LE 'NUMBER_LT 'UNDEF 'NOP 'INDIRECT 'NUMBER_ADD 'NUMBER_MUL 'PUSH) . more*)
            (loop more* (+ cur-offset 1) (+ rust-offset 1))]
          [((or 'TAIL_CALL) m n . more*)
            (loop more* (+ cur-offset 3) (+ rust-offset 1))]
          [('CLOSURE _a _b _c _d _e _f . more*) 
            (loop more* (+ cur-offset 7) (+ rust-offset 1))]
          [() (error "never reach here1")]
          [else (error "never reach here2" insn*)])])))

(define rewrite-insn*
  (case-lambda
    [(insn*) (rewrite-insn* insn* 0)]
    [(insn* idx)
      (match insn*
        [('CLOSURE size arg-len optional? num-free-vars _stack-size _src . more*)
          (format #t "            Op::Closure {size: ~a, arg_len: ~a, is_optional_arg: ~a, num_free_vars: ~a},\n"
             (adjust-offset insn* idx size) arg-len (if optional? "true" "false") num-free-vars)
           (rewrite-insn* more* (+ idx 7))]
        [((and (or 'FRAME 'TEST 'LOCAL_JMP 'BRANCH_NOT_GE 'BRANCH_NOT_EQV 'BRANCH_NOT_GT 'BRANCH_NOT_LE 'BRANCH_NOT_LT 'BRANCH_NOT_NUMBER_EQUAL 'BRANCH_NOT_NULL) insn) offset . more*)
          (format #t "            Op::~a(~a),\n" (insn->string insn) (adjust-offset insn* idx offset))
          (rewrite-insn* more* (+ idx 2))] 
        [((and (or 'CONSTANT) insn) #f . more*)
          (format #t "            Op::~a(Object::False),\n" (insn->string insn))
          (rewrite-insn* more* (+ idx 2))]      
        [((and (or 'CONSTANT) insn) #t . more*)
          (format #t "            Op::~a(Object::True),\n" (insn->string insn))
          (rewrite-insn* more* (+ idx 2))]              
        [((and (or 'CONSTANT) insn) () . more*)
          (format #t "            Op::~a(Object::Nil),\n" (insn->string insn))
          (rewrite-insn* more* (+ idx 2))]                                    
        [((and (or 'CONSTANT) insn) (? number? n) . more*)
          (format #t "            Op::~a(Object::Number(~a)),\n" (insn->string insn) n)
          (rewrite-insn* more* (+ idx 2))]
        [((and (or 'CONSTANT) insn) (? char? c) . more*)
          (format #t "            Op::~a(Object::Char('~a')),\n" (insn->string insn) c)
            (rewrite-insn* more* (+ idx 2))]               
        [((and (or 'CONSTANT) insn) ((? number? n)) . more*)
          (format #t "            Op::~a(vm.gc.cons(Object::Number(~a), Object::Nil)),\n" (insn->string insn) n)
            (rewrite-insn* more* (+ idx 2))]       
        [((and (or 'CONSTANT) insn) (((? number? n))) . more*)
          (format #t "            Op::~a(vm.gc.list1(vm.gc.list1(Object::Number(~a)))),\n" (insn->string insn) n)
            (rewrite-insn* more* (+ idx 2))]                   
        [((and (or 'CONSTANT) insn) ((? number? a) (? number? b)) . more*)
          (format #t "            Op::~a(vm.gc.list2(Object::Number(~a), Object::Number(~a))),\n" (insn->string insn) a b)
            (rewrite-insn* more* (+ idx 2))]       
        [((and (or 'CONSTANT) insn) ((? string? a) (? string? b)) . more*)
          (format #t "            Op::~a(vm.gc.list2(vm.gc.new_string(~s), vm.gc.new_string(~s))),\n" (insn->string insn) a b)
            (rewrite-insn* more* (+ idx 2))]                  
        [((and (or 'CONSTANT) insn) ((? number? a) (? number? b) (? number? c)) . more*)
          (format #t "            Op::~a(vm.gc.list3(Object::Number(~a), Object::Number(~a), Object::Number(~a))),\n" (insn->string insn) a b c)
            (rewrite-insn* more* (+ idx 2))]                             
        [((and (or 'CONSTANT) insn) (? symbol? n) . more*)
          (format #t "            Op::~a(vm.gc.symbol_intern(\"~a\")),\n" (insn->string insn) n)
          (rewrite-insn* more* (+ idx 2))]         
        [((and (or 'CONSTANT) insn) (? string? s) . more*)
          (format #t "            Op::~a(vm.gc.new_string(~s)),\n" (insn->string insn) s)
          (rewrite-insn* more* (+ idx 2))]              
        [((and (or 'TAIL_CALL) insn) m n . more*)
          (format #t "            Op::~a(~a, ~a),\n" (insn->string insn) m n)
          (rewrite-insn* more* (+ idx 3))]              
        [((and (or 'ENTER 'BOX 'MAKE_CONTINUATION) insn) n . more*)
          (format #t "            Op::~a(~a),\n" (insn->string insn) n)
          (rewrite-insn* more* (+ idx 2))]                
        [((and (or 'ASSIGN_GLOBAL 'DEFINE_GLOBAL 'REFER_GLOBAL) insn) (? symbol? n) . more*)
          (format #t "            Op::~a(vm.gc.intern(\"~a\")),\n" (insn->string insn) n)
          (rewrite-insn* more* (+ idx 2))]          
        [((and (or 'CALL 'DISPLAY 'LEAVE 'LET_FRAME 'RETURN 'ASSIGN_FREE 'REFER_FREE 'REFER_LOCAL 'ASSIGN_LOCAL 'FRAME 'REFER_LOCAL) insn) n . more*)
          (format #t "            Op::~a(~a),\n" (insn->string insn) n)
          (rewrite-insn* more* (+ idx 2))]
        [((and (or 'APPEND2 'MAKE_VECTOR 'VECTOR_LENGTH 'READ_CHAR 'HALT 'SET_CAR 'SET_CDR 'EQ 'PAIR_P 'SYMBOL_P 'NOT 'CAR 'CDR 'CADR 'CONS 'NUMBER_EQUAL 'NUMBER_GE 'NUMBER_GT 'NUMBER_LE 'NUMBER_LT 'NOP 'NULL_P 'INDIRECT 'PUSH 'NUMBER_ADD 'NUMBER_MUL 'UNDEF) insn) . more*)
          (format #t "            Op::~a,\n" (insn->string insn))
          (rewrite-insn* more*  (+ idx 1))]
        [() #f]
        [else (display insn*)])]))

(define (file->sexp* file)
  (call-with-input-file file
    (lambda (p)
      (let loop ([sexp (read p)]
                 [sexp* '()])
        (cond
         [(eof-object? sexp) (reverse sexp*)]
         [else
          (loop (read p) (cons sexp sexp*))])))))

(define (expected->rust expected)
  (match expected
    [(? char? c)
      (format "Object::Char('~a')" c)]    
    [(? symbol? s)
      (format "vm.gc.symbol_intern(\"~a\")" s)]
    [(? string? s)
      (format "vm.gc.new_string(\"~a\")" s)]
    ['undef "Object::Undef"]
    [#t "Object::True"]
    [#f "Object::False"]
    [() "Object::Nil"]
    [(a . b)
      (format "Object::Pair(vm.gc.alloc(Pair::new(~a, ~a))" (expected->rust a) (expected->rust b))]
    [(? number? n) (format "Object::Number(~a)" n)]
    [else (error "expected->rust" expected)]))

(define (main args)
  (let* ([op-file (cadr args)]
         [scm-file (regexp-replace-all #/\.op$/ op-file ".scm")]
         [test-name ((rxmatch #/([^\/]+)\.op$/ op-file) 1)]
         [expr* (file->sexp* scm-file)]
         [sexp* (file->sexp* op-file)])     
    (match expr*
      [(expr expected size)
        (cond
          [(pair? expected)
        (format #t "
    // ~s => ~s
    #[test]
    fn test_~a() {
        let mut vm = Vm::new();        
        let ops = vec![\n" expr expected test-name)        
        (let ([insn* (vector->list (car sexp*))])
          (rewrite-insn* insn*)
          (format #t "        ];
        test_ops_with_size_as_str(&mut vm, ops, \"~a\", ~a);
    }\n" expected size))]        
          [(string? expected)
        (format #t "
    // ~s => ~s
    #[test]
    fn test_~a() {
        let mut vm = Vm::new();        
        let ops = vec![\n" expr expected test-name)        
        (let ([insn* (vector->list (car sexp*))])
          (rewrite-insn* insn*)
          (format #t "        ];
        test_ops_with_size_as_str(&mut vm, ops, \"\\\"~a\\\"\", ~a);
    }\n" expected size))]
          [else
        (format #t "
    // ~s => ~s
    #[test]
    fn test_~a() {
        let mut vm = Vm::new();        
        let ops = vec![\n" expr expected test-name)        
        (let ([insn* (vector->list (car sexp*))]
              [expected (expected->rust expected)])
          (rewrite-insn* insn*)
          (format #t "        ];
        let expected = ~a;
        test_ops_with_size(&mut vm, ops, expected, ~a);
    }\n" expected size))])]
      [else (write sexp*)])))

(main (command-line))