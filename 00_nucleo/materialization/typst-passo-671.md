---
# P671 — A verificação de Python (P666-669) corre sempre, mesmo sem fontes variáveis?

> **Passo:** 671
> **Data:** 2026-07-10
> **Foco:** P670 mostrou que o tempo absoluto do cristalino em documentos micro quase duplicou (de ~150-170ms para ~250-290ms) de forma uniforme nos 34 documentos, enquanto o vanilla subiu muito menos (~90ms para ~110-130ms). O relatório atribuiu isto a "variação de ambiente", mas essa explicação não distingue bem entre os dois lados — ruído de ambiente afectaria os dois de forma parecida. Um acréscimo fixo e uniforme sugere um custo de arranque novo, coincidindo com a introdução das verificações de Python/fontTools em P666-669. Este passo confirma se essa verificação corre sempre, mesmo em documentos sem fontes variáveis.
> **Tipo:** Sonda directa. Correcção se confirmado.
> **Tamanho:** S–M.
> **ADR-0108 EM VIGOR.** Uma diferença uniforme e consistente entre 34 documentos não é "ruído" só porque é conveniente explicá-la assim — ruído não costuma ser tão regular.

---

## Sonda

### Bissecção temporal directa: antes e depois de P666-669

```bash
git log --oneline | grep -iE "P665|P669" | head -5
```

Medir um documento micro simples, sem fontes variáveis, no commit imediatamente antes de P666 e no commit actual:

```bash
# checkout do commit antes de P666
cargo build --release --bin typst
hyperfine --warmup 3 --runs 10 './target/release/typst tools/perf/corpus/test-array.typ /tmp/p671-antes.pdf'

# checkout do commit actual (depois de P669)
cargo build --release --bin typst
hyperfine --warmup 3 --runs 10 './target/release/typst tools/perf/corpus/test-array.typ /tmp/p671-depois.pdf'
```

Comparar directamente os dois tempos, isolando o efeito de P666-669 de qualquer outra variação de ambiente entre sessões diferentes de benchmark.

### Confirmar se a verificação de Python corre incondicionalmente

```bash
grep -n "variable_font_instancer_available\|python_for_instancer\|TYPST_CRYSTALLINE_PYTHON" 03_infra/src/pipeline.rs 03_infra/src/font_variant.rs
```

Confirmar exactamente quando esta verificação é chamada — só quando o documento usa uma fonte variável com eixos não-default (o que `test-array.typ` não faz), ou em todos os documentos, independentemente do que usam.

### Critério de fecho da sonda

- [ ] Tempo medido directamente, antes e depois de P666-669, no mesmo ambiente, mesma sessão.
- [ ] Confirmado se a verificação de Python corre incondicionalmente ou só quando necessário.
- [ ] Se confirmado que corre sempre: localizado exactamente onde, com `file:line`.

---

## Implementação, se confirmado

Se a verificação de disponibilidade de Python estiver a correr sempre (por exemplo, invocando um subprocesso ou verificando o ambiente no arranque da pipeline, antes de saber se o documento sequer usa fontes variáveis): mover a verificação para dentro do caminho condicional que só é atingido quando uma fonte variável com eixos não-default é de facto detectada, não antes.

### Critério de fecho da implementação

- [ ] Verificação de Python só corre quando o documento usa fontes variáveis com eixos não-default.
- [ ] Documentos sem fontes variáveis voltam ao tempo de arranque anterior a P666.
- [ ] Testes de P667/P668/P669 (erro claro quando Python indisponível, para documentos que usam VF) continuam a passar sem regressão.

---

## Validação

```bash
hyperfine --warmup 3 --runs 10 './target/release/typst tools/perf/corpus/test-array.typ /tmp/p671-corrigido.pdf'
```

Confirmar que o tempo volta a aproximar-se do valor de P618 (~150ms), não do valor inflacionado de P670 (~250ms).

```bash
cargo test --workspace
crystalline-lint .
python3 tools/perf/benchmark-p507.py
```

Repetir o benchmark completo, confirmando que os documentos micro voltam a um rácio próximo de P618/P657, não o rácio inflacionado de P670.

---

## Critério de fecho do passo

- [ ] Bissecção directa confirmando ou refutando a hipótese, com números, não suposição.
- [ ] Se confirmado: verificação de Python corrigida para só correr quando necessário.
- [ ] Documentos sem fontes variáveis com tempo restaurado.
- [ ] Testes de P667-669 sem regressão.
- [ ] Sem regressão em `cargo test --workspace`.
- [ ] `crystalline-lint .` limpo.
- [ ] Benchmark completo repetido, confirmando o rácio corrigido.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p671.md`, com hash do commit.
- [ ] Se a hipótese for refutada: investigar a causa real do aumento uniforme, não aceitar "variação de ambiente" sem mais confirmação.
