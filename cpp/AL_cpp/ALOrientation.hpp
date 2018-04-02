#pragma once

#include <glm/glm.hpp>

namespace glmlv {

struct ALOrientation {
    glm::vec3 at, up; // NOTE: The order matters!
};
static_assert(sizeof(ALOrientation) == 6*sizeof(float), "");

} // namespace glmlv

