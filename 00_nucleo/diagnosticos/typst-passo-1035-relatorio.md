# Passo 1035 — argumentos `label` em `cite`/`link`/`footnote`

**Data**: 2026-08-13
**Estado**: **investigação fechada, código bloqueado no gate**. Fases A e B completas e
medidas; Fase C (gate ADR-0127) **aberta e à espera do dono**. Nenhuma linha de código
escrita — ver §"Por que parou".

---

## Proveniência

`HEAD = 0c8b64a41` (P1033). Árvore limpa no momento das medições desta secção
(`git status` sem modificações). Vanilla: `/usr/local/bin/typst`,
md5 `36da18895eeb5e0136c068a7634e3f82`, **byte-a-byte idêntico** a
`lab/typst-original/target/release/typst` (baseline ratificado `a51e02804`); a string
`typst 0.15.1 (e0e8ca4d)` que ele imprime é a armadilha 1 do `CLAUDE.md`, não a tag.
Cristalino: `target/release/typst` reconstruído de `0c8b64a41` (`typst 0.15.0 (0c8b64a4)`).
Medições 17:30–17:40 -03:00. Documentos em
`/tmp/claude-1000/-repos-Antigravity-typst-crystalline/…/scratchpad/p1035`.

---

## Fase A — as três assinaturas, medidas

| Entrada | Vanilla | Cristalino |
|---|---|---|
| `#cite("netwok")` | `error: expected label, found string` + hint `use \`<netwok>\` or \`label("netwok")\`` | aceita → renderiza `[netwok]` |
| `#cite(<netwok>)` | compila → `[1]` | **erro**: `cite() espera key como string, recebeu label` |
| `#link(<intro>)[Ir para intro]` | compila | **erro**: `link() espera URL como string, recebeu label` |
| `A#footnote[Um] <fn> B#footnote(<fn>) C#footnote[Dois]` | `A1 B1 C2` (a referência reutiliza o nº 1) | **erro**: `footnote() espera content ou string, recebeu label` |

**Achado novo, não registado em P1031**: a linha 2. O relatório original media apenas que
`cite` aceita `string` a mais; não registou que **rejeita `label`** — isto é, a forma
correcta da linguagem também não funciona. A polaridade não está só invertida, está
invertida nas duas direcções.

O alvo de `cite` é confirmado pela suite do próprio vanilla —
`lab/typst-original/tests/suite/model/cite.typ:140-149` fixa a mensagem **e o hint**, com
dois casos (label literal válido → `use \`<netwok>\` or \`label("netwok")\``; literal
inválido `%@&#*!\` → só `use \`label("…")\``).

## Fase B — não há causa comum

As três funções têm **três** `match` escritos à mão, em ficheiros diferentes, sem cast
partilhado:

- `01_core/src/compiler/stdlib/structural/bibliography.rs:380-400` (`native_cite`)
- `01_core/src/compiler/stdlib/structural/markup.rs:147-167` (`native_link`)
- `01_core/src/compiler/stdlib/structural/footnote.rs:34-52` (`native_footnote`)

Logo **o fixe são três**, não um. E os três têm custos muito diferentes — é isso que
decide o gate:

| função | o que falta | custo |
|---|---|---|
| `cite` | mapear `Value::Label` → key, e passar a recusar `Value::Str`. A conversão já existe noutro sítio: `stdlib/ref.rs:36` faz `label.0.as_str().into()` | **não toca entidades** |
| `link` | `LinkElem { url: EcoString, body }` (`entities/elements/link.rs:20-23`) não tem onde guardar um destino-label; exige um destino soma (URL \| Label) | **campo público de entidade** |
| `footnote` | `FootnoteElem { body, numbering }` não representa "referência a nota existente"; o vanilla tem `FootnoteBody::Reference(Label)` e o gate `!self.is_ref()` no contador | **campo público de entidade** |

## Custo de compatibilidade da inversão em `cite` (a pergunta explícita da Fase C)

Medido, não presumido:

- `.typ` no repositório fora de `lab/`: **0** ocorrências de `#cite("…")`.
- As duas ocorrências existentes estão em `lab/typst-original/tests/suite/model/cite.typ` —
  a suite do vanilla, onde são **testes que exigem o erro**.
- Testes Rust nossos: **3** call sites em fonte Typst (`compiler/eval/tests.rs:10130,
  10146, 10161`, todos `#cite("key", style: …)`). Os outros 73 hits de `cite("` são o
  construtor Rust `Content::cite(…)`, superfície interna, não linguagem.

**Conclusão**: a inversão custa 3 asserções de teste e nenhum documento. Não é o obstáculo.

## Por que parou (gate ADR-0127)

O obstáculo é `link`/`footnote`: ambos exigem **alterar campo público de entidade**, que é
o ponto **1** da lista de paragem obrigatória da ADR-0127 — e a regra de bolso da §5 dessa
ADR ("em caso de dúvida, parar") não chega a ser necessária, porque **os três L0 já tinham
classificado este trabalho, antes deste passo**:

- `entities/elements/footnote.md` — *"Gate ADR-0127 (contrato público + comportamento),
  passo próprio. **Não implementado aqui.**"*
- `entities/elements/cite.md` §P1031 — *"mudar o tipo de `key` é mudança de contrato
  público (`CiteElem.key`) e de comportamento por defeito, logo gate ADR-0127 e passo
  próprio."*
- `entities/elements/link.md` — links internos estão em **scope-out** explícito desde P422.

Escrever o código sem a confirmação repetiria o desvio de P952, que a própria ADR-0127
tipifica como a violação que a motivou.

## O que o dono tem de decidir

1. **Âmbito**: os três de uma vez, ou só `cite` (que não toca entidades e pode seguir em
   fluxo contínuo como correcção de paridade) — deixando `link`/`footnote` para um passo de
   contrato?
2. **Forma do destino de `link`**: `LinkElem.url: EcoString` → um `enum LinkTarget { Url,
   Label }`, ou campo `dest` novo? Afecta `FrameItem::Link` e o consumidor PDF (destinos
   internos não existem hoje — scope-out de P422).
3. **Forma da nota-referência**: seguir o vanilla (`FootnoteBody::Reference(Label)` +
   `!is_ref()` a travar o contador, já citado no L0 de `footnote.md`), ou representação
   própria?
4. **Confirmação da inversão de `cite`** com o custo medido acima (3 asserções).

Aprovado o âmbito, o resto do passo é mecânico: L0 primeiro com hash, testes RED com as
mensagens exactas da suite do vanilla (incluindo o hint), implementação, `crystalline-lint`
e `cargo test --workspace`.
