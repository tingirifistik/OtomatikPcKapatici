from  os import system
from time import sleep


while True:
    try:
        sor = int(input("1- Otomatik kapatma zamanını ayarla\n2- Otomatik kapatmayı iptal et\n3- Çıkış\n\nSeçim: "))
    except ValueError:
        system("cls")
        print("Lütfen sayı giriniz..")
        sleep(4)
        system("cls")
        continue
    if sor == 1:
        system("cls")
        try:
            sor1 = int(input("Bilgisayarın kaç dakika sonra kapanmasını istiyorsun: "))
        except ValueError:
            system("cls")
            print("Lütfen sayı giriniz..")
            sleep(4)
            system("cls")
            continue
        hesapla = sor1 * 60
        system("shutdown -s -f -t {}".format(hesapla))
        system("cls")
    elif sor == 2:
        system("cls")
        system("shutdown -a")
    elif sor == 3:
        break
    else:
        system("cls")
        print("Lütfen doğru bir seçim yapınız..")
        sleep(4)
        system("cls")
