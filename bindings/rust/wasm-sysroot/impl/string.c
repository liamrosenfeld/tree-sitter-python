#pragma once

#include <string.h>

void* memcpy(void* dest, const void* src, unsigned long n) {
    return __builtin_memcpy(dest, src, n);
}
