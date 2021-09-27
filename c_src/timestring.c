#include <time.h>

void c_timestring(int hour, char* buffer) {
    time_t rawtime;
    time(&rawtime);
    if (hour)
        rawtime = rawtime + 3600;
    ctime_r(&rawtime, buffer);
}
