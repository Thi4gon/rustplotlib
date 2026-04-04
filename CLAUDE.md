# RustPlotLib — Contexto para Agents

Reimplementação completa do matplotlib em Rust puro. Drop-in replacement via PyO3.

## Linguagem

- Sempre responder em português (pt-BR)

## Estado Atual — v4.0.0

- 40+ plot types 2D, 7 plot types 3D
- 70+ colormaps (35 base + 35 reversed)
- 21 módulos Python compatíveis com API matplotlib
- 267 testes passando
- Output: PNG (tiny-skia), SVG nativo, PDF, GIF, janela interativa
- Performance: até 30x mais rápido que matplotlib
- Zero blocos `unsafe` em todo o código Rust

## Estrutura do Projeto

```
src/                          # Core Rust
├── lib.rs                    # Registro de módulos PyO3
├── figure.rs                 # Figure + bindings PyO3 (savefig, render)
├── axes.rs                   # Axes 2D (~1500 linhas)
├── axes3d.rs                 # Axes 3D
├── projection3d.rs           # Câmera + projeção 3D
├── transforms.rs             # Coordenadas de dados → pixels
├── colors.rs                 # Parse de cores (named, hex, RGB, RGBA)
├── text.rs                   # Renderização de texto (ab_glyph + DejaVu Sans)
├── ticker.rs                 # Cálculo automático de ticks
├── svg_renderer.rs           # Output SVG nativo
├── window.rs                 # Janela interativa (winit + softbuffer)
└── artists/                  # Um arquivo por tipo de gráfico (35 arquivos)
    ├── mod.rs                # Trait Artist
    ├── line2d.rs, scatter.rs, bar.rs, hist.rs, image.rs, ...
    └── legend.rs

python/rustplotlib/           # Camada Python
├── pyplot.py                 # API principal (drop-in matplotlib.pyplot)
├── style/                    # Temas (dark_background, ggplot, seaborn, bmh, fivethirtyeight)
├── animation.py              # FuncAnimation + GIF
├── ticker.py                 # 12 Formatters + 10 Locators funcionais
├── dates.py, colors.py, patches.py, ...
└── mpl_toolkits/mplot3d/     # Suporte 3D

tests/                        # Testes Python (267 testes)
```

## Stack Técnica

| Componente | Tecnologia |
|---|---|
| Renderização 2D | tiny-skia |
| Fontes | ab_glyph + DejaVu Sans embutida |
| PNG | crate png |
| SVG | Renderer customizado |
| Janela | winit + softbuffer |
| 3D | Projeção ortográfica customizada |
| Bindings Python | PyO3 + maturin |
| NumPy interop | crate numpy (PyO3) |

## Como Adicionar Features

### Novo tipo de gráfico (caminho completo):
1. `src/artists/novo_plot.rs` — implementar trait Artist (draw, data_bounds, legend_label, legend_color)
2. `src/artists/mod.rs` — registrar módulo
3. `src/axes.rs` — adicionar método no Axes
4. `src/figure.rs` — adicionar binding PyO3
5. `python/rustplotlib/pyplot.py` — wrapper Python (AxesProxy + função module-level)
6. `tests/test_*.py` — testes
7. `ROADMAP.md` — marcar checkbox `[x]`
8. `README.md` — atualizar tabela de features

### Feature simples (só Python):
Se pode ser construída em cima das funções Rust existentes, implementar direto em `pyplot.py`.

## Comandos de Desenvolvimento

```bash
cd /Users/thi4gon/Documents/workspace/matplot
source .venv/bin/activate
maturin develop              # Build debug
maturin develop --release    # Build release (benchmarks)
pytest tests/ -v             # Rodar todos os testes
cargo check                  # Verificar compilação Rust
```

## Regras de Código

- **Zero `unsafe`** — sem exceção
- **Sem `.unwrap()` em input do usuário** — usar proper error handling
- **Validar inputs na fronteira PyO3** — path traversal, dimensões, tipos
- **Todo feature novo precisa de teste**
- **Manter compatibilidade com API do matplotlib** — mesmos nomes de funções e parâmetros
- **Atualizar ROADMAP.md e README.md** ao completar features

## Roadmap Ativo (próximo: v5.0.0)

Próximos itens prioritários (ver ROADMAP.md para lista completa):

1. **Jupyter inline backend** — rich display protocol, _repr_png_
2. **Backends interativos** — Qt, GTK, macOS native
3. **Widgets funcionais** — Slider, Button, CheckButtons com renderização real
4. **Features interativas** — mouse events, zoom/pan, rotação 3D
5. **Triangulation plots** — tricontour, tricontourf, tripcolor (atualmente stubs)

## Git

- Branch principal: `master`
- Repo: `https://github.com/Thi4gon/rustplotlib`
- Commits seguem conventional commits: `feat:`, `fix:`, `docs:`, `release:`
