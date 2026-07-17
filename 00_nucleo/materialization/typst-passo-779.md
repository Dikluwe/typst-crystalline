---
# P779 — Auditoria de `validate.py`: fixar rasterizador/resolução; registo da lição

> **Passo:** 779
> **Data:** 2026-07-16
> **Foco:** P778 confirmou que `validate.py` (usado em P776 e P777 para medir AE) produziu resíduos de 195/138 que não existem com a metodologia já estabelecida nesta linha de trabalho (`mutool draw -r 300` + `compare -metric AE`) — só apareciam com `pdftoppm -r 150`. Ninguém auditou o que `validate.py` fazia internamente antes de confiar nos seus números, ao contrário da disciplina de comandos explícitos usada em todos os outros passos desta cadeia. Isto custou dois passos (P776, P777) a perseguir um artefacto de medição. Este passo é curto: lê `validate.py`, corrige-o para usar a metodologia padrão, e regista a lição.
> **Tipo:** Sonda + Correcção de ferramenta de validação (não código do produto).
> **Tamanho:** S.
> **ADR-0108 EM VIGOR** — qualquer script de validação usado num relatório de paridade tem de ser auditado, não só nomeado.
> **Dependências:** P778 (achado do artefacto de medição, commit `0e355978c07c8892d34a0e168470edc8beac4ecc`).

---

## Sonda — ler `validate.py`

```bash
find . -iname "validate.py" -path "*temp_p776*" -o -iname "validate.py" -path "*p777*" 2>/dev/null
cat temp_p776/validate.py 2>/dev/null | grep -n "pdftoppm\|mutool\|dpi\|-r \|resolution"
```

Confirmar exactamente:
1. Que ferramenta de rasterização o script usa (`pdftoppm`, `mutool`, outra).
2. A que resolução.
3. Se essa escolha foi decidida em algum momento anterior desta linha de trabalho, ou se o script foi escrito de raiz nalgum dos passos P776/P777 sem referência à convenção já estabelecida.

---

## Correcção

Editar `validate.py` (ou o script equivalente, onde quer que esteja no repositório de trabalho — confirmar se é um artefacto temporário em `temp_p776/` ou se deveria ser promovido a um script permanente do projecto) para usar exactamente:

```python
# mutool draw -o <out>.png -r 300 <in>.pdf
```

em vez de `pdftoppm` a qualquer resolução, replicando a metodologia usada manualmente em P763c/P763e/P767b/P767c/P775/P778.

Se `validate.py` for reutilizável para além desta linha de trabalho (parece ser, dado ter sido usado em três passos), considerar promovê-lo para um local permanente (ex: `lab/parity/tools/` ou equivalente à convenção já usada por outros scripts de paridade mencionados nesta conversa, como `decompor_so_vanilla.py`), com a resolução/ferramenta fixada e documentada no próprio script (comentário ou constante nomeada, não um valor solto).

---

## Registo da lição

Adicionar ao handoff/CLAUDE.md (ou documento de regras equivalente) uma linha explícita, no mesmo espírito das regras já numeradas lá:

> Qualquer script de validação (`validate.py` ou equivalente) usado para gerar números de um relatório de paridade tem de ser lido e confirmado — não confiado pelo nome — antes de aceitar os seus resultados. A resolução e ferramenta de rasterização usadas têm de bater com a convenção do projecto (`mutool draw -r 300`), e essa convenção deve estar fixada no próprio script, não deixada a cargo de quem o escreveu num passo específico.

Não precisa de número de regra formal se o handoff não tiver esse mecanismo — registar no local que já existe para isto (ex: secção de "regras em vigor" do handoff, ou onde as lições de P745-762 sobre "diferença mecânica sem prova directa" já estão registadas).

---

## Validação

```bash
cd temp_p776
../lab/.venv/bin/python validate.py
```

Confirmar que os números voltam a bater com o que P778 já mediu manualmente (AE=0 para as 8 orientações com `mutool -r 300`).

---

## Critério de fecho do passo

- [ ] `validate.py` lido e a ferramenta/resolução original confirmada.
- [ ] Script corrigido para usar `mutool draw -r 300`, consistente com a convenção já estabelecida.
- [ ] Reexecução de `validate.py` confirma AE=0 para as 8 orientações (bate com P778).
- [ ] Lição registada por escrito, em local persistente (handoff ou equivalente), não só neste relatório.
- [ ] Decisão registada sobre se `validate.py` fica temporário (`temp_p776/`, a apagar) ou é promovido a ferramenta permanente do projecto.

---

## Próximo passo

Nenhum — este passo fecha a lição. Retomar a varredura da stdlib em P772a (lote 3).
