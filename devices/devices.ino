#include <WiFi.h>
#include <WiFiClientSecure.h>
#include <SPIFFS.h>
#include <string>


#define WIFI "ScholzLAN", "#ScholzLAN!"
#define IOT_HOST "192.168.8.127", 3000

using namespace std;

const char* client_cert = "-----BEGIN CERTIFICATE-----"
"MIIDGzCCAgMCFE07mLU0mlBuhcmqAvXELDSZImbjMA0GCSqGSIb3DQEBCwUAMEkx"
"CzAJBgNVBAYTAkFVMQswCQYDVQQIDAJTVDESMBAGA1UECgwJTXlDb21wYW55MQow"
"CAYDVQQLDAErMQ0wCwYDVQQDDARBbGV4MCAXDTIzMDYyMzEwMDkwMVoYDzIwNTAx"
"MTA4MTAwOTAxWjBJMQswCQYDVQQGEwJBVTELMAkGA1UECAwCU1QxEjAQBgNVBAoM"
"CU15Q29tcGFueTEKMAgGA1UECwwBKzENMAsGA1UEAwwEQWxleDCCASIwDQYJKoZI"
"hvcNAQEBBQADggEPADCCAQoCggEBAMODg0XAe+TNE56DtfOw7YFPxXgDWK4ZPVNt"
"fThVdG8FTmyg6yxWUueHhWS0uaTcw/1dEROJ+g0WhnmizkpX9AnJzayETsCKkYyx"
"hm2WnxngAwlSrkggEUV+214G03Y68iUhThV7knqkPc4IUt8JIWe4UNYcBjZoXhzV"
"i3zMG98rfpaYvukcZSu7WehaeNAg6PuyAsp/IjnNNFUyMCM4q/CLn68NF91Gn8N5"
"+EiFPrVXrq0zSUcxgurSmNfTd2G/oTg1CMrzIUeiCy2Hd+dbqLRbxzvsjEQx6AyS"
"kiLQcHYaX7HUloyEHq4hxiFFR8kmqxU1TzSWmDwLrXWExrTDbH0CAwEAATANBgkq"
"hkiG9w0BAQsFAAOCAQEAeQY+xba/1mv0gzcaPje2pB94s6Arjdi1SPxv+BdiGNI0"
"UJ7TapQBahgvhy74qQfZElVmt/m4jjGoaxpI3mfQ+QiJ/fspN+3DTMGJh5K9Smop"
"r7+0wGPC5TsP6/V7VUksprNgCZNV0YluIpUar1jmuya+eR98z5ejEIirU7DHI4RR"
"ihsCbYoLfnwpox6NFqurif2JvZEms9roa36yvLyEJFt6oPF/wSqiYIcWNazOXsOS"
"E3okHLawzQVVtXfSeUfJdu1fZFUYgoTqAoZhWdNTV/MTWk4CWHPtDWIkxzbwG5Dk"
"TMxnlulg52xEX3eQZfLzIHEvLH8i01Qi4V2855fjaw=="
"-----END CERTIFICATE-----";

