
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

	url := os.Getenv("NATS_URL")
	if url == "" {
		url = nats.DefaultURL
	}

	// NOTE: we could also provide functional options
	// instead of creating stream from Config.
	jetConf := jet.Config{
		StreamURL:   url,
		StreamName:  streamName,
		DurableName: botName,
		PubSubject:  pubSubject,
		SubSubject:  subSubject,
	}
	s, err := jet.NewStream(ctx, jetConf)
	if err != nil {
		log.Fatalf("failed creating JetStream: %v", err)
	}

	// NOTE: we could also provide functional options
	// instead of creating llm from Config.
	llmConf := llm.Config{
		ModelName:  modelName,
		HistSize:   histSize,
		SeedPrompt: seedPrompt,
	}
	l, err := llm.New(llmConf)
	if err != nil {
		log.Fatal("failed creating LLM client: ", err)
	}

	// NOTE: we could also provide functional options
	// instead of creating tts from Config.
	ttsConf := tts.DefaultConfig()
	ttsConf.VoiceID = voiceID
	t, err := tts.New(*ttsConf)
	if err != nil {
		log.Fatal("failed creating TTS client: ", err)
	}

	pipeReader, pipeWriter := io.Pipe()

	log.Println("created pipe reader")

	// chunks for TTS stream
	ttsChunks := make(chan []byte, 100)
	// chunk for JetStream
	jetChunks := make(chan []byte, 100)
	prompts := make(chan string)
	// ttsDone for signalling we're done talking
	ttsDone := make(chan struct{})

	g, ctx := errgroup.WithContext(ctx)

	log.Println("launching workers")

	g.Go(func() error {
		return t.Stream(ctx, pipeWriter, ttsChunks, ttsDone)
	})
	g.Go(func() error {
		return l.Stream(ctx, prompts, jetChunks, ttsChunks)
	})
	g.Go(func() error {
		return s.Reader.Read(ctx, prompts)
	})
	g.Go(func() error {