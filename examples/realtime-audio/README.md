## Overview

This example sends the `audio.wav` to the Realtime API over WebSocket and saves the returned audio chunks as
`response.wav` in the example directory.

Set `OPENAI_API_KEY`, then run `cargo run -p realtime-audio` from the repository root.

The bundled [audio.wav](audio.wav) was recorded by [VoxDai](https://github.com/VoxDai) and is provided under
the [MIT License](../../LICENSE) of this repository.

## Sample Output

```
Client secret created
Realtime WebSocket connected
writer: sent session.update
writer: sent input_audio_buffer.append
writer: sent input_audio_buffer.commit
writer: sent response.create
reader: received session.created
reader: received session.updated
reader: received input_audio_buffer.committed
reader: received response.created
reader: received response.output_item.added
reader: received conversation.item.added
reader: received response.content_part.added
reader: saved examples/realtime-audio/response.wav
reader: transcript: Hey, that’s the million-dollar question...
reader: received response.content_part.done
reader: received conversation.item.done
reader: received response.output_item.done
reader: response completed
```
