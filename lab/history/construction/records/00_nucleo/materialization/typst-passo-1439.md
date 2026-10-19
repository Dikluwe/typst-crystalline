# Passo 1439 — Materializar `math.overbrace` com identidade e geometria de spreader

> **Nota de preservação (limpeza de 2026-09-19):** este passo é mantido como
> registro histórico. Menções a selos, recibos, certificados, manifests, hashes
> administrativos, papéis de agentes e gates do protocolo revogado não criam
> requisitos vigentes. Os artefatos removidos foram consolidados em
> relatório consolidado da limpeza; o comportamento normativo
> permanece nos Prompts L0 e nos testes funcionais atuais.


**Estado:** PROPOSTA L0 — AGUARDA CONFIRMAÇÃO HUMANA (ADR-0127)  
**Data da medição:** 2026-09-17T22:46:49-03:00  
**Commit medido:** `243fa2784675bd1eb3710cc4fb03b2b6f4e24b92`  
**Árvore medida:** limpa (`git status --short` e `git diff HEAD --stat` sem saída)  
**Regime:** `executed_without_isolation_attestation`

## 1. Próxima unidade nominal

O inventário residual do Passo 1422 ordena, depois de `math.norm`,
`math.overbrace`, `math.overbracket`, `math.overline`, `math.overparen`,
`math.overshell`, `math.primes`, `math.root`, `math.round`, `math.stretch`,
`math.underbrace`, `math.underbracket`, `math.underparen` e
`math.undershell`. Portanto a próxima rota nominal é `math.overbrace`.
`math.abs`, deliberadamente não retomada pela sequência dos passos recentes,
continua residual e não é declarada fechada.

## 2. Medição antes da decisão

### 2.1 Fonte vanilla ratificada

No pin `586e1bd43`, `math/underover.rs` declara `OverbraceElem` com
`body: Content` obrigatório e posicional e `annotation: Option<Content>`
opcional e posicional. `math/ir/resolve.rs` resolve o body, cria o glifo
U+23DE (`⏞`) com classe `Diacritic`, stretch horizontal de 100% e shortfall
zero, e o combina como acento superior com largura exata. O item resultante
herda a classe matemática do body. Quando há anotação, ela é adicionada como
script superior.

Hashes identificador administrativo das fontes medidas:

| Fonte | identificador administrativo |
|---|---|
| `lab/typst-original/crates/typst-library/src/math/underover.rs` | [dado administrativo removido] |
| `lab/typst-original/crates/typst-library/src/math/ir/resolve.rs` | [dado administrativo removido] |

### 2.2 Observáveis públicos medidos no vanilla

Com `/usr/local/bin/typst` ratificado:

- `repr(math.overbrace)` produz `"overbrace"`;
- `repr(math.overbrace([x]))` produz `overbrace(body: [x])`;
- `repr(math.overbrace([x], [n]))` produz
  `overbrace(body: [x], annotation: [n])`;
- omissão e presença explícita diferem: `overbrace([x], none)` conserva
  `annotation: none` no `repr`;
- `none` e string convertem para conteúdo no body; nesting e spread canônico
  conservam identidade;
- a forma bare `$ overbrace(x) $` produz a mesma identidade;
- ausência do body, terceiro positional, `body:`, `annotation:` e named
  desconhecido falham; inteiro no body é rejeitado e inteiro como segundo
  positional produz o diagnóstico vanilla `unexpected argument`.

### 2.3 Estado cristalino medido

A forma qualificada inexiste. A forma bare partilha um braço genérico que
silencia named e positional excedente, baixa diretamente para
`MathUnderover` e perde a identidade: os casos medidos aparecem como
`xover("⏞")` e `xover("⏞")over(n)`. A geometria física madura já existe em
`layout/underover.rs`, inclusive variantes/assemblies horizontais, attachment,
largura exata e script da anotação; o defeito é de contrato, morfologia e
fronteira de argumentos, não ausência dessa mecânica.

Hashes identificador administrativo dos consumers antes da proposta:

| Consumer | identificador administrativo |
|---|---|
| `01_core/src/entities/elements/math_underover.rs` | [dado administrativo removido] |
| `01_core/src/entities/content.rs` | [dado administrativo removido] |
| `01_core/src/compiler/stdlib/structural/math.rs` | [dado administrativo removido] |
| `01_core/src/compiler/eval/math.rs` | [dado administrativo removido] |
| `01_core/src/compiler/eval/repr.rs` | [dado administrativo removido] |
| `01_core/src/compiler/math/layout/underover.rs` | [dado administrativo removido] |
| `01_core/src/compiler/math/layout/mod.rs` | [dado administrativo removido] |
| `01_core/src/compiler/math/layout/spacing.rs` | [dado administrativo removido] |
| `01_core/src/compiler/eval/mod.rs` | [dado administrativo removido] |
| `01_core/src/compiler/introspect.rs` | [dado administrativo removido] |
| `01_core/src/compiler/introspect/locatable.rs` | [dado administrativo removido] |
| `01_core/src/compiler/layout/mod.rs` | [dado administrativo removido] |
| `03_infra/src/query_helpers.rs` | [dado administrativo removido] |

