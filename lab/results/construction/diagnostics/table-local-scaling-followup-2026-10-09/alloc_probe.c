// Diagnostic-only glibc allocation request counter. Not a live-heap profiler.
#define _GNU_SOURCE
#include <errno.h>
#include <fcntl.h>
#include <stdatomic.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>

extern void *__libc_malloc(size_t);
extern void *__libc_calloc(size_t, size_t);
extern void *__libc_realloc(void *, size_t);
extern void __libc_free(void *);
extern void *__libc_memalign(size_t, size_t);

static _Atomic unsigned long long malloc_calls, calloc_calls, realloc_calls;
static _Atomic unsigned long long memalign_calls, free_calls, requested_bytes;
static void count(_Atomic unsigned long long *counter, size_t bytes) {
    atomic_fetch_add_explicit(counter, 1, memory_order_relaxed);
    atomic_fetch_add_explicit(&requested_bytes, (unsigned long long)bytes, memory_order_relaxed);
}
void *malloc(size_t size) {
    void *p = __libc_malloc(size);
    if (p) count(&malloc_calls, size);
    return p;
}
void *calloc(size_t n, size_t size) {
    void *p = __libc_calloc(n, size);
    if (p) count(&calloc_calls, size == 0 || n <= SIZE_MAX / size ? n * size : 0);
    return p;
}
void *realloc(void *old, size_t size) {
    void *p = __libc_realloc(old, size);
    if (p) count(&realloc_calls, size);
    return p;
}
void free(void *p) {
    if (p) atomic_fetch_add_explicit(&free_calls, 1, memory_order_relaxed);
    __libc_free(p);
}
int posix_memalign(void **out, size_t alignment, size_t size) {
    if (alignment < sizeof(void *) || (alignment & (alignment - 1))) return EINVAL;
    void *p = __libc_memalign(alignment, size);
    if (!p) return ENOMEM;
    *out = p;
    count(&memalign_calls, size);
    return 0;
}
void *aligned_alloc(size_t alignment, size_t size) {
    if (!alignment || (alignment & (alignment - 1)) || size % alignment) { errno = EINVAL; return NULL; }
    void *p = __libc_memalign(alignment, size);
    if (p) count(&memalign_calls, size);
    return p;
}
__attribute__((destructor)) static void report(void) {
    const char *path = getenv("ALLOC_PROFILE_OUT");
    if (!path) return;
    int fd = open(path, O_CREAT | O_TRUNC | O_WRONLY, 0600);
    if (fd < 0) return;
    char line[512];
    int n = snprintf(line, sizeof(line),
        "{\"malloc_calls\":%llu,\"calloc_calls\":%llu,\"realloc_calls\":%llu,"
        "\"memalign_calls\":%llu,\"free_calls\":%llu,\"requested_bytes\":%llu}\n",
        atomic_load(&malloc_calls), atomic_load(&calloc_calls), atomic_load(&realloc_calls),
        atomic_load(&memalign_calls), atomic_load(&free_calls), atomic_load(&requested_bytes));
    if (n > 0 && n < (int)sizeof(line)) {
        ssize_t written = write(fd, line, (size_t)n);
        (void)written;
    }
    close(fd);
}
