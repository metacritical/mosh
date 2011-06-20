(define-c-defined-const AF_INET)
(define-c-defined-const AF_INET6)
(define-c-defined-const AF_UNSPEC)
(define-c-defined-const SOCK_STREAM)
(define-c-defined-const SOCK_DGRAM)
(define-c-defined-const SOCK_RAW)
(define-c-defined-const AI_ADDRCONFIG)
(define-c-defined-const AI_ALL)
(define-c-defined-const AI_CANONNAME)
(define-c-defined-const AI_NUMERICHOST)
(define-c-defined-const AI_NUMERICSERV)
(define-c-defined-const AI_PASSIVE)
(define-c-defined-const AI_V4MAPPED)
(define-c-defined-const IPPROTO_TCP)
(define-c-defined-const IPPROTO_UDP)
(define-c-defined-const IPPROTO_RAW)
(define-c-defined-const SHUT_RD)
(define-c-defined-const SHUT_WR)
(define-c-defined-const SHUT_RDWR)
;; signals
(define-c-defined-const SIGHUP)
(define-c-defined-const SIGINT)
(define-c-defined-const SIGQUIT)
(define-c-defined-const SIGILL)
(define-c-defined-const SIGTRAP)
(define-c-defined-const SIGABRT)
(define-c-defined-const SIGEMT)
(define-c-defined-const SIGFPE)
(define-c-defined-const SIGKILL)
(define-c-defined-const SIGBUS)
(define-c-defined-const SIGSEGV)
(define-c-defined-const SIGSYS)
(define-c-defined-const SIGPIPE)
(define-c-defined-const SIGALRM)
(define-c-defined-const SIGTERM)
(define-c-defined-const SIGURG)
(define-c-defined-const SIGTHR)
(define-c-defined-const SIGUSR1)
(define-c-defined-const SIGUSR2)
(define-c-defined-const SIGCHLD)
(define-c-defined-const SIGCONT)
(define-c-defined-const SIGSTOP)
(define-c-defined-const SIGTSTP)
(define-c-defined-const SIGTTIN)
(define-c-defined-const SIGTTOU)
(define-c-defined-const SIGPOLL)
(define-c-defined-const SIGPROF)
(define-c-defined-const SIGSYS)
(define-c-defined-const SIGVTALRM)
(define-c-defined-const SIGXCPU)
(define-c-defined-const SIGXFSZ)
(define-c-defined-const SIGIO)
(define-c-defined-const SIGPWR)
(define-c-defined-const SIGINFO)
(define-c-defined-const SIGLOST)
(define-c-defined-const SIGWINCH)
(define-c-defined-const-uint32_t MSG_OK)
(define-c-defined-const-uint32_t MSG_STARTED)
(define-c-defined-const-uint32_t MSG_INTERRUPTED)
(define-c-defined-const-uint32_t MSG_TIMER)
(define-c-defined-const-uint32_t MSG_READ_READY)
(define-c-defined-const-uint32_t MSG_WRITE_READY)
(define-c-defined-const-uint32_t MSG_TEXT)
(define-size-of char)
(define-size-of bool)
(define-size-of short)
(define-size-of unsigned short)
(define-size-of int)
(define-size-of unsigned int)
(define-size-of long)
(define-size-of unsigned long)
(define-size-of unsigned long long)
(define-size-of long long)
(define-size-of void*)
(define-size-of size_t)
(define-size-of float)
(define-size-of double)
(define-align-of char)
(define-align-of bool)
(define-align-of short)
(define-align-of unsigned short)
(define-align-of int)
(define-align-of unsigned int)
(define-align-of long)
(define-align-of unsigned long)
(define-align-of unsigned long long)
(define-align-of long long)
(define-align-of void*)
(define-align-of size_t)
(define-align-of float)
(define-align-of double)
(define-align-of int8_t)
(define-align-of int16_t)
(define-align-of int32_t)
(define-align-of int64_t)