## 3. Classificação anti-deriva

- Identidade `overbrace`, assinatura/casts, presença explícita, `repr`, classe
  herdada e geometria observável são linguagem (ADR-0107).
- `AccentItem`, `ScriptsItem`, a árvore IR e a redução cristalina para
  `MathUnderover` são mecânica; podem divergir desde que os observáveis sejam
  preservados.
- Inferência: shortfall zero + largura exata expressam que a chave deve cobrir
  exatamente o corpo antes da centralização da anotação. Refutação: witness
  vanilla em que a chave fique deliberadamente aquém do corpo ou a anotação
  determine a largura-alvo da chave.
- Reutilizar somente o carrier genérico não basta: ele não distingue a função
  pública nem omissão de `annotation` de `annotation: none`.

## 4. Decisão L0 proposta

1. Adicionar `MathOverbraceElem { body, annotation, annotation_explicit }` ao
   owner coeso `entities/elements/math_underover.rs`; não criar Prompt órfão.
2. Adicionar `Content::MathOverbrace(Arc<MathOverbraceElem>)`. O bit de
   presença é morfológico e nunca é inferido do valor convertido.
3. Registrar `math.overbrace`; a forma bare delega ao mesmo builder. Exigir um
   body posicional e no máximo uma anotação posicional, reproduzindo casts e
   diagnósticos medidos, spread e avaliação única.
4. Projetar `overbrace(body: ..., annotation?: ...)`; emitir `annotation`
   somente quando fornecida e conservar explicitamente `none`.
5. O layout owner reduz localmente a variante à composição madura equivalente:
   body com U+23DE acima e, se presente, anotação superior. A free function
   fica em `layout/underover.rs`; o match central permanece magro (ADR-0109).
6. A classe esquerda/direita herda recursivamente o body. A chave permanece
   diacrítica, estica horizontalmente com shortfall zero e não troca a classe.
7. Reconstruções transformam body e anotação, preservam presença e identidade.
   `ObservationEq` compara os três campos.
8. A variante é não-locatable e terminal nos walkers introspectivos e helpers
   aproximados L3; o walker estrutural L1 continua dono dos filhos.

As outras 12 rotas que sucedem `overbrace` no inventário e `math.abs`
permanecem fora de escopo.

## 5. Gate ADR-0127

Esta proposta cria elemento e variante públicos e corrige o comportamento por
defeito da rota bare. Portanto este commit contém apenas passo e L0 e **PARA**.
Testes, código e resselo começam somente após confirmação humana explícita.

## 6. Execução após confirmação

1. Criar oráculos RED para binding qualificado/bare, aridade, casts, presença,
   `repr`, nesting, classe herdada, geometria e walkers.
2. Confirmar RED cristalino e GREEN vanilla onde aplicável.
3. Implementar carrier, builders, dispatch e layout na forma B.
4. Atacar mutantes: apagar identidade/presença, aceitar named/extra, trocar
   U+23DE, inverter posição, perder classe herdada e deixar de esticar.
5. Exigir score `1.0` para os mutantes definidos; resselo somente após V15/V26.
6. Validar `cargo fmt --check`, `cargo build --workspace`, suítes focal/completa,
   `git diff --check` e `crystalline-lint .`, registrando proveniência.

## 7. Confirmação humana

O dono confirmou explicitamente o gate ADR-0127 após o commit de proposta
`1476196c1b327cf9fdc18f54685fe51cc1eb249e`. A cadeia RED→GREEN, ataques,
resselo e veredito está autorizada sem ampliar o escopo acima.

## 8. Materialização

P1439 foi materializado em 2026-09-18 sob o regime
`executed_without_isolation_attestation`. O RED atribuível apresentou nove
erros de compilação centrados na ausência de `MathOverbraceElem`,
`Content::MathOverbrace` e helpers P1439. O candidato fechou quatro testes
focais e duas expectativas legadas P906 sincronizadas com a nova identidade.

A primeira revisão de oráculo corrigiu a expectativa `Normal` para
`Alphabetic`, pois o contrato congelado exige herança da classe do body. Uma
tentativa adversarial abortou sem crédito após sete execuções por receita M08
ambígua. Reancorada a receita, a campanha matou 36/36 mutantes. A posterior
sincronização do arquivo protegido `compiler/eval/tests.rs` invalidou esse
selo; a recampanha final repetiu as ordens direta, direta repetida e reversa e
matou novamente 36/36, score `1.0`, zero `Unknown`.

O binário cristalino reproduziu as formas qualificadas e bare, annotation
omitida/presente/`none`, nesting e sete diagnósticos negativos medidos no
vanilla. `cargo fmt --check`, `git diff --check`, `cargo build --workspace` e
`crystalline-lint .` terminaram verdes. A suíte completa executou 5.866 testes
L1: 5.858 verdes e oito falhas históricas já presentes no baseline P1438;
nenhuma toca arquivos ou observáveis P1439.
