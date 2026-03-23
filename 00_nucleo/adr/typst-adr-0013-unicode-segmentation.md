# ⚖️ ADR-0013: `unicode_segmentation` → `[l1_allowed_external]`

**Status**: `PROPOSTO`
**Data**: 2026-03-23

---

## Contexto

O lexer usa `unicode_segmentation::UnicodeSegmentation` para
iterar sobre grapheme clusters — a unidade perceptual de "caractere"
como um humano a vê, em contraste com codepoints Unicode ou bytes.

No Typst, grapheme clusters são a unidade correcta para:
- Contar a largura visual de texto (um emoji composto é 1 grapheme,
  mas múltiplos codepoints)
- Determinar posições de cursor em edição interactiva
- Calcular o comprimento de strings em funções Typst expostas ao
  utilizador (`str.len()` conta graphemes, não bytes)

A alternativa de implementar o algoritmo UAX #29 (Unicode Text
Segmentation) em L1 seria reproduzir um algoritmo de estado
finito com tabelas de propriedades de centenas de entradas — o
algoritmo correcto para grapheme clusters não é trivial.

---

## Análise de pureza

| Propriedade | Estado |
|-------------|--------|
| Zero I/O | ✓ — algoritmo e tabelas sem acesso a sistema |
| Zero estado global mutável | ✓ — iteradores sem estado partilhado |
| Determinismo total | ✓ — mesma string, mesmos graphemes, em qualquer ambiente |
| Dependências transitivas | ✓ — zero dependências externas |

---

## Decisão

`unicode_segmentation` é adicionado a `[l1_allowed_external]`:

```toml
[l1_allowed_external]
rust = [
    "thiserror",
    "comemo",
    "unicode_ident",
    "unicode_math_class",
    "unicode_script",
    "unicode_segmentation",
]
```

O trait `UnicodeSegmentation` é usado internamente no lexer via
extension methods em `&str` — não aparece em assinaturas públicas
de L1. V14 não dispara.

---

## Nota sobre o grupo Unicode

Com ADR-0013, o grupo de crates Unicode (0010–0013) está completo.
Todas partilham as mesmas propriedades de pureza e a mesma
justificação: são extensões do sistema de tipos da linguagem para
o domínio de processamento de texto, não infraestrutura.

O precedente estabelecido por este grupo: crates que implementam
standards internacionais (Unicode, ISO) sem I/O e sem estado
mutável são elegíveis para `[l1_allowed_external]` quando o domínio
genuinamente as exige para correctitude funcional — não por
conveniência.

---

## Prompts afectados

| Prompt | Natureza da mudança |
|--------|---------------------|
| `00_nucleo/prompts/rules/parse.md` | Documentar `unicode_segmentation` como externo autorizado; referenciar ADR-0013 |

---

## Consequências

**Positivas**: O lexer trata texto como humanos o percepcionam —
correctitude semântica para documentos multilíngue com emoji e
scripts compostos.

**Negativas**: Quarta crate Unicode em `[l1_allowed_external]` —
padrão estabelecido nas ADRs 0010–0012. A whitelist começa a ter
dimensão — justificada pelo domínio, mas a monitorizar.

**Neutras**: `UnicodeSegmentation` é um trait de extensão; não cria
tipos novos em L1. A interface pública de L1 não expõe nenhum tipo
desta crate.

---

## Alternativas consideradas

| Alternativa | Prós | Contras |
|-------------|------|---------|
| Inlining do algoritmo UAX #29 | Zero dependências | Algoritmo não trivial com tabelas extensas; manutenção custosa |
| Contar codepoints em vez de graphemes | Sem dependência | Comportamento incorrecto para emoji, scripts compostos — regressão visível para utilizadores |

---

## Referências

- Unicode UAX #29: https://www.unicode.org/reports/tr29/
- `unicode-segmentation`: https://github.com/unicode-rs/unicode-segmentation
- Diagnóstico Passo 4 — `UnicodeSegmentation` em lexer.rs
- ADR-0010, ADR-0011, ADR-0012 — grupo Unicode completo
