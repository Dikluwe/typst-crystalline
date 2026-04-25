# Ideias para projecto futuro — Ferramenta de geração automática de historiograma

**Data**: 2026-04-25.
**Origem**: passo P156A (geração via LLM externa do
historiograma de 155+ passos).
**Natureza**: documento de ideias **sem compromisso de
implementação**. Esboço de viabilidade, arquitectura
candidata, e casos de uso. Não é spec, não é roadmap.
**Localização sugerida**: `00_nucleo/diagnosticos/`.

---

## §1 — Motivação

P156A demonstrou que é possível extrair um historiograma
estruturado dos 155+ relatórios de materialização via LLM
externa lendo cabeçalhos + sumários. Custo: leitura de
~258 ficheiros (~150-200k tokens) + síntese. Tempo aproximado:
~10-15 minutos de execução paralela em 7 agentes.

Esta operação é **repetitiva** se o historiograma for
regenerado periodicamente (e.g. cada 10-20 passos novos, ou
quando uma fase fecha). Cada regeneração:
- Lê os mesmos ~258 ficheiros + os novos.
- Extrai os mesmos campos estruturados.
- Re-deriva as mesmas estatísticas.

Pergunta: **será que a maior parte deste trabalho é
parseável determinísticamente?**

---

## §2 — Análise de viabilidade

Decomposição do trabalho de geração de historiograma em três
classes de informação:

### §2.1 Classe A — Parseável determinísticamente (~70%)

Informação extraível por regex / parsing de markdown
estruturado:

1. **Identificador do passo**: nome do ficheiro
   (`typst-passo-NNN[-suffix].md` ou
   `typst-passo-NNN[-suffix]-relatorio.md`).
2. **Título**: primeira linha `# ...`.
3. **Data**: padrão `**Data**:` ou `**Data de produção**:`
   no cabeçalho. **Lacuna**: ausente em maioria dos relatórios
   pré-P98.
4. **Natureza/Tipo declarado**: padrão `**Natureza**:` ou
   `**Tipo**:`.
5. **Escopo declarado**: padrão `**Escopo**:` ou similar.
   **Lacuna**: convenção pós-P84.8g; raro em passos antigos.
6. **ADRs criadas/actualizadas/revogadas**: regex sobre
   menções a `ADR-NNNN` + classificação por contexto
   (cabeçalho, secção "ADRs", linha "Revoga"/"Revisto por").
7. **DEBTs abertos/fechados**: regex sobre `DEBT-NN` +
   classificação por contexto ("aberto", "fechado",
   "actualizado", "transitou").
8. **Tests count**: regex sobre `\d+ passed`, `\d+ tests`,
   ou `Tests: N → M`.
9. **Ficheiros tocados**: secções com diffs ou listas de
   `pathname.rs`.
10. **Listagem de sub-passos**: cabeçalhos `## NNN.X — ...`.
11. **Cross-reference passo→ADR**: indexar o conjunto de
    relatórios que mencionam cada ADR/DEBT; deduzir ciclos
    de vida.

### §2.2 Classe B — Parseável parcialmente (~25%)

Informação extraível por heurística + risco de falso
positivo / negativo:

1. **Classificação de tipo dominante** (substantivo /
   administrativo / diagnóstico / arqueológico / fecho-DEBT
   / investigação / refino):
   - Heurística: keywords no cabeçalho ("inventário",
     "auditoria", "diagnóstico", "fecho", "fase X").
   - Sufixo `A` → diagnóstico.
   - Sufixo `B` → substantivo (materialização pós-A).
   - Padrão `-relatorio.md` com TestsΔ=0 e secção "ADRs" vazia
     → administrativo.
   - Erros possíveis: passos híbridos (P32 fecho-DEBT +
     administrativo + substantivo) requerem regra de
     desempate.

2. **Padrão metodológico aplicado** (diagnóstico-primeiro /
   tudo-num-passo / arqueológico):
   - Heurística: presença de secção "Diagnóstico obrigatório"
     ou subsecção `## NNN.A.1`.
   - Erros: passos antigos (P0-P10) usam estrutura diferente.

3. **Reformulações detectadas**:
   - Heurística: nome do ficheiro com sufixo `-vN`; ou
     menção explícita "reformulação" / "v2" / "cancela v1"
     no cabeçalho.
   - Erros: reformulações silenciosas dentro de um único
     ficheiro (e.g. P102 redefiniu spec mid-execução)
     não-detectáveis sem leitura cuidadosa.

