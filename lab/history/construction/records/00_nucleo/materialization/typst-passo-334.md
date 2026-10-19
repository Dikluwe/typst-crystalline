# Tarefa P334 — Lote F-1: a fronteira de extensão E1 no produto (aditivo)

**Repositório de trabalho**: typst-crystalline (raiz).
**Número do passo**: P334 (confirmar livre).
**Pré-condição**: P333 fechado — L0 `entities/f_fronteira_e1.md` redigido
(design-ahead, warning V7 esperado), plano de lotes, baseline da lente,
suíte 2708, lint 0 (produto). Se não, parar.
**Tipo**: **Lote F-1** — o primeiro código de produto do F. Aditivo;
content-preserving; os 65 nativos NÃO mudam.
**Fontes**: L0 `prompts/entities/f_fronteira_e1.md` (a spec), ADR-0106,
`f-plano-lotes-passo-333.md` (F-1), `f-spike2-show-passo-333.md` (S1–S7),
spike E1 (`lab/spikes/f-extensao/e1/` — referência de leitura; **nunca
importar de `lab/`**), baseline lente (`tekt-cargo-dsm@98d8f9e`), baseline
perf 10× (P330).
**Commits**: "Passo 334 — aprovação da Trava (registro)" (1º, tree limpo) e
"Passo 334 — lote F-1".

---

## Parte 0 — Aprovação da Trava (registro; zero código)

O dono aprovou o L0 no checkpoint do P333, com as respostas:

1. **L0 aprovado** — blanket `impl<T: Element> DynElement for T`; chain
   única; `Value` fechado; canal `Set*` (F-2); `#show` por S3–S6.
2. **Trava-Q1**: guard/lifecycle vive na **camada de realização**
   (`rules/`), em invólucro **transparente** uniforme nativo+dinâmico —
   NÃO no `Content` nem no trait. **Condição do dono**: a transparência do
   invólucro é provada por teste quando o invólucro nascer (F-1 se nascer
   aqui; senão no lote que o trouxer) — `plain_text`/`is_empty`/`map_*` e
   closures de usuário não o veem.
3. **Trava-Q2 confirmada**: `dyn_kind_name()` estável `Eq` + `get_field`
   são contrato obrigatório do elemento dinâmico (S1/S7).
4. **`Value::Custom` fica fora** — gatilho de reabertura registrado.
5. **Plano de lotes aprovado**: F-1 → F-2 → fila incremental; trava
   ADR-0105 cláusula 3 construída em F-1/F-2 (antes de relaxar o
   compilador).
6. **R4 da lente**: sessão do `tekt-cargo-dsm` abre em paralelo (não
   bloqueia F-1); F-1 usa a medição grosseira (`--comparar`).

Gravar: nota de aprovação na ADR-0106 (ou adendo) + estado do L0
`f_fronteira_e1.md` muda de design-ahead para **ativo** neste lote (o
código F-1 declara `@prompt entities/f_fronteira_e1.md`; o warning V7 deve
**limpar**). Hash do L0 sincronizado (`--fix-hashes`) APÓS qualquer ajuste
da Fase A.

---

## Lote F-1 — Fase A (reconhecimento) → checkpoint → Fase B (código)

### Fase A — plano de toque (precedente do modelo de lotes)

1. Auditar o L0 §3a contra o estado atual do código (algum desvio desde
   P333? registrar). Greps: onde o hub fecha exaustividade hoje; onde o
   registro será injetado (pureza L1 — quem constrói o pipeline em
   `04_wiring`); onde `extract_payload`/`locatable`/`content_hash` listam
   variantes.
2. Emitir o **checkpoint** no chat: plano de toque (arquivos × mudança),
   contagem prevista de sites (~15–25 — se estourar muito, parar e
   reportar), e qualquer ambiguidade do L0 encontrada (ambiguidade de
   desenho = parar; ambiguidade mecânica = decidir e registrar).

### Fase B — o código (após o checkpoint; aditivo)

Conforme o L0 §3a, na ordem:

