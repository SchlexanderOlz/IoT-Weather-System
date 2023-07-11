# Protocol
#### This file describes the encoding standard of messages send by a device

## How to use
Messages are always in the format of bytes. A message which is send to the server contains **headers**. Which headers exist is desribed in the following document. A header is always followed by the information it should carry. String need to be terminated by a **null-byte**.

#### These are the existing device-type headers
The device-type decides how the following bytes will be interpreted.

The first byte send is interpreted as the device-type


| Bytes | DeviceType |
| --- | --- |
| 0x1 | Weather-Device |


| Bytes | Meaning |
| --- | --- |
| 0x1 | ID/name |
| 0x2 | temperature |
| 0x3 | humidity |
| 0x4 | light-level |
| 0x5 | pressure |