#ifndef MYCLIB_H
#define MYCLIB_H

#ifdef __cplusplus
extern "C" {
#endif

// 简单函数：两个整数相加
int c_add(int a, int b);

// 打印传入的 C 字符串（库内部打印）
void c_greet(const char *s);

#ifdef __cplusplus
}
#endif

#endif // MYCLIB_H
