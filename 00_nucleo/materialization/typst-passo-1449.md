# Passo 1449 — Materializar `math.underbracket` com identidade e geometria de spreader inferior

> **Nota de preservação (limpeza de 2026-09-19):** este passo é mantido como
> registro histórico. Menções a selos, recibos, certificados, manifests, hashes
> administrativos, papéis de agentes e gates do protocolo revogado não criam
> requisitos vigentes. Os artefatos removidos foram consolidados em
> relatório consolidado da limpeza; o comportamento normativo
> permanece nos Prompts L0 e nos testes funcionais atuais.


**Estado:** IMPLEMENTADO E SELADO EM PRODUTO
**Data da medição:** 2026-09-18T18:41:20-03:00  
**Commit medido:** `3d9a0e63abc1e235d04128a5937d7313ff92819c`  
**Árvore medida:** `5aaf1f3d6163cee905bfaf723ae66de4c5173b4e`, limpa (`git status --short` sem saída)  
**Regime:** `executed_without_isolation_attestation`

## 1. Próxima unidade nominal

O inventário residual P1422 ordena, após `math.underbrace`,
`math.underbracket`, `math.underparen` e `math.undershell`. P1448 fechou
`underbrace`; portanto a próxima unidade autocontida é `math.underbracket`.
As duas rotas seguintes permanecem fora deste recorte.

## 2. Medição antes da decisão

### 2.1 Fonte vanilla ratificada

No pin `586e1bd43`, `math/underover.rs:66` declara `UnderbracketElem` com
`body: Content` obrigatório/posicional e `annotation: Option<Content>`
opcional/posicional. `math/ir/resolve.rs:1313-1327` resolve a construção pelo
helper comum `resolve_underoverspreader` (`:1418-1468`): U+23B5 (`⎵`) abaixo
do body, classe `Diacritic`, stretch horizontal de 100% com shortfall zero e
annotation, quando presente, como script inferior. `AccentItem::create`
conserva a classe matemática bruta do body.

Hashes identificador administrativo das fontes medidas:

| Fonte | identificador administrativo |
|---|---|
| `lab/typst-original/crates/typst-library/src/math/underover.rs` | [dado administrativo removido] |
| `lab/typst-original/crates/typst-library/src/math/ir/resolve.rs` | [dado administrativo removido] |

### 2.2 Observáveis públicos medidos no vanilla

Com `/usr/local/bin/typst` ratificado (`typst 0.15.1 (586e1bd4)`):

- `repr(math.underbracket)` produz `"underbracket"`;
- body-only produz `underbracket(body: [x])`;
- com annotation produz `underbracket(body: [x], annotation: [n])`;
- `underbracket([x], none)` conserva `annotation: none`, enquanto a omissão
  não assenta esse campo;
- `none` e string convertem para conteúdo no body; spread e nesting conservam
  identidade;
- `fields()` contém sempre `body` e contém `annotation` apenas quando
  explicitamente fornecida; `func()` devolve `underbracket`;
- `$ underbracket(x, n) $` produz a mesma identidade pública;
- ausência do body, terceiro positional, `body:`, `annotation:` e named
  desconhecido falham; inteiro no body é rejeitado como cast e inteiro como
  annotation produz `unexpected argument`.

Fronteira negativa medida:

- ausência do body: `missing argument: body`;
- terceiro positional: `unexpected argument`;
- `body:`: `the argument body is positional`;
- `annotation:` e named desconhecido: `unexpected argument: <nome>`;
- body inteiro: `expected content, found integer`;
- annotation inteira: `unexpected argument`.

### 2.3 Estado cristalino medido

A forma qualificada falha com `module math does not contain underbracket`. A
forma bare baixa diretamente para `MathUnderover`: o witness é
`equation(block: true, body: xunder("⎵")under(n))`, apagando identidade e
presença. Terceiro positional é ignorado e named desconhecido é silenciado. A
geometria madura já existe em `layout/underover.rs`; o defeito é de contrato,
morfologia e fronteira de argumentos.

Hashes identificador administrativo dos consumers antes da proposta:

| Consumer | identificador administrativo |
|---|---|
| `01_core/src/entities/elements/math_underover.rs` | [dado administrativo removido] |
| `01_core/src/entities/content.rs` | [dado administrativo removido] |
| `01_core/src/compiler/stdlib/structural/math.rs` | [dado administrativo removido] |
| `01_core/src/compiler/eval/math.rs` | [dado administrativo removido] |
| `01_core/src/compiler/eval/repr.rs` | [dado administrativo removido] |
| `01_core/src/compiler/eval/bindings/field_access.rs` | [dado administrativo removido] |
| `01_core/src/compiler/math/layout/underover.rs` | [dado administrativo removido] |
| `01_core/src/compiler/math/layout/mod.rs` | [dado administrativo removido] |
| `01_core/src/compiler/math/layout/spacing.rs` | [dado administrativo removido] |
| `01_core/src/compiler/eval/mod.rs` | [dado administrativo removido] |
| `01_core/src/compiler/introspect.rs` | [dado administrativo removido] |
| `01_core/src/compiler/introspect/locatable.rs` | [dado administrativo removido] |
| `01_core/src/compiler/layout/mod.rs` | [dado administrativo removido] |
| `03_infra/src/query_helpers.rs` | [dado administrativo removido] |

