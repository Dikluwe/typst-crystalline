# ADR-0115 — Infraestrutura de benchmark para o scanner/lexer

**Estado:** `EM VIGOR` (Passo 441 — infra de benchmark criada; `cargo bench` funcional e reprodutível).
**Decisão do dono (registada):** usar `criterion` 0.5 em `benches/` na raiz do workspace para medir o tempo de lex/tokenização do scanner; criar corpus de 5 inputs representativos; métrica primária é ns/byte; critério de reprodutibilidade é variação < 3% entre 3 runs sequenciais.
**ADRs relacionadas:** ADR-0032 (política de `unsafe` em L1), ADR-0030 (performance é domínio de L1), ADR-0014 (scanner herdado de `unscanny`).

---

## Contexto

DEBT-42 (Passo 84.8a) regista 7 ocorrências de `unsafe { self.string.get_unchecked(start..end) }` em `01_core/src/engine/lexer/scanner.rs`. A ADR-0032 estabelece que `unsafe` em L1 só pode permanecer como excepção permanente se um benchmark reprodutível demonstrar regressão inaceitável ao eliminá-lo, registada em ADR específica com número concreto.

Antes de poder executar esse benchmark, o projecto precisava de infraestrutura: framework, localização, inputs de stress e critérios de reprodutibilidade.

## Decisão

1. **Framework:** `criterion` 0.5 — padrão de facto em Rust, já disponível em cache local, fornece estatísticas robustas (outlier detection, confidence intervals) e relatórios HTML/CSV.
2. **Localização:** crate `typst-benches` em `benches/` na raiz do workspace. Isto isola o código de benchmark de `01_core/src/`, mantendo L1 livre de dependências de benchmarking.
3. **API medida:** `typst_core::engine::lexer::Lexer::new(src, SyntaxMode::Markup).next()` — a API pública mínima que consome tokens. O struct `Lexer` passou a ser `pub` (apenas visibilidade; sem alteração funcional).
4. **Inputs:** 5 ficheiros `.typ` em `benches/corpus/` cobrindo cenários distintos:
   - `b1_hello.typ` — micro input (~11 bytes).
   - `b2_text.typ` — texto corrido (~5 KB).
   - `b3_math.typ` — math denso (~3 KB).
   - `b4_code.typ` — código denso (~4 KB).
   - `b5_utf8.typ` — UTF-8 multibyte denso (~2 KB).
5. **Métrica:** tempo de lex por byte (ns/byte), normalizando por tamanho do input.
6. **Reprodutibilidade:** 3 execuções sequenciais; variação entre mediana e média inferior a 3%.

## Consequências

- **Positivas.** DEBT-42 fica desbloqueado: o P442 pode agora medir o impacto real de remover `get_unchecked`, e o P443 pode tomar a decisão com números.
- **Custos.** +1 crate no workspace (`typst-benches`); +1 dev-dependency (`criterion`); tempo adicional de compilação para `cargo bench`.
- **Riscos.** Benchmarks locais são sensíveis a thermal throttling e load do sistema; o critério de 3 runs com variação < 3% mitiga mas não elimina o ruído.

## Alternativas consideradas

- **`iai-callgrind`**: rejeitada — depende de Valgrind/Callgrind, menos portável e mais pesado para o objectivo de medir regressão percentual.
- **Benchmarks dentro de `01_core/benches/`**: rejeitado — poluiria a crate L1 com dependências de benchmark e exigiria expor mais API interna.
- **Benchmark manual com `std::time::Instant`**: rejeitado — `criterion` oferece estatísticas e relatórios superiores sem custo significativo.
