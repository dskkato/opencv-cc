#pragma once

#ifdef __cplusplus
extern "C" {
#endif

void write_as_gray(const char* fname, const int rows, const int cols, unsigned char*buf);
void write_as_rgb(const char* fname, const int rows, const int cols, unsigned char*buf);

#ifdef __cplusplus
}
#endif