#include <Adafruit_BMP085.h>
#include <BH1750.h>
#include <Wire.h>
#include <SimpleDHT.h>
#include <WiFi.h>
#include <WiFiClientSecure.h>
#include <SPIFFS.h>
#include <string>
#include <utility>
#include <sstream>
#include <exception>
#include <queue>
#include <cstdint>
#include <cstring>

#define WIFI "ScholzLAN", "ScholzLAN"
#define IOT_HOST "192.168.8.181", 3010U
#define THERM_PIN 2U
#define INTERVAL 1000U
#define DEVICE_NAME "thermomether_1\0"

using namespace std;

namespace device
{
  class IoTException : public exception
  {
  public:
    IoTException(const string &message) : message_(message){};

    virtual const char *what() const noexcept override
    {
      return message_.c_str();
    }

  private:
    string message_;
  };

  WiFiClientSecure connection;
  SimpleDHT11 thermometer;
  BH1750 light_sensor;
  Adafruit_BMP085 pressure_sensor;

  queue<vector<uint8_t>> cache;

  bool server_connect()
  {
    connection.setInsecure();
    Serial.printf("[*]Trying to connect to server on IP: %s Port: %d\n", IOT_HOST);
    return (bool)connection.connect(IOT_HOST);
  }

  bool wifi_connect()
  {
    WiFi.begin(WIFI);
    delay(1000);
    return WiFi.status() == WL_CONNECTED;
  }

  bool send_data(vector<uint8_t> data) {
      string str(data.begin(), data.end());
      return (bool)connection.print(str.c_str());
  }


  pair<float, float> get_temperature()
  {
    float temperature, humidity = 0;

    int err = SimpleDHTErrSuccess;
    if ((err = thermometer.read2(&temperature, &humidity, NULL)) != SimpleDHTErrSuccess)
    {
      throw IoTException("[-]Couldnt read data from thermometer");
    }

    return std::make_pair(temperature, humidity);
  }

  float get_light()
  {
    float level = light_sensor.readLightLevel();

    if (level < 0)
    {
      throw IoTException("[-]Error when trying to read light-level");
    }
    return level;
  }

  int32_t get_pressure()
  {
    return pressure_sensor.readPressure();
  }

  void send_cache_data()
  {
    while (!cache.empty())
    {
      send_data(cache.front());
      cache.pop();
    }
  }

  vector<uint8_t> gather_data()
  {
    vector<uint8_t> bytes = {0x1, 0x1};
    bytes.resize(bytes.size() + sizeof(DEVICE_NAME));
    memcpy(bytes.data(), &DEVICE_NAME, sizeof(DEVICE_NAME));

    pair<float, float> temperature = get_temperature();
    bytes.push_back(0x2);
    bytes.resize(bytes.size() + sizeof(float));
    memcpy(bytes.data() + bytes.size() - sizeof(float), &temperature.first, sizeof(float));
    bytes.push_back(0x3);
    bytes.push_back(temperature.second);

    float light_level = get_light();
    bytes.push_back(0x4);
    bytes.resize(bytes.size() + sizeof(float));
    memcpy(bytes.data() + bytes.size() - sizeof(float), &light_level, sizeof(float));

    uint32_t pressure = (uint32_t)get_pressure();
    bytes.push_back(0x5);
    bytes.resize(bytes.size() + sizeof(uint32_t));
    memcpy(bytes.data() + bytes.size() - sizeof(uint32_t), &pressure, sizeof(uint32_t));

    return bytes;
  }
}

void setup()
{
  Serial.begin(115200);

  while (!device::wifi_connect())
  {
    Serial.println("[~]Connecting to WiFi...");
  }
  Serial.println("[*]Connected to WiFi");

  if (!device::server_connect())
  {
    Serial.println("[-]Connection to server failed");
  }
  else
  {
    Serial.println("[*]Connected to server");
  }

  Serial.println("[*]Connecting to thermometer");
  device::thermometer = SimpleDHT11(THERM_PIN);

  Wire.begin();
  if (device::light_sensor.begin())
  {
    Serial.println("[*]BH1750 Advanced begin");
  }
  else
  {
    Serial.println("[-]Error initialising BH1750");
  }

  if (!device::pressure_sensor.begin())
  {
    Serial.println("[-]Could not find a valid BMP185 sensor, check wiring!");
  }
  else
  {
    Serial.println("[*]Succesfully connected to BMP180");
  }
}

void loop()
{
  delay(INTERVAL);
  Serial.println("[*]Gathering data");

  vector<uint8_t> data;
  try
  {
    data = device::gather_data();
  }
  catch (device::IoTException e)
  {
    Serial.println(e.what());
    return;
  }

  Serial.println("[*]Sending data");
  if (!device::send_data(data))
  {
    Serial.println("[-]No connection to server");
    device::cache.push(data);
    if (device::server_connect())
    {
      Serial.println("[*]Reconnected to server");
      device::send_cache_data();
    }
  }
}
