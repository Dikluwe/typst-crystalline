# Passo 124.A — Inventário de disciplina CLI

**Data**: 2026-04-24
**Método**: checks empíricos contra binário `./target/release/typst`
pós-123.

---

## Check 1 — stdout em compilação normal

```bash
$ typst /tmp/clean.typ -o /tmp/clean.pdf
exit=0
stdout length: 0
```

**Vazio**. Gate 124.A.1 **passa** — nenhum bug latente de produção.

---

## Check 2 — Formato PDF

### Header (primeiros 8 bytes)

```
0000000   %   P   D   F   -   1   .   7
```

**`%PDF-1.7`** — header OpenPDF standard. `starts_with(b"%PDF-")`
aceita.

### Trailer (últimos 16 bytes)

```
0000000   t   x   r   e   f  \n   6   6   9  \n   %   %   E   O   F  \n
```

`%%EOF` seguido de newline; aparece nos últimos ~6 bytes. Janela
de 16 bytes com `windows(5) == b"%%EOF"` é robusta.

---

## Check 3 — Input misto (warning + error) viável?

Input testado:
```typ
#set text(font: "X")
#variavel_desconhecida
```

Output em stderr:
```
/tmp/.../mix.typ:1:11: warning: text: propriedade 'font' ainda não suportada
  hint: ver ADR-0040 para propriedades cobertas por set text
/tmp/.../mix.typ:2:2: error: unknown variable: variavel_desconhecida
```

Exit code: **1**.

**Warning antes de error** — ordem confirmada. Gate 124.A.5
**passa**. Teste de ordem incluído.

---

## Decisões

| Gate | Resultado | Acção |
|------|-----------|-------|
| 1 — stdout vazio | ✓ | `disciplina_stdout_vazio_em_sucesso` + `_em_erro` |
| 2 — header `%PDF-` | ✓ | `disciplina_pdf_magic_header` |
| 2 — trailer `%%EOF` | ✓ (últimos 16 bytes) | `disciplina_pdf_trailer_eof` |
| 3 — stderr vazio em clean | ✓ (confirmado em tests 114) | `disciplina_stderr_vazio_em_compilacao_limpa` |
| 4 — exit 0 → PDF não-vazio | ✓ | `disciplina_exit_zero_implica_pdf_nao_vazio` |
| 5 — warnings antes de errors | ✓ | `disciplina_warnings_antes_de_errors` |

**6 testes** incluídos. Zero candidatos futuros — todas as
invariantes testáveis hoje.
