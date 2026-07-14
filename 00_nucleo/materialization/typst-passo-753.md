---
# P753 — Fonte por defeito do cristalino deve bater com a do vanilla (`Libertinus Serif`, não `Liberation Serif`)

> **Passo:** 753
> **Data:** 2026-07-14
> **Foco:** P752 encontrou que o cristalino usa `Liberation Serif` como fonte por defeito, enquanto o vanilla usa `Libertinus Serif`, e atribuiu isto a "fontes diferentes" sem confirmar a causa. Este passo confirma se é uma limitação de ambiente (Libertinus Serif não instalada) ou uma divergência de política (lista de fontes por defeito diferente da do vanilla), e corrige.
> **Tipo:** Sonda + Implementação.
> **Tamanho:** S se for só actualizar uma lista de nomes; M se for preciso obter/embutir os ficheiros de fonte.
> **ADR-0108 EM VIGOR.**
> **Dependências:** P752 (onde a divergência foi encontrada e atribuída sem confirmação).

---

## Sonda

### Confirmar a lista completa de fontes por defeito do vanilla, não só o nome principal

```bash
grep -rn "Libertinus\|DejaVu\|default.*font\|FALLBACK" lab/typst-original/crates/typst-library/src/text/mod.rs lab/typst-original/crates/typst-kit/src/fonts.rs 2>/dev/null | head -30
```

O vanilla normalmente usa uma **cadeia** de fontes por defeito (serifada principal + fallbacks para símbolos, emoji, CJK, etc.), não uma só. Confirmar a lista completa e a ordem.

### Confirmar se `Libertinus Serif` está disponível neste ambiente

```bash
fc-list | grep -i libertinus
find / -iname "*libertinus*" 2>/dev/null | grep -i serif
```

Confirmar se os ficheiros de fonte existem no sistema, ou se precisam de ser obtidos/embutidos.

### Confirmar a lista de fontes por defeito actual do cristalino

```bash
grep -rn "Liberation\|default.*font\|FALLBACK" 01_core/src/ 03_infra/src/ --include="*.rs" | grep -i font | grep -v test | head -30
```

Confirmar onde a lista actual está definida, e se `Liberation Serif` é uma escolha deliberada (talvez como fallback por `Libertinus` não estar disponível na altura em que essa parte foi escrita) ou um valor arbitrário nunca revisitado.

### Critério de fecho da sonda

- [ ] Lista completa de fontes por defeito do vanilla confirmada (não só o nome principal).
- [ ] Confirmado se `Libertinus Serif` está disponível neste ambiente.
- [ ] Lista actual do cristalino confirmada, com a razão histórica (se encontrável) para `Liberation Serif`.

---

## Implementação

Se `Libertinus Serif` (e o resto da cadeia de fallback confirmada) estiver disponível: actualizar a lista de fontes por defeito do cristalino para bater exactamente com o vanilla, mesma ordem.

Se não estiver disponível: obter os ficheiros de fonte (licença SIL Open Font License, redistribuível) e embutir/instalar no ambiente de build, seguindo o mecanismo já usado para outras fontes deste projecto.

### Critério de fecho da implementação

- [ ] Fonte por defeito do cristalino bate com o vanilla, mesma cadeia de fallback, mesma ordem.
- [ ] Documentos já testados nesta conversa que dependiam da fonte por defeito (não especificaram `font:` explicitamente) re-verificados, dado que isto muda a aparência visual de qualquer texto sem fonte explícita.

---

## Validação

```bash
cat > /tmp/p753-fonte.typ <<'EOF'
X
EOF
lab/typst-original/target/release/typst compile /tmp/p753-fonte.typ /tmp/p753-vanilla.pdf
mutool show /tmp/p753-vanilla.pdf | grep -i "BaseFont"
./target/release/typst /tmp/p753-fonte.typ /tmp/p753-depois.pdf
mutool show /tmp/p753-depois.pdf | grep -i "BaseFont"
```

Confirmar que a fonte embutida é a mesma.

```bash
mutool show /tmp/p753-vanilla.pdf 4 2>&1 | head -10
mutool show /tmp/p753-depois.pdf 4 2>&1 | head -10
```

Confirmar se o resíduo de ~0,03pt (P752) desaparece agora que a fonte é a mesma.

```bash
cargo test --workspace
crystalline-lint .
```

Dado o alcance (fonte por defeito afecta qualquer documento sem `font:` explícito), correr o corpus completo e confirmar visualmente uma amostra de documentos já testados nesta conversa, para confirmar que a mudança visual é a esperada (aproximação ao vanilla), não uma regressão de aparência.

---

## Critério de fecho do passo

- [ ] Sonda completa, cadeia de fallback confirmada, disponibilidade confirmada.
- [ ] Fonte por defeito corrigida para bater com o vanilla.
- [ ] Resíduo de ~0,03pt (P752) verificado — eliminado ou reduzido ainda mais.
- [ ] Sem regressão em `cargo test --workspace`, corpus completo verificado.
- [ ] `crystalline-lint .` limpo.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p753.md`, com hash do commit.
