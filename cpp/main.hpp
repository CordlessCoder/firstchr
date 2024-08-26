#include <cstddef>
#include <uchar.h>

extern "C" struct string {
    char  *ptr;
    size_t len;
};

extern "C" string without_first_char_mnem(string data);
extern "C" string without_first_char_basalt(string data);
extern "C" string without_first_char_vxppy(string data);
