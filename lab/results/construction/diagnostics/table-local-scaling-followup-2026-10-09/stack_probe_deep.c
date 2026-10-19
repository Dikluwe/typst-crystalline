// Diagnostic-only deterministic sample of glibc allocation-request stacks.
#define _GNU_SOURCE
#include <dlfcn.h>
#include <execinfo.h>
#include <fcntl.h>
#include <stdatomic.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>

extern void *__libc_malloc(size_t);
extern void *__libc_calloc(size_t, size_t);
extern void *__libc_realloc(void *, size_t);
extern void *__libc_memalign(size_t, size_t);
extern void __libc_free(void *);

#define MAX_SAMPLES 40000
#define DEPTH 48
struct sample { unsigned long long ordinal, bytes; int depth; void *pcs[DEPTH]; };
static struct sample samples[MAX_SAMPLES];
static _Atomic unsigned long long calls;
static _Atomic unsigned int used;
static __thread int inside;

static void record(size_t bytes) {
    if (inside) return;
    unsigned long long ordinal = atomic_fetch_add_explicit(&calls, 1, memory_order_relaxed) + 1;
    if (ordinal % 64) return;
    unsigned int index = atomic_fetch_add_explicit(&used, 1, memory_order_relaxed);
    if (index >= MAX_SAMPLES) return;
    inside = 1;
    struct sample *s = &samples[index];
    s->ordinal = ordinal;
    s->bytes = bytes;
    s->depth = backtrace(s->pcs, DEPTH);
    inside = 0;
}

void *malloc(size_t n) { void *p = __libc_malloc(n); if (p) record(n); return p; }
void *calloc(size_t n, size_t size) {
    void *p = __libc_calloc(n, size);
    if (p) record(size && n <= SIZE_MAX / size ? n * size : 0);
    return p;
}
void *realloc(void *old, size_t n) { void *p = __libc_realloc(old, n); if (p) record(n); return p; }
int posix_memalign(void **out, size_t align, size_t n) {
    if (align < sizeof(void *) || (align & (align - 1))) return 22;
    void *p = __libc_memalign(align, n); if (!p) return 12;
    *out = p; record(n); return 0;
}
void *aligned_alloc(size_t align, size_t n) {
    void *p = __libc_memalign(align, n); if (p) record(n); return p;
}
void free(void *p) { __libc_free(p); }

__attribute__((destructor)) static void report(void) {
    const char *path = getenv("STACK_PROFILE_OUT");
    if (!path) return;
    int fd = open(path, O_CREAT | O_TRUNC | O_WRONLY, 0600);
    if (fd < 0) return;
    FILE *f = fdopen(fd, "w");
    if (!f) { close(fd); return; }
    unsigned int limit = atomic_load_explicit(&used, memory_order_relaxed);
    if (limit > MAX_SAMPLES) limit = MAX_SAMPLES;
    fprintf(f, "calls=%llu samples=%u interval=64\n", atomic_load(&calls), limit);
    for (unsigned int i = 0; i < limit; i++) {
        struct sample *s = &samples[i];
        fprintf(f, "%llu %llu", s->ordinal, s->bytes);
        for (int j = 0; j < s->depth; j++) {
            Dl_info info;
            if (dladdr(s->pcs[j], &info) && info.dli_fbase)
                fprintf(f, " %lx", (unsigned long)((uintptr_t)s->pcs[j] - (uintptr_t)info.dli_fbase));
            else fprintf(f, " 0");
        }
        fputc('\n', f);
    }
    fclose(f);
}
