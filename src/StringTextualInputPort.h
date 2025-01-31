/*
 * StringTextualInputPort.h - 
 *
 *   Copyright (c) 2008  Higepon(Taro Minowa)  <higepon@users.sourceforge.jp>
 *
 *   Redistribution and use in source and binary forms, with or without
 *   modification, are permitted provided that the following conditions
 *   are met:
 *
 *   1. Redistributions of source code must retain the above copyright
 *      notice, this list of conditions and the following disclaimer.
 *
 *   2. Redistributions in binary form must reproduce the above copyright
 *      notice, this list of conditions and the following disclaimer in the
 *      documentation and/or other materials provided with the distribution.
 *
 *   THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 *   "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 *   LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 *   A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 *   OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 *   SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED
 *   TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 *   PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
 *   LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
 *   NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
 *   SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 *
 *  $Id: StringTextualInputPort.h 261 2008-07-25 06:16:44Z higepon $
 */

#ifndef SCHEME_STRING_TEXTUAL_INPUT_PORT_
#define SCHEME_STRING_TEXTUAL_INPUT_PORT_

#include "TextualInputPort.h"

namespace scheme {

class StringTextualInputPort : public TextualInputPort
{
public:
    explicit StringTextualInputPort(const ucs4string& str);
    ~StringTextualInputPort() override;

    ucs4char getChar() override;
    void unGetChar(ucs4char c) override;
    ucs4string toString() override;
    int close() override;
    bool isClosed() const override;
    bool hasPosition() const override;
    bool hasSetPosition() const override;
    Object position() const override;
    bool setPosition(int64_t position) override;
    int getLineNo() const override;
    Transcoder* transcoder() const override;

private:
    bool isClosed_{false};
    ucs4string buffer_;
    ucs4string::size_type index_{0};
    int lineNo_{1};
};

} // namespace scheme

#endif // SCHEME_STRING_TEXTUAL_INPUT_PORT_
