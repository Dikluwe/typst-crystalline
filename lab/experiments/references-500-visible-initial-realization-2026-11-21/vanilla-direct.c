/* Time the document realization and paged index in the pinned vanilla ELF. */
#define _GNU_SOURCE
#include <fcntl.h>
#include <link.h>
#include <stdatomic.h>
#include <stdint.h>
#include <stdlib.h>
#include <sys/mman.h>
#include <time.h>
#include <unistd.h>

struct cell {
    _Atomic uint64_t calls;
    _Atomic uint64_t ns;
    _Atomic uint64_t first;
};
struct trace {
    _Atomic uint64_t publication;
    struct cell calibrate;
    struct cell counters[4][2];
};
struct registers { uintptr_t a, b; };
typedef struct registers (*root_fn)(uintptr_t, uintptr_t, uintptr_t, uintptr_t,
                                     uintptr_t, uintptr_t, uintptr_t);
typedef struct registers (*index_fn)(uintptr_t, uintptr_t, uintptr_t);
static struct trace *trace;
static root_fn root;
static index_fn index_pages;
static uintptr_t base;

static uint64_t cpu(void) {
    struct timespec now;
    if (clock_gettime(CLOCK_THREAD_CPUTIME_ID, &now)) _exit(80);
    return (uint64_t)now.tv_sec * 1000000000 + (uint64_t)now.tv_nsec;
}

static void record(struct cell *cell, uint64_t value) {
    if (!atomic_fetch_add_explicit(&cell->calls, 1, memory_order_relaxed))
        atomic_store_explicit(&cell->first, value, memory_order_relaxed);
    atomic_fetch_add_explicit(&cell->ns, value, memory_order_relaxed);
}

static struct cell *cell(unsigned i) {
    uint64_t publication = atomic_load_explicit(&trace->publication, memory_order_relaxed);
    if (publication < 1 || publication > 4) _exit(81);
    return &trace->counters[publication - 1][i];
}

static struct registers timed_root(uintptr_t a, uintptr_t b, uintptr_t c, uintptr_t d,
                                  uintptr_t e, uintptr_t f, uintptr_t g) {
    if ((uintptr_t)__builtin_return_address(0) != base + 0x265bda6)
        return root(a, b, c, d, e, f, g);
    uint64_t begin = cpu();
    struct registers result = root(a, b, c, d, e, f, g);
    record(cell(0), cpu() - begin);
    return result;
}

static struct registers timed_index(uintptr_t a, uintptr_t b, uintptr_t c) {
    if ((uintptr_t)__builtin_return_address(0) != base + 0x26a710d)
        return index_pages(a, b, c);
    uint64_t begin = cpu();
    struct registers result = index_pages(a, b, c);
    record(cell(1), cpu() - begin);
    return result;
}

static uintptr_t replace(const struct dl_phdr_info *info, uintptr_t offset,
                         uintptr_t original, uintptr_t dest) {
    uintptr_t *slot = (void *)(base + offset);
    for (ElfW(Half) i = 0; i < info->dlpi_phnum; ++i) {
        const ElfW(Phdr) *seg = &info->dlpi_phdr[i];
        if (seg->p_type != PT_GNU_RELRO ||
            (uintptr_t)slot < base + seg->p_vaddr ||
            (uintptr_t)slot + sizeof *slot > base + seg->p_vaddr + seg->p_memsz) continue;
        uintptr_t prior = *slot;
        if (prior != base + original) _exit(82);
        const size_t size = sysconf(_SC_PAGESIZE);
        void *page = (void *)((uintptr_t)slot / size * size);
        if (mprotect(page, size, PROT_READ | PROT_WRITE)) _exit(83);
        *slot = dest;
        if (mprotect(page, size, PROT_READ)) _exit(84);
        return prior;
    }
    _exit(85);
}

static int install(struct dl_phdr_info *info, size_t size, void *arg) {
    (void)size;
    (void)arg;
    if (*info->dlpi_name) return 0;
    base = info->dlpi_addr;
    root = (root_fn)replace(info, 0x31e0fb0, 0x2cc01a0, (uintptr_t)timed_root);
    index_pages = (index_fn)replace(info, 0x31e25b0, 0x25f3980, (uintptr_t)timed_index);
    return 1;
}

__attribute__((constructor)) static void start(void) {
    const char *path = getenv("TEKT_REALIZATION_TRACE");
    if (!path) _exit(86);
    int fd = open(path, O_RDWR | O_CLOEXEC);
    if (fd < 0) _exit(87);
    trace = mmap(NULL, sizeof *trace, PROT_READ | PROT_WRITE, MAP_SHARED, fd, 0);
    close(fd);
    if (trace == MAP_FAILED || dl_iterate_phdr(install, NULL) != 1) _exit(88);
    for (unsigned i = 0; i < 256; ++i) {
        uint64_t begin = cpu();
        record(&trace->calibrate, cpu() - begin);
    }
}
