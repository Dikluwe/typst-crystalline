# Passo 1436 — convergência vertical completa de `math.lr`

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

| artefato | SHA-256 |
|---|---|
| vanilla `typst-library/src/math/lr.rs` | `2a762699e7d3eed9a0b4b2d1afbbf5b58649339ec47ec12a2a82907519c107e0` |
| vanilla `typst-library/src/math/ir/resolve.rs` | `115d775641509a755a1b7f4bd6da26cc8502a09e0e23e303d38d4a7cd76e0112` |
| cristalino `entities/elements/math_delimited.rs` | `bef47c8e80861a30db5952d776735e624b5020630a4f8a0e4a153b0e78ae0c40` |
| cristalino `entities/content.rs` | `68905c3fbf0362aebff7ea387d154aa1263f27ebb4cac8fe60cd219f2bfaeebf` |
| cristalino `compiler/stdlib/structural/math.rs` | `dc00f8de1475f287980b4bcbefdec855ddd43f22aaa1fd23d6e7657d6b1b5043` |
| cristalino `compiler/eval/math.rs` | `f963332267a4dcd55927b849dd8590088b690b16e547d632aec4e5d7d0ffc7ce` |
| cristalino `compiler/eval/repr.rs` | `4606f22b7b0ff35770c57f2a80964da9dba55f83c4663c0097d1aae1eef7e11c` |
| cristalino `compiler/math/layout/delimited.rs` | `dbb280feb558499ae3a208201081134ed0250e8823e91756ee7975617b36fa5c` |
| cristalino `compiler/math/layout/mod.rs` | `e429e97fb0f3265d841b3d2761339c3691446f8bcd6cde49552b09a0c7ec8f60` |
| cristalino `compiler/math/layout/spacing.rs` | `3f3d605f9f05cc81c29acfea9001479e4da96fa13b43cebd8bcb244f643775f7` |
| cristalino `compiler/eval/mod.rs` | `97c110bc50eff8882bc38b1cb2be289baadbe26d8daf4b9c94d890cacba4c2f6` |
| cristalino `compiler/introspect.rs` | `8900e3a01a7642074c93c74dc1b9b87e30118640f4a2b30f4c06345f47a83e92` |
| cristalino `compiler/introspect/locatable.rs` | `f962956984e830c3a3627382f4913b2308c39a576060333771c6a5890e84c7ca` |
| cristalino `compiler/layout/mod.rs` | `7fab6d2e2d897944bdfe5b8404801545bbb9011b9ddc531f05e96bef84a60b42` |
| cristalino `infra/query_helpers.rs` | `37ed92e4396708796671bb62b55e1af9185edbe3e84883f66d76167f14a7546c` |

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
