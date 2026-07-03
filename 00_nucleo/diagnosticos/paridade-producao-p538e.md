# Relatório de Paridade de Produção — P538e

**Data:** 2026-07-03  
**Passo:** 538e  
**Prompt L0:** `00_nucleo/prompts/infra/shaper.md` (hash `355f8197`)  
**Dependências:** P534 (fallback multi-script), P538 (constatação da diferença entre fonte explícita e default)

## Objectivo

Investigar por que `Hello 你好 مرحبا` sem `#set text(font: ...)` perde os caracteres CJK e árabes, enquanto o mesmo texto com `#set text(font: "DejaVu Sans")` funciona.

## Sonda

### Passo 1 — isolar variáveis

Três scripts de teste:

```typst
// sem fonte
Hello 你好 مرحبا

// Helvetica explícita
#set text(font: "Helvetica", size: 20pt)
Hello 你好 مرحبا

// DejaVu Sans explícita (caso P534)
#set text(font: "DejaVu Sans", size: 40pt)
Hello 你好 مرحبا
```

Resultado `pdftotext` / `mutool draw -F text`:

| Caso | Resultado |
|------|-----------|
| Sem fonte | `Hello` + `?? ?????` |
| Helvetica | `Hello` + `?? ?????` |
| DejaVu Sans | `Hello 你好 مرحبا` |

Conclusão da sonda: **não é um problema de "fonte explícita vs default"**. O mecanismo de fallback é accionado nos dois casos; a diferença está na fonte específica.

### Passo 2 — confirmar causa raiz

Teste unitário temporário com `SystemWorld::with_system_fonts()` mostrou:

```
Helvetica no FontBook: false
Helvetica result: [Discriminant(0)]  // FrameItem::Text (não shapeado)
DejaVu result: [Discriminant(1), ...] // FrameItem::TextShaped
```

A fonte default do cristalino é `"Helvetica"` (definida em `StyleChain::font()` / `TextStyle::from`), mas essa fonte **não existe** no FontBook deste sistema. Quando `resolve_candidates` falha, o shaper antigo desistia e devolvia `FrameItem::Text` não-shapeado, que o export PDF emite sem fonte válida → caracteres não extraíveis.

Quando o utilizador declara `"DejaVu Sans"`, essa fonte existe, `resolve_candidates` consegue resolver, e o fallback por carácter (P534) funciona.

### Passo 3 — perigo do fallback global carácter-a-carácter

A primeira tentativa de correcção (deixar `primary` vazio e recair no fallback global do FontBook) revelou um segundo problema: o FontBook começa por fontes especializadas (ex.: `MathJax_AMS`), que podem cobrir `'H'` mas não `'e'`. Isso faz `split_run_by_font` quebrar `"Hello"` em `"H"` + `"ello"`, e cada sub-run carrega o `text` completo no PDF, produzindo duplicação (`Helloello`).

Portanto, a correcção não pode ser simplesmente "usar o primeiro candidato do FontBook".

## Implementação

Em `03_infra/src/shaper.rs`, `try_shape` agora:

1. Tenta resolver a `FontList` declarada.
2. Se falhar (fonte default `"Helvetica"` ausente, ou outra fonte inexistente), tenta uma **lista ordenada de fontes padrão de fallback**:
   - DejaVu Sans
   - Noto Sans
   - Liberation Sans
   - FreeSans
   - Arial
3. A primeira fonte padrão que existir no `FontBook` torna-se a primária, e o fallback por carácter (P534) continua a funcionar sobre ela.
4. Se nenhuma fonte padrão existir, recai no fallback global carácter-a-carácter (caso raro).

```rust
const DEFAULT_FALLBACK_FONTS: &[&str] = &[
    "DejaVu Sans",
    "Noto Sans",
    "Liberation Sans",
    "FreeSans",
    "Arial",
];
```

## Ficheiros alterados

- `03_infra/src/shaper.rs` — fallback padrão quando a fonte declarada não existe; teste P538e.

## Testes automáticos

Comando executado:

```bash
cargo test --workspace
crystalline-lint .
```

Resultado: todos os testes passam; linter limpo.

Teste novo:

- `p538e_fonte_default_ausente_usa_fallback_padrao` — cria um `FontWorld` com DejaVu Sans e Noto Sans CJK, pede `"Helvetica"` (inexistente), e verifica que o texto misto `Hello 你好` é shapeado com ambos os scripts presentes.

## Teste manual

Script:

```typst
Hello 你好 مرحبا
```

Resultado `mutool draw -F text`:

```
Hello
你好ابحرم
```

Todos os caracteres são extraíveis. O árabe aparece invertido no `mutool` (problema de bidireccionalidade pré-existente, P484), mas os glifos estão presentes.

Comparação com Typst vanilla (`/usr/local/bin/typst`):

```
Hello 你好 ابحرم
```

Vanilla também mostra os três scripts. A diferença residual é de layout (o cristalino separa os scripts em linhas diferentes; o vanilla mantém uma linha). Esta diferença não é objecto de P538e.

## Caso P534 sem regressão

O teste original de P534 continua a funcionar:

```typst
#set text(font: "DejaVu Sans", size: 40pt)
Hello 你好 مرحبا
```

Resultado: `Hello 你好 مرحبا` extraível.

## Conclusão

P538e está concluído. A causa raiz era a ausência da fonte default `"Helvetica"` no FontBook do sistema, que fazia o shaper desistir antes de aplicar o fallback multi-script. A correcção introduz uma lista de fontes padrão de fallback, garantindo que documentos sem `#set text(font: ...)` usam uma fonte real do sistema e preservam caracteres de múltiplos scripts.
