# Passo 124 — Relatório (testes de disciplina CLI)

**Data**: 2026-04-24
**Precondição**: Passo 123 encerrado; 1035 total tests; zero
violations; 51 ADRs activas.
**Natureza**: puramente defensivo. Zero código de produção.
Materializa em testes automatizados invariantes estruturais
que existiam como convenção.
**ADR**: **sem tocar**. Apenas valida ADR-0046 (CLI mínima,
stdout/stderr discipline) e ADR-0045 (warnings antes de errors).

---

## Sumário

+7 testes de integração em `04_wiring/tests/cli.rs`
(prefixo `disciplina_`). Cobrem:

1. **stdout vazio** em sucesso.
2. **stdout vazio** em erro.
3. **PDF magic header** `%PDF-`.
4. **PDF trailer** `%%EOF` nos últimos 16 bytes.
5. **stderr vazio** em compilação limpa.
6. **Exit 0 → PDF não-vazio**.
7. **Warnings antes de errors** em input misto.

Todos passam sem mudar L1/L2/L3/L4 de produção. Gate 124.A
(inventário empírico) positivo em todas as 3 dimensões.

**811 L1 + 24 L2 + 186 L3 + 21 L4 (+7)** + 6 ignorados =
**1042 total** (+7 novos testes). Zero violations. **51 ADRs
activas** (+0).

---

## 124.A — Inventário (gates empíricos)

Inventário completo em
`00_nucleo/diagnosticos/inventario-disciplina-cli-passo-124.md`.

### Gate 1 — stdout vazio em compilação normal

```
$ typst clean.typ -o clean.pdf
exit=0
stdout length: 0
```

Vazio. **Nenhum bug latente de produção.**

### Gate 2 — Formato PDF exacto

```
Header (8 bytes): %   P   D   F   -   1   .   7
Trailer (16 bytes last): t  x  r  e  f \n  6  6  9 \n  %  %  E  O  F \n
```

Header é `%PDF-1.7`; `%%EOF` aparece nos últimos 6 bytes (com
newline terminal). Assertions `starts_with(b"%PDF-")` e janela
de 16 bytes com `windows(5) == b"%%EOF"` cobrem variações.

### Gate 3 — Input misto viável

Input:
```typ
#set text(font: "X")
#variavel_desconhecida
```

Output stderr:
```
mix.typ:1:11: warning: text: propriedade 'font' ainda não suportada
  hint: ver ADR-0040 para propriedades cobertas por set text
mix.typ:2:2: error: unknown variable: variavel_desconhecida
```

Warning aparece antes de error. **Teste de ordem incluído.**

---

## 124.B — Implementação

### Ficheiro tocado

`04_wiring/tests/cli.rs` — 7 novos testes (~200 linhas).

### Testes

| # | Nome | Afirma |
|---|------|--------|
| 1 | `disciplina_stdout_vazio_em_sucesso` | `result.stdout.is_empty()` + exit 0 |
| 2 | `disciplina_stdout_vazio_em_erro` | `result.stdout.is_empty()` + exit 1 |
| 3 | `disciplina_pdf_magic_header` | `bytes.starts_with(b"%PDF-")` |
| 4 | `disciplina_pdf_trailer_eof` | `tail_16.windows(5).any(== b"%%EOF")` |
| 5 | `disciplina_stderr_vazio_em_compilacao_limpa` | `stderr.is_empty()` |
| 6 | `disciplina_exit_zero_implica_pdf_nao_vazio` | `metadata.len() > 0` |
| 7 | `disciplina_warnings_antes_de_errors` | `pos(warning:) < pos(error:)` |

### Zero código de produção

L1, L2, L3, L4 `main.rs` — intactos. Nenhum prompt L0 tocado.
`crystalline-lint --fix-hashes` reportou **"Nothing to fix"**.

---

## 124.C — Encerramento

### `cargo test --workspace`

```
test result: ok. 811 passed ...       (L1 inalterado)
test result: ok. 186 passed, 6 ignored (L3 inalterado)
test result: ok. 24 passed  ...       (L2 inalterado)
test result: ok. 21 passed  ...       (L4 +7: disciplina_*)
```

### `crystalline-lint .`

```
✓ No violations found
```

### Números finais

| Métrica | Antes (Passo 123) | Depois |
|---------|------:|-------:|
| L1 tests | 811 | 811 |
| L2 tests | 24 | 24 |
| L3 tests | 186 | 186 |
| L4 tests | 14 | **21** (+7) |
| **Total** | **1035** | **1042** (+7) |
| Ignorados | 6 | 6 |
| Violations | 0 | 0 |
| ADRs activas | 51 | 51 |
| DEBTs abertos | 11 | 11 |

---

## Descobertas empíricas registadas

1. **PDF header**: `%PDF-1.7` (8 bytes) — versão fixa. Test
   usa `starts_with(b"%PDF-")` para aceitar outra versão se
   alguma vez mudar.
2. **PDF trailer**: `%%EOF\n` (6 bytes). Janela de 16 é
   conservadora — seria suficiente 8.
3. **Input misto confirma ADR-0045**: warnings colectados no
   Sink são drenados **antes** do error do eval ser emitido.
   Mesmo com eval a abortar no segundo token, o set-text do
   primeiro já escreveu no Sink.
4. **stdout 100% limpo**: nem em sucesso, nem em erro, nem em
   warning-only. `println!` parasita seria apanhado
   imediatamente por 2 testes.

---

## Lições

1. **Gate empírico antes de escrever testes**: 3 comandos shell
   (stdout check, od -c, input misto) em 5 segundos confirmaram
   viabilidade dos 7 testes. Evita commitar teste que depende
   de comportamento não verificado.

2. **Input misto é sinal de qualidade do Sink**: que warnings
   colectados em comemo-tracked data sobrevivam a erro fatal
   de eval e sejam ordenados correctamente — é propriedade
   emergente dos Passos 104–108. Teste 7 materializa essa
   garantia.

3. **Teste defensivo tem ROI futuro**: se um `println!` de
   debug por engano entrar em L3 ou L4 num passo futuro,
   `disciplina_stdout_vazio_*` apanha antes do PR ser merged.
   Custo: 40 linhas; valor: evita bug "por que o PDF está
   corrompido?" em pipelines shell que assumem stdout binary.

4. **`windows(5).any(== b"%%EOF")` > index fixo**: PDFs podem
   ter padding variável após EOF. Procurar numa janela é
   robusto a variações.

5. **Zero mudança em prompt L0 → zero hash refix**: passo que
   só adiciona testes é totalmente não-invasivo. Lint
   `Nothing to fix` confirma.

---

## Limitações aceites

1. **PDF não é validado como "bem formado"**: não há parser
   PDF. Se o exporter produzir magic+EOF mas xref corrupto,
   testes passam. Validação rigorosa é passo dedicado com
   parser externo — registado em candidatos futuros.
2. **Sem teste de binário em stdout** (`-o -` para stdout):
   cristalino não suporta. Quando suportar, disciplina
   stdout muda de "sempre vazio" para "só bytes PDF em modo
   stdout". Teste actualiza.
3. **Teste de ordem cobre só 1 caso**: não valida se *dois
   warnings* mantêm ordem, ou *dois errors*. Conjunto
   warnings→errors é o invariante principal da ADR-0045;
   ordem dentro de cada bucket é determinada pelo comemo
   (Sink) e não é testada aqui.

---

## Estado pós-Passo 124

### Testes L4 (21 total)

- 5 testes básicos (Passo 114): warn, err eval, err I/O, sem
  args, sucesso limpo.
- 2 testes output (Passo 120): omitido, flag `-o`.
- 3 testes font-path (Passo 122): explícito, repetível,
  inexistente.
- 3 testes env vars (Passo 123): TYPST_ROOT, flag>env,
  TYPST_FONT_PATHS delimiter.
- 1 teste root (Passo 121): explícito.
- **7 testes disciplina** (Passo 124): este passo.

### Trabalho futuro

1. **Validador PDF real**: parser externo (lopdf ou similar)
   em dev-dependency; teste opcional.
2. **stdout binary mode (`-o -`)**: passo dedicado se
   surgir procura.
3. **Disciplina ANSI**: se `--color=always` + stderr não-tty,
   cores aparecem correctamente. Já testado indirectamente
   em `diagnostic::tests::formato_com_cores_*` (L2).
4. **Teste de subcomandos** (`compile`, `watch`, `query`) —
   quando existirem.
