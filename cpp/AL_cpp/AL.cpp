#include <glmlv/al_check_error.hpp>
#include <AL/al.h>
#include <AL/alc.h>
#include <iostream>

namespace glmlv {

bool checkAlError() {
    auto e = alGetError();
    if(e != AL_NO_ERROR) {
        std::cerr << "OpenAL: An error was generated: " << e << std::endl;
        return false;
    }
    return true;
}
bool checkAlcError(ALCdevice* dev) {
    auto e = alcGetError(dev);
    if(e != ALC_NO_ERROR) {
        std::cerr << "ALC: An error was generated: " << e << std::endl;
        return false;
    }
    return true;
}

}
