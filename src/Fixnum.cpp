/*
 * Fixnum.cpp - 
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
 *  $Id: Fixnum.cpp 183 2008-07-04 06:19:28Z higepon $
 */

#include "Object.h"
#include "Object-inl.h"
#include "Ratnum.h"
#include "Flonum.h"
#include "Compnum.h"
#include "Fixnum.h"
#include "Bignum.h"

using namespace scheme;

Object Fixnum::atan(fixedint n)
{
    return Object::makeFlonum(::atan(static_cast<double>(n)));
}

Object Fixnum::asin(fixedint n)
{
    return Object::makeFlonum(::asin(static_cast<double>(n)));
}

Object Fixnum::acos(fixedint n)
{
    return Object::makeFlonum(::acos(static_cast<double>(n)));
}

Object Fixnum::tan(fixedint n)
{
    if (n == 0) {
        // exact 0
        return Object::makeFixnum(0);
    } else {
        return Object::makeFlonum(::tan(static_cast<double>(n)));
    }
}

Object Fixnum::sin(fixedint n)
{
    if (n == 0) {
        // exact 0
        return Object::makeFixnum(0);
    } else {
        return Object::makeFlonum(::sin(static_cast<double>(n)));
    }
}

Object Fixnum::cos(fixedint n)
{
    if (n == 0) {
        // exact 1
        return Object::makeFixnum(1);
    } else {
        return Object::makeFlonum(::cos(static_cast<double>(n)));
    }
}

Object Fixnum::exp(fixedint n)
{
    if (n == 0) {
        // exact 1
        return Object::makeFixnum(1);
    } else {
        return Object::makeFlonum(::exp(static_cast<double>(n)));
    }
}

Object Fixnum::log(fixedint n)
{
    if (n == 1) {
        // exact 0
        return Object::makeFixnum(0);
    } else {
        return Object::makeFlonum(::log(static_cast<double>(n)));
    }
}

Object Fixnum::integerDiv(fixedint x, fixedint y)
{
    return Bignum::makeInteger(fxdiv(x, y));
}

Object Fixnum::sqrt(Object n)
{
    MOSH_ASSERT(n.isFixnum());
    const fixedint value = n.toFixnum();
    if (value == 0) {
        return n;
    } else if (value > 0) {
        const double root = ::sqrt(static_cast<double>(value));
        const int rootAsInt = static_cast<int>(floor(root));
        // exact
        if (rootAsInt * rootAsInt == value) {
            return Object::makeFixnum(rootAsInt);
        } else {
            return Object::makeFlonum(root);
        }
    } else { // negative
        const double root = ::sqrt(static_cast<double>(-value));
        const fixedint rootAsInt = static_cast<fixedint>(floor(root));
        // exact
        if (rootAsInt * rootAsInt == -value) {
            return Object::makeCompnum(Object::makeFixnum(0), Object::makeFixnum(rootAsInt));
        } else {
            return Object::makeCompnum(Object::makeFlonum(0.0), Object::makeFlonum(root));
        }
    }
}

// N.B. the result of abs can be Bignum
Object Fixnum::abs(fixedint n)
{
    return Bignum::makeInteger(::abs(n));
}
