#include "matrix.h"

Matrix::Matrix() {}

void Matrix::begin() {
    matrix.begin();
}

void Matrix::displayStaticText(const char* text, uint32_t color) {
    matrix.clear();
    matrix.beginDraw();
    matrix.stroke(color);
    matrix.textFont(Font_5x7);
    matrix.beginText(0, 1, color);
    matrix.println(text);
    matrix.endText();
    matrix.endDraw();
}

void Matrix::displayScrollingText(const char* text, uint32_t color) {
    matrix.clear();
    matrix.beginDraw();
    matrix.stroke(color);
    matrix.textScrollSpeed(50);
    matrix.textFont(Font_5x7);
    matrix.beginText(0, 1, color);
    matrix.println(text);
    matrix.endText(SCROLL_LEFT);
    matrix.endDraw();
}
