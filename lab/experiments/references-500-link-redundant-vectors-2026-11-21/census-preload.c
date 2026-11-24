// Observe only the magic-prefixed copy performed by the instrumented reducer.
#define _GNU_SOURCE
#include <dlfcn.h>
#include <errno.h>
#include <fcntl.h>
#include <stdint.h>
#include <stdlib.h>
#include <unistd.h>

static void *(*libc_copy)(void *, const void *, size_t);
static int trace_fd = -1;

__attribute__((constructor)) static void begin(void) {
    libc_copy = dlsym(RTLD_NEXT, "memcpy");
    const char *path = getenv("TEKT_LINK_VECTOR_CENSUS");
    if (path != NULL) {
        trace_fd = open(path, O_WRONLY | O_APPEND | O_CREAT | O_CLOEXEC, 0600);
        if (trace_fd < 0) abort();
    }
}

static uint64_t read_le(const unsigned char *bytes) {
    uint64_t result = 0;
    for (int i = 0; i < 8; i++) {
        result |= (uint64_t)bytes[i] << (8 * i);
    }
    return result;
}

void *memcpy(void *destination, const void *source, size_t size) {
    const unsigned char *bytes = source;
    if (trace_fd >= 0 && size == 112 && bytes[0] == 0x1f &&
        read_le(bytes) == UINT64_C(0xb27ce64254ba881f) &&
        read_le(bytes + 8) == UINT64_C(0x2710a95ebc07f361)) {
        ssize_t written;
        do {
            written = write(trace_fd, source, size);
        } while (written < 0 && errno == EINTR);
        if (written != (ssize_t)size) abort();
    }
    if (libc_copy != NULL) return libc_copy(destination, source, size);
    unsigned char *out = destination;
    for (size_t i = 0; i < size; i++) out[i] = bytes[i];
    return destination;
}
