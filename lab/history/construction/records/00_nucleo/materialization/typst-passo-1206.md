# P1206 — dissolver o prompt coletivo de atomização

**Estado:** EXECUTADO — BIJEÇÃO GREEN; PREFLIGHT GLOBAL BLOQUEADO
**Baseline condicionado:** P1205 GREEN; V15=1.

Auditar os 29 consumers de `compiler/atomizacao_elementos.md`. Criar um Prompt
L0 específico por consumer de introspecção/layout, reaproveitando owner já
existente somente quando a bijeção for comprovada. Converter a obrigação
transversal da forma B ADR-0109 em `_nuclei/layout/element-form-b.toml` para os
prompts aplicáveis. O Markdown coletivo não conserva owner: preservar história
em diagnóstico e removê-lo de `prompts/` após todos os headers repointados.

Não zerar imports, não usar despacho dinâmico e não mover lógica: este lote é
somente documental. Gate final: V15 1→0, V26=0, todos os 29 sources ausentes de
V5 e byte-idênticos fora da linhagem, testes layout/introspect e build verdes.
Só após V15=V26=0 executar `--fix-hashes --dry-run` duplo; qualquer mutação
global exige passo posterior próprio. Fechar
`typst-p1206-fechamento-bijecao-l0.md` com inventário final e índice vazio.

## Resultado

V15=0 e V26=0; 29 owners sem V5 focal e sources byte-idênticos fora da
linhagem; 147 testes de introspecção, 788 de layout e build GREEN; índice
vazio. O dry-run duplo foi executado sem mutação e falhou deterministicamente
por 21 consumers preexistentes sem metadata canônica. O reparo global fica para
passo próprio. Evidência completa em
`00_nucleo/diagnosticos/typst-p1206-fechamento-bijecao-l0.md`.
