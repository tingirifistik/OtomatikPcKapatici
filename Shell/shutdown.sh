#!/bin/bash

while true; do
    clear
    echo -e "1- Set the shutdown time\n2- Cancel the shutdown\n3- Exit\n"
    read -p ">>>" menu
    clear
    if [ "$menu" == "1" ]; then
        read -p "How many minutes later do you want the computer to shutdown: " time
        clear
        shutdown -h +$time
        sleep 3
    elif [ "$menu" == "2" ]; then
        shutdown -c
        echo "The shutdown has been canceled.."
        sleep 2
    elif [ "$menu" == "3" ]; then
        break
    else
        echo "Please make a valid choice.."
        sleep 2
    fi
done