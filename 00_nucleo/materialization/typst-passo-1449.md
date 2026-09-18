# Passo 1449 — Materializar `math.underbracket` com identidade e geometria de spreader inferior

**Estado:** PROPOSTA L0 — AGUARDA CONFIRMAÇÃO HUMANA (ADR-0127)  
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

Hashes SHA-256 das fontes medidas:

| Fonte | SHA-256 |
|---|---|
| `lab/typst-original/crates/typst-library/src/math/underover.rs` | `49fe33135766b83c16d27f84c51bd8f867cd5f3f790b075242e8043661a7e298` |
| `lab/typst-original/crates/typst-library/src/math/ir/resolve.rs` | `115d775641509a755a1b7f4bd6da26cc8502a09e0e23e303d38d4a7cd76e0112` |

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

Hashes SHA-256 dos consumers antes da proposta:

| Consumer | SHA-256 |
|---|---|
| `01_core/src/entities/elements/math_underover.rs` | `7c70beef6260b6703a06c34eb84b9dbb1d388957f64d4a504819126b311181b4` |
| `01_core/src/entities/content.rs` | `c4d5095faef366da8dec93baf2e7e320762251e92e876b5135c48a2e40dbb9c3` |
| `01_core/src/compiler/stdlib/structural/math.rs` | `cdf5f23652eb08e2c49f925d63d1fcd777bf5732087454d039dfe89310140f27` |
| `01_core/src/compiler/eval/math.rs` | `19fd0073799bad417172f3fbf283ca8fe1780fd3ffcfa220df8b996b7fbdfde4` |
| `01_core/src/compiler/eval/repr.rs` | `e2b1f2844b5bee307acd8b2d25e5c85e2f32bf4fd8e8bbd0ac5fce6e1413f5b9` |
| `01_core/src/compiler/eval/bindings/field_access.rs` | `32f23dbbb2a5055e3581f81d37bde3b746e19be951e7daa0bb3e226cb9dd123e` |
| `01_core/src/compiler/math/layout/underover.rs` | `8ba062ab97a12083f14b2bb998fec59e9d16b573120b9e9b453d2bf14afb720d` |
| `01_core/src/compiler/math/layout/mod.rs` | `00d367e1c1df4a80cf391ab755124b11b89c7156545f8a490d3ba96811ea98a3` |
| `01_core/src/compiler/math/layout/spacing.rs` | `9d7e6febb59ea270137c22f9d6b13ddfa1f1952dc3cd01d97b61b390d7e78620` |
| `01_core/src/compiler/eval/mod.rs` | `20581cad7edb06620a19b1ce01457423eb074ed59b02af6d797feefccc57bed2` |
| `01_core/src/compiler/introspect.rs` | `7e7413b1196f63523c6b554c80779beef0a6b6760d46373c51313328ff2ee519` |
| `01_core/src/compiler/introspect/locatable.rs` | `6e9893ece9f3ce654d04199329270fb1899289948a2a6322ab365fe7b2bf0808` |
| `01_core/src/compiler/layout/mod.rs` | `694a5574293fa2b45eddb2a48672e00817fa3f77c43ac25a386bb0fa935e2189` |
| `03_infra/src/query_helpers.rs` | `6a244e9f674502f3290f7b1c303a4ad2683203b269979cd295e6f28ed9bfc576` |

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
