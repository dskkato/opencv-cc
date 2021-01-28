#include "wrapper.h"
#include "opencv2/opencv.hpp"

void write_as_gray(const char* fname, const int rows, const int cols, unsigned char*buf)
{
    cv::Mat src(rows, cols, CV_8UC3, buf);
    cv::Mat dst;
    cv::cvtColor(src, dst, cv::COLOR_RGB2GRAY);
    cv::imwrite(fname, dst);
}

void write_as_rgb(const char* fname, const int rows, const int cols, unsigned char*buf)
{
    cv::Mat src(rows, cols, CV_8UC3, buf);
    cv::imwrite(fname, src);
}
