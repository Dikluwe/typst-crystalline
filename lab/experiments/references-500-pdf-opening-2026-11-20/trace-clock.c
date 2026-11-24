#define _GNU_SOURCE
#include <dlfcn.h>
#include <fcntl.h>
#include <stdint.h>
#include <stdlib.h>
#include <time.h>
#include <unistd.h>

static int output = -1;
static int (*real_clock)(clockid_t, struct timespec *);

static uint64_t ns(struct timespec value) {
    return (uint64_t)value.tv_sec * 1000000000 + (uint64_t)value.tv_nsec;
}

__attribute__((constructor)) static void start(void) {
    real_clock = dlsym(RTLD_NEXT, "clock_gettime");
    const char *path = getenv("TEKT_TRACE_CLOCK");
    if (!real_clock || !path) _exit(80);
    output = open(path, O_WRONLY | O_CREAT | O_EXCL | O_CLOEXEC, 0600);
    if (output < 0) _exit(81);
}

int clock_gettime(clockid_t clock, struct timespec *value) {
    if (!real_clock) real_clock = dlsym(RTLD_NEXT, "clock_gettime");
    int status = real_clock(clock, value);
    if (status == 0 && clock == CLOCK_REALTIME && output >= 0) {
        struct timespec cpu;
        if (real_clock(CLOCK_THREAD_CPUTIME_ID, &cpu)) _exit(82);
        uint64_t pair[2] = {ns(*value), ns(cpu)};
        if (write(output, pair, sizeof pair) != sizeof pair) _exit(83);
    }
    return status;
}
