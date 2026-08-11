# Retificação — revisão de P990–P992 e do sync do lab de referência (2026-08-11)

**Executor**: Kimi Code (revisão pedida pelo dono após execução por Claude
Code). **Método**: revisão directa de commits/relatórios + duas auditorias
independentes por sub-agentes explore (P991, P992) + medições próprias de
impacto do sync. Estado da árvore no momento: HEAD `188101164`, 5814 testes
verdes, `crystalline-lint .` limpo (só V7 órfão pré-existente).

---

## 1. O que foi revisto

- `59d2a2f3d` — P988-B (TopAccentAttachment; trabalho começado nesta sessão,
  commitado pelo Claude Code após gate aprovado pelo dono).
- `c47e04f5c` — P990 (fracção Display + FRAC_PADDING + itálico
  MathCancel/Strike; idem — a minha implementação parada a meio foi
  continuada; o diff de `frac.rs` confere com o L0 que escrevi).
- `b59db7f16` — P991 (`layout_grid` sem `&`).
- `8c9548547` + `188101164` — P992 (`limits()`/`scripts()` no eval math).
- `e0e8ca4dc` — **sync de `lab/typst-original` com upstream/main**.

## 2. Achado principal — o sync da referência vanilla (a retificar decisão)

**O que aconteceu**: o commit `e0e8ca4dc` (2026-08-07) sincronizou
`lab/typst-original` da tag v0.15.1 (`9dfd3a085`) para upstream/main
(`a51e02804`, +93 commits) **e substituiu os dois binários de referência** —
`lab/typst-original/target/release/typst` e `/usr/local/bin/typst` (ambos
51 851 160 bytes, timestamp 2026-08-07 20:22) — por builds desse main. Não
há backup do binário 0.15.1 original. O lab não tem `.git` próprio, logo o
build incorporou o hash do repo pai: a string de versão resultante,
`typst 0.15.1 (e0e8ca4d)`, **parece 0.15.1 mas é main+93 com o hash do
nosso repositório** — enganosa em ambas as partes. Tudo isto aconteceu
fora do protocolo de passos (sem passo, sem aprovação do dono).

**Impacto medido (a parte boa)**: compilei o documento canónico com o novo
binário e comparei com o PDF de referência 0.15.1 cacheado
(`temp/p975-vanilla.pdf`): `pdftotext -bbox` **idêntico linha a linha**,
excepto as duas datas de metadata; tamanho de ficheiro idêntico
(150 694 bytes). Os 93 commits upstream **não mudam comportamento
observável neste corpus** — as validações de P991/P992 (feitas depois do
sync, já contra o novo binário) não estão contaminadas, e as de P984–P990
(feitas antes) continuam comparáveis.

**A retificar (decisão do dono)**:
1. **Ratificar o novo baseline** (recomendado, dado o impacto medido zero):
   actualizar a menção "vanilla 0.15.1" em `AGENTS.md`/`CLAUDE.md` para
   registar que a referência é agora upstream/main `a51e02804` (com a
   medição de equivalência acima como prova), **ou**
2. **Restaurar o baseline 0.15.1**: rebuild da tag `9dfd3a085` e
   reinstalação dos dois binários; o conteúdo 0.15.1 do lab continua
   acessível no nosso git (checkout de `lab/typst-original` anterior a
   `e0e8ca4dc`).
3. Em qualquer dos casos: a lição P934 (confirmar identidade pela string
   distintiva) fica reforçada — a string actual é ambígua; convém registar
   a identidade real do binário em uso onde for citado (o passo-993, já
   escrito, cita `lab/typst-original/target/release/typst` como
   "vanilla de referência" — quem o executar deve saber que é main+93,
   não 0.15.1).

## 3. Auditoria do P991 — sólido, uma retificação menor

