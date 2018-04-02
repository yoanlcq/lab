#pragma once

#include <AL/alc.h>

namespace glmlv {

bool checkAlError();
bool checkAlcError(ALCdevice*);

} // namespace glmlv
