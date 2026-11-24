/* Laboratory timing around verified call instructions in one Tekt ELF. */
#define _GNU_SOURCE
#include <fcntl.h>
#include <link.h>
#include <stdatomic.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>
#include <sys/mman.h>
#include <time.h>
#include <unistd.h>

enum { OBSERVE, FINISH, NORMALIZE, COLUMNS, INDEX, RECIPES, SCOPE_COUNT };
struct cell {
    _Atomic uint64_t calls;
    _Atomic uint64_t ns;
};
struct trace {
    _Atomic uint64_t publication;
    struct cell calibrate;
    struct cell counters[4][SCOPE_COUNT];
};
struct registers { uintptr_t a, b; };
typedef struct registers (*call_fn)(uintptr_t, uintptr_t, uintptr_t,
                                     uintptr_t, uintptr_t, uintptr_t);
struct point { unsigned scope; unsigned long site, callee, got; };
static const struct point points[] = {
#include "points.inc"
};
static call_fn originals[SCOPE_COUNT];
static struct trace *trace;

static uint64_t cpu(void) {
    struct timespec now;
    if (clock_gettime(CLOCK_THREAD_CPUTIME_ID, &now)) _exit(90);
    return (uint64_t)now.tv_sec * 1000000000 + (uint64_t)now.tv_nsec;
}

static void record(struct cell *cell, uint64_t elapsed) {
    atomic_fetch_add_explicit(&cell->calls, 1, memory_order_relaxed);
    atomic_fetch_add_explicit(&cell->ns, elapsed, memory_order_relaxed);
}

#define WRAPPER(NAME, ID) \
static struct registers NAME(uintptr_t a, uintptr_t b, uintptr_t c, \
                             uintptr_t d, uintptr_t e, uintptr_t f) { \
    uint64_t publication = atomic_load_explicit(&trace->publication, memory_order_relaxed); \
    if (publication < 1 || publication > 4 || !originals[ID]) _exit(91); \
    uint64_t begin = cpu(); \
    struct registers result = originals[ID](a, b, c, d, e, f); \
    record(&trace->counters[publication - 1][ID], cpu() - begin); \
    return result; \
}
WRAPPER(observe, OBSERVE)
WRAPPER(finish, FINISH)
WRAPPER(normalize, NORMALIZE)
WRAPPER(columns, COLUMNS)
WRAPPER(index_body, INDEX)
WRAPPER(recipes, RECIPES)

static const call_fn wrappers[] = {observe, finish, normalize, columns, index_body, recipes};

static int install(struct dl_phdr_info *info, size_t size, void *unused) {
    (void)size;
    (void)unused;
    if (*info->dlpi_name) return 0;
    const uintptr_t image = info->dlpi_addr;
    const size_t page_size = sysconf(_SC_PAGESIZE);
    void *hint = (void *)((image + 0x10000000) / page_size * page_size);
    unsigned char *stubs = mmap(hint, page_size, PROT_READ | PROT_WRITE,
                                MAP_PRIVATE | MAP_ANONYMOUS | MAP_FIXED_NOREPLACE, -1, 0);
    if (stubs == MAP_FAILED) _exit(92);
    for (unsigned i = 0; i < sizeof points / sizeof *points; ++i) {
        struct point p = points[i];
        unsigned char *site = (void *)(image + p.site);
        int32_t offset;
        if (p.got) {
            memcpy(&offset, site + 2, sizeof offset);
            if (site[0] != 0xff || site[1] != 0x15 ||
                (int64_t)p.site + 6 + offset != (int64_t)p.got ||
                *(uintptr_t *)(image + p.got) != image + p.callee) _exit(93);
        } else {
            memcpy(&offset, site + 1, sizeof offset);
            if (*site != 0xe8 || (int64_t)p.site + 5 + offset != (int64_t)p.callee) _exit(93);
        }
        if (originals[p.scope]) _exit(94);
        originals[p.scope] = (call_fn)(image + p.callee);
        unsigned char *stub = stubs + 16 * i;
        const unsigned char jump[] = {0xff, 0x25, 0, 0, 0, 0};
        const uintptr_t target = (uintptr_t)wrappers[p.scope];
        memcpy(stub, jump, sizeof jump);
        memcpy(stub + sizeof jump, &target, sizeof target);
        int64_t delta = (int64_t)(uintptr_t)stub - (int64_t)((uintptr_t)site + 5);
        if (delta < INT32_MIN || delta > INT32_MAX) _exit(95);
        offset = delta;
        void *page = (void *)((uintptr_t)site / page_size * page_size);
        if (mprotect(page, page_size, PROT_READ | PROT_WRITE)) _exit(96);
        if (p.got) {
            site[0] = 0xe8;
            site[5] = 0x90;
        }
        memcpy(site + 1, &offset, sizeof offset);
        if (mprotect(page, page_size, PROT_READ | PROT_EXEC)) _exit(97);
    }
    if (mprotect(stubs, page_size, PROT_READ | PROT_EXEC)) _exit(98);
    return 1;
}

__attribute__((constructor)) static void start(void) {
    const char *path = getenv("TEKT_REALIZATION_TRACE");
    if (!path) _exit(99);
    int fd = open(path, O_RDWR | O_CLOEXEC);
    if (fd < 0) _exit(100);
    trace = mmap(NULL, sizeof *trace, PROT_READ | PROT_WRITE, MAP_SHARED, fd, 0);
    close(fd);
    if (trace == MAP_FAILED || dl_iterate_phdr(install, NULL) != 1) _exit(101);
    for (unsigned i = 0; i < 256; ++i) {
        uint64_t begin = cpu();
        record(&trace->calibrate, cpu() - begin);
    }
}
