# Passo 1436 — convergência vertical completa de `math.lr`

> **Nota de preservação (limpeza de 2026-09-19):** este passo é mantido como
> registro histórico. Menções a selos, recibos, certificados, manifests, hashes
> administrativos, papéis de agentes e gates do protocolo revogado não criam
> requisitos vigentes. Os artefatos removidos foram consolidados em
> relatório consolidado da limpeza; o comportamento normativo
> permanece nos Prompts L0 e nos testes funcionais atuais.


## Estado medido antes da decisão

P1435 selou o fragmento P1431 de `math.mat`; restam 16 rotas da coorte
matemática P1422. A próxima dependência causal é `math.lr`: ela é o elemento
base usado pelo vanilla para delimitadores explícitos e pelas funções
`abs`/`norm`/`round`, enquanto o cristalino conserva apenas uma reescrita
parcial para `MathDelimited` e não registra a função qualificada.

Medição executada em `2026-09-17T17:05:28-03:00`, sobre HEAD limpo
`01b8ba560433f5897405163988e0df4ce8d6abf9`, tree
`38883ed3b8b7ceaaa47ad78a5f43fdac51018acf`. `git status --short` não
produziu linhas. Entradas principais:

| artefato | identificador administrativo |
|---|---|
| vanilla `typst-library/src/math/lr.rs` | [dado administrativo removido] |
| vanilla `typst-library/src/math/ir/resolve.rs` | [dado administrativo removido] |
| cristalino `entities/elements/math_delimited.rs` | [dado administrativo removido] |
| cristalino `entities/content.rs` | [dado administrativo removido] |
| cristalino `compiler/stdlib/structural/math.rs` | [dado administrativo removido] |
| cristalino `compiler/eval/math.rs` | [dado administrativo removido] |
| cristalino `compiler/eval/repr.rs` | [dado administrativo removido] |
| cristalino `compiler/math/layout/delimited.rs` | [dado administrativo removido] |
| cristalino `compiler/math/layout/mod.rs` | [dado administrativo removido] |
| cristalino `compiler/math/layout/spacing.rs` | [dado administrativo removido] |
| cristalino `compiler/eval/mod.rs` | [dado administrativo removido] |
| cristalino `compiler/introspect.rs` | [dado administrativo removido] |
| cristalino `compiler/introspect/locatable.rs` | [dado administrativo removido] |
| cristalino `compiler/layout/mod.rs` | [dado administrativo removido] |
| cristalino `infra/query_helpers.rs` | [dado administrativo removido] |

O alvo permanece o vanilla ratificado
`586e1bd43fae6c9a973218163d3165c53ab8d16d`. A fonte declara
`LrElem { size: Rel<Length> = 100%, body: Content }`. Os posicionais são
variádicos: zero produz conteúdo vazio e dois ou mais são unidos por vírgula
matemática. O resolver aplica short-fall `0.1em`, usa `size` explícito ou
cascata, estica delimitadores nas extremidades e preserva o corpo como
conteúdo público. Um `LrElem` implícito aninhado só é absorvido quando seu
tamanho efetivo é o default.

Sondas públicas diretas no binário ratificado mediram:

| expressão | resultado público |
|---|---|
| `repr(math.lr())` | `lr(body: [])` |
| `repr(math.lr([x]))` | `lr(body: [x])` |
| `repr(math.lr([x], [y]))` | `lr(body: sequence([x], [,], [y]))` |
| `repr(math.lr(size: 150%, [x]))` | `lr(size: 150% + 0pt, body: [x])` |
| `repr(math.lr([x], size: 2em))` | `lr(size: 0% + 2em, body: [x])` |
| `size: 100%` explícito | o campo permanece no `repr` |
| `math.lr("x")` / `math.lr(none)` | conteúdo `[x]` / vazio |
| `math.lr(1)` | `expected content, found integer` |
| `size: auto` / `size: 1` | `expected relative length, found ...` |
| `body:` / named desconhecido | `unexpected argument: <nome>` |
| `size` duplicado | `duplicate argument: size` |
| `$ lr(x, y) $` | `lr(body: sequence([x], [,], [y]))` |
| `$ lr((x), size: #150%) $` | carrier exterior conserva `size` e body `lr(...)` interior |

O estado cristalino diverge em cinco pontos observáveis: `make_math_module`
não registra `lr`; a sintaxe bare devolve o body ou o degrada a
`MathDelimited`; `size:` não chega ao carrier; `repr` perde a identidade
`lr`; e o layout só dispõe do tamanho automático de `MathDelimited`, sem
precedência explícito > cascata > default.

## Classificação e decisão proposta

Identidade `lr`, corpo variádico, vírgulas, `size`, presença explícita,
cascata, diagnósticos, `repr` e geometria do stretch são semântica, sintaxe e
morfologia da língua (ADR-0107). O nome do struct Rust, a forma do enum e os
loops de layout são mecânica. A declaração `LrElem` e `resolve_lr` sustentam a
intenção; ela seria refutada por outra definição no pin ou por sonda pública
válida incompatível.

1. adicionar `MathLrElem` ao owner já coeso `math_delimited.rs`, separado do
   carrier mecânico `MathDelimitedElem`, com `body`, `Rel<Length>` e bit de
   presença de `size`;
