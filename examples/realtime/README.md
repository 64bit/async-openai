## Overview

The example takes input from stdin, for every input two client events are sent:
1.  "conversation.item.create" with content of type "input_text"
2.  "response.create" 

All the output happens on stderr, so conversation can be continued on stdin. To stop type "quit" and press enter.

Code is based on https://github.com/snapview/tokio-tungstenite/blob/master/examples/client.rs

## Sample Output 

```
WebSocket handshake complete
session.created                  | 
age of sun?
conversation.item.created        | 
response.created                 | 
response.output_item.added       | 
conversation.item.created        | 
response.content_part.added      | 
response.audio_transcript.delta  | The
response.audio_transcript.delta  | Sun
response.audio_transcript.delta  | is
response.audio_transcript.delta  | about
response.audio_transcript.delta  | 
response.audio_transcript.delta  | 4
response.audio_transcript.delta  | .
response.audio.delta             | 
response.audio.delta             | 
response.audio_transcript.delta  | 6
response.audio.delta             | 
response.audio_transcript.delta  | billion
response.audio.delta             | 
response.audio.delta             | 
response.audio_transcript.delta  | years
response.audio_transcript.delta  | old
response.audio_transcript.delta  | .
response.audio_transcript.delta  | It
response.audio.delta             | 
response.audio.delta             | 
response.audio_transcript.delta  | formed
response.audio.delta             | 
response.audio_transcript.delta  | from
response.audio_transcript.delta  | the
response.audio_transcript.delta  | gravitational
response.audio_transcript.delta  | collapse
response.audio_transcript.delta  | of
response.audio_transcript.delta  | a
response.audio.delta             | 
response.audio.delta             | 
response.audio.delta             | 
response.audio.delta             | 
response.audio.delta             | 
response.audio.delta             | 
response.audio.delta             | 
response.audio.delta             | 
response.audio_transcript.delta  | region
response.audio.delta             | 
response.audio_transcript.delta  | within
response.audio_transcript.delta  | a
response.audio_transcript.delta  | large
response.audio_transcript.delta  | molecular
response.audio_transcript.delta  | cloud
response.audio_transcript.delta  | .
response.audio_transcript.delta  | It
response.audio.delta             | 
response.audio.delta             | 
response.audio.delta             | 
response.audio.delta             | 
response.audio.delta             | 
response.audio.delta             | 
response.audio.delta             | 
response.audio.delta             | 
response.audio_transcript.delta  | 's
response.audio.delta             | 
response.audio_transcript.delta  | currently
response.audio_transcript.delta  | in
response.audio_transcript.delta  | the
response.audio_transcript.delta  | middle
response.audio_transcript.delta  | of
response.audio_transcript.delta  | its
response.audio_transcript.delta  | life
response.audio_transcript.delta  | cycle
response.audio_transcript.delta  | ,
response.audio.delta             | 
response.audio.delta             | 
response.audio.delta             | 
response.audio.delta             | 
response.audio.delta             | 
response.audio.delta             | 
response.audio.delta             | 
response.audio.delta             | 
response.audio_transcript.delta  | expected
response.audio.delta             | 
response.audio_transcript.delta  | to
response.audio_transcript.delta  | last
response.audio_transcript.delta  | for
response.audio.delta             | 
response.audio.delta             | 
response.audio.delta             | 
response.audio.delta             | 
response.audio.delta             | 
response.audio.delta             | 
response.audio.delta             | 
response.audio.delta             | 
response.audio_transcript.delta  | another
response.audio.delta             | 
response.audio_transcript.delta  | 
response.audio_transcript.delta  | 5
response.audio_transcript.delta  | billion
response.audio_transcript.delta  | years
response.audio_transcript.delta  | or
response.audio_transcript.delta  | so
response.audio_transcript.delta  | .
response.audio.delta             | 
response.audio.delta             | 
response.audio.delta             | 
response.audio.delta             | 
response.audio.delta             | 
response.audio.delta             | 
response.audio.delta             | 
response.audio.delta             | 
response.audio.delta             | 
response.audio.delta             | 
response.audio.delta             | 
response.audio.delta             | 
response.audio.delta             | 
response.audio.delta             | 
response.audio.done              | 
response.audio_transcript.done   | 
response.content_part.done       | 
response.output_item.done        | [Some(Assistant)]: The Sun is about 4.6 billion years old. It formed from the gravitational collapse of a region within a large molecular cloud. It's currently in the middle of its life cycle, expected to last for another 5 billion years or so.

response.done                    | 
rate_limits.updated              | 
quit
```
