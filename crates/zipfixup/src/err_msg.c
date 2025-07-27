#include <stdarg.h>
#include <stdio.h>

void recoil_log_formatted(int flags, char* path, int line, char* buffer);

void err_msg(int flags, char* path, int line, char* format, ...)
{
    char buffer[1020];
    va_list args;
    va_start(args, format);
    vsnprintf(buffer, 1020, format, args);
    recoil_log_formatted(flags, path, line, buffer);
    va_end(args);
}