2. adicionar `Content::MathLr(Arc<MathLrElem>)` e constructor completo,
   preservando todos os campos em igualdade, hash, debug, plain-text,
   walkers, `map_content` e `map_text`;
3. registrar `math.lr` com posicionais `Content` variádicos e somente
   `size:` nomeado; zero é válido, múltiplos recebem vírgulas entre si e cada
   ocorrência é avaliada exatamente uma vez;
4. fazer a sintaxe bare `lr(...)` construir o mesmo carrier, sem reescrever
   antecipadamente para `MathDelimited` nem descartar named, duplicados,
   spreads ou erros;
5. ligar `math.lr.size` à `StyleChain`, com precedência argumento explícito >
   cascata > `100%`, sem transformar valor cascaded em presença explícita;
6. projetar `repr` como `lr(size?, body: ...)`, sempre mostrando body e
   mostrando size somente quando causalmente explícito;
7. delegar o match estático ao owner `delimited.rs`, que resolve o corpo sem
   o mutar, aplica short-fall `0.1em`, estica as cercas exteriores e preserva
   o comportamento de item único e `lr` implícito aninhado do pin;
8. manter `MathDelimitedElem` para usos mecânicos já certificados, inclusive
   `binom`, e não inferir identidade `lr` a partir de qualquer par arbitrário;
9. preservar P912/P919/P1132 e as reconstruções de estilo/default; a nova
   variante recebe transformação apenas no body e conserva tamanho/presença.

## Limites explícitos

P1436 não fecha as outras 15 rotas P1422. Em particular, não registra
`math.mid`, `math.abs`, `math.norm` ou `math.round`; não cria o carrier de
`mid`; não altera a tabela geral de símbolos chamáveis; e não promete bytes
idênticos de PDF/raster. Essas três funções wrapper tornam-se candidatas
posteriores sobre o carrier `lr`, mas não recebem crédito neste passo.

Falha de build/execução, timeout, OOM, swap, stream truncado, observador opaco
ou identidade não verificável são `Unknown` e bloqueiam. `Unknown` nunca é
convertido em sucesso.

## Gate ADR-0127

O passo adiciona variante e constructor públicos, registra função pública e
muda a representação e a geometria por default/cascata. Incide nos itens 1,
2 e 4 do ADR-0127.

Fluxo obrigatório:

1. atualizar antes do código os treze L0 proprietários afetados;
2. commitar esta proposta e os L0s;
3. **PARAR sem alterar L1 nem headers de linhagem**;
4. continuar somente após confirmação explícita do dono;
5. após confirmação, produzir manifesto, contrato, oráculos, ataques e testes
   segregados, selar, obter RED atribuível e só então implementar.

## Regime posterior ao gate

Aplicar o protocolo completo `tekt-materializacao-segregada`, executado sem
atestação de isolamento no workspace partilhado. O contrato deve cobrir
binding, zero/um/muitos posicionais, casts, named/duplicados/spread, presença,
cascata, convergência bare/qualificada, carrier, walkers, repr, delimitadores,
item único, nesting, short-fall e regressões P912/P919/P1132. Mutation score
exigido: `1.0`; `Unknown` é fail-closed.

## Gates de fecho após confirmação

- RED atribuível do binding e do carrier `MathLr`;
- RED de variádicos, casts, duplicados, presença e cascata;
- RED morfológico de body vazio, simples, sequence e nesting;
- RED geométrico discriminando `100%`, `150%`, `2em` e cascata;
- GREEN bilateral das rotas bare e qualificada;
- controles `MathDelimited`/`binom` e P912/P919/P1132 preservados;
- três ordens determinísticas e hashes selados intactos;
- `cargo fmt --all -- --check`, testes focais, `cargo build`,
  `crystalline-lint .` e `git diff --check` verdes.

## Confirmação do gate

**Confirmado explicitamente pelo dono em 2026-09-17.** A materialização fica
autorizada somente dentro do recorte completo de `math.lr` e dos treze L0
proprietários acima. `math.mid`, `math.abs`, `math.norm`, `math.round` e as
demais rotas matemáticas residuais permanecem fora de escopo.

## Fecho materializado

P1436 foi implementado e selado em 2026-09-17 sob o regime
`executed_without_isolation_attestation`. O RED atribuível precedeu o
candidato; o GREEN final contém cinco testes L1 e um teste L3. A campanha
física cobriu 12 mutantes nas ordens direta, repetida e reversa: 36/36
execuções eficazes foram mortas, score `1.0`, zero `Unknown`.

A primeira materialização de M03 atingiu por engano outro braço zero anterior
e foi preservada como recibo negativo; a receita corrigida selecionou a
segunda ocorrência, dentro de `math_lr_from_args`, e foi morta 3/3. O agregado
não atribui crédito à receita defeituosa. Certificado:
relatório consolidado da limpeza.

Os gates finais `cargo fmt --all -- --check`, `git diff --check`, os testes
focais, `cargo build --workspace` e `crystalline-lint .` terminaram com exit
zero. O linter reportou zero drift/violations; warnings e infos históricos
fora do recorte não foram convertidos em crédito. As rotas declaradas fora de
escopo continuam abertas.
