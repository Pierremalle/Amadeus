#include "matrix/matrix.h"
#include <WiFi.h>
#include "arduino_secrets.h"

#define SAMPLE_RATE 16000        // Fréquence d'échantillonnage
#define PACKET_SIZE 2048         // Nombre d'échantillons envoyés par paquet (4096 octets)
#define AUDIO_PIN A0             // Microphone analogique
#define BUTTON_PIN 2             // Bouton d'enregistrement

Matrix matrix;
char ssid[] = SECRET_SSID;
char pass[] = SECRET_PASS;

const char* server = "192.168.87.189";
const int port = 9000;

WiFiClient client;
bool isRecording = false;
bool lastButtonState = HIGH;
uint32_t totalSamples = 0;

void setup() {
  Serial.begin(9600);
  matrix.begin();
  pinMode(BUTTON_PIN, INPUT_PULLUP);

  Serial.println("Connecting to WiFi...");
  WiFi.begin(ssid, pass);
  while (WiFi.status() != WL_CONNECTED) {
      delay(1000);
      Serial.print(".");
  }
  Serial.println("\nConnected to WiFi!");
  matrix.displayScrollingText("Ready", 0x00FF00);
}

bool readButton() {
  bool state = digitalRead(BUTTON_PIN);
  if (state == LOW && lastButtonState == HIGH) {
    lastButtonState = LOW;
    return true;
  }
  if (state == HIGH) {
    lastButtonState = HIGH;
  }
  return false;
}

void sendWavHeader() {
  uint32_t byteRate = SAMPLE_RATE * 2; // 16-bit mono
  uint16_t blockAlign = 2;
  
  byte wavHeader[44] = {
      'R', 'I', 'F', 'F', 0, 0, 0, 0,   // ChunkID & ChunkSize (0 pour streaming)
      'W', 'A', 'V', 'E',
      'f', 'm', 't', ' ', 16, 0, 0, 0,  // Subchunk1ID & Subchunk1Size
      1, 0, 1, 0,                        // AudioFormat & NumChannels
      (byte)(SAMPLE_RATE & 0xFF), (byte)((SAMPLE_RATE >> 8) & 0xFF), (byte)((SAMPLE_RATE >> 16) & 0xFF), (byte)((SAMPLE_RATE >> 24) & 0xFF), 
      (byte)(byteRate & 0xFF), (byte)((byteRate >> 8) & 0xFF), (byte)((byteRate >> 16) & 0xFF), (byte)((byteRate >> 24) & 0xFF),
      blockAlign, 0, 16, 0,               // BlockAlign & BitsPerSample
      'd', 'a', 't', 'a', 0, 0, 0, 0      // Subchunk2ID & Subchunk2Size (0 pour streaming)
  };
  
  client.write(wavHeader, 44);
  Serial.println("WAV header sent.");
}

void loop() {
  if (readButton()) {
    isRecording = !isRecording;
    
    if (isRecording) {
      Serial.println("Recording started...");
      matrix.displayScrollingText("Rec", 0xFF0000);

      if (!client.connect(server, port)) {
        Serial.println("Failed to connect to server.");
        isRecording = false;
      } else {
        sendWavHeader();  // Envoie l'en-tête WAV au début
        totalSamples = 0;
      }
    } else {
      Serial.println("Recording stopped.");
      matrix.displayScrollingText("Stopped", 0xFF00FF);
      client.stop();
    }
  }

  if (isRecording && client.connected()) {
    int16_t audioBuffer[PACKET_SIZE];

    for (int i = 0; i < PACKET_SIZE; i++) {
      int value = analogRead(AUDIO_PIN);
      audioBuffer[i] = map(value, 0, 1023, -32768, 32767);
    }

    client.write((byte*)audioBuffer, PACKET_SIZE * 2);
    totalSamples += PACKET_SIZE;
    Serial.println("Sent audio packet...");
  }

  delay(50);
}
