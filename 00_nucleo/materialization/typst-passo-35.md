# Passo 35 — Paridade de parsing via oráculo

**Pré-condições**:
- Passo 34 concluído: 430 testes L1 + 51 testes L3, zero violations
- Branch: `cristalino/migration`

**Verificação antes de começar**:
```bash
# Confirmar que parity_runner existe e funciona
cargo run -p parity_runner -- --help 2>/dev/null \
  || ls lab/parity_runner/src/ 2>/dev/null \
  || echo "parity_runner não encontrado"

# Ver estrutura actual do parity_runner
cat lab/parity_runner/src/main.rs 2>/dev/null | head -50

# Confirmar que lab/typst-original compila
cargo build -p typst 2>/dev/null | tail -3 \
  || echo "typst original não compila — verificar"

# Confirmar testes de paridade existentes (40 testes mencionados no estado)
grep -rn "#\[test\]\|fn parity_" \
  lab/parity_runner/src/ 2>/dev/null | head -20
```

**Parar se `parity_runner` não existir ou `lab/typst-original` não compilar.**

---

## Contexto

Os passos 1–34 construíram o motor cristalino de forma incremental.
Antes de entrar na fase de matemática (Passo 36), é necessário validar
que o parser cristalino produz árvores de sintaxe idênticas ao oráculo
para o corpus de documentos relevante.

O `parity_runner` já existia com 40 testes de paridade de parsing
(conforme o ficheiro de estado). Este passo:

1. Corre todos os testes de paridade existentes e regista resultados
2. Adiciona testes de paridade específicos para matemática (equações,
   frações, attachments) que o Passo 34 introduziu
3. Regista divergências encontradas — não as corrige neste passo
   (correcções são trabalho do motor, não do parser)
4. Produz um relatório de cobertura de paridade que serve de baseline
   para o Passo 36

**O que este passo não faz**: corrigir divergências de eval ou layout.
O foco é exclusivamente no parsing — `SyntaxNode` produzido pelo
cristalino vs oráculo para o mesmo input.

---

## Tarefa 1 — Diagnóstico do parity_runner

```bash
# Ver os testes existentes em detalhe
cat lab/parity_runner/src/main.rs

# Ver CompactNode — como normaliza a árvore para comparação
cat lab/parity_runner/src/compact.rs 2>/dev/null \
  || grep -n "CompactNode\|compact_cristalino\|compact_original" \
     lab/parity_runner/src/*.rs | head -20

# Correr os 40 testes existentes e ver resultados
cargo test -p parity_runner 2>&1

# Ver exemplos de documentos usados nos testes
grep -rn "MockWorld\|typst-original\|parse\|source" \
  lab/parity_runner/src/*.rs | head -20
```

**Parar. Reportar:**
1. Quantos dos 40 testes passam actualmente?
2. Quais falham e com que divergência (resumo por categoria)?
3. `CompactNode` normaliza o quê — ignora spans? Ignora whitespace?
   Como trata erros de parse?

---

## Tarefa 2 — Testes de paridade para matemática

Adicionar ao `parity_runner` testes para os `SyntaxKind`s matemáticos
introduzidos no Passo 34:

```rust
// Em lab/parity_runner/src/main.rs ou ficheiro de testes separado

#[test]
fn parity_equation_inline_simples() {
    let src = "$x$";
    assert_eq!(compact_cristalino(src), compact_original(src),
        "divergência em equação inline simples");
}

#[test]
fn parity_equation_block_simples() {
    let src = "$ x^2 $";
    assert_eq!(compact_cristalino(src), compact_original(src),
        "divergência em equação block simples");
}

#[test]
fn parity_equation_frac() {
    let src = "$ frac(a, b) $";
    assert_eq!(compact_cristalino(src), compact_original(src),
        "divergência em frac");
}

#[test]
fn parity_equation_attach_sup() {
    let src = "$ x^2 $";
    assert_eq!(compact_cristalino(src), compact_original(src),
        "divergência em attach sup");
}

#[test]
fn parity_equation_attach_sub() {
    let src = "$ x_i $";
    assert_eq!(compact_cristalino(src), compact_original(src),
        "divergência em attach sub");
}

#[test]
fn parity_equation_attach_sub_sup() {
    let src = "$ x_i^2 $";
    assert_eq!(compact_cristalino(src), compact_original(src),
        "divergência em attach sub+sup");
}

#[test]
fn parity_equation_root() {
    let src = "$ sqrt(x) $";
    assert_eq!(compact_cristalino(src), compact_original(src),
        "divergência em sqrt");
}

#[test]
fn parity_equation_complexa() {
    let src = "$ sum_(i=0)^n x_i^2 $";
    assert_eq!(compact_cristalino(src), compact_original(src),
        "divergência em equação complexa com sum");
}

#[test]
fn parity_equacao_inline_em_texto() {
    let src = "O valor de $x^2 + y^2$ é positivo.";
    assert_eq!(compact_cristalino(src), compact_original(src),
        "divergência em equação inline em texto");
}

#[test]
fn parity_equacao_com_texto_literal() {
    let src = "$ \"resultado\" = x $";
    assert_eq!(compact_cristalino(src), compact_original(src),
        "divergência em equação com texto literal");
}
```

**Nota**: se `compact_cristalino` e `compact_original` não existirem
com estes nomes exactos, adaptar para a API actual do `parity_runner`.
Reportar os nomes reais.

---

## Tarefa 3 — Correr paridade completa e registar resultados

```bash
# Correr todos os testes de paridade (existentes + novos)
cargo test -p parity_runner 2>&1 | tee /tmp/parity_results.txt

# Sumário: quantos passam / falham
grep -E "^test.*ok$|^test.*FAILED$" /tmp/parity_results.txt | sort

# Para cada teste falhado, ver a divergência completa
cargo test -p parity_runner 2>&1 | grep -A 20 "FAILED\|thread.*panicked"
```

---

## Tarefa 4 — Registar divergências em `DEBT.md`

Para cada categoria de divergência encontrada, adicionar entrada em
`DEBT.md`. Não corrigir — registar.

Formato:

```markdown
### DEBT-9 — Divergência de parsing: [categoria] — BAIXA/MÉDIA/ALTA

**Descoberto no Passo 35 (paridade de parsing)**

**Descrição**: [o que diverge e como]

**Exemplo**:
- Input: `[snippet que diverge]`
- Cristalino: `[CompactNode cristalino]`
- Oráculo: `[CompactNode oráculo]`

**Impacto**: [afecta motor de equações? afecta eval? afecta só casos edge?]

**Quando resolver**: Passo 36+ (se afecta matemática) / Passo futuro
```

Usar DEBT-9, DEBT-10, etc. para cada categoria distinta. Uma categoria
pode cobrir múltiplos testes se a causa raiz é a mesma.

---

## Tarefa 5 — Relatório de cobertura de paridade

Criar `00_nucleo/materialization/parity-baseline-passo-35.md`:

```markdown
# Baseline de Paridade — Passo 35

**Data**: [data]
**Testes existentes antes do passo**: 40
**Testes adicionados (matemática)**: 10
**Total**: 50

## Resultados

| Categoria | Total | Passam | Falham |
|-----------|-------|--------|--------|
| Markup geral | N | N | N |
| Código/eval | N | N | N |
| Matemática (novos) | 10 | N | N |
| **Total** | 50 | N | N |

## Divergências por categoria

[lista de divergências encontradas, com DEBT# associado]

## Conclusão para Passo 36

[o que o motor de equações pode assumir sobre o parsing ser correcto,
e o que ainda diverge e requer atenção]
```

Este documento fica em `00_nucleo/materialization/` e serve como
referência para o Passo 36.

---

## Verificação final

```bash
cargo test -p typst-core 2>&1 | tail -3
cargo test -p typst-infra 2>&1 | tail -3
cargo test -p parity_runner 2>&1 | tail -5
cargo build 2>&1 | tail -3
crystalline-lint .
crystalline-lint --fix-hashes .
crystalline-lint .
# ✓ No violations found

# Confirmar que baseline existe
ls 00_nucleo/materialization/parity-baseline-passo-35.md
```

Critérios de conclusão:
- Todos os 40 testes de paridade existentes corridos e resultados registados ✓
- 10 testes de paridade matemática adicionados ✓
- Divergências encontradas registadas como DEBT-9+ em `DEBT.md` ✓
- `parity-baseline-passo-35.md` criado em `00_nucleo/materialization/` ✓
- Testes L1 e L3 não regridem (430 + 51 base) ✓
- Zero violations ✓

---

## Ao terminar, reportar

**Dos 40 testes existentes:**
- Quantos passam / falham?
- Quais as categorias de divergência encontradas?

**Dos 10 testes de matemática:**
- Quantos passam?
- `MathPrimes` e `MathAlignPoint` como `Content::Empty` causam
  divergência de parsing? (confirmar ou descartar)

**Divergências registadas:**
- Lista de DEBT-9+ criados com categoria e impacto estimado.

**Conclusão:**
- O parser cristalino está pronto para o Passo 36 (motor de equações)?
  Go / No-Go com razão.

**Go para Passo 36 — motor de equações (ADR-0032, fase obrigatória).**