; POSIX errno.h [not including objects marked obsolescent in POSIX 2008]
(define-c-defined-const E2BIG)
(define-c-defined-const EACCES)
(define-c-defined-const EADDRINUSE)
(define-c-defined-const EADDRNOTAVAIL)
(define-c-defined-const EAFNOSUPPORT)
(define-c-defined-const EAGAIN)
(define-c-defined-const EALREADY)
(define-c-defined-const EBADF)
(define-c-defined-const EBADMSG)
(define-c-defined-const EBUSY)
(define-c-defined-const ECANCELED)
(define-c-defined-const ECHILD)
(define-c-defined-const ECONNABORTED)
(define-c-defined-const ECONNREFUSED)
(define-c-defined-const ECONNRESET)
(define-c-defined-const EDEADLK)
(define-c-defined-const EDESTADDRREQ)
(define-c-defined-const EDOM)
(define-c-defined-const EDQUOT)
(define-c-defined-const EEXIST)
(define-c-defined-const EFAULT)
(define-c-defined-const EFBIG)
(define-c-defined-const EHOSTUNREACH)
(define-c-defined-const EIDRM)
(define-c-defined-const EILSEQ)
(define-c-defined-const EINPROGRESS)
(define-c-defined-const EINTR)
(define-c-defined-const EINVAL)
(define-c-defined-const EIO)
(define-c-defined-const EISCONN)
(define-c-defined-const EISDIR)
(define-c-defined-const ELOOP)
(define-c-defined-const EMFILE)
(define-c-defined-const EMLINK)
(define-c-defined-const EMSGSIZE)
(define-c-defined-const EMULTIHOP)
(define-c-defined-const ENAMETOOLONG)
(define-c-defined-const ENETDOWN)
(define-c-defined-const ENETRESET)
(define-c-defined-const ENETUNREACH)
(define-c-defined-const ENFILE)
(define-c-defined-const ENOBUFS)
(define-c-defined-const ENODEV)
(define-c-defined-const ENOENT)
(define-c-defined-const ENOEXEC)
(define-c-defined-const ENOLCK)
(define-c-defined-const ENOLINK)
(define-c-defined-const ENOMEM)
(define-c-defined-const ENOMSG)
(define-c-defined-const ENOPROTOOPT)
(define-c-defined-const ENOSPC)
(define-c-defined-const ENOSR)
(define-c-defined-const ENOSTR)
(define-c-defined-const ENOSYS)
(define-c-defined-const ENOTCONN)
(define-c-defined-const ENOTDIR)
(define-c-defined-const ENOTEMPTY)
(define-c-defined-const ENOTRECOVERABLE)
(define-c-defined-const ENOTSOCK)
(define-c-defined-const ENOTSUP)
(define-c-defined-const ENOTTY)
(define-c-defined-const ENXIO)
(define-c-defined-const EOPNOTSUPP)
(define-c-defined-const EOVERFLOW)
(define-c-defined-const EOWNERDEAD)
(define-c-defined-const EPERM)
(define-c-defined-const EPIPE)
(define-c-defined-const EPROTO)
(define-c-defined-const EPROTONOSUPPORT)
(define-c-defined-const EPROTOTYPE)
(define-c-defined-const ERANGE)
(define-c-defined-const EROFS)
(define-c-defined-const ESPIPE)
(define-c-defined-const ESRCH)
(define-c-defined-const ESTALE)
(define-c-defined-const ETIME)
(define-c-defined-const ETIMEDOUT)
(define-c-defined-const ETXTBSY)
(define-c-defined-const EWOULDBLOCK)
(define-c-defined-const EXDEV)

; confstr requests
(define-c-defined-const _CS_PATH)
(define-c-defined-const _CS_POSIX_V7_ILP32_OFF32_CFLAGS)
(define-c-defined-const _CS_POSIX_V7_ILP32_OFF32_LDFLAGS)
(define-c-defined-const _CS_POSIX_V7_ILP32_OFF32_LIBS)
(define-c-defined-const _CS_POSIX_V7_ILP32_OFFBIG_CFLAGS)
(define-c-defined-const _CS_POSIX_V7_ILP32_OFFBIG_LDFLAGS)
(define-c-defined-const _CS_POSIX_V7_ILP32_OFFBIG_LIBS)
(define-c-defined-const _CS_POSIX_V7_LP64_OFF64_CFLAGS)
(define-c-defined-const _CS_POSIX_V7_LP64_OFF64_LDFLAGS)
(define-c-defined-const _CS_POSIX_V7_LP64_OFF64_LIBS)
(define-c-defined-const _CS_POSIX_V7_LPBIG_OFFBIG_CFLAGS)
(define-c-defined-const _CS_POSIX_V7_LPBIG_OFFBIG_LDFLAGS)
(define-c-defined-const _CS_POSIX_V7_LPBIG_OFFBIG_LIBS)
(define-c-defined-const _CS_POSIX_V7_THREADS_CFLAGS)
(define-c-defined-const _CS_POSIX_V7_THREADS_LDFLAGS)
(define-c-defined-const _CS_POSIX_V7_WIDTH_RESTRICTED_ENVS)
(define-c-defined-const _CS_V7_ENV)
