/*
 * RegexpProcedures.cpp - <regexp> procedures.
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
 *  $Id: RegexpProcedures.cpp 183 2008-07-04 06:19:28Z higepon $
 */

#include "Object.h"
#include "Object-inl.h"
#include "Pair.h"
#include "Pair-inl.h"
#include "SString.h"
#include "Symbol.h"
#include "Regexp.h"
#include "VM.h"
#include "RegexpProcedures.h"
#include "ProcedureMacro.h"

using namespace scheme;

Object scheme::regexpReplaceEx(VM* theVM, int argc, const Object* argv)
{
    DeclareProcedureName("regexp-replace");
    checkArgumentLength(3);

    argumentAsRegexp(0, regexp);
    argumentCheckString(1, text);
    argumentCheckString(2, sub);

    return regexp->replace(text, sub);
}

Object scheme::regexpReplaceAllEx(VM* theVM, int argc, const Object* argv)
{
    DeclareProcedureName("regexp-replace-all");
    checkArgumentLength(3);

    argumentAsRegexp(0, regexp);
    argumentCheckString(1, text);
    argumentCheckString(2, sub);

    return regexp->replaceAll(text, sub);
}

Object scheme::rxmatchEx(VM* theVM, int argc, const Object* argv)
{
    DeclareProcedureName("rxmatch");
    checkArgumentLength(2);
    argumentAsRegexp(0, regexp);
    argumentAsString(1, text);

    const Object returnValue = regexp->match(text->data());
    if (regexp->isErrorOccured()) {
        callAssertionViolationAfter(theVM,
                                    procedureName,
                                    regexp->errorMessage(),
                                    regexp->irritants());
        return Object::Undef;
    } else {
        return returnValue;
    }
}

Object scheme::regexpPEx(VM* theVM, int argc, const Object* argv)
{
    DeclareProcedureName("regexp?");
    checkArgumentLength(1);
    return Object::makeBool(argv[0].isRegexp());
}

Object scheme::regexpTostringEx(VM* theVM, int argc, const Object* argv)
{
    DeclareProcedureName("regexp->string");
    argumentAsRegexp(0, regexp);
    return Object::makeString(regexp->pattern());
}

Object scheme::rxmatchStartEx(VM* theVM, int argc, const Object* argv)
{
    DeclareProcedureName("rxmatxh-start");
    checkArgumentLengthBetween(1, 2);
    if (argv[0].isFalse()) {
        return Object::False;
    }

    argumentAsRegMatch(0, regMatch);
    Object returnValue;
    if (argc == 2) {
        argumentAsFixnumToInt(1, index);
        returnValue = Object::makeFixnum(regMatch->matchStart(index));
    } else {
        returnValue = Object::makeFixnum(regMatch->matchStart(0));
    }
    if (regMatch->isErrorOccured()) {
        callAssertionViolationAfter(theVM,
                                    procedureName,
                                    regMatch->errorMessage(),
                                    regMatch->irritants());
        return Object::Undef;
    } else {
        return returnValue;
    }
}

Object scheme::rxmatchEndEx(VM* theVM, int argc, const Object* argv)
{
    DeclareProcedureName("rxmatch-end");
    checkArgumentLengthBetween(1, 2);
    if (argv[0].isFalse()) {
        return Object::False;
    }

    argumentAsRegMatch(0, regMatch);
    Object returnValue;
    if (argc == 2) {
        argumentAsFixnumToInt(1, index);
        returnValue = Object::makeFixnum(regMatch->matchEnd(index));
    } else {
        returnValue = Object::makeFixnum(regMatch->matchEnd(0));
    }

    if (regMatch->isErrorOccured()) {
        callAssertionViolationAfter(theVM,
                                    procedureName,
                                    regMatch->errorMessage(),
                                    regMatch->irritants());
        return Object::Undef;
    } else {
        return returnValue;
    }
}

Object scheme::rxmatchAfterEx(VM* theVM, int argc, const Object* argv)
{
    DeclareProcedureName("rxmatch-after");
    checkArgumentLengthBetween(1, 2);
    if (argv[0].isFalse()) {
        return Object::False;
    }

    argumentAsRegMatch(0, regMatch);
    Object returnValue;
    if (argc == 2) {
        argumentAsFixnumToInt(1, index);
        returnValue = regMatch->matchAfter(index);
    } else {
        returnValue = regMatch->matchAfter(0);
    }

    if (regMatch->isErrorOccured()) {
        callAssertionViolationAfter(theVM,
                                    procedureName,
                                    regMatch->errorMessage(),
                                    regMatch->irritants());
        return Object::Undef;
    } else {
        return returnValue;
    }
}

Object scheme::rxmatchBeforeEx(VM* theVM, int argc, const Object* argv)
{
    DeclareProcedureName("rxmatch-before");
    checkArgumentLengthBetween(1, 2);
    if (argv[0].isFalse()) {
        return Object::False;
    }

    argumentAsRegMatch(0, regMatch);
    Object returnValue;
    if (argc == 2) {
        argumentAsFixnumToInt(1, index);
        returnValue = regMatch->matchBefore(index);
    } else {
        returnValue = regMatch->matchBefore(0);
    }

    if (regMatch->isErrorOccured()) {
        callAssertionViolationAfter(theVM,
                                    procedureName,
                                    regMatch->errorMessage(),
                                    regMatch->irritants());
        return Object::Undef;
    } else {
        return returnValue;
    }
}

Object scheme::rxmatchSubstringEx(VM* theVM, int argc, const Object* argv)
{
    DeclareProcedureName("rxmatch-substring");
    checkArgumentLengthBetween(1, 2);
    if (argv[0].isFalse()) {
        return Object::False;
    }

    argumentAsRegMatch(0, regMatch);
    Object returnValue;
    if (argc == 2) {
        argumentAsFixnumToInt(1, index);
        returnValue = regMatch->matchSubString(index);
    } else {
        returnValue = regMatch->matchSubString(0);
    }

    if (regMatch->isErrorOccured()) {
        callAssertionViolationAfter(theVM,
                                    procedureName,
                                    regMatch->errorMessage(),
                                    regMatch->irritants());
        return Object::Undef;
    } else {
        return returnValue;
    }
}

Object scheme::regMatchProxy(VM* theVM, int argc, const Object* argv)
{
    if (argc == 2 && argv[1] == Symbol::AFTER) {
        return rxmatchAfterEx(theVM, 1, argv);
    } else if (argc == 2 && argv[1] == Symbol::BEFORE) {
        return rxmatchBeforeEx(theVM, 1, argv);
    } else {
        return rxmatchSubstringEx(theVM, argc, argv);
    }
}
