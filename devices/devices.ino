#include <WiFi.h>
#include <WiFiClientSecure.h>
#include <SPIFFS.h>
#include <string>



#define WIFI "ScholzLAN", "#ScholzLAN!"
#define IOT_HOST "192.168.8.181", 3000U


using namespace std;

const char* client_cert = "-----BEGIN CERTIFICATE-----\n"
"MIIDGzCCAgMCFE07mLU0mlBuhcmqAvXELDSZImbjMA0GCSqGSIb3DQEBCwUAMEkx\n"
"CzAJBgNVBAYTAkFVMQswCQYDVQQIDAJTVDESMBAGA1UECgwJTXlDb21wYW55MQow\n"
"CAYDVQQLDAErMQ0wCwYDVQQDDARBbGV4MCAXDTIzMDYyMzEwMDkwMVoYDzIwNTAx\n"
"MTA4MTAwOTAxWjBJMQswCQYDVQQGEwJBVTELMAkGA1UECAwCU1QxEjAQBgNVBAoM\n"
"CU15Q29tcGFueTEKMAgGA1UECwwBKzENMAsGA1UEAwwEQWxleDCCASIwDQYJKoZI\n"
"hvcNAQEBBQADggEPADCCAQoCggEBAMODg0XAe+TNE56DtfOw7YFPxXgDWK4ZPVNt\n"
"fThVdG8FTmyg6yxWUueHhWS0uaTcw/1dEROJ+g0WhnmizkpX9AnJzayETsCKkYyx\n"
"hm2WnxngAwlSrkggEUV+214G03Y68iUhThV7knqkPc4IUt8JIWe4UNYcBjZoXhzV\n"
"i3zMG98rfpaYvukcZSu7WehaeNAg6PuyAsp/IjnNNFUyMCM4q/CLn68NF91Gn8N5\n"
"+EiFPrVXrq0zSUcxgurSmNfTd2G/oTg1CMrzIUeiCy2Hd+dbqLRbxzvsjEQx6AyS\n"
"kiLQcHYaX7HUloyEHq4hxiFFR8kmqxU1TzSWmDwLrXWExrTDbH0CAwEAATANBgkq\n"
"hkiG9w0BAQsFAAOCAQEAeQY+xba/1mv0gzcaPje2pB94s6Arjdi1SPxv+BdiGNI0\n"
"UJ7TapQBahgvhy74qQfZElVmt/m4jjGoaxpI3mfQ+QiJ/fspN+3DTMGJh5K9Smop\n"
"r7+0wGPC5TsP6/V7VUksprNgCZNV0YluIpUar1jmuya+eR98z5ejEIirU7DHI4RR\n"
"ihsCbYoLfnwpox6NFqurif2JvZEms9roa36yvLyEJFt6oPF/wSqiYIcWNazOXsOS\n"
"E3okHLawzQVVtXfSeUfJdu1fZFUYgoTqAoZhWdNTV/MTWk4CWHPtDWIkxzbwG5Dk\n"
"TMxnlulg52xEX3eQZfLzIHEvLH8i01Qi4V2855fjaw==\n"
"-----END CERTIFICATE-----\n";

