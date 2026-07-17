---
# P765b (lote 1) — Varredura da stdlib: próximo grupo de `lacuna-inventario`

> **Passo:** 765b (lote 1)
> **Data:** 2026-07-15
> **Foco:** P765a fechou os 4 itens `divida-confirmada` da lente de 2026-07-15. O grupo maior por tamanho, `lacuna-inventario` (400 itens), tem dois módulos já amostrados no diagnóstico original (`lente-falta-migrar-2026-07-15.md`, secção 4.2): `typst_library::foundations` (93 itens, quase todo `calc.*`, já confirmado implementado — lacuna de granularidade do inventário, não dívida) e `typst_library::diag` (24 itens, infra-estrutura Rust de erro, não símbolo de língua). Nenhum dos dois é candidato real. Este lote identifica e verifica o próximo módulo por tamanho dentro de `lacuna-inventario`, excluindo esses dois já descartados.
> **Tipo:** Sonda + Implementação directa (correcção imediata se o achado for pequeno e isolado, como em P765a).
> **Tamanho:** M.
> **ADR-0108 EM VIGOR.** **ADR-0107** — mecânica-não-língua fora do critério de paridade; distinguir símbolo de língua de infra-estrutura Rust antes de classificar qualquer item como dívida.
> **Dependências:** P765a (lote 0, fechado). Lista congelada `00_nucleo/diagnosticos/lente-lista-B-2026-07-15.txt` (641 linhas TSV, já existente).

---

## Sonda — identificar o próximo módulo por tamanho

```bash
awk -F'\t' '$1=="lacuna-inventario"' 00_nucleo/diagnosticos/lente-lista-B-2026-07-15.txt \
  | cut -f2 \
  | sed 's/::[^:]*$//' \
  | sort | uniq -c | sort -rn | head -15
```

Confirmar a contagem por módulo directamente (não assumir os números já citados em diagnósticos anteriores, que podem ter usado outro corte). Excluir `typst_library::foundations` e `typst_library::diag` (já descartados) e identificar o terceiro maior módulo.

Para cada item do módulo escolhido, aplicar a mesma disciplina de P765a: ler o código-fonte real antes de classificar, não confiar só na ausência/presença aparente.

```bash
# Modelo — repetir por item do módulo escolhido
grep -n "<símbolo>" 01_core/src/engine/stdlib/*.rs 01_core/src/entities/*.rs 2>/dev/null
```

---

## Implementação

Só para achados classificados como `bug real` (mesma régua de P765a: comparação directa de comportamento/saída contra o vanilla, não presença/ausência de string). Corrigir um a um, cada correcção com o seu próprio commit/teste, seguindo o padrão já usado em P765a (código + teste + comparação de `repr()`/renderização).

Se o módulo escolhido revelar mais de ~5 bugs reais distintos, não tentar corrigir todos neste mesmo lote — registar a lista completa e propor lote 1a/1b conforme o volume, em vez de acumular um passo grande demais para validar com segurança.

---

## Validação

```bash
cargo test --workspace
crystalline-lint .
```

Para cada bug corrigido, documento de sonda mínimo comparando vanilla vs cristalino, mesmo formato usado em P765a.

---

## Critério de fecho do passo

- [ ] Módulo seguinte por tamanho identificado por contagem directa, não por memória de diagnóstico anterior.
- [ ] Cada item do módulo lido no código-fonte antes de classificar (bug real / diferença aceitável ADR-0107 / lacuna de granularidade do inventário / falso-positivo).
- [ ] Bugs reais corrigidos um a um, com teste e comparação directa contra o vanilla.
- [ ] Se o volume exceder o razoável para um lote, dividido explicitamente (não forçado num só passo).
- [ ] `cargo test --workspace` verde.
- [ ] `crystalline-lint .` zero violações.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p765b.md`, com a tabela de classificação completa do módulo (não só os corrigidos).

---

## Próximo passo

P765c (lote 2): módulo seguinte por tamanho, mesma metodologia — a lista `lacuna-inventario` (400 itens) provavelmente exige vários lotes; não tentar esgotá-la num só passo.