## 3. Classificação anti-deriva

- Identidade `underbracket`, assinatura/casts, presença explícita, `repr`,
  fields, classe herdada e geometria observável são linguagem (ADR-0107).
- `AccentItem`, `ScriptsItem`, árvore IR e redução cristalina para
  `MathUnderover` são mecânica e podem divergir.
- Inferência: stretch de 100% e shortfall zero exigem que o bracket cubra o
  body antes da centralização da annotation. Refutação: witness vanilla em
  que o bracket fique deliberadamente aquém do body ou a annotation defina o
  alvo de stretch.
- Reutilizar somente o carrier genérico não basta: ele não preserva função,
  fields nem presença causal da annotation.

## 4. Decisão L0 proposta

1. Adicionar `MathUnderbracketElem { body, annotation,
   annotation_explicit }` ao owner coeso
   `entities/elements/math_underover.rs`.
2. Adicionar `Content::MathUnderbracket(Arc<MathUnderbracketElem>)`; presença
   explícita é morfologia e nunca é inferida do valor convertido.
3. Registrar `math.underbracket`; a forma bare delega ao mesmo builder. Exigir
   body posicional e no máximo uma annotation posicional, com casts,
   diagnósticos, spread, ordem e avaliação única medidos.
4. Projetar `underbracket(body: ..., annotation?: ...)`, emitindo annotation
   somente quando fornecida e conservando `none` explícito; fields e `func()`
   seguem a identidade pública.
5. O layout owner reduz localmente a variante à composição madura: body com
   U+23B5 abaixo e, se presente, annotation inferior. A free function fica em
   `layout/underover.rs`; o match central permanece magro (ADR-0109).
6. A classe esquerda/direita herda recursivamente o body. O bracket permanece
   diacrítico, estica horizontalmente com shortfall zero e não troca a classe.
7. Reconstruções transformam body e annotation, preservam presença e
   identidade; `ObservationEq` compara os três campos.
8. A variante é não-locatable e terminal nos walkers introspectivos e helpers
   aproximados L3; o walker estrutural L1 continua dono dos filhos.

As rotas `underparen` e `undershell` permanecem fora de escopo.

## 5. Gate ADR-0127

Esta proposta cria elemento e variante públicos e corrige o comportamento por
defeito da rota bare. Portanto este commit contém apenas passo e L0 e **PARA**.
Testes, código e resselo começam somente após confirmação humana explícita.

## 6. Execução após confirmação

1. Congelar manifesto, contrato, oráculos e mutantes antes do candidato.
2. Criar testes RED para binding qualificado/bare, aridade, casts, presença,
   fields, `repr`, nesting, classe herdada, geometria e walkers.
3. Confirmar RED cristalino e GREEN vanilla onde aplicável.
4. Implementar carrier, builders, dispatch e layout na forma B.
5. Atacar mutantes que apagam identidade/presença, aceitam named/extra,
   trocam U+23B5, invertem posição, perdem classe herdada ou stretch.
6. Exigir score `1.0`, zero `Unknown` não planejado e repetição/reordenação.
7. Validar `cargo fmt --check`, `cargo build --workspace`, suítes focal e
   completa, `git diff --check` e `crystalline-lint .`, com proveniência.

## 7. Confirmação humana

O dono confirmou explicitamente o gate ADR-0127 em 2026-09-18 após o commit
de proposta `5c7fbb632`. A cadeia RED→GREEN, ataques, resselo e veredito está
autorizada sem ampliar o escopo acima.

## 8. Execução e veredito

O RED focal produziu 9 erros atribuíveis aos símbolos P1449 ausentes. Após a
materialização, 5/5 oráculos P1449 ficaram GREEN; a expectativa histórica
P906 foi migrada da representação agregada para a identidade pública nova e
também passou. A campanha adversarial rejeitou 39/39 execuções (13 mutantes,
ordem forward, repetição forward e reverse), score `1.0` e zero `Unknown`.

Em 2026-09-18T19:13:30-03:00, sobre a working tree não commitada baseada em
`5c7fbb632694c8cd4cc3f1033b860fc9d0461436`, passaram `cargo fmt --check`,
`git diff --check`, `cargo build --workspace` e `crystalline-lint .`. A suíte
completa mediu 5913 testes: 5905 passaram e as mesmas 8 falhas históricas
permaneceram, nenhuma atribuível ao P1449. Recibos e selo ficam em
`00_nucleo/diagnosticos/p1449-*`.
