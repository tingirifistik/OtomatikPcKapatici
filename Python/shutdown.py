from os import system, name
from time import sleep

if name == "posix":
    shutdown = "shutdown -h +"
    cancel = "shutdown -c"
    multiply = 1
elif name == "nt":
    shutdown = "shutdown /s /t "
    cancel = "shutdown /a"
    multiply = 60
else:
    exit()

while 1:
    system("cls||clear")
    menu = input("1- Set the shutdown time\n2- Cancel the shutdown\n3- Exit\n\n>>>")
    system("cls||clear")
    if menu == "1":
        try:
            time = int(input("How many minutes later do you want the computer to shutdown: "))
        except ValueError:
            print("\nPlease type number..")
            sleep(1.5)
            continue
        system(shutdown+str(time*multiply))
    elif menu == "2":
        system(cancel)
        print("The shutdown has been canceled..")
        sleep(1.5)
    elif menu == "3":
        break
    else:
        print("Please make a valid choice..")
        sleep(1.5)