const char* client_key = "-----BEGIN RSA PRIVATE KEY-----\n"
"MIIEowIBAAKCAQEAw4ODRcB75M0TnoO187DtgU/FeANYrhk9U219OFV0bwVObKDr\n"
"LFZS54eFZLS5pNzD/V0RE4n6DRaGeaLOSlf0CcnNrIROwIqRjLGGbZafGeADCVKu\n"
"SCARRX7bXgbTdjryJSFOFXuSeqQ9zghS3wkhZ7hQ1hwGNmheHNWLfMwb3yt+lpi+\n"
"6RxlK7tZ6Fp40CDo+7ICyn8iOc00VTIwIzir8Iufrw0X3Uafw3n4SIU+tVeurTNJ\n"
"RzGC6tKY19N3Yb+hODUIyvMhR6ILLYd351uotFvHO+yMRDHoDJKSItBwdhpfsdSW\n"
"jIQeriHGIUVHySarFTVPNJaYPAutdYTGtMNsfQIDAQABAoIBADzaPAuDw4wNQGP/\n"
"/qvgj6vF1mJqODnH5UyIMYdNIbZEYopw84GTRK6Hgb88eOOVSvMa+muocEmOj0Bg\n"
"qfB9u5koVHmznIdVGcQ+pOaLEO4OySMnttMAGCAXQdPcMAkGg8OGwDTzhz3gzNPJ\n"
"x3Ff+CO+PHrHsi9RAKnc1VN8e9hTt8G74y1FCLNmWeyNJjADCYD6EVn7D9XH9SGO\n"
"K52ifXVhsInJMKG5FajSwzuEd4ERg3XfoSKKMrJ7aHjG1ioxHMAJRO460+U86JQi\n"
"NjKPkzvd0UBI3nt0smfkVDnmFqGmIPHMRPGp8XDIPYv8/9XoSs1Rh0NxDbrGcCWO\n"
"CjmbIMkCgYEA7kexoWUzNpFZ8lH+t5CmZfCHQy9MwSAlkucI4NF5fB32G2AvbYdp\n"
"irbAh7K81YwQtpFf/NIUCF4loa6lz8tRpRc5zxLNQAIuKLu15KhsliBu0V02yOCj\n"
"vVrbGR7n6zgp2QtIqQfKBkw3lp6ZCUmCOGR8LYS/jFreFInil2Kiqe8CgYEA0g2l\n"
"XZuAsGRtIpMh0EsvZGVO6gaFdtoNgIeY0RD2CvOaFgxT7wYl4aUhGqmds7cuxAH7\n"
"uRCySLdrpFobnuPS061xiX64lz/FFkOZRUYj/KwPIh7w0Km4tr9jIjCVTkADa1Eq\n"
"IH+YZYHYvJKHjKZyNoeBBlCuF8EY+m+cECaC7FMCgYBEv+DLfWR6YRO/q9K/affo\n"
"v653PwV5T2on6YsROr8/hXGF0om8o25bPIFuZa7AOopz8pvsWTy4cVmQPdDPlI05\n"
"AeuHCJi52CczT32avNGfuzneJHPzH4V+0+EVbWpi3rCMVAoh4wJyMLl1NstW96Of\n"
"27ZMd9nAiFM9P89l+GqQdwKBgHE6F9y4mQr8Z3POudhs3tQQZBR0hXcGnWaeMTrp\n"
"GKfYnkOqS4g52Suj9o9HxRF9+gCATS96F5BpBuYmCGOCzT97bAtAusIVBuk0cSdN\n"
"YtKt8M5kvUzNYBtqWUaxx5nRmcY8jyfLf9ufvbQQjG69vi00p9E+pkl5tjww84Al\n"
"MkevAoGBAK46L2rjG8qbdO/6cVCvvVJMf4Qfl8CyVNcbYlt9p8nmhgDjgBgrb3be\n"
"aoVE6CSE67r0Ml8KdfbjrHXL7iwHUMnJazi8PaIBCgEob/WHkEyP74Ww+r7iyruy\n"
"nyWPgRZXfObqYAvAQ59AW1RidZVAFvIxIB9N9WMlwr8TJ1oHc3UK\n"
"-----END RSA PRIVATE KEY-----\n";

