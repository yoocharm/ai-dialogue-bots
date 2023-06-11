
package main

import (
	"bufio"
	"context"
	"flag"
	"fmt"
	"io"
	"log"
	"os"
	"os/signal"
	"time"

	"github.com/milosgajdos/bot-banter/gobot/jet"
	"github.com/milosgajdos/bot-banter/gobot/llm"
	"github.com/milosgajdos/bot-banter/gobot/tts"

	"github.com/gopxl/beep"
	"github.com/gopxl/beep/mp3"
	"github.com/gopxl/beep/speaker"
	"github.com/nats-io/nats.go"
	"golang.org/x/sync/errgroup"
)

var (
	histSize   uint
	seedPrompt string
	modelName  string
	streamName string
	botName    string
	pubSubject string
	subSubject string
	voiceID    string
)

func init() {
	flag.UintVar(&histSize, "hist-size", defaultHistSize, "chat history size")
	flag.StringVar(&seedPrompt, "seed-prompt", defaultSeedPrompt, "seed prompt")
	flag.StringVar(&modelName, "model-name", defaultModelName, "LLM model")
	flag.StringVar(&streamName, "stream-name", defaultStreamName, "jetstream name")
	flag.StringVar(&botName, "bot-name", defaultBotName, "bot name")
	flag.StringVar(&pubSubject, "pub-subject", defaultPubSubject, "bot publish subject")
	flag.StringVar(&subSubject, "sub-subject", defaultSubSubject, "bot subscribe subject")
	flag.StringVar(&voiceID, "voice-id", defaultVoiceID, "play HT voice ID")
}

func main() {
	flag.Parse()

	ctx := context.Background()
	ctx, cancel := context.WithCancel(ctx)
	sigTrap := make(chan os.Signal, 1)
	signal.Notify(sigTrap, os.Interrupt)
	defer func() {
		signal.Stop(sigTrap)
		cancel()
	}()
	go func() {
		<-sigTrap
		log.Println("shutting down: received SIGINT...")
		cancel()
	}()