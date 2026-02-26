from gpiozero import Button, LED
from time import sleep
import sys

class Board():
    def __init__(self):
        self.btn = Button(21, bounce_time = 0.035)
        self.led = LED(19)
    def blink(self, n = 1):
        for _ in range(n):  
            self.led.on()
            sleep(0.1)
            self.led.off()
            sleep(0.1)
    def exit(self):
        self.btn.close()
        self.led.close()
        sys.exit