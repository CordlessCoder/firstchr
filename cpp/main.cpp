#include <cstddef>
#include <cstdint>
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

constexpr uint8_t LUT[16] = {1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 1, 3, 4};

inline std::string_view without_first_char_mnem_inner(std::string_view text) {
    switch (text.length()) {
    case 0:
        return text;
    default:
        unsigned char lead     = static_cast<unsigned char>(text[0]);
        size_t        char_len = LUT[lead >> 4];

        text.remove_prefix(char_len);
        return text;
    }
}

extern "C" string without_first_char_mnem(string data) {
    std::string_view text(data.ptr, data.len);
    text = without_first_char_mnem_inner(text);
    return string{const_cast<char *>(text.data()), text.length()};
}

inline std::string_view without_first_char_vxppy_inner(std::string_view text) {
    char          trim_length = 0;
    unsigned char lead        = text[0];

    if (lead < 0x80) {
        trim_length = 1;
    } else if ((lead & 0xE0) == 0xC0) {
        trim_length = 2;
    } else if ((lead & 0xF0) == 0xE0) {
        trim_length = 3;
    } else if ((lead & 0xF8) == 0xF0) {
        trim_length = 4;
    } else {
        return text;
    }
    text.remove_prefix(trim_length);
    return text;
}

extern "C" string without_first_char_vxppy(string data) {
    std::string_view text(data.ptr, data.len);
    text = without_first_char_vxppy_inner(text);
    return string{const_cast<char *>(text.data()), text.length()};
}
