#pragma once

#include <AL/al.h>
#include <AL/alc.h>
#include <glm/glm.hpp>

#include "al_check_error.hpp"
#include "ALOrientation.hpp"

namespace glmlv {

struct ALHandle {
    ALCdevice* m_Device = nullptr;
    ALCcontext* m_Context = nullptr;

    ~ALHandle() {
        alcDestroyContext(m_Context);
        alcCloseDevice(m_Device);
    }
    ALHandle() {
        m_Device = alcOpenDevice(nullptr);
        if(!m_Device) {
            auto msg = "Unable to init OpenAL default device.";
            std::cerr << msg << std::endl;
            throw std::runtime_error(msg);
        }
        checkAlcError(m_Device);
        std::cout << "Opened OpenAL default device" << std::endl;
        m_Context = alcCreateContext(m_Device, nullptr);
        alcMakeContextCurrent(m_Context);
        checkAlcError(m_Device);

        alGetError(); // clear error code
    }
    ALHandle(const ALHandle& o) = delete;
    ALHandle& operator=(const ALHandle& o) = delete;
    ALHandle(ALHandle&& o) {
        std::swap(m_Device, o.m_Device);
        std::swap(m_Context, o.m_Context);
    }
    ALHandle& operator=(ALHandle&& o) {
        std::swap(m_Device, o.m_Device);
        std::swap(m_Context, o.m_Context);
        return *this;
    }
    void setListenerGain(float gain) const { alListenerf( AL_GAIN, gain); checkAlError(); }
    float getListenerGain() const { float v; alGetListenerf(AL_GAIN, &v); checkAlError(); return v; }
    void setListenerPosition (const glm::vec3& v) const { alListenerfv(AL_POSITION,  &v[0]); checkAlError(); }
    void setListenerVelocity (const glm::vec3& v) const { alListenerfv(AL_VELOCITY,  &v[0]); checkAlError(); }
    void setListenerOrientation(const ALOrientation& v) const { alListenerfv(AL_ORIENTATION, &v.at[0]); checkAlError(); }
    ALOrientation getListenerOrientation() const { ALOrientation v; alGetListenerfv(AL_ORIENTATION, &v.at[0]); checkAlError(); return v; }
    glm::vec3 getListenerPosition() const { glm::vec3 v; alGetListenerfv(AL_POSITION,  &v[0]); checkAlError(); return v; }
    glm::vec3 getListenerVelocity() const { glm::vec3 v; alGetListenerfv(AL_VELOCITY,  &v[0]); checkAlError(); return v; }
};

} // namespace glmlv
