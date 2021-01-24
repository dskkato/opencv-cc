#pragma once

#ifdef __cplusplus
extern "C" {
#endif

void write_from_opencv(const char* fname, const int rows, const int cols, unsigned char*buf);

#ifdef __cplusplus
}
#endif