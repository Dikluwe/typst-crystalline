---
# P583 — Confirmar eliminação de `font_size_pt` + alcance do bug de escape/shorthand

> **Passo:** 583
> **Data:** 2026-07-05
> **Foco:** Duas verificações que P581 e P582 deixaram por fazer. Primeiro: P582 afirma que `font_size_pt` foi "completamente eliminado", sem mostrar uma busca a provar isso — a mesma afirmação já falhou uma vez nesta sequência (P579). Segundo: P581 encontrou e corrigiu, de passagem, um bug de caracteres escapados e abreviaturas tipográficas a serem ignorados pelo avaliador de markup — não se sabe há quanto tempo isto existia nem se algum documento do corpus dependia do comportamento errado.
> **Tipo:** Verificação directa.
> **Tamanho:** S.
> **ADR-0108 EM VIGOR.** Uma afirmação de "completamente eliminado" precisa de uma busca a provar, não de uma caixa marcada.
> **Dependências:** P582 (afirmação sobre `font_size_pt`), P581 (correcção incidental de escape/shorthand).

---

## Parte 1 — Confirmar que `font_size_pt` foi mesmo eliminado

```bash
grep -rn "font_size_pt" 01_core/src/ 03_infra/src/ --include="*.rs"
```

Se a busca devolver zero resultados, a afirmação de P582 fica confirmada com prova. Se devolver algum resultado, a afirmação estava errada, e cada ocorrência precisa de ser avaliada da mesma forma que P579/P580 já fizeram.

### Critério de fecho da Parte 1

- [ ] Busca corrida, resultado registado (zero, ou lista de ocorrências).
- [ ] Se houver ocorrências: cada uma avaliada e corrigida, ou justificada por escrito porque fica.

---

## Parte 2 — Alcance do bug de escape/shorthand

### Confirmar há quanto tempo o bug existia

```bash
git log --oneline -- 01_core/src/rules/eval/mod.rs | tail -30
git log -p --follow -- 01_core/src/rules/eval/mod.rs | grep -n "Expr::Escape\|Expr::Shorthand\|Expr::Linebreak" | head -20
```

Confirmar desde quando o braço genérico (`_ => Ok(Value::None)`) capturava estes três casos, sem tratamento próprio.

### Procurar no corpus documentos que usem estas construções

```bash
grep -rln '\\#\|\\\$\|\\&\|\\\*\|\.\.\.\|--' lab/parity/corpus/ --include="*.typ"
```

Para cada ficheiro encontrado, confirmar se o teste de paridade já cobria esse ficheiro, e se alguma vez passou apesar do carácter escapado estar a desaparecer — o que confirmaria que o teste nunca verificou o conteúdo a esse nível de detalhe.

### Critério de fecho da Parte 2

- [ ] Origem do bug confirmada no histórico do git, não só "existia antes".
- [ ] Corpus verificado por documentos que usam escape/shorthand.
- [ ] Se algum documento do corpus usar estas construções e o teste tiver passado antes da correcção: confirma um ponto cego de cobertura, a registar, não só a corrigir em silêncio.

---

## Critério de fecho do passo

- [ ] Parte 1: busca de `font_size_pt` corrida e documentada.
- [ ] Parte 2: origem e alcance do bug de escape/shorthand confirmados.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p583.md`, com hash do commit.
- [ ] Inventário actualizado se for encontrado algo novo.
