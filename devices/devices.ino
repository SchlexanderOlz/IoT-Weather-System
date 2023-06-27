#include <SimpleDHT.h>
#include <WiFi.h>
#include <WiFiClientSecure.h>
#include <SPIFFS.h>
#include <string>
#include <utility>
#include <sstream>
#include <exception>


#define WIFI "ScholzLAN", "#ScholzLAN!"
#define IOT_HOST "192.168.8.181", 3000U
#define DHT_PIN 2

using namespace std;


class IoTException : public exception {
  public:
  IoTException(const string& message) : message_(message) {};

  virtual const char* what() const noexcept override {
    return message_.c_str();
  }

  private:
  string message_;
};


WiFiClientSecure connection;
SimpleDHT11 thermometer;

void setup()
{
  Serial.begin(115200);

  WiFi.begin(WIFI);
  while (WiFi.status() != WL_CONNECTED) {
    delay(1000);
    Serial.println("[~]Connecting to WiFi...");
  }
  Serial.println("[*]Connected to WiFi");


  // Connect to server

  if (!connect()) {
    Serial.println("[-]Connection to server failed");
    return;
  } else {
    Serial.println("[*]Connected to server");
  }

  Serial.println("[*]Connecting to thermometer");
  thermometer = SimpleDHT11(DHT_PIN);
}


void loop()
{
  delay(5000);
  Serial.println("[*]Gathering data");

  string data;
  try {
    data = gather_data();
  } catch (IoTException e) {
    Serial.println(e.what());
    return;
  }

  Serial.println("[*]Sending data");
  if (!send_data(data)) {
    Serial.println("[-]No connection to server");
    if (connect()) {
      Serial.println("[*]Reconnected to server");
    }
  }
}

bool connect() {
  connection.setInsecure();
  Serial.printf("[*]Trying to connect to server on IP: %s Port: %d\n", IOT_HOST);
  return (bool)connection.connect(IOT_HOST);
}

bool send_data(string data) {
  return (bool)connection.print(data.c_str());
}

pair<float, float> measure_temperature() {
  float temperature, humidity = 0;

  int err = SimpleDHTErrSuccess;
  if ((err=thermometer.read2(&temperature, &humidity, NULL)) != SimpleDHTErrSuccess) {
    throw IoTException("[-]Couldnt read data from thermometer");
  }

  return std::make_pair(temperature, humidity);
}

string gather_data() {
  
  pair<float, float> temperature = measure_temperature();

  stringstream data_stream;
  data_stream << "{\"temperature\" : " << temperature.first << ", \"sensor_id\" : \"temp_name\", \"humidity\" :" << temperature.second << "}";
  Serial.println(data_stream.str().c_str());
  return data_stream.str();
}

