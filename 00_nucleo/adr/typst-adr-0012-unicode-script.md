# ⚖️ ADR-0012: `unicode_script` → `[l1_allowed_external]`

**Status**: `PROPOSTO`
**Data**: 2026-03-23

---

## Contexto

O lexer usa `unicode_script::{Script, UnicodeScript}` para detectar
o script de caracteres (Latin, Cyrillic, Greek, Han, Arabic, etc.).

No Typst, a detecção de script serve para decisões de segmentação
de texto que afectam o comportamento do lexer em modo markup:
transições entre scripts influenciam onde o lexer pode inserir
quebras implícitas e como agrupa sequências de texto.

A alternativa de implementar detecção de script em L1 exigiria
reproduzir a tabela Unicode Script Property — centenas de ranges
de codepoints mantidos pelo Unicode Consortium e actualizados em
cada versão do standard.

---

## Análise de pureza

| Propriedade | Estado |
|-------------|--------|
| Zero I/O | ✓ — tabelas compiladas em tempo de compilação |
| Zero estado global mutável | ✓ — funções puras sobre dados estáticos |
| Determinismo total | ✓ — mesma entrada, mesma saída em qualquer ambiente |
| Dependências transitivas | ✓ — zero dependências externas |

---

## Decisão

`unicode_script` é adicionado a `[l1_allowed_external]`:

```toml
[l1_allowed_external]
rust = [
    "thiserror",
    "comemo",
    "unicode_ident",
    "unicode_math_class",
    "unicode_script",
]
```

O tipo `Script` de `unicode_script` é usado internamente no lexer
e não aparece em assinaturas públicas de L1 — V14 não dispara.

---

## Prompts afectados

| Prompt | Natureza da mudança |
|--------|---------------------|
| `00_nucleo/prompts/rules/parse.md` | Documentar `unicode_script` como externo autorizado; referenciar ADR-0012 |

---

## Consequências

**Positivas**: O lexer detecta correctamente transições de script
em texto multilíngue sem reproduzir tabelas Unicode manualmente.

**Negativas**: Terceira crate Unicode em `[l1_allowed_external]` —
padrão estabelecido nas ADRs 0010 e 0011.

**Neutras**: `Script` não escapa para a interface pública de L1;
é um detalhe de implementação do lexer.

---

## Alternativas consideradas

| Alternativa | Prós | Contras |
|-------------|------|---------|
| Inlining das tabelas em L1 | Zero dependências | Tabelas extensas; actualização manual a cada versão Unicode |
| Substituir por heurística simples | Sem dependência | Comportamento incorrecto para texto multilíngue — regressão funcional |

---

## Referências

- Unicode Script Property: https://www.unicode.org/reports/tr24/
- `unicode-script`: https://github.com/nickel-lang/unicode-script
- Diagnóstico Passo 4 — `Script`, `UnicodeScript` em lexer.rs
