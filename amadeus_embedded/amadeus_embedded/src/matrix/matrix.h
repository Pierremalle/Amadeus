#ifndef MATRIX_H
#define MATRIX_H

#include "ArduinoGraphics.h"
#include "Arduino_LED_Matrix.h"

class Matrix {
public:
    // Constructor
    Matrix();

    // Initialize the matrix
    void begin();

    // Display static text on the matrix with given color
    void displayStaticText(const char* text, uint32_t color);

    // Display scrolling text on the matrix with given color
    void displayScrollingText(const char* text, uint32_t color);

private:
    ArduinoLEDMatrix matrix;
};

#endif // MATRIX_H
