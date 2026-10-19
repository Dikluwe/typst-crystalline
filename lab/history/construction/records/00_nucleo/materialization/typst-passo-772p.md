---
# P772p — L0 + Implementação: formato de imagem inválido/desconhecido deve ser erro de compilação

> **Passo:** 772p
> **Data:** 2026-07-16
> **Foco:** P772k confirmou (reconfirmando P650, nunca corrigido desde então) que `#image()` com formato não reconhecido ou corrompido é silenciosamente omitido — `eprintln!` no terminal, PDF gerado sem a imagem, exit 0. O vanilla trata isso como erro de compilação (`error: failed to decode image (...)`, exit 1, amarrado ao span do `#image()`). Causa: a única verificação de formato acontece tarde demais, em L3 (exportação PDF), quando já não há caminho para devolver um `SourceDiagnostic` amarrado ao span original. Este passo move a verificação para L1 (tempo de avaliação de `native_image`), fechando também a lacuna de `image::svg` (P772k) do ponto de vista do observável — não implementando SVG, mas parando de fingir sucesso quando o formato não é suportado.
> **Tipo:** L0 + Implementação (nova dependência injetada em `EvalContext`/`World`, padrão análogo a `ImageSizer` já existente).
> **Tamanho:** M.
> **ADR-0108 EM VIGOR.** **Regra de ouro do CLAUDE.md** — L0 antes de código, dado ser decisão arquitetural nova (novo trait injetado), não patch pontual.
> **Dependências:** P772k (achado, commit `61b7edee78fdae9b020e458f5989f638cbf04096`), P650 (achado original, nunca corrigido).

---

## Sonda — confirmar o mecanismo exato do vanilla

```bash
grep -n "fn decode\|failed to decode image\|ImageError" lab/typst-original/crates/typst-library/src/visualize/image/*.rs 2>/dev/null
```

Confirmar:
1. Em que ponto exato do vanilla a decodificação acontece (tempo de avaliação de `image()`, não de exportação) — confirma que o cristalino está estruturalmente atrasado, não só com uma mensagem diferente.
2. A mensagem de erro exata para formato desconhecido vs formato reconhecido mas corrompido (podem ser mensagens distintas — confirmar antes de assumir uma só).
3. Se o vanilla decodifica a imagem inteira nesse momento (custo de decodificação em avaliação) ou só valida o cabeçalho/formato, deixando a decodificação completa para exportação (mais barato, mais próximo do que já existe em L3).

```bash
cat > /tmp/p772p-bogus.typ <<'EOF'
#image("bogus.png")
EOF
printf '\x00\x01\x02\x03garbage-not-an-image' > /tmp/p772p-bogus.png
lab/typst-original/target/release/typst compile /tmp/p772p-bogus.typ 2>&1
```

---

## Decisão de âmbito

| Opção | Custo | Quando escolher |
|---|---|---|
| Validar cabeçalho/formato em L1 (leve), decodificação completa continua em L3 | Baixo — só verificação de magic bytes | Se o vanilla também faz assim (confirmar na sonda) |
| Decodificar a imagem inteira em L1 | Alto — duplica trabalho já feito em L3, ou move-o inteiro | Só se o vanilla de fato decodifica em avaliação e há razão para replicar (ex: erros que só aparecem na decodificação completa, não no cabeçalho) |

Registar a escolha com base na sonda, não por conveniência de implementação.

---

## Implementação

### 1. L0

Escrever `00_nucleo/prompts/infra/image-format-validator.md` (ou nome análogo ao já usado para `ImageSizer`), especificando o trait novo (nome a definir — `ImageFormatValidator` ou reaproveitar `ImageSizer` se fizer sentido, confirmar sobreposição de responsabilidade antes de criar um trait redundante).

### 2. Trait em L1

```rust
pub trait ImageFormatValidator {
    fn validate(&self, data: &[u8]) -> Result<(), ImageFormatError>;
}
```

### 3. Implementação em L3

Reutilizar a lógica de `detect_format` já existente em `03_infra/src/export/images.rs` — não duplicar, extrair para um local partilhado entre L1 (validação em avaliação) e L3 (exportação), se a arquitetura permitir sem violar a fronteira L1/L3.

### 4. Ligação em `native_image`

`01_core/src/engine/stdlib/figure_image.rs`: depois de `world.read_bytes`, antes de devolver `Value::Content`, invocar o validador. Se inválido, devolver `SourceResult::Err` amarrado ao span do argumento do `#image()`.

---

## Validação

```bash
cat > /tmp/p772p-test-corrupt.typ <<'EOF'
#image("bogus.png")
EOF
./target/release/typst compile /tmp/p772p-test-corrupt.typ 2>&1
```

Confirmar erro de compilação, exit 1, mensagem próxima à do vanilla, span correto (apontando para `"bogus.png"` no documento, não `<detached>` — reaproveitar a disciplina de span estabelecida em P772b/P772d).

```bash
cat > /tmp/p772p-test-svg.typ <<'EOF'
#image("test.svg")
EOF
./target/release/typst compile /tmp/p772p-test-svg.typ 2>&1
```

Confirmar que SVG (ainda não suportado) agora dá erro de compilação claro em vez de omissão silenciosa — não implementa SVG, mas para de mentir que teve sucesso.

```bash
# Confirmar que PNG/JPEG válidos continuam funcionando sem regressão
cat > /tmp/p772p-test-valid.typ <<'EOF'
#image("valid.png")
EOF
```

```bash
cargo test --workspace
crystalline-lint .
```

---

## Critério de fecho do passo

- [ ] Mecanismo do vanilla confirmado (ponto de validação, mensagens exatas, profundidade de decodificação).
- [ ] Decisão de âmbito registrada (validação leve vs decodificação completa em L1).
- [ ] L0 escrito antes do código.
- [ ] Trait em L1, implementação reaproveitando `detect_format` de L3, sem duplicar lógica.
- [ ] `native_image` liga a validação, erro amarrado ao span correto.
- [ ] Formato inválido/corrompido dá erro de compilação, não omissão silenciosa.
- [ ] SVG (ainda não suportado) dá erro claro, não omissão silenciosa.
- [ ] PNG/JPEG válidos sem regressão.
- [ ] `cargo test --workspace` verde.
- [ ] `crystalline-lint .` zero violações.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p772p.md`.

---

## Próximo passo

Seguir com §2.2 (mensagem de mutação de variável capturada) ou §2.4 (hint de subtração), ou reconfirmar `lacuna-inventario` — conforme combinado, todos os itens da lista serão feitos, a ordem exata fica em aberto.
