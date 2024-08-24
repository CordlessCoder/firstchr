#include <cstddef>
#include <string_view>
#include <uchar.h>

#include "main.hpp"

inline std::string_view without_first_char_basalt_inner(std::string_view text) {
    if (text.empty()) {
        return text;
    }
    unsigned char lead     = static_cast<unsigned char>(text[0]);
    size_t        char_len = 0;

    if (lead < 0x80) {
        char_len = 1;
    } else if ((lead >> 5) == 0b110) {
        char_len = 2;
    } else if ((lead >> 4) == 0b1110) {
        char_len = 3;
    } else if ((lead >> 3) == 0b11110) {
        char_len = 4;
    } else {
        // Invalid UTF-8
        return text;
    }
    return text.substr(char_len);
}

extern "C" string without_first_char_basalt(string data) {
    std::string_view text(data.ptr, data.len);
    text = without_first_char_basalt_inner(text);
    return string{const_cast<char *>(text.data()), text.length()};
}
