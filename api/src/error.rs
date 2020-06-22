//! WebSocket API - Errors.

//
// MIT License
//
// Copyright (c) 2019 Stegos AG
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use failure::Fail;

#[derive(Debug, Fail)]
pub enum KeyError {
    #[fail(display = "Input/Output Error: file={}, error={:?}", _0, _1)]
    InputOutputError(String, std::io::Error),
    #[fail(display = "Failed to parse key in file: file={}, error={:?}", _0, _1)]
    ParseError(String, base64::DecodeError),
    #[fail(display = "Failure decoding payload: error={}", _0)]
    DecodeError(base64::DecodeError),
    #[fail(display = "Invalid key size: expectet={}, actual={}", _0, _1)]
    InvalidKeySize(usize, usize),
}
