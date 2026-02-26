from board import Board
from morse import mrsCode
from time import time, sleep
from enum import Enum
from gpiozero import Device
from webhook import sendDiscordMsg

class State(Enum):
    IDLE = 0
    LISTENING = 1

class MorseBoard:
    DOT_MAX = 0.15
    DASH_MAX = 0.7
    BUILD_TIME = 0.7
    PUSH_MSG = 2
    #Put your own webhook
    WEBHOOK = ""
    SENDER = "Simon" #Put your own name

    def __init__(self):
        self.board = Board()
        self.led = self.board.led
        self.btn = self.board.btn

        self.state = State.IDLE
        self.msg = []
        self.cBuilder = ""
        self.isPressed = False
        self.timeStamp = time()

        # Button event hooks
        self.btn.when_pressed = self.pressed
        self.btn.when_released = self.released

    def buildC(self, duration):
        if duration < self.DOT_MAX:
            self.cBuilder += "."
            print(".")
        elif duration < self.DASH_MAX:
            self.cBuilder += "-"
            print("-")

    def pressed(self):
        self.isPressed = True
        self.led.on()
        if self.state == State.LISTENING:
            self.timeStamp = time()

    def released(self):
        self.isPressed = False
        self.led.off()
        if self.state == State.LISTENING:
            duration = time() - self.timeStamp
            self.buildC(duration)
            self.timeStamp = time()

    def finalizeChar(self, char):
        if char:
            if char == "<" and self.msg:
                self.msg.pop()
            elif char == ">":
                self.msg.append(" ")
            else:
                self.msg.append(char)
            self.board.blink()
            print(self.msg)
        else:
            print("invalid:", self.cBuilder)
        self.cBuilder = ""
        self.timeStamp = time()

    def run(self):
        print("Press to start")
        try:
            while True:
                if self.state == State.IDLE and self.isPressed:
                    self.board.blink(2)
                    print("listening...")
                    self.state = State.LISTENING

                if self.isPressed:
                    duration = time() - self.timeStamp
                    if duration > self.PUSH_MSG and self.msg:
                        finalMsg = "".join(self.msg)
                        print(f"Final message: {finalMsg}")
                        self.board.blink(2)
                        self.msg = []
                        sendDiscordMsg(self.WEBHOOK, finalMsg, self.SENDER)
                        self.state = State.IDLE

                if not self.isPressed and time() - self.timeStamp > self.BUILD_TIME and self.cBuilder:
                    char = mrsCode.get(self.cBuilder)
                    self.finalizeChar(char)

                sleep(0.01)
        except KeyboardInterrupt:
            print("Program terminated")
        finally:
            self.board.exit()