---
# P667 — O que acontece sem a dependência de Python para fontes variáveis?

> **Passo:** 667
> **Data:** 2026-07-10
> **Foco:** P666 revelou que a instanciação estática de fontes variáveis (a correcção da pendência de P525) depende de chamar Python (`fontTools.varLib.instancer`), configurado via `TYPST_CRYSTALLINE_PYTHON`. Isto nunca tinha sido mencionado explicitamente antes nesta conversa. Confirmar o que acontece quando essa variável não está definida, ou quando o Python/fontTools não estão disponíveis — se volta ao bug original de P525 em silêncio, ou se avisa claramente.
> **Tipo:** Verificação directa. Correcção se confirmado o problema.
> **Tamanho:** S.
> **ADR-0108 EM VIGOR.**
> **Dependências:** P666 (onde a dependência foi mencionada pela primeira vez), P525 (o bug original que esta dependência corrige).

---

## Verificação

### Confirmar o comportamento sem a variável de ambiente

```bash
unset TYPST_CRYSTALLINE_PYTHON
cat > /tmp/p667-vf-sem-python.typ <<'EOF'
#set text(font: "Ubuntu Sans", size: 40pt)
Hello world.

#set text(weight: 700)
Bold hello.
EOF
./target/release/typst /tmp/p667-vf-sem-python.typ /tmp/p667.pdf 2>&1
echo "Exit code: $?"
pdffonts /tmp/p667.pdf 2>/dev/null
mutool draw -o /tmp/p667.png -r 150 /tmp/p667.pdf 2>/dev/null
```

Confirmar: o comando falha com erro claro, produz aviso e continua com a instância default (voltando ao bug de P525, mas avisando), ou falha em silêncio, sem nada a indicar que a instanciação não aconteceu?

### Confirmar com Python definido mas `fontTools` em falta

```bash
export TYPST_CRYSTALLINE_PYTHON=/usr/bin/python3
python3 -c "import fontTools" 2>&1  # confirmar se está instalado neste ambiente; se estiver, usar um venv sem fontTools para o teste
./target/release/typst /tmp/p667-vf-sem-python.typ /tmp/p667-b.pdf 2>&1
echo "Exit code: $?"
```

### Localizar no código

```bash
grep -n "TYPST_CRYSTALLINE_PYTHON\|instantiate_variable_font" 03_infra/src/font_variant.rs | head -20
```

Confirmar se há tratamento de erro para os dois casos (variável não definida, Python sem `fontTools`), e o que esse tratamento faz.

### Critério de fecho da verificação

- [ ] Comportamento sem a variável de ambiente confirmado.
- [ ] Comportamento com Python mas sem `fontTools` confirmado.
- [ ] Código de tratamento de erro localizado e revisto.

---

## Decisão

Se o comportamento actual for "falha em silêncio, volta ao bug de P525 sem avisar": isto é uma falha silenciosa nova, da mesma categoria de todas as encontradas em P633-656. Corrigir para produzir, no mínimo, um aviso claro ("fontes variáveis não puderam ser instanciadas correctamente; a saída pode ter pesos visuais incorrectos"), ou, preferencialmente, um erro que impeça a compilação de produzir um PDF visualmente errado sem aviso nenhum.

Se já houver tratamento adequado: confirmar e documentar.

---

## Implementação, se necessário

Adicionar verificação explícita, no arranque da compilação ou no primeiro uso de uma fonte variável, de que a dependência de Python está disponível e funcional, com erro ou aviso claro se não estiver — não deixar a falha ser descoberta só ao inspeccionar o PDF final.

---

## Validação

```bash
cargo test --workspace
crystalline-lint .
```

---

## Critério de fecho do passo

- [ ] Comportamento sem Python/fontTools confirmado nos dois cenários.
- [ ] Se confirmada falha silenciosa: corrigida, com aviso ou erro claro.
- [ ] Sem regressão em `cargo test --workspace`.
- [ ] `crystalline-lint .` limpo.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p667.md`, com hash do commit.
- [ ] Documentação de instalação/requisitos do projecto (se existir) actualizada para mencionar a dependência de Python/fontTools como requisito para fontes variáveis.
