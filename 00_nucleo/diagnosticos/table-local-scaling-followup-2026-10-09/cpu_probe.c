// Diagnostic-only CPU-time stack samples; not part of the compiler.
#define _GNU_SOURCE
#include <dlfcn.h>
#include <execinfo.h>
#include <fcntl.h>
#include <signal.h>
#include <stdatomic.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <sys/time.h>
#include <unistd.h>

#define MAX_SAMPLES 2048
#define DEPTH 48
struct sample { int depth; void *pcs[DEPTH]; };
static struct sample samples[MAX_SAMPLES];
static _Atomic unsigned int used;
static _Atomic unsigned int delivered;

static void sample_cpu(int signal_number) {
    (void)signal_number;
    atomic_fetch_add_explicit(&delivered, 1, memory_order_relaxed);
    unsigned int index = atomic_fetch_add_explicit(&used, 1, memory_order_relaxed);
    if (index < MAX_SAMPLES)
        samples[index].depth = backtrace(samples[index].pcs, DEPTH);
}

__attribute__((constructor)) static void start(void) {
    void *prime[DEPTH];
    (void)backtrace(prime, DEPTH);
    struct sigaction action = {0};
    action.sa_handler = sample_cpu;
    sigemptyset(&action.sa_mask);
    action.sa_flags = SA_RESTART;
    sigaction(SIGPROF, &action, NULL);
    struct itimerval timer = {0};
    timer.it_interval.tv_usec = 1000;
    timer.it_value.tv_usec = 1000;
    setitimer(ITIMER_PROF, &timer, NULL);
}

__attribute__((destructor)) static void report(void) {
    struct itimerval disabled = {0};
    setitimer(ITIMER_PROF, &disabled, NULL);
    const char *path = getenv("CPU_PROFILE_OUT");
    if (!path) return;
    int fd = open(path, O_CREAT | O_TRUNC | O_WRONLY, 0600);
    if (fd < 0) return;
    FILE *file = fdopen(fd, "w");
    if (!file) { close(fd); return; }
    unsigned int limit = atomic_load_explicit(&used, memory_order_relaxed);
    if (limit > MAX_SAMPLES) limit = MAX_SAMPLES;
    fprintf(file, "delivered=%u samples=%u interval_us=1000\n",
            atomic_load_explicit(&delivered, memory_order_relaxed), limit);
    for (unsigned int i = 0; i < limit; ++i) {
        struct sample *s = &samples[i];
        for (int j = 0; j < s->depth; ++j) {
            Dl_info info;
            if (dladdr(s->pcs[j], &info) && info.dli_fbase)
                fprintf(file, "%s%lx", j ? " " : "", (unsigned long)((uintptr_t)s->pcs[j] - (uintptr_t)info.dli_fbase));
            else fprintf(file, "%s0", j ? " " : "");
        }
        fputc('\n', file);
    }
    fclose(file);
}
