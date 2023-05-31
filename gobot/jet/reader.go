
package jet

import (
	"context"
	"fmt"
	"log"

	"github.com/nats-io/nats.go/jetstream"
)

type Reader struct {
	cons    jetstream.Consumer
	subject string
}