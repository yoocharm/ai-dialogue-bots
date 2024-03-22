
# ai-dialogue-bots

This is a Proof of Concept (PoC) demonstrating how two bots can engage in an autonomous conversational "banter" using technologies such as [Large Language Model (LLM)](https://en.wikipedia.org/wiki/Large_language_model) and [Text To Speech (TTS)](https://simple.wikipedia.org/wiki/Text_to_speech). It exploits [NATS Jetstream](https://docs.nats.io/nats-concepts/jetstream) for message routing, [ollama](https://ollama.com/) as the LLM of choice and PlayHT API available at [play.ht](https://play.ht) for TTS speech synthesis.

[![Build Status](https://github.com/yoocharm/ai-dialogue-bots/workflows/CI/badge.svg)](https://github.com/yoocharm/ai-dialogue-bots/actions?query=workflow%3ACI) [![go.dev reference](https://img.shields.io/badge/go.dev-reference-007d9c?logo=go&logoColor=white&style=flat-square)](https://pkg.go.dev/github.com/yoocharm/ai-dialogue-bots)[![License: Apache-2.0](https://img.shields.io/badge/License-Apache--2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

This project was built solely for educational purposes and, hence, may be ridden with bugs, inefficiencies, etc. You should consider this project as highly experimental and use it at your own risk.

A sample conversation between the bots can be viewed here:

[![Bot Banter](http://img.youtube.com/vi/w01DsREeKU4/0.jpg)](http://www.youtube.com/watch?v=w01DsREeKU4)

Detailed explanation of the bot conversation flow and the project's architecture, along with the tasks, Goroutines, and Channels involved have been elaborated in the subsequent sections of this file. Step by step instructions to meet the prerequisites and run the project have also been provided for your convenience.

Please note that you will be required to install [nats](https://nats.io/), [ollama](https://ollama.com/), and create an account on [play.ht](https://play.ht/) along with some platform specific sound/audio libraries (instructions provided for Ubuntu Linux).

Once everything is set up, you can run the bots using the provided instructions. Do ensure that `gobot` is prompted first, as it kickstarts the conversation and `rusbot` will wait for `gobot` before it responds.