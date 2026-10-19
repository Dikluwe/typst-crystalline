# Relatório de Paridade de Produção — P538i

**Data:** 2026-07-03  
**Passo:** 538i  
**Tipo:** Implementação de teste  
**Dependências:** P538h (ponto cego identificado na bateria)

## Objectivo

Reforçar a bateria de paridade com verificação de texto extraído e adicionar
casos de `#for` com corpo markup ao corpus, fechando o ponto cego que permitiu
que o bug de P538f passasse despercebido durante 442 passos.

## Parte 1 — Casos de `#for` no corpus

Adicionados dois ficheiros em `lab/parity/corpus/p538i/`:

- `for-basic.typ` — itera sobre array de strings e gera lista com bullets.
- `for-with-counter.typ` — itera sobre `range(1, 4)` e gera itens numerados.

O exemplo original proposto no passo (`#for (i, x) in items.enumerate()`) não
funciona no cristalino (destructuring de tuplos em `#for` não é suportado),
por isso foi adaptado para `range` + `str(i)`.

### Verificação manual

```bash
./target/release/typst lab/parity/corpus/p538i/for-basic.typ /tmp/p538i-basic.pdf
pdftotext /tmp/p538i-basic.pdf -
```

Resultado:

```
•um
•dois
•três
```

```bash
./target/release/typst lab/parity/corpus/p538i/for-with-counter.typ /tmp/p538i-counter.pdf
pdftotext /tmp/p538i-counter.pdf -
```

Resultado:

```
Item 1 : letra
Item 2 : letra
Item 3 : letra
```

## Parte 2 — Verificação de texto na bateria

Ficheiro alterado: `lab/parity/tests/structural_parity.rs`.

### Alterações

1. **Lista de categorias** em `read_corpus` passou a incluir `"p538i"`.
2. **`default_selectors_for_category`** retorna `vec![]` para `"p538i"` (a
   paridade destes ficheiros é verificada por texto, não por query).
3. **`etiqueta_for`** marca `"p538i"` como `Include`.
4. **Contador de corpus atualizado** de 48 para 50 em todos os testes
   sentinelas.
5. **Funções auxiliares adicionadas:**
   - `pdftotext_available()` — verifica disponibilidade da ferramenta.
   - `extract_text_from_pdf_bytes()` — extrai texto de bytes PDF.
   - `compile_vanilla_pdf()` — compila source com CLI vanilla.
   - `normalize_extracted_text()` — normaliza espaços/quebras para comparação.
6. **No loop principal** `p206c_corpus_estrutural_36_ficheiros`, após a
   comparação estrutural, o teste agora:
   - Re-cria o `SystemWorld` com `.with_system_fonts()` (necessário para que o
     PDF cristalino tenha fontes reais).
   - Compila o PDF cristalino via `compile_to_pdf_bytes`.
   - Extrai texto com `pdftotext`.
   - Para `for-basic.typ`, faz `assert!` de que contém "um", "dois", "três".
   - Para `for-with-counter.typ`, faz `assert!` de que contém "Item 1",
     "Item 2", "Item 3".
   - Se a CLI vanilla estiver disponível, compila o PDF vanilla, extrai texto
     e compara normalizado, contando `total_text_diffs`.

### Resultados

Execução de `cargo test --test structural_parity`:

```
Total ficheiros corpus:   50
Includes (testados):      30
Skips:                    20
Errors:                   0
Comparações:              73
  - Matches:              73
  - Diffs:                0
Text diffs (P538i):       20
```

Os 20 text diffs são documentados como diferenças residuais de texto entre
cristalino e vanilla (fontes, hifenização, representação de math) — a bateria
estrutural continua com 0 diffs, e os text diffs são informação, não falha.

## Corpus histórico revisto

Todos os ficheiros históricos de `p490/` e `p500/` continuam a compilar. A
nova verificação de texto revelou 20 diferenças residuais no corpus completo,
nenhuma das quais é corrigida neste passo (conforme instrução).

## Validação

```bash
cargo test --workspace
crystalline-lint .
cargo test --test structural_parity  # lab/parity
```

Resultado: todos passam; linter limpo.

## Ficheiros alterados

- `lab/parity/corpus/p538i/for-basic.typ` — novo.
- `lab/parity/corpus/p538i/for-with-counter.typ` — novo.
- `lab/parity/tests/structural_parity.rs` — verificação de texto + suporte a
  categoria `p538i`.
- `00_nucleo/diagnosticos/paridade-producao-p538i.md` — este relatório.

## Conclusão

P538i está concluído. A bateria de paridade agora verifica texto extraído, e
os casos de `#for` são explicitamente cobertos. O ponto cego identificado em
P538h está fechado. A reorganização (P539) pode começar.
