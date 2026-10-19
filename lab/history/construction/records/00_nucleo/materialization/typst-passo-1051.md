# Passo 1051 — Corrigir `Ascent`/`Descent` divergente no `FontDescriptor` exportado

**Tipo**: Investigar → gate (`ADR-0127` — metadado do PDF, avaliar se conta como
categoria 2/3 mesmo não afectando posição visual) → corrigir.
**Achado (P1050)**: o `FontDescriptor` embutido no PDF do cristalino declara
`Ascent`/`Descent` diferentes do vanilla, mesmo com a mesma fonte de origem embutida —
descoberto ao inspeccionar o stream de conteúdo directamente. Vanilla: `Ascent = 918`,
`Descent = -222` (unidades de fonte, projecta caixa de 12.54pt a 11pt). Cristalino:
projecta 11.00pt — não bate.
**Pré-condição**: `git status` limpo. HEAD ≥ Passo 1050.

---

## Fase A — Localizar onde o `FontDescriptor` é construído

1. Encontrar o exportador PDF (`03_infra/src/export/fonts.rs` ou equivalente, já citado
   no P1050 e no achado V18) — confirmar onde `Ascent`/`Descent` são escritos no
   dicionário `/FontDescriptor`.
2. Confirmar se os valores vêm de:
   - Leitura directa da tabela `hhea`/`OS/2` da fonte embutida (correcto — estes campos
     são por definição da fonte, não calculados pelo compilador); ou
   - Um valor calculado/hardcoded pelo cristalino, divergente do que a fonte realmente
     declara.
3. Confirmar o mecanismo do vanilla — de onde exactamente lê `918`/`-222` para a mesma
   fonte (`file:line`).

## Fase B — Corrigir a fonte da divergência

Se for leitura errada da tabela da fonte (tabela errada, campo errado, ou conversão de
unidades incorrecta): corrigir para ler o campo certo.

Se for valor hardcoded/calculado: substituir por leitura directa da fonte, mesma
disciplina da emenda do P1042 (constantes tipográficas têm de vir da fonte, não de um
número fixo).

## Fase C — Classificar o gate

Isto é metadado do PDF, não posição de glifo — não muda o output *visual* renderizado.
Mas é output *observável* (extractores de texto, leitores de ecrã, ferramentas de
acessibilidade e qualquer verificação PDF/A dependem destes campos) — confirmar se conta
como categoria 2/3 do `ADR-0127` antes de corrigir, ou se é fluxo contínuo por ser
correcção de metadado sem efeito de renderização. Não presumir — decidir explicitamente
e registar a classificação escolhida no relatório.

## Fase D — Critérios de verificação

```
Dado um documento com texto simples, fonte embutida
Quando exportado a PDF
Então o /FontDescriptor tem Ascent/Descent idênticos aos que o vanilla emite para a
  mesma fonte (comparar valor exacto, não só efeito de renderização)
```

Não-regressão: nenhum teste de posição/render deve mudar (isto é metadado, não
geometria) — confirmar explicitamente que nenhum PDF muda de aparência, só o dicionário
de fonte.

## Fase E — Implementar e validar

```
crystalline-lint .
cargo test --workspace
```
Inspeccionar o `/FontDescriptor` directamente no PDF exportado (mesmo método usado para
encontrar o achado — extrair e comparar o dicionário, não confiar em ferramenta
intermediária).

---

## Resultado esperado

`Ascent`/`Descent` no `FontDescriptor` do cristalino batem exactamente com o vanilla para
a mesma fonte, sem alterar nenhuma posição de render. Com este passo, o achado do P1050
fica corrigido, não só catalogado — e a leva do linter (V16-V19) fica completamente
fechada antes de avançar para V20.
