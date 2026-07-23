# Relatório — typst-passo-861: verificação de estado

**Data:** 2026-07-23  
**Executor:** Kimi Code (agente principal; prompt lido de `00_nucleo/materialization/typst-passo-861.md`).  
**Proveniência das medições:** working tree sobre commit `06a336b0c3ae42949c2bced6c4c6511b9d89bebd`. Binários: cristalino `target/release/typst`, vanilla `lab/typst-original/target/release/typst` (0.15.0). Nenhum código foi alterado neste passo.

---

## Item 1 — Status real da migração de `Duration`

### Estado: **feita e commitada**.

Evidência:

```bash
$ git log --oneline -- 01_core/src/entities/duration.rs | head -5
06a336b0c chore: relatórios e execuções dos passos P858 a P860 e atualizações no engine
0661aef91 [P797] cargo fmt (formatação global)
84be3351b P400: materializa Value::Duration (intervalo de tempo L1 puro)
```

O commit `06a336b0c` é anterior ao HEAD actual; o commit `e110cc119` ("relatórios e execuções dos passos P851 a P858 e actualizações em engine e entities") contém a migração propriamente dita. Diff resumido:

- `01_core/src/entities/duration.rs`: `nanos: u64` → `nanos: i128`; constructores e acessores assinados; `impl Neg`; `to_string()` para negativos.
- `01_core/src/entities/value.rs`: `cast_duration` aceita inteiros/flutuantes negativos.
- `01_core/src/engine/eval/operators.rs`: aritmética de `Duration` com sinal.
- `01_core/src/engine/stdlib/primitives_constructors.rs`: constructor aceita componentes negativos; parsing de prefixo `-`.
- `01_core/src/engine/eval/repr.rs`: `repr_duration` para componentes negativos.
- `01_core/src/engine/eval/tests.rs`: testes end-to-end para durações negativas.
- `00_nucleo/prompts/entities/duration.md`: actualizado com a representação com sinal.

### Teste concreto

```bash
$ echo '#repr(-duration(seconds: 3))' > /tmp/duration_test.typ
$ ./target/release/typst /tmp/duration_test.typ -o /tmp/duration_test.pdf
$ pdftotext /tmp/duration_test.pdf -
duration(seconds: -3)
```

Resultado: o caso que motivou P850 (`#repr(-duration(seconds: 3))`) está resolvido e a paridade com o vanilla é total.

---

## Item 2 — Achado #41 (F2, de P843): fusão de texto no parser

### Estado: **continua sem correção**.

O cristalino funde texto corrido num único nó `Text` durante o parsing. O vanilla mantém `Text + Space + Text` (três nós) e deixa essa reagrupação para a fase `realize`.

Evidência no código cristalino (`01_core/src/engine/parse/markup.rs:96-102`):

```rust
SyntaxKind::Text
| SyntaxKind::Linebreak
| SyntaxKind::Escape
| SyntaxKind::Shorthand
| SyntaxKind::SmartQuote
| SyntaxKind::Link
| SyntaxKind::Label => p.eat(),
```

`SyntaxKind::Text` é consumido diretamente como um único nó; o espaço entre palavras não gera um nó `Space` separado.

### Teste concreto

```typst
#repr([hello world])
```

- **Cristalino:** `[hello world]`
- **Vanilla:** `[hello world]`

O `repr` bate em texto (P843 corrigiu isso), mas a morfologia interna do content tree continua diferente. Este achado é, como P859 estabeleceu, sintoma da ausência de uma fase de composição de conteúdo equivalente ao `realize` do vanilla — o vanilla mantém a granularidade até essa fase; o cristalino decide a granularidade no parser.

**Referências cruzadas:**
- `00_nucleo/diagnosticos/typst-passo-843-relatorio.md` §#41 (F2), limitação 2.
- `00_nucleo/diagnosticos/typst-passo-859-relatorio.md` §Grupo 3.

