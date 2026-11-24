/* Lab-only timing of the pinned x86-64 ELF's SetRule call to styled_with_map. */
#define _GNU_SOURCE
#include <fcntl.h>
#include <link.h>
#include <stdint.h>
#include <stdlib.h>
#include <sys/mman.h>
#include <time.h>
#include <unistd.h>

typedef void (*wrap_call)(void *, void *, uintptr_t, uintptr_t);
static wrap_call styled;
static uintptr_t base;
static int output;

static uint64_t cpu(void) {
    struct timespec now;
    if (clock_gettime(CLOCK_THREAD_CPUTIME_ID, &now)) _exit(90);
    return (uint64_t)now.tv_sec * 1000000000 + (uint64_t)now.tv_nsec;
}

static void record(uint64_t kind, uint64_t begin, uint64_t end) {
    uint64_t row[] = {kind, begin, end};
    if (write(output, row, sizeof row) != sizeof row) _exit(91);
}

static void timed(void *out, void *content, uintptr_t a, uintptr_t b) {
    if ((uintptr_t)__builtin_return_address(0) != base + 0x24dbb2c) {
        styled(out, content, a, b);
        return;
    }
    uint64_t begin = cpu();
    styled(out, content, a, b);
    record(1, begin, cpu());
}

static int install(struct dl_phdr_info *info, size_t size, void *arg) {
    (void)size;
    (void)arg;
    if (*info->dlpi_name) return 0;
    base = info->dlpi_addr;
    wrap_call *slot = (void *)(base + 0x31e1bf0);
    for (ElfW(Half) i = 0; i < info->dlpi_phnum; i++) {
        const ElfW(Phdr) *segment = &info->dlpi_phdr[i];
        if (segment->p_type != PT_GNU_RELRO ||
            (uintptr_t)slot < base + segment->p_vaddr ||
            (uintptr_t)slot + sizeof *slot > base + segment->p_vaddr + segment->p_memsz) continue;
        if ((uintptr_t)*slot != base + 0x287f458) _exit(92);
        styled = *slot;
        size_t size = sysconf(_SC_PAGESIZE);
        void *page = (void *)((uintptr_t)slot / size * size);
        if (mprotect(page, size, PROT_READ | PROT_WRITE)) _exit(93);
        *slot = timed;
        if (mprotect(page, size, PROT_READ)) _exit(94);
        return 1;
    }
    _exit(95);
}

__attribute__((constructor)) static void start(void) {
    const char *path = getenv("TEKT_VANILLA_WRAP");
    if (!path) _exit(96);
    output = open(path, O_WRONLY | O_CREAT | O_EXCL | O_CLOEXEC, 0600);
    if (output < 0) _exit(97);
    /* The read-pair baseline measures the same clock without application work. */
    for (int i = 0; i < 128; i++) {
        uint64_t begin = cpu();
        record(0, begin, cpu());
    }
    if (dl_iterate_phdr(install, NULL) != 1) _exit(98);
}
