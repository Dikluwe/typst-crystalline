# Relatório de Verificação — P596

**Data:** 2026-07-07  
**Passo:** 596  
**Foco:** Confirmar qual binário vanilla foi usado em P595 e validar a afirmação sobre `page(columns:)`  
**Hash do commit de base:** `3e5b487a8073746121fc1876cde773cdb75cac3b`  
**Hash do commit de fecho:** `4a9d3be16270a254007b42a918dd12886a0fb5e9`

---

## Binário vanilla em P595

P595 usou exclusivamente:

```text
lab/typst-original/target/release/typst
```

Identificação exacta:

```bash
$ lab/typst-original/target/release/typst --version
typst 0.15.0 (969087ec)
```

Este é **o mesmo binário** que P553/P554 usaram para o vanilla 0.15.0:

| Passo | Binário vanilla | Versão |
|---|---|---|
| P553 | `lab/typst-original/target/release/typst` | 0.15.0 (969087ec) |
| P554 | `lab/typst-original/target/release/typst` | 0.15.0 (969087ec) |
| P595 | `lab/typst-original/target/release/typst` | 0.15.0 (969087ec) |

Também foi verificado o vanilla 0.14.2 (`/usr/local/bin/typst`, `typst 0.14.2 (b33de9de)`) para comparar; o comportamento é idêntico ao 0.15.0 nos testes de P595.

---

## Testes directos

### Documento 1 — `#lorem(30)` apenas, com `height: 200pt`

```typst
#set page(columns: 2, height: 200pt)
#lorem(30)
```

- **Vanilla 0.15.0:** 1 página, **uma coluna** (conteúdo curto, não transborda).
- **Cristalino:** 1 página, **duas colunas** (força colunas mesmo com conteúdo curto).

### Documento 2 — `#lorem(200)` com `height: 200pt`

```typst
#set page(columns: 2, height: 200pt)
#lorem(200)
```

- **Vanilla 0.15.0:** 2 páginas, **duas colunas visíveis** na primeira página.
- **Cristalino:** colunas visíveis.

### Documento 3 — Documento exacto de P595 (nota grande)

```typst
#set page(columns: 2, height: 200pt)
#lorem(30)#footnote[
  Esta é uma nota de rodapé muito longa, com texto suficiente para não caber no espaço restante de uma coluna pequena, testando o que acontece quando isto excede o espaço disponível na página.
]
#lorem(30)
```

- **Vanilla 0.15.0:** 1 página, **uma coluna**, nota no rodapé.
- **Cristalino (após P595):** 2 páginas, texto dividido em colunas, aviso de overflow emitido.

### Documento 4 — Documento de P595 com nota enorme

```typst
#set page(columns: 2, height: 200pt)
#lorem(30)#footnote[
  Esta é uma nota de rodapé extremamente longa. #lorem(200)
]
#lorem(30)
```

- **Vanilla 0.15.0:** 2 páginas, **duas colunas**, nota no rodapé e continuação.
- **Cristalino (após P595):** emite aviso; nota visível.

---

## Conclusão

A afirmação de P595 de que "a versão vanilla em quarentena não suporta `page(columns:)` da mesma forma" está **imprecisa**. O binário é o mesmo de P553/P554, e o vanilla 0.15.0 **suporta** `#set page(columns: 2, height: 200pt)`. O que aconteceu em P595 foi que o documento específico (nota grande com pouco texto principal) não transbordou o suficiente para o vanilla ativar a segunda coluna. Quando o conteúdo é aumentado (nota enorme ou `#lorem(200)`), o vanilla produz colunas normalmente.

A correção implementada em P595 (detectar overflow de footnotes e emitir aviso) continua válida. O que deve ser corrigido é o relatório de P595, para deixar claro que:

1. O binário vanilla é o mesmo de P553/P554.
2. O vanilla suporta `page(columns:)`, mas o documento de teste de P595 não atingiu o limiar de transbordo para ativar colunas.
3. A decisão de paridade foi tomada ao nível linguístico (visibilidade/avisos), não porque o vanilla não suporte colunas.

---

## Ficheiros de verificação

- `/tmp/p596-colunas.typ`
- `/tmp/p596-vanilla.pdf`
- `/tmp/p596-so-colunas.typ`
- `/tmp/p596-so-colunas.pdf`
- `/tmp/p596-colunas-longo.typ`
- `/tmp/p596-colunas-longo.pdf`
- `/tmp/p596-nota-grande.typ`
- `/tmp/p596-nota-grande.pdf`
- `/tmp/p596-nota-enorme.typ`
- `/tmp/p596-nota-enorme.pdf`

Temporários, não commitados.
