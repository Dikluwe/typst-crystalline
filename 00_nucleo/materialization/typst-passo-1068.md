# L0 — Passo 1068: Expansão dos Módulos de Constantes por Domínio (`export/`, `stdlib/text/`)

**Gate**: `ADR-0127` — não é mudança de comportamento por defeito (valores
permanecem os mesmos, só muda onde vivem), mas é refactor estrutural em dois
domínios novos, tocando imports em múltiplos arquivos. **Requer confirmação do
dono antes de executar**, mesmo padrão de cautela já usado para mudanças
multi-arquivo nesta conversa (P1062, P1065, P1067).

**Base**: pendência registada no documento de continuação — P1058 criou
`layout/vanilla_defaults.rs` como piloto (`par.spacing`, `par.leading`,
`block.spacing`), confirmado sem virar hub (Fase F, fan-in contido dentro de
`layout/`). A decisão de replicar para `export/`/`stdlib/text/` ficou condicional
a esse piloto correr bem — correu, mas a expansão nunca foi escrita como passo.

---

## Parte 0 — Ler o prompt e o código reais do piloto antes de replicar

Per o protocolo do manifesto (prompt antes de código): não tenho, nesta
conversa, nem o L0 do P1058 nem o conteúdo real de
`01_core/src/compiler/layout/vanilla_defaults.rs`. Só tenho a menção dele no
documento de continuação (resumo, não o arquivo). Antes de desenhar a expansão,
preciso de ver o padrão real, não presumir a partir do nome.

**Pode subir**:
- `00_nucleo/prompts/compiler/layout/vanilla_defaults.md` (se existir com esse
  nome — confirmar caminho real primeiro, dado o histórico de nomes duplicados
  desta conversa).
- `01_core/src/compiler/layout/vanilla_defaults.rs`.

Sem isto, este L0 fica incompleto na Parte 2 — a Parte 1 (auditoria) pode
prosseguir de forma independente.

## Parte 1 — Auditoria: que constantes hardcoded existem em `export/` e `stdlib/text/`

Antes de criar qualquer módulo novo, levantar o que há para mover — não assumir
que a quantidade ou a natureza dos candidatos em `export/`/`stdlib/text/` é
semelhante ao piloto de `layout/` (`par.spacing`/`par.leading`/`block.spacing`
eram valores de tipografia com proveniência vanilla clara; `export/` pode ter
constantes de natureza diferente — formato de arquivo, versão de spec PDF, etc.
— que podem não caber no mesmo padrão).

```bash
grep -rnE "^\s*(pub\s+)?const\s+\w+.*=.*(0\.\d+|[0-9]+\.[0-9]+)" \
  03_infra/src/export/ 01_core/src/compiler/stdlib/text/ 2>/dev/null

grep -rn "// rationale:\|// ref:\|// spec:" \
  03_infra/src/export/ 01_core/src/compiler/stdlib/text/ 2>/dev/null
```

Para cada candidato encontrado:
1. É valor com proveniência específica (decisão de design/vanilla), como no
   piloto — ou é geometria/constante universal (mesma distinção já feita para
   V21 nas Categorias 1A/1B/1C, P1064-1067)? Se for universal, não é candidato a
   este módulo — é candidato a `// rationale:` no local, não a extracção.
2. Já tem citação (`// ref:`/`// spec:`) no local, ou está sem proveniência?
3. Quantos pontos de uso (fan-in) — se um valor só é usado num único arquivo,
   extrair para módulo de domínio pode não valer a pena (o próprio P1058
   presumivelmente só extraiu valores com mais de um consumidor — confirmar
   isto na Parte 0, não assumir).

## Parte 2 — Desenho, condicional à Parte 0

Só depois de ler o padrão real do piloto: propor
`03_infra/src/export/vanilla_defaults.rs` e/ou
`01_core/src/compiler/stdlib/text/vanilla_defaults.rs`, replicando a mesma
estrutura (nomes, forma de citação, forma de import) — não uma variante
inventada.

**Não presumir que os dois domínios recebem tratamento idêntico.** `export/` é
L3 (infra); `stdlib/text/` é L1 (core). As restrições de camada (`ADR-0004`,
zero I/O em L1) podem exigir formas diferentes de módulo de constantes nos dois
— confirmar antes de aplicar o mesmo desenho aos dois sítios.

## Critério de conclusão

- Parte 0: prompt e código do P1058 lidos, padrão real confirmado (fan-in
  mínimo para extrair, forma de citação, etc.).
- Parte 1: inventário de candidatos em `export/` e `stdlib/text/`, classificados
  (proveniência específica vs universal, já citado vs não).
- Parte 2: desenho proposto, não executado — execução é passo seguinte,
  condicional a confirmação do dono.
- Nenhuma mudança de código neste passo.
