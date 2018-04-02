#pragma once

#include <AL/al.h>
#include <glm/glm.hpp>
#include "al_check_error.hpp"
#include "ALBuffer.hpp"

namespace glmlv {

class ALSource {
    ALuint m_ALId = 0;
public:
    ~ALSource() {
        alDeleteSources(1, &m_ALId);
    }
    ALSource() {
        alGenSources(1, &m_ALId);
        assert(m_ALId);
        checkAlError();
    }
    ALSource(const ALBuffer& b): ALSource() {
        setBuffer(b);
    }
    ALSource(const ALSource& o) = delete;
    ALSource& operator=(const ALSource& o) = delete;
    ALSource(ALSource&& o) { std::swap(m_ALId, o.m_ALId); }
    ALSource& operator=(ALSource&& o) { std::swap(m_ALId, o.m_ALId); return *this; }

    ALuint alId() const { return m_ALId; }
    void setBuffer(const ALBuffer& b) const {
        alSourcei(m_ALId, AL_BUFFER, b.alId());
        checkAlError();
    }
    void setLooping(bool loop) const {
        alSourcei(m_ALId, AL_LOOPING, loop);
        checkAlError();
    }
    void play()   const { alSourcePlay  (m_ALId); checkAlError(); }
    void pause()  const { alSourcePause (m_ALId); checkAlError(); }
    void stop()   const { alSourceStop  (m_ALId); checkAlError(); }
    void rewind() const { alSourceRewind(m_ALId); checkAlError(); }
    void setGain(float gain) const { alSourcef(m_ALId, AL_GAIN, gain); checkAlError(); }
    float getGain() const { float v; alGetSourcef(m_ALId, AL_GAIN, &v); checkAlError(); return v; }
    void setPosition (const glm::vec3& v) const { alSourcefv(m_ALId, AL_POSITION,  &v[0]); checkAlError(); }
    void setVelocity (const glm::vec3& v) const { alSourcefv(m_ALId, AL_VELOCITY,  &v[0]); checkAlError(); }
    void setDirection(const glm::vec3& v) const { alSourcefv(m_ALId, AL_DIRECTION, &v[0]); checkAlError(); }
    glm::vec3 getPosition () const { glm::vec3 v; alGetSourcefv(m_ALId, AL_POSITION,  &v[0]); checkAlError(); return v; }
    glm::vec3 getVelocity () const { glm::vec3 v; alGetSourcefv(m_ALId, AL_VELOCITY,  &v[0]); checkAlError(); return v; }
    glm::vec3 getDirection() const { glm::vec3 v; alGetSourcefv(m_ALId, AL_DIRECTION, &v[0]); checkAlError(); return v; }
    enum class State { Playing, Paused, Stopped };
    State getState() const {
        ALint i;
        alGetSourcei(m_ALId, AL_SOURCE_STATE, &i);
        checkAlError();
        switch(i) {
        case AL_PLAYING: return State::Playing;
        case AL_PAUSED : return State::Paused;
        case AL_STOPPED: return State::Stopped;
        }
        assert(false && "alGetSourcei returned an invalid state!");
    }
    float  getPlaybackPositionInSeconds() const { ALfloat v; alGetSourcef(m_ALId, AL_SEC_OFFSET, &v);  checkAlError(); return v; }
    size_t getPlaybackPositionInSamples() const { ALint i; alGetSourcei(m_ALId, AL_SAMPLE_OFFSET, &i); checkAlError(); return i; }
    size_t getPlaybackPositionInBytes()   const { ALint i; alGetSourcei(m_ALId, AL_BYTE_OFFSET, &i);   checkAlError(); return i; }
};

} // namespace glmlv