const char* server_ca = "-----BEGIN CERTIFICATE-----\n"
"MIIDITCCAgkCFGsFuVNi+cMuoTglSNJY8tLJ+rPGMA0GCSqGSIb3DQEBCwUAMEwx\n"
"CzAJBgNVBAYTAkFVMQswCQYDVQQIDAJTVDENMAsGA1UEBwwEU1RVQjESMBAGA1UE\n"
"CgwJTXlDb21wYW55MQ0wCwYDVQQDDARBbGV4MCAXDTIzMDYyMzEwMDg0MFoYDzIw\n"
"NTAxMTA4MTAwODQwWjBMMQswCQYDVQQGEwJBVTELMAkGA1UECAwCU1QxDTALBgNV\n"
"BAcMBFNUVUIxEjAQBgNVBAoMCU15Q29tcGFueTENMAsGA1UEAwwEQWxleDCCASIw\n"
"DQYJKoZIhvcNAQEBBQADggEPADCCAQoCggEBALbK3M+fFrO9Zg9hSW3wmLB6FrnR\n"
"nafWCxnydu3IVYbGvAsLo233V0Hs1Q1JZ1eQRjgKlPeDJaiXzdyDZOtaKhVJ/T9c\n"
"vBtSSTJ7NWtgAhG2WVfT1kcCHjwsNM9q2r3O/8DZQbdIu1/OqlXWbVoD5s9WmWAK\n"
"btNIlnH4TajXSbfrp0ljqO4rdvGRzIqBFQR+qbayTtc3aGWyzZRh75YOQS1WtMpR\n"
"ckq/hCdY0XnXhMEph8Tpbh/fMYbm0F2J36WH3XhXrcf6OQkJRCnYbkgRuw5PXDHm\n"
"FWMD4AYBTIX8Dy4JFU6BOfL9Hiu2IFj+CPWwJBr6QvHGSlHL/kOwDxcuybUCAwEA\n"
"ATANBgkqhkiG9w0BAQsFAAOCAQEAqrsQtS1Ypq9mFPc5m3bcvPeOv3nRvGMGZIXe\n"
"RLxY8ThLgSVN1qgy9sMNVNtx+a54bIwY/62iysb+IqB5Ns2isOb0aiNa7/YOhfZG\n"
"IIuRI6NlpaViAS6ZLEj8YlqaGp3kJ4vP+dWVqqeiMxsD3/tuKyJT95FnOGTKod6P\n"
"fErem/ugdftxNFn1j485bFk6QSCB5Qe75tTG5/tDB+VyANkf7XHuwrgqiCQ8XQFu\n"
"tic8Qasf5hq+koHRFnqR5elVEJ5iNF2Kbn9PB5hrHixNzMPGXLhDQ12Yf5EVoJCT\n"
"0sgUDJJ2b9nVnt5NiZoT8+GfhZ4zZMcIrIt52NQLy+OZYEIIgQ==\n"
"-----END CERTIFICATE-----\n";


void setup()
{
  if(!SPIFFS.begin(true)){
        Serial.println("[-]An Error has occurred while mounting SPI1FFS");
        return;
  }

  Serial.begin(115200);

  WiFi.begin(WIFI);
  while (WiFi.status() != WL_CONNECTED) {
    delay(1000);
    Serial.println("[~]Connecting to WiFi...");
  }
  Serial.println("[*]Connected to WiFi");



  // Load server CA certificate

  // Load client private key

  WiFiClientSecure client;
  client.setInsecure();

  client.setCACert(server_ca);
  client.setCertificate(client_cert);
  client.setPrivateKey(client_key);

  Serial.printf("[*] Trying to connect to server on IP: %s Port: %d\n", IOT_HOST);


  if (!client.connect(IOT_HOST)) {
    Serial.println("[-]Connection to server failed");
    return;
  }
  Serial.println("[*]Connected to server");
}

char* getContent(char* filename) {
  File certFile = SPIFFS.open(filename, "r");
  if (!certFile) {
    Serial.println("Failed to open client certificate file");
    return "";
  }
  size_t certSize = certFile.size();
  char* certBuf(new char[certSize]);
  certFile.readBytes(certBuf, certSize);
  certFile.close();
  return certBuf;
}


void loop()
{
  delay(5000);
  string data = gather_data();
  send_data(data);
}


string gather_data() {
  Serial.println("[*]Gathering data");
  return "";
}

int send_data(string data) {
  Serial.println("[*]Sending data");
  return 0;
}
