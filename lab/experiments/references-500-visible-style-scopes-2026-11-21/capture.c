/* Lab-only timing of the snapshot capture call in the verified Tekt ELF. */
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

struct counter {
    _Atomic uint64_t count;
    _Atomic uint64_t ns;
};

struct trace {
    _Atomic uint64_t publication;
    struct counter counters[5];
};

typedef void (*capture_fn)(void *, void *, void *, void *);
static capture_fn capture;
static struct trace *trace;

static uint64_t cpu(void) {
    struct timespec now;
    if (clock_gettime(CLOCK_THREAD_CPUTIME_ID, &now)) _exit(90);
    return (uint64_t)now.tv_sec * 1000000000 + (uint64_t)now.tv_nsec;
}

static void accumulate(unsigned i, uint64_t elapsed) {
    atomic_fetch_add_explicit(&trace->counters[i].count, 1, memory_order_relaxed);
    atomic_fetch_add_explicit(&trace->counters[i].ns, elapsed, memory_order_relaxed);
}

static void timed(void *out, void *engine, void *ctx, void *prev) {
    const uint64_t i = atomic_load_explicit(&trace->publication, memory_order_relaxed);
    if (i < 1 || i > 4) _exit(91);
    uint64_t begin = cpu();
    capture(out, engine, ctx, prev);
    accumulate(i, cpu() - begin);
}

static int install(struct dl_phdr_info *info, size_t size, void *arg) {
    (void)size;
    (void)arg;
    if (*info->dlpi_name) return 0;
    const uintptr_t image = info->dlpi_addr;
    capture = (capture_fn)(image + 0x2035df0);
    unsigned char *site = (void *)(image + 0x1fe392c);
    const unsigned char expected[] = {0xe8, 0xbf, 0x24, 0x05, 0x00};
    if (memcmp(site, expected, sizeof expected)) _exit(92);
    const size_t page_size = sysconf(_SC_PAGESIZE);
    void *hint = (void *)((image + 0x10000000) / page_size * page_size);
    unsigned char *stub = mmap(hint, page_size, PROT_READ | PROT_WRITE,
                               MAP_PRIVATE | MAP_ANONYMOUS | MAP_FIXED_NOREPLACE, -1, 0);
    if (stub == MAP_FAILED) _exit(93);
    /* jmp [rip+0] followed by the destination address. */
    const unsigned char jump[] = {0xff, 0x25, 0, 0, 0, 0};
    const uintptr_t destination = (uintptr_t)timed;
    memcpy(stub, jump, sizeof jump);
    memcpy(stub + sizeof jump, &destination, sizeof destination);
    if (mprotect(stub, page_size, PROT_READ | PROT_EXEC)) _exit(94);
    const int64_t delta = (uintptr_t)stub - ((uintptr_t)site + sizeof expected);
    if (delta < INT32_MIN || delta > INT32_MAX) _exit(95);
    const int32_t relative = delta;
    void *page = (void *)((uintptr_t)site / page_size * page_size);
    if (mprotect(page, page_size, PROT_READ | PROT_WRITE)) _exit(96);
    memcpy(site + 1, &relative, sizeof relative);
    if (mprotect(page, page_size, PROT_READ | PROT_EXEC)) _exit(97);
    return 1;
}

__attribute__((constructor)) static void start(void) {
    const char *path = getenv("TEKT_CAPTURE_TRACE");
    if (!path) _exit(98);
    int fd = open(path, O_RDWR | O_CLOEXEC);
    if (fd < 0) _exit(99);
    trace = mmap(NULL, sizeof *trace, PROT_READ | PROT_WRITE, MAP_SHARED, fd, 0);
    close(fd);
    if (trace == MAP_FAILED || dl_iterate_phdr(install, NULL) != 1) _exit(100);
    for (unsigned i = 0; i < 512; ++i) {
        uint64_t begin = cpu();
        accumulate(0, cpu() - begin);
    }
}
