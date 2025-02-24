#include <iostream>
#include <thread>
#include <chrono>
#include <cstdlib>
using namespace std;

#if __linux__
    const string kapat = "shutdown -h +";
    const string iptal = "shutdown -c";
    const int zort = 1;
#elif _WIN32
    const string kapat = "shutdown /s /t ";
    const string iptal = "shutdown /a";
    const int zort = 60;
#endif

int main(){    
    int menu, secim;
    while(true){
        system("cls||clear");
        cout<<"1- Otomatik kapatma zamanını ayarla\n2- Otomatik kapatmayı iptal et\n3- Çıkış\n\nSeçim: ";
        cin>>menu;
        system("cls||clear");
        if (menu==1){
            cout<<"Bilgisayarın kaç dakika sonra kapanmasını istiyorsun: ";
            cin>>secim;
            system((kapat+to_string(zort*secim)).c_str());
        }
        else if (menu==2){
            system(iptal.c_str());
            cout<<"Otomatik kapatma iptal edildi.."<<endl;
            std::this_thread::sleep_for(std::chrono::seconds(2));
        }
        else if (menu==3){
            break;
        }
        else{
            cout<<"Doğru bir seçim yapınız.."<<endl;
            this_thread::sleep_for(std::chrono::seconds(2));
        }
    }
    return 0;
}