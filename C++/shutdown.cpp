#include <iostream>
#include <thread>
#include <chrono>
#include <cstdlib>
using namespace std;

#if __linux__
    const string shutdown = "shutdown -h +";
    const string cancel = "shutdown -c";
    const int multiply = 1;
#elif _WIN32
    const string shutdown = "shutdown /s /t ";
    const string cancel = "shutdown /a";
    const int multiply = 60;
#endif

int main(){    
    int menu, choice;
    while(true){
        system("cls||clear");
        cout<<"1- Set the shutdown time\n2- Cancel the shutdown\n3- Exit\n\n>>>";
        cin>>menu;
        system("cls||clear");
        if (menu==1){
            cout<<"How many minutes later do you want the computer to shutdown: ";
            cin>>choice;
            system((shutdown+to_string(multiply*choice)).c_str());
        }
        else if (menu==2){
            system(cancel.c_str());
            cout<<"The shutdown has been canceled.."<<endl;
            std::this_thread::sleep_for(std::chrono::seconds(2));
        }
        else if (menu==3){
            break;
        }
        else{
            cout<<"Please make a valid choice.."<<endl;
            this_thread::sleep_for(std::chrono::seconds(2));
        }
    }
    return 0;
}