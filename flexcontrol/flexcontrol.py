import serial
import time
ser = serial.Serial("COM8", 115200, timeout=0)

while True:
    data = ser.read(100)
    if data:
        print(data)
    time.sleep(0.01666)
    