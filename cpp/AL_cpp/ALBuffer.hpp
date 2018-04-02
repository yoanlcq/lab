#pragma once

#include <AL/al.h>
#include "al_check_error.hpp"
#include "wav.hpp"

namespace glmlv {

class ALBuffer {
    ALuint m_ALId = 0;
public:
    ~ALBuffer() {
        alDeleteBuffers(1, &m_ALId);
    }
    ALBuffer() {
        alGenBuffers(1, &m_ALId);
        assert(m_ALId);
        checkAlError();
    }
    ALBuffer(const PcmWav& wav): ALBuffer() {
        loadWav(wav);
    }
    ALBuffer(const ALBuffer& o) = delete;
    ALBuffer& operator=(const ALBuffer& o) = delete;
    ALBuffer(ALBuffer&& o) { std::swap(m_ALId, o.m_ALId); }
    ALBuffer& operator=(ALBuffer&& o) { std::swap(m_ALId, o.m_ALId); return *this; }

    ALuint alId() const { return m_ALId; }
    void loadWav(const PcmWav& wav) const {
        ALenum format = -1;
        /**/ if(wav.channel_count==1 && wav.bits_per_sample== 8) format = AL_FORMAT_MONO8;
        else if(wav.channel_count==1 && wav.bits_per_sample==16) format = AL_FORMAT_MONO16;
        else if(wav.channel_count==2 && wav.bits_per_sample== 8) format = AL_FORMAT_STEREO8;
        else if(wav.channel_count==2 && wav.bits_per_sample==16) format = AL_FORMAT_STEREO16;
        assert(format != -1 && "No suitable OpenAL format found!");

        alBufferData(m_ALId, format, wav.data.data(), wav.data.size(), wav.sample_rate);
        checkAlError();
    }
};

} // namespace glmlv