4. **Dependências sequenciais**:
   - Heurística: secção "Pré-condição" ou "Precondição"
     no cabeçalho menciona passo anterior por número.
   - Erros: dependências implícitas (sem menção formal)
     não-detectáveis.

5. **Distribuição por escopo**:
   - Heurística: declaração explícita ou inferência por
     número de produtos.
   - Erros: passos sem declaração explícita exigem
     inferência mais frágil.

6. **Pares A→B**:
   - Heurística: emparelhar passo `NNNa` com `NNNb` por
     prefixo numérico.
   - Confiável.

### §2.3 Classe C — Exige humano/LLM (~5%)

Informação que requer síntese semântica:

1. **Conclusões metodológicas** (§4 do historiograma):
   "que padrões funcionaram e por quê" — exige interpretação.
2. **Antipadrões observados**: requer reconhecimento de
   padrões emergentes não documentados explicitamente.
3. **Recomendações empíricas**: derivadas da síntese, não
   extraíveis directamente.
4. **Narrativa cronológica** (§1.2 do historiograma): texto
   livre por cluster.
5. **Análise de "antes vs depois"** quando reformulação
   silenciosa ocorreu.
6. **Lacunas factuais** classificadas semanticamente
   (e.g. "lacuna documental" vs "metadado ausente").

### §2.4 Conclusão de viabilidade

Estimativa: **~70% parseável determinísticamente, ~25% por
heurística com tolerância, ~5% exige LLM ou humano**.

Comparação com a estimativa similar mencionada na spec do
P156A ("~70% parseável; ~25% parcial; ~5% exige humano/LLM"):
**confirmada empiricamente após este passo**.

Implicação: ferramenta automática **pode produzir 95% do
historiograma** (incluindo Classe A + Classe B com flags
de incerteza). Os restantes 5% (Classe C) ficam como
secções vazias / TODO para preenchimento posterior.

---

## §3 — Convenções do projecto cristalino que tornam parsing
viável

O projecto Typst Cristalino adoptou várias convenções que
beneficiam parsing automático. Lista-se aqui o que **já
existe** (sem compromisso de formalizar mais):

1. **Nomenclatura de ficheiros**:
   - `typst-passo-NNN[-suffix].md`: spec.
   - `typst-passo-NNN[-suffix]-relatorio.md`: relatório
     pós-execução (a partir de P98).
   - Sufixos: `-vN` (fork), letra minúscula `a`/`b` (sub-passo
     associado à mesma numeração).
2. **Cabeçalhos canónicos** (a partir de P145 para ADRs;
   P84.8g para vocabulário):
   - `**Status**:` com valores canónicos em backticks.
   - `**Data**:` (formato YYYY-MM-DD).
   - `**Natureza**:` (relatórios pós-P140A frequentemente).
   - `**Escopo**:` (XS/S/M/M+/L/XL — mais consistente
     pós-P83).
3. **Secções típicas** (`## Sumário`, `## Sub-passos`,
   `## Verificação`, `## Critério`, `## O que pode sair
   errado`, `## Notas operacionais`, `## Próximo passo`).
4. **Convenção `-RN`** para revisões de ADR (formalizada
   P84.8f).
5. **Vocabulário canónico de status** (6 valores em
   português, P84.8g):
   `PROPOSTO`, `IDEIA`, `EM VIGOR`, `IMPLEMENTADO`,
   `REVOGADO`, `ADIADO`.
6. **Hashes de prompt L0** em código L1 (`@prompt-hash`
   header) — não usado no historiograma mas demonstra
   cultura de linkagem determinística.
7. **DEBT.md unificado** em `00_nucleo/DEBT.md` (movido
   em P83.5).
8. **README.md de índice ADRs** (criado em P84.8h).
9. **Sufixo `A` para diagnóstico-primeiro** — convenção
   estabelecida implicitamente em P131A e formalizada
   em P140A / P154A / spec P156A.

Estas convenções **não foram criadas para parsing**, mas
emergiram organicamente. A consequência feliz: **parsing é
viável sem mais formalização**.

---

## §4 — Esboço arquitectural

Esboço **não-comprometedor**. Apenas para guiar discussão
futura. Linguagem e estrutura são candidatas, não decisões.

### §4.1 Componentes candidatos

```
historiograma-tool/
├── src/
│   ├── lib.rs                   API pública
│   ├── parser/
│   │   ├── filename.rs          Extrai #/sufixo/tipo do nome
│   │   ├── header.rs            Extrai cabeçalho (título, data, natureza)
│   │   ├── adr_ref.rs           Detecta ADR-NNNN com classificação
│   │   ├── debt_ref.rs          Detecta DEBT-NN com classificação
│   │   ├── tests.rs             Detecta contagem de testes
│   │   └── sections.rs          Identifica secções típicas
│   ├── classifier/
│   │   ├── tipo.rs              Classe B: tipo dominante
│   │   ├── escopo.rs            Classe B: inferência escopo
│   │   ├── padrão.rs            Classe B: padrão metodológico
│   │   └── reformulação.rs      Classe B: detecção fork/refactor
│   ├── aggregator/
│   │   ├── linha_temporal.rs    Constrói tabela §1
│   │   ├── padrões.rs           Agrega §2
│   │   ├── ciclo_vida_adr.rs    §3.5
│   │   ├── ciclo_vida_debt.rs   §3.4
│   │   └── saúde.rs             §3.7 (tests/ADRs/DEBTs ao longo)
│   ├── output/
│   │   ├── markdown.rs          Render historiograma .md
│   │   └── visualizations.rs    Mermaid + ASCII fallback
│   └── lacunas.rs               Detecta + reporta lacunas
└── tests/
    └── integration.rs           Testes contra corpus real
```

### §4.2 Fluxo de execução

```
1. Listar ficheiros: ls 00_nucleo/materialization/typst-passo-*.md
2. Para cada ficheiro:
   a. parser/filename → Identificador + sufixo + se é spec ou relatório
   b. Ler primeiras N linhas (configurável; default 80)
   c. parser/header → título, data, natureza
   d. parser/adr_ref + parser/debt_ref → conjuntos com classificação
   e. parser/tests → TestsΔ
   f. parser/sections → presença/ausência de secções
   g. classifier/* → tipo, escopo, padrão, reformulação
3. Aggregator:
   a. Construir linha temporal (tabela §1)
   b. Agregar padrões (lista por categoria)
   c. Cruzar referências ADR↔passo, DEBT↔passo
   d. Calcular ciclos de vida (distância média/máxima)
   e. Calcular saúde (tests cumulativos)
4. lacunas.rs → produzir relatório de lacunas factuais
   (campos ausentes, classificações ambíguas, datas omissas)
5. output/markdown → render
   a. Gerar §1 (tabela + narrativa cronológica esqueleto)
   b. Gerar §2 (padrões agregados com estatísticas)
   c. Gerar §3 (análise: reformulações + ciclo de vida + saúde)
   d. Gerar §4 — secção esqueleto com TODO marker
      (Classe C — exige LLM ou humano)
   e. Gerar §5/§6 (referências e coexistência)
6. output/visualizations → Mermaid + ASCII fallback inline
```

### §4.3 Onde o LLM/humano entra

Após geração automática:
- §4 (Conclusões metodológicas): LLM ou humano lê o output
  da ferramenta + os passos completos (sem optimização) e
  redige conclusões.
- §1.2 (narrativa cronológica): LLM produz texto humano-legível
  a partir das tabelas de §1.1.
- Antipadrões observados (§3.6): LLM ou humano identifica
  padrões emergentes não-explícitos.

Heurística: a ferramenta produz **scaffolding de 95%**; LLM
ou humano preenche os **5% interpretativos**. Custo total
estimado vs P156A: redução de ~80% em tokens LLM e ~70%
em tempo.

### §4.4 Tecnologia candidata

Sem compromisso. Opções:

- **Rust** (consistente com o projecto): `pulldown-cmark`
  para markdown, `regex` ou `nom` para extracção.
- **Python**: ecosistema mais rápido para prototipagem
  (markdown-it-py, pandas para agregação).
- **Shell + awk + jq**: trivial para campos simples; insuficiente
  para classificação Classe B.

Trade-off: Rust dá determinismo + integração com o projecto;
Python dá iteração rápida; Shell é dispensável após primeira
classe simples.

**Decisão recomendada (não comprometida)**: prototipar em
Python; portar para Rust se a ferramenta for adoptada.

### §4.5 Política de erro / lacuna

A ferramenta **nunca infere conteúdo ausente**. Quando um
campo não pode ser extraído, regista como lacuna factual com
3 níveis:

- **Vermelho**: campo crítico ausente (e.g. título, número
  do passo).
- **Amarelo**: campo opcional ausente (e.g. data, escopo).
- **Verde**: campo derivado por heurística (e.g. tipo
  classificado por keywords).

Output inclui apêndice com lista de lacunas. O utilizador
pode então editar manualmente os relatórios para preencher
lacunas críticas.

---

## §5 — Casos de uso

### §5.1 No projecto Typst Cristalino

1. **Regeneração periódica do historiograma** (a cada 10-20
   passos novos ou no fecho de uma fase).
2. **Auditoria de DEBTs** (ciclo de vida automático).
3. **Auditoria de ADRs** (transições de status).
4. **Detecção precoce de antipadrões** (e.g. cascata de
   DEBTs sem fecho imediato).
5. **Métricas de saúde** (tests/ADRs/DEBTs ao longo do
   tempo) integradas em CI.

### §5.2 Noutros projectos com convenções similares

Projectos que adoptem estrutura similar (passos numerados +
relatórios separados + DEBT.md + ADRs em pasta dedicada)
beneficiam directamente. Exemplos potenciais:

1. **Outros projectos de re-implementação de software** (modelo
   Typst Cristalino reaplicado a outro compilador / runtime).
2. **Projectos com ciclo de revisão arquitectural formal**
   (ADRs como standard).
3. **Projectos académicos** que documentam metodologia em
   passos materializados.

### §5.3 Limitações para projectos sem convenções

A ferramenta **não funciona** em projectos que:
- Não usam relatórios estruturados (diários de bordo livres).
- Não têm convenção ADR.
- Não usam DEBT.md ou inventário equivalente.
- Misturam código + documentação no mesmo ficheiro sem
  delimitadores claros.

Para estes, a leitura por LLM (modelo P156A) continua
necessária.

---

## §6 — Comparação ferramenta vs LLM

| Critério | LLM (modelo P156A) | Ferramenta (esta proposta) |
|----------|--------------------|---------------------------|
| Custo por execução | ~150-200k tokens + ~10-15 min paralelo | ~zero tokens + segundos |
| Cobertura | ~100% (com lacunas declaradas) | ~95% (Classe A + B) |
| Repetibilidade | Variabilidade entre execuções | Determinístico |
| Adaptabilidade a novas convenções | Imediata (LLM aprende do contexto) | Requer mudança de código |
| Conclusões metodológicas | Produzidas em-line | Requer pós-processamento por LLM ou humano |
| Lacunas detectadas | Por leitura semântica | Por ausência de campos parseáveis |
| Custo de implementação | Zero (LLM já existe) | Médio (~M+ a XL conforme escopo) |
| Custo de manutenção | Zero (LLM evolui) | Baixo se convenções estáveis |

**Recomendação**: ferramenta complementa LLM. Não substitui.
Cenário ideal:
- Ferramenta gera scaffolding em segundos.
- LLM revisa + preenche §4 + redige §1.2 narrativa.
- Custo total: ~30k tokens LLM (vs 150-200k actual).

---

## §7 — Esforço estimado

**Não-comprometido**. Apenas para informar decisão futura.

- Protótipo Python (Classe A apenas): **~M (2-4h)**.
- Protótipo Python (Classe A + B): **~L (4-6h)**.
- Versão Rust completa com lacunas + Mermaid + integração
  CI: **~XL (10h+)**.
- Adaptação a outro projecto com convenções similares:
  **~M-L** (configuração + ajustes regex).

---

## §8 — Não-objectivos

A ferramenta **não pretende**:
- Substituir o LLM para conclusões metodológicas.
- Inferir conteúdo ausente.
- Editar relatórios históricos (são imutáveis por
  convenção).
- Substituir blueprint snapshot (são complementares).
- Gerar specs de passos futuros.
- Decidir prioridades.

---

## §9 — Próximos passos (não-comprometidos)

Se este documento motivar implementação:

1. **Decisão humana**: vale a pena? Frequência de
   regeneração justifica investimento?
2. **Protótipo mínimo**: parser de Classe A em Python
   (~2h) para validar viabilidade real.
3. **Avaliação**: comparar output do protótipo vs P156A
   em campos comuns (título, ADRs, DEBTs).
4. **Decisão de continuar** ou abandonar.
5. **ADR formal** se ferramenta for adoptada (modelo
   ADR para infra interna do projecto).

**Importante**: este documento é **ideias-only**. Não
existe compromisso temporal, ninguém está atribuído, e
não bloqueia nenhum passo. Pode ser ignorado
indefinidamente sem custo.

---

## §10 — Referências

- `00_nucleo/diagnosticos/historiograma-passos.md`
  (output do P156A — input para validação da ferramenta).
- `00_nucleo/diagnosticos/blueprint-projecto.md`
  (snapshot complementar).
- `00_nucleo/materialization/typst-passo-156a.md` (spec
  que originou esta ideia).
- `00_nucleo/adr/README.md` (convenções actuais).