---

## Item 3 — Itens do Grupo 2 de P859 (sem sintoma observado)

Para cada item foi testado um caso simples nos dois binários.

| # | Item | Teste | Vanilla | Cristalino | Sintoma observado hoje? |
|---|---|---|---|---|---|
| 1 | Ausência de `ParElem` | `#show par: it => strong(it)` `hello world` | Aceita e aplica a regra | Erro: "show par: apenas `set block(spacing: ..)` é reconhecido" | **Sim** — o cristalino rejeita `show par` como regra de elemento |
| 2 | Ausência de agrupamento de listas | `- first`<br><br>`- second` | Duas listas separadas com espaçamento | Uma única lista compacta | **Sim** — listas separadas por parágrafo são fundidas no cristalino |
| 3 | Símbolos math fora de `$...$` | `pi` | Renderiza "pi" como texto | Renderiza "pi" como texto | Não — ambos degradam para texto |
| 4 | Regex show rule cross-node | `#show regex("h.*o"): it => [FOUND]`<br>`hello world` | `FOUNDrld` (match atravessa space) | `FOUND` (match só "hello") | **Sim** — regex no cristalino não atravessa `Space` |

**Conclusão:** dois dos quatro itens (ausência de `ParElem` e agrupamento de listas) já apresentavam sintomas observáveis em casos simples; P859 classificou-os como "sem sintoma observado ainda" porque a triagem anterior não os exercitou directamente. Os outros dois (símbolos math isolados e regex cross-node) continuam sem sintoma observável em casos mínimos — embora o regex cross-node seja uma divergência documentada (scope-out).

---

## Item 4 — Notas soltas nunca abertas como achado

| Nota | Teste | Vanilla | Cristalino | Estado |
|---|---|---|---|---|
| `#text(size:)` como chamada | `#text("hello", size: 20pt)` | Aceita | Erro: "text() argumento nomeado desconhecido: 'size'" | **Divergência real e activa** |
| `box(width:)` em unidade absoluta | `#box(width: 100pt, height: 50pt, fill: red)` | Funciona (box vermelho) | Funciona (box vermelho) | **Resolvida / nunca foi problema** |
| Extensão do arquivo de saída do CLI | `typst simple.typ -o simple.png` | Gera PNG | Gera PDF | **Divergência real e activa** |
| `#set page(height: auto)` | `#set page(height: auto)` `hello` | Aceita | Erro: "expected length, float, or int, found auto" | **Divergência real e activa** |

**Conclusão:** três das quatro notas soltas são divergências reais e activas. Apenas `box(width:)` em unidade absoluta funciona igual nos dois lados (provavelmente resolvida incidentalmente por passos anteriores).

---

## Resumo executivo

- **Duration:** migrado, testado e a paridade está confirmada.
- **Achado #41:** continua aberto; é uma divergência de morfologia interna, não de output textual.
- **Grupo 2 de P859:** dois itens (ausência de `ParElem` e agrupamento de listas) revelam sintomas simples que não tinham sido testados; dois itens (símbolos math isolados e regex cross-node) continuam sem sintoma mínimo.
- **Notas soltas de P831/P848:** três divergências activas (`#text(size:)`, extensão de output do CLI, `#set page(height: auto)`); uma já funciona (`box(width:)` absoluto).

Nenhuma correção de código foi feita neste passo.

---

## Referências

- `00_nucleo/materialization/typst-passo-861.md`
- `00_nucleo/diagnosticos/typst-passo-850-relatorio.md`
- `00_nucleo/diagnosticos/typst-passo-843-relatorio.md`
- `00_nucleo/diagnosticos/typst-passo-859-relatorio.md`
- `00_nucleo/diagnosticos/typst-passo-831-relatorio.md`
- `00_nucleo/diagnosticos/typst-passo-848-relatorio.md`
- `01_core/src/engine/parse/markup.rs:96-102`
