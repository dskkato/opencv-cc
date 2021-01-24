#include "wrapper.h"
#include "opencv2/imgproc.hpp"
#include "opencv2/imgcodecs.hpp"

#ifdef __cplusplus
extern "C" {
#endif

void write_from_opencv(const char* fname, const int rows, const int cols, unsigned char*buf)
{
    cv::Mat src(rows, cols, CV_8UC3, buf);
    cv::Mat dst;
    cv::cvtColor(src, dst, cv::COLOR_RGB2GRAY);
    cv::imwrite(fname, dst);
}

#ifdef __cplusplus
}
#endif