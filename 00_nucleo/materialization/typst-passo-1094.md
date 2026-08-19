# L0 — Passo 1094: Auditoria — Outros Literais de Conversão Truncados

**Gate**: nenhum para a auditoria em si; correcções que resultarem seguem
`ADR-0127` individualmente.

**Base**: achado do P1093 (literais `28.346`/`2.8346` truncados em vez das
razões exactas `3600/127`/`360/127`) — candidato a padrão recorrente, não
caso único.

---

## 1. Reaproveitar a extracção que já existe, não inventar ferramenta nova

O `crystalline-lint` (regra V21) já faz o trabalho pesado de extrair
literais numéricos com contexto de todo o código Rust do projecto — é o
mesmo mecanismo já usado nesta conversa desde P1064. Não escrever um `grep`
novo do zero; reaproveitar o output já estruturado:

```bash
crystalline-lint --checks v21 --format json .
```

(ou o formato que já foi usado para produzir as tabelas do P1064-1069 — usar
o mesmo, não inventar um novo formato de saída).

Isto dá uma lista de candidatos já filtrada (literais sem citação/
proveniência), que é exactamente o universo onde um truncamento escondido
teria mais probabilidade de estar — se já tivesse citação correcta, era
menos provável que estivesse errado sem ninguém reparar.

## 2. Lista de razões conhecidas para comparar

Não adivinhar às cegas — testar cada literal candidato contra razões com
significado físico/matemático real, mesma disciplina de "proveniência antes
de aceitar":

- Conversões de unidade tipográfica: `72/2.54` (cm), `72/25.4` (mm), outras
  se existirem no domínio do projecto (`72/72.27` para pontos didot/Truchet,
  se for relevante ao contexto de exportação).
- `π`, `180/π` (grau↔radiano) — relevante a `transforms.rs`/rotações, já
  citado nesta conversa.
- Proporções conhecidas em tipografia (razão áurea, `4/3`, etc.) — só se
  aparecerem candidatos plausíveis, não procurar activamente sem sinal.

Para cada literal do V21 com 4+ casas decimais fixas: calcular a diferença
contra a razão candidata mais próxima; se a diferença for da ordem de
`10⁻⁴`-`10⁻³` (mesma escala do erro de `28.346` vs `28.346457`), é candidato
forte a truncamento, não coincidência.

## 3. Sinal mais forte — comparar contra o que o vanilla já faz

Onde o vanilla usa uma razão de inteiros exacta para o mesmo conceito (como
`abs.rs::raw_scale`, já citado no P1093), e o cristalino tem literal decimal
para o mesmo conceito — isso é o sinal mais confiável, mais forte que só
"parece perto de uma razão bonita". Para cada candidato do §2, verificar se
existe equivalente no vanilla (`lab/typst-original/`) antes de classificar
como achado real.

## 4. Não propor mudança ao linter neste passo

Mesma fronteira já estabelecida nesta conversa — usar o `crystalline-lint`
como está, não redesenhar nada nele. Se esta auditoria confirmar que o
padrão é recorrente (mais do que este 1 caso), **isso sim** pode justificar
sugerir ao mantenedor do `tekt-linter` uma regra nova dedicada (candidato a
`V25` ou similar — número a confirmar, mesma disciplina do pedido anterior
sobre referências `.md`). Sugestão, não implementação — decisão de quem
mantém a ferramenta.

## Critério de conclusão

- Lista de candidatos extraída via `crystalline-lint --checks v21`, não
  `grep` improvisado.
- Cada candidato testado contra razão conhecida (§2) com diferença calculada,
  não "parece parecido".
- Confirmação cruzada contra o vanilla (§3) para os candidatos que passarem
  o teste numérico.
- Se houver mais de 1 caso real confirmado: recomendação (não execução) de
  regra nova ao `tekt-linter`, citando os casos reais como motivação — mesmo
  formato já usado para a sugestão anterior sobre `.md`.