const char* client_key = "-----BEGIN RSA PRIVATE KEY-----"
"MIIEowIBAAKCAQEAw4ODRcB75M0TnoO187DtgU/FeANYrhk9U219OFV0bwVObKDr"
"LFZS54eFZLS5pNzD/V0RE4n6DRaGeaLOSlf0CcnNrIROwIqRjLGGbZafGeADCVKu"
"SCARRX7bXgbTdjryJSFOFXuSeqQ9zghS3wkhZ7hQ1hwGNmheHNWLfMwb3yt+lpi+"
"6RxlK7tZ6Fp40CDo+7ICyn8iOc00VTIwIzir8Iufrw0X3Uafw3n4SIU+tVeurTNJ"
"RzGC6tKY19N3Yb+hODUIyvMhR6ILLYd351uotFvHO+yMRDHoDJKSItBwdhpfsdSW"
"jIQeriHGIUVHySarFTVPNJaYPAutdYTGtMNsfQIDAQABAoIBADzaPAuDw4wNQGP/"
"/qvgj6vF1mJqODnH5UyIMYdNIbZEYopw84GTRK6Hgb88eOOVSvMa+muocEmOj0Bg"
"qfB9u5koVHmznIdVGcQ+pOaLEO4OySMnttMAGCAXQdPcMAkGg8OGwDTzhz3gzNPJ"
"x3Ff+CO+PHrHsi9RAKnc1VN8e9hTt8G74y1FCLNmWeyNJjADCYD6EVn7D9XH9SGO"
"K52ifXVhsInJMKG5FajSwzuEd4ERg3XfoSKKMrJ7aHjG1ioxHMAJRO460+U86JQi"
"NjKPkzvd0UBI3nt0smfkVDnmFqGmIPHMRPGp8XDIPYv8/9XoSs1Rh0NxDbrGcCWO"
"CjmbIMkCgYEA7kexoWUzNpFZ8lH+t5CmZfCHQy9MwSAlkucI4NF5fB32G2AvbYdp"
"irbAh7K81YwQtpFf/NIUCF4loa6lz8tRpRc5zxLNQAIuKLu15KhsliBu0V02yOCj"
"vVrbGR7n6zgp2QtIqQfKBkw3lp6ZCUmCOGR8LYS/jFreFInil2Kiqe8CgYEA0g2l"
"XZuAsGRtIpMh0EsvZGVO6gaFdtoNgIeY0RD2CvOaFgxT7wYl4aUhGqmds7cuxAH7"
"uRCySLdrpFobnuPS061xiX64lz/FFkOZRUYj/KwPIh7w0Km4tr9jIjCVTkADa1Eq"
"IH+YZYHYvJKHjKZyNoeBBlCuF8EY+m+cECaC7FMCgYBEv+DLfWR6YRO/q9K/affo"
"v653PwV5T2on6YsROr8/hXGF0om8o25bPIFuZa7AOopz8pvsWTy4cVmQPdDPlI05"
"AeuHCJi52CczT32avNGfuzneJHPzH4V+0+EVbWpi3rCMVAoh4wJyMLl1NstW96Of"
"27ZMd9nAiFM9P89l+GqQdwKBgHE6F9y4mQr8Z3POudhs3tQQZBR0hXcGnWaeMTrp"
"GKfYnkOqS4g52Suj9o9HxRF9+gCATS96F5BpBuYmCGOCzT97bAtAusIVBuk0cSdN"
"YtKt8M5kvUzNYBtqWUaxx5nRmcY8jyfLf9ufvbQQjG69vi00p9E+pkl5tjww84Al"
"MkevAoGBAK46L2rjG8qbdO/6cVCvvVJMf4Qfl8CyVNcbYlt9p8nmhgDjgBgrb3be"
"aoVE6CSE67r0Ml8KdfbjrHXL7iwHUMnJazi8PaIBCgEob/WHkEyP74Ww+r7iyruy"
"nyWPgRZXfObqYAvAQ59AW1RidZVAFvIxIB9N9WMlwr8TJ1oHc3UK"
"-----END RSA PRIVATE KEY-----";


void setup()
{
  Serial.begin(115200);

  WiFi.begin(WIFI);
  while (WiFi.status() != WL_CONNECTED) {
    delay(1000);
    Serial.println("[~]Connecting to WiFi...");
  }
  Serial.println("[*]Connected to WiFi");

  /*if (!SPIFFS.begin()) {
    Serial.println("Failed to mount file-system");
    return;
  }*/
  

  WiFiClientSecure client;
  client.setCertificate(client_cert);
  client.setPrivateKey(client_key);
  if (!client.connect(IOT_HOST)) {
    Serial.println("[-]Connection to server failed");
    return;
  }
  Serial.println("[*]Connected to server");
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
