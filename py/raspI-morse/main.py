from board import Board
from morse import mrsCode
from time import time, sleep
from enum import Enum
from gpiozero import Device

board = Board()
led = board.led
btn = board.btn

DOT_MAX = 0.15
DASH_MAX = 0.8
BUILD_TIME = 0.7
PUSH_MSG = 2.2

msg = []
cBuilder = ""
timeStamp = time()
isPressed = False
class State(Enum):
    IDLE = 0
    LISTENING = 1

state = State.IDLE

def buildC(duration):
    global cBuilder
    if duration < DOT_MAX:
        cBuilder += "."
        print(".")
    elif duration < DASH_MAX:
        cBuilder += "-"
        print("-")

def pressed():
    global isPressed, timeStamp, state
    if state == State.LISTENING:
        timeStamp = time()
        isPressed = True

def released():
    global isPressed, timeStamp, state
    if  state == State.LISTENING:
        duration = time() - timeStamp
        buildC(duration)
        isPressed = False
        timeStamp = time()

def finalizeChar(char):
    global msg, timeStamp, cBuilder
    if char:
        if char == "<" and msg:
            msg.pop()
        elif char == ">":
            msg.append(" ")
        else:
            msg.append(char)
        board.blink()
        print(msg)
    else:
        print("invalid:", cBuilder)
    cBuilder = ""
    timeStamp = time()

btn.when_pressed = pressed
btn.when_released = released

try:
    while True:
        if state == State.IDLE:
            print("Press to start")
            btn.wait_for_press()
            board.blink(2)
            print("listening...")
            state = State.LISTENING

        if isPressed:
            duration = time() - timeStamp
            if duration > PUSH_MSG and msg:
                finalMsg = "".join(msg)
                print(f"Final message: {finalMsg}")
                board.blink(2)
                btn.wait_for_release()
                msg = []
                state = State.IDLE

        if not isPressed and time() - timeStamp > BUILD_TIME and cBuilder:
            char = mrsCode.get(cBuilder)
            finalizeChar(char)

        sleep(0.01)
except KeyboardInterrupt:
    print("Program terminated")
finally:
    board.exit()
    
    