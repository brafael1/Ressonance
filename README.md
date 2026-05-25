# Ressonance

Um player de música TUI moderno feito com [Ratatui](https://ratatui.rs/).

![Licença](https://img.shields.io/badge/licen%C3%A7a-GPL--3.0-blue)
![Rust](https://img.shields.io/badge/rust-2021-orange)

## Funcionalidades

- **Interface TUI** — Interface no terminal com playlist, painel "tocando agora" e controles por teclado
- **Reprodução de Áudio** — Reproduz, pausa, retoma e navega entre faixas
- **Suporte a Metadados** — Lê tags ID3 (título, artista, álbum) dos arquivos de áudio
- **Detecção de Duração** — Duração das faixas via `ffprobe`
- **Busca** — Filtra faixas por título, artista ou álbum
- **Seletor de Pasta** — Diálogo nativo para adicionar pastas de música
- **Barra de Progresso** — Indicador visual com tempo decorrido/total
- **Popups de Status** — Mensagens de feedback para ações e erros
- **Formatos Suportados** — MP3, FLAC, WAV, OGG, M4A

## Requisitos

- [Rust](https://www.rust-lang.org/) (edition 2021)
- [FFmpeg](https://ffmpeg.org/) — `ffplay` e `ffprobe` precisam estar disponíveis no `$PATH`

## Instalação

```bash
git clone https://github.com/brafael1/Ressonance.git
cd Ressonance
cargo build --release
```

O binário ficará em `target/release/music-player`.

## Uso

```bash
cargo run --release
```

### Atalhos de Teclado

| Tecla | Ação |
|---|---|
| `q` | Sair |
| `Espaço` | Reproduzir / Pausar |
| `n` | Próxima faixa |
| `p` | Faixa anterior |
| `s` | Buscar faixas |
| `a` | Adicionar pasta de músicas |
| `r` | Recarregar diretório atual |
| `↑` / `↓` | Selecionar faixa |
| `Enter` | Reproduzir faixa selecionada |
| `Esc` | Sair do modo de busca |

## Arquitetura

```
src/
├── main.rs              # Ponto de entrada & loop de eventos
├── lib.rs               # Exportações da biblioteca
├── app/
│   ├── mod.rs           # Struct App (AppState + AudioState)
│   ├── playback.rs      # Lógica de reprodução/pausa/avanço
│   ├── library.rs        # Carregamento de diretório & escaneamento de metadados
│   └── update.rs         # Sincronização do estado de áudio & avanço automático
├── player/
│   ├── app_state.rs      # Estado da UI (playlist, busca, status)
│   ├── state.rs          # Enum PlayerState
│   ├── track.rs          # Modelo de dados Track
│   ├── playlist.rs       # Gerenciamento da playlist
│   ├── metadata.rs       # Leitura de tags ID3 & descoberta de arquivos
│   └── format.rs         # Formatação de duração
├── audio/
│   ├── mod.rs             # Módulo de áudio + helper de duração via ffprobe
│   ├── state.rs           # AudioState (canal assíncrono de comandos)
│   ├── command.rs          # Enum AudioCommand
│   ├── player.rs           # Gerenciamento do processo ffplay (SIGSTOP/SIGCONT)
│   └── thread.rs           # Loop de eventos da thread de áudio
├── ui/
│   ├── mod.rs             # Renderização principal
│   ├── layout.rs          # Cálculo de layout
│   └── components/
│       ├── header.rs       # Barra de título
│       ├── playlist.rs     # Lista de faixas com busca
│       ├── now_playing.rs  # Painel "tocando agora"
│       ├── footer.rs       # Barra de status & popups
│       ├── visualizer.rs   # Lógica do visualizador (WIP)
│       └── visualizer_render.rs
├── input.rs              # Manipulador de entrada do teclado
└── terminal.rs           # Setup/teardown do terminal
```

A reprodução de áudio usa `ffplay` em uma thread dedicada com `SIGSTOP`/`SIGCONT` para pausar/retomar. Os comandos são enviados via um canal Tokio unbounded.

## Licença

[GPL-3.0](./LICENSE)