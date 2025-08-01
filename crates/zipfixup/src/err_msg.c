#include <stdarg.h>
#include <stdio.h>

// Rust function to further process the buffer
void err_msg_log(int flags, char* path, int line, char* buffer);

// C function to handle varargs
void err_msg_printf(int flags, char* path, int line, char* format, ...)
{
    char buffer[1020];
    va_list args;
    va_start(args, format);
    vsnprintf(buffer, 1020, format, args);
    err_msg_log(flags, path, line, buffer);
    va_end(args);
}