1. **Trait `DynElement`** (object-safe) + **blanket
   `impl<T: Element> DynElement for T`** em `entities/elements/mod.rs` (ou
   onde o L0 mandar): os métodos do `Element` + `dyn_kind_name()` +
   `get_field()` + eq/hash/clone dinâmicos (downcast interno do blanket;
   o usuário não escreve nada disso).
2. **Variante `Content::Dynamic(Arc<dyn DynElement>)`** + os arms do hub
   (~9: 6 matches + eq + hash + payload/locatable) — dispatch de 1 linha,
   idêntico aos 65.
3. **Registro injetado** (construtores por nome; sem global — instanciado
   no `04_wiring`, atravessa por parâmetro como o L0 fixar). Elemento
   desconhecido = erro declarado, não panic.
4. **A trava ADR-0105 cláusula 3 nasce aqui**: o registro é o primeiro
   ponto onde a exaustividade do compilador não cobre — escrever o
   **teste-varre-registro** (todo nome registrado constrói, despacha pelos
   6 métodos, round-trip de `get_field`) ANTES de o registro entrar no
   caminho de execução.
5. **Fixture dos dois públicos**: o `callout` como elemento de **teste**
   (fora dos 65, em fixtures/tests — não vira elemento nativo): caminho
   Rust (implementa `trait Element`, registra) e caminho typst até onde o
   produto de hoje permite (construção por nome via registro; `#set`/
   `#show` ficam para F-2+ — registrar a fronteira).
6. **Teste de object-safety** (`Arc<dyn DynElement>` construível, 6
   dispatches) + testes do blanket (um nativo qualquer atravessa o
   `DynElement` com o mesmo resultado do caminho estático).
7. **B3** (`world_types::Styles(())` stub morto): remover **se** este lote
   tocar a superfície que o referencia; senão, fica para F-2 (registrar
   qual).
8. Linhagem: arquivos novos declaram `@prompt entities/f_fronteira_e1.md`;
   `--fix-hashes`; o V7 do L0 limpa.

**Não fazer**: tocar os 65 módulos nativos; implementar realização/guards/
`#show` (S2–S6 são F-2+; só os ganchos de contrato S1/S7 entram); canal
`Set*`; qualquer mudança content-preserving-violadora (asserção existente
alterada = bug).

### Verificação (gates do lote)

- `cargo build` limpo; suíte com `RUST_MIN_STACK=33554432` — **2708
  existentes verdes + N novos** (registrar N; zero asserções alteradas).
- `crystalline-lint .` = **0 violations e 0 warnings** (o V7 limpou).
- **Lente** (`tekt-cargo-dsm@98d8f9e`, mesmos comandos do baseline P333):
  `--comparar` antes/depois — esperado: `edges(content→elements::*)`
  inalterado (+ a porta nova); **0 edges elemento→elemento mantido**;
  ciclos não pioram. Registrar os números.
- **Perf**: rerun do 10× (mesmo corpus/método P330) — nativos sem regressão
  (esperado dentro do σ de 0.6518 ± 0.0057; o `dyn` não está no caminho dos
  nativos).

---

## Relatório (`typst-passo-334-relatorio.md` + resumo no chat)

- Parte 0: onde a aprovação mora; estado do L0 (ativo; hash).
- Fase A: o plano de toque real vs previsto (~15–25 sites); ambiguidades
  encontradas e como foram tratadas.
- Fase B: o diff por item (1–8); o teste-varre-registro (a trava) descrito;
  a fixture dos dois públicos; destino de B3.
- Verificação: suíte (2708 + N), lint 0/0, lente antes/depois (números),
  perf antes/depois.
- **Contabilidade do F**: F-1 fechado; próximo F-2 (canal `Set*`, ~108,
  válvula `SetPage` declarada); estado da sessão R4 da lente se houver
  novidade.
- `git log --oneline` (2 commits isoláveis); `git status` limpo.
- Caveat do stack (`RUST_MIN_STACK=33554432`).

## Fora de escopo (confirmado)

F-2+ (canal `Set*`, realização/guards/`#show`, `Styled`, de-bake, folhas);
consertos B1 (→F-2) e B2 (→F-3); refinamentos da lente (sessão própria,
R1–R5); otimizações (perf é gate de não-regressão, não alvo).
