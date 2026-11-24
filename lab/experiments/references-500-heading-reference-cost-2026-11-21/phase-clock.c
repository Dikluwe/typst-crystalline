/* Read CPU at existing phase clocks without modifying the executable. */
#define _GNU_SOURCE
#include <dlfcn.h>
#include <fcntl.h>
#include <link.h>
#include <stdatomic.h>
#include <stdint.h>
#include <stdlib.h>
#include <sys/mman.h>
#include <time.h>
#include <unistd.h>
#include <unwind.h>

struct mark { uint64_t publication, pc, cpu_ns; };
struct trace {
    _Atomic uint64_t publication, length;
    struct mark marks[1024];
};
struct range { uintptr_t lo, hi; };
static const struct range ranges[] = {
#include "ranges.inc"
};
static uintptr_t base;
static struct trace *trace;
static int (*system_clock)(clockid_t, struct timespec *);

struct caller { uintptr_t pc; int seen; };

static _Unwind_Reason_Code caller_after_timespec(struct _Unwind_Context *frame, void *state) {
    struct caller *caller = state;
    uintptr_t pc = _Unwind_GetIP(frame) - base;
    if (caller->seen) {
        caller->pc = pc;
        return _URC_END_OF_STACK;
    }
    caller->seen = pc >= TIMESPEC_LO && pc < TIMESPEC_HI;
    return _URC_NO_REASON;
}

int clock_gettime(clockid_t clock, struct timespec *value) {
    if (!system_clock) system_clock = dlsym(RTLD_NEXT, "clock_gettime");
    const uintptr_t return_pc = (uintptr_t)__builtin_return_address(0) - base;
    if (trace && clock == CLOCK_MONOTONIC && return_pc >= TIMESPEC_LO && return_pc < TIMESPEC_HI) {
        struct caller caller = {0};
        _Unwind_Backtrace(caller_after_timespec, &caller);
        for (unsigned i = 0; i < sizeof ranges / sizeof *ranges; ++i) {
            if (caller.pc < ranges[i].lo || caller.pc >= ranges[i].hi) continue;
            struct timespec now;
            if (system_clock(CLOCK_THREAD_CPUTIME_ID, &now)) _exit(90);
            uint64_t position = atomic_fetch_add_explicit(&trace->length, 1, memory_order_relaxed);
            if (position >= sizeof trace->marks / sizeof *trace->marks) _exit(91);
            trace->marks[position] = (struct mark){
                atomic_load_explicit(&trace->publication, memory_order_relaxed), caller.pc,
                (uint64_t)now.tv_sec * 1000000000 + (uint64_t)now.tv_nsec,
            };
            break;
        }
    }
    return system_clock(clock, value);
}

static int image_base(struct dl_phdr_info *info, size_t size, void *arg) {
    (void)size;
    (void)arg;
    if (*info->dlpi_name) return 0;
    base = info->dlpi_addr;
    return 1;
}

__attribute__((constructor)) static void start(void) {
    const char *path = getenv("TEKT_REALIZATION_TRACE");
    if (!path) _exit(92);
    int fd = open(path, O_RDWR | O_CLOEXEC);
    if (fd < 0) _exit(93);
    struct trace *mapped = mmap(NULL, sizeof *mapped, PROT_READ | PROT_WRITE, MAP_SHARED, fd, 0);
    close(fd);
    if (mapped == MAP_FAILED || dl_iterate_phdr(image_base, NULL) != 1) _exit(94);
    trace = mapped;
}