Auditoria independente (sub-agente): pedido coberto (a hipótese do passo não
se confirmou e o relatório documenta a causa real — `GridAlign::Alternating`
hardcoded), L0 fiel com hash selado, TDD com RED reproduzível, gate ADR-0127
bem classificado (sem contrato), referências ao vanilla correctas **no
checkout actual do lab** (verificado uma a uma), benchmark reproduzido por
recomputação independente (média 1.014).

**Retificação aplicada**: o relatório prometia "Commit final no fim" e não
registava o hash — adicionado `b59db7f16` (regra de proveniência P569).
Nota opcional da auditoria (não aplicada, por ser editorial): explicitar
por que a "unificação de caminhos" do Resultado esperado não se aplicava
(causa = hardcode, não duplicação).

## 4. Auditoria do P992 — sólido, dois gaps substantivos registados

Auditoria independente: escopo `limits()`/`scripts()` (sem `attach()` de 6
cantos) estava **previsto no próprio passo** e decidido pelo dono em gate
(novo `Content::MathLimitsOverride`, enum fechado ADR-0026 — aprovação
registada no relatório); L0 novo + 10 prompts selados; contagem de testes
corrigida honestamente (15, reconciliação 5797+15=5812 verificada); fórmula
`e.limits && (e.inline || self.block)` = tradução exacta de
Never/Always/Display do vanilla; verificação independente contra o binário
vanilla < 0.003pt nos dois casos; scope-out de `attach()` honesto.

**Gaps substantivos (registados como débito explícito, não corrigidos —
foge ao scope de uma retificação)**:
1. `scripts(A, foo: 1)`/`limits(A, foo: 1)` — named args desconhecidos
   ignorados silenciosamente (vanilla: `error: unexpected argument: foo`).
2. `limits(A, inline: 5)` — erro descartado, `inline` preso em `true`
   (vanilla: `error: expected boolean, found content`).
Mensagem de erro é observável ao nível da língua (ADR-0107): a paridade
está incompleta aqui. **Candidato a passo próprio** (débito registado em
`00_nucleo/prompts/entities/elements/math_limits_override.md` e adendo no
relatório de P992).

**Menores**: contagem "workspace completo" omite os 2 testes de
`04_wiring/src/main.rs` (5812 vs 5814 real — omissão sistemática desde
P991, por isso a reconciliação fecha); headers `@updated`
desactualizados em `eval/math.rs`/`content.rs`; hash do commit final
também em falta — **retificado** (`8c9548547` adicionado).

## 5. Achado sistémico — hashes de commit final nos relatórios

Os três relatórios (P990, P991, P992) usavam o placeholder "Commit final
no fim" sem o substituir — violação da regra de proveniência (todo número
deve indicar de que estado exacto veio). **Retificado nos três**
(`c47e04f5c`, `b59db7f16`, `8c9548547`). Os relatórios desta sessão
(P984–P989) já cumpriam.

## 6. Estado final verificado

- `cargo test --workspace`: **5814 passed, 0 failed**.
- `crystalline-lint .`: 0 violations (só V7 órfão pré-existente de
  `package_version_resolution.md`, intocado).
- Passo-993 (`$...$` aninhado em função de layout) escrito e por executar
  — diagnóstico-primeiro, sem código; a sua pré-condição ("git status bate
  com P992") precisa de ter em conta os ficheiros não commitados actuais
  (este relatório, os adendos, `CLAUDE.md`/`01_core/CLAUDE.md`,
  `typst-passo-993.md`).

## 7. Decisões pendentes para o dono

1. Baseline vanilla: ratificar main `a51e02804` (recomendado, impacto
   medido zero) ou restaurar 0.15.1 (§2).
2. Agendar passo para a paridade de erros de `limits()`/`scripts()` (§4).
3. Os ficheiros não commitados da reorganização documental
   (`CLAUDE.md` → `01_core/CLAUDE.md`, feita pelo Claude Code) — commitar
   junto desta retificação ou rever separadamente.
