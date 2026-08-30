# P1211 — método de comparação pareada com composição e separação independentes

**Estado:** EXECUTADO — MÉTODO FECHADO; PILOTO PENDENTE  
**Data:** 2026-08-26  
**Dependência:** P1210 executado como RED de medição  
**Baseline funcional:** vanilla ratificado `a51e02804`  
**Origem metodológica:** experimento P0114 do `tekt-linter`, arquivo
`00_nucleo/tekt-linter-passo-0114-lab-contrapesos-v19-v20.md`, SHA-256
`5cdaba7db03540ff355b602df7d259ebd6e10b8f97279aced0e64c3872374723`, lido no estado
`3fd957745d1b3dec264d2924c0b35a4804f6e4c2` com o arquivo ainda não versionado.

## 1. Pergunta fechada

Para uma mesma mudança funcional da linguagem Typst, executada em condições comparáveis,
o desenho cristalino permite localizar, modificar, verificar e estender o comportamento
com menor custo e menor risco que o desenho vanilla ratificado?

O método não tenta provar que “mais separação é melhor” nem que “menos código é melhor”.
Ele testa duas perguntas independentes:

1. há composição dispensável — responsabilidades diferentes fundidas sem necessidade?
2. há separação dispensável — uma única responsabilidade espalhada sem necessidade?

As respostas não se cancelam e não produzem score líquido. Um desenho pode conter, ao
mesmo tempo, composição justificada, separação justificada e fragmentação local.

## 2. Hipóteses falsificáveis

### H1 — localidade do cristalino

Para uma tarefa pareada, o cristalino exige tocar menos unidades semanticamente não
proprietárias e produz menos regressões fora do owner.

H1 é refutada quando a tarefa equivalente exige caça transversal igual ou maior,
alterações fora dos owners declarados ou regressões laterais não explicadas por diferenças
de escopo funcional.

### H2 — separação justificada

As fronteiras adicionais do cristalino correspondem a diferenças reais de
responsabilidade, camada, efeito, contrato ou observável e permitem compreender cada
unidade localmente.

H2 é refutada quando a fronteira apenas espalha uma decisão única, replica conhecimento,
obriga navegação sem ganho de responsabilidade ou cria coordenação artificial entre
owners.

### H3 — composição justificada

As decisões que permanecem compostas — por exemplo o `match` exaustivo central —
preservam uma garantia estática ou uma visão integral que seria perdida ao separá-las.

H3 é refutada quando a composição mistura lógica bespoke de várias features, exige edição
do núcleo para mudanças locais sem benefício demonstrável ou impede testes focais.

### H4 — custo de evolução

Depois de alcançar a mesma baseline funcional, adicionar uma extensão pareada toca menos
responsabilidades e causa menos retrabalho no cristalino.

H4 é refutada se a extensão equivalente requer mais owners, mais gates não causais ou
mais reparos fora da feature, depois de excluir resselo e diferenças mecânicas.

## 3. Unidade experimental: tarefa pareada

Uma observação não é um commit, arquivo ou número de linhas. É uma **tarefa pareada** com:

- o mesmo programa Typst inicial;
- o mesmo observável RED no nível de sintaxe, semântica, morfologia ou produto;
- o mesmo resultado GREEN esperado;
- escopo funcional congelado antes de qualquer implementação;
- uma execução no vanilla ratificado e outra no cristalino;
- agentes/sessões separados, mesma instrução e mesmo orçamento de tempo;
- ordem alternada entre pares para reduzir efeito de aprendizagem.

O vanilla de comparação deve partir de `a51e02804`. O cristalino deve registrar HEAD,
working tree e hashes conforme a regra de proveniência. Nenhum lado pode receber a solução
do outro antes do fechamento do par.

## 4. Pré-condição funcional

Uma tarefa só entra na comparação arquitetural quando o harness prova que os dois lados
partem do mesmo comportamento relevante ou quando o RED é exatamente a mesma divergência
introduzida de forma controlada.

Estados permitidos:

| Estado | Uso |
|---|---|
| `FUNCTIONALLY-ALIGNED` | ambos possuem o mesmo comportamento inicial |
| `CONTROLLED-RED` | o mesmo defeito foi introduzido por transformação congelada |
| `NOT-COMPARABLE` | comportamento, feature, ambiente ou oracle diferem |
| `HARNESS-GAP` | não há como provar equivalência da tarefa |

`NOT-COMPARABLE` e `HARNESS-GAP` não entram em denominador de manutenibilidade.

## 5. Dois livros-razão, nenhum cancelamento

### 5.1 Livro de composição

Registra decisões concentradas na mesma unidade.

| Estado | Significado |
|---|---|
| `COMPOSED` | múltiplas alternativas/responsabilidades estão juntas |
| `JUSTIFIED-COMPOSITION` | a união preserva exaustividade, atomicidade ou leitura integral necessária |
| `OVER-COMPOSED-CANDIDATE` | responsabilidades distintas parecem fundidas |
| `UNKNOWN-COMPOSITION` | evidência insuficiente |

### 5.2 Livro de separação

Registra decisões distribuídas por fronteiras.

| Estado | Significado |
|---|---|
| `SEPARATED` | trabalho atravessa mais de uma unidade |
| `JUSTIFIED-SEPARATION` | fronteira corresponde a owner, camada, efeito ou contrato distinto |
| `FRAGMENTED-CANDIDATE` | uma decisão única parece espalhada sem diferença necessária |
| `UNKNOWN-SEPARATION` | evidência insuficiente |

Um `JUSTIFIED-COMPOSITION` não dá crédito contra um `FRAGMENTED-CANDIDATE`; um
`JUSTIFIED-SEPARATION` não apaga um `OVER-COMPOSED-CANDIDATE`. As quatro contagens e suas
interseções permanecem visíveis.

## 6. O que conta como fronteira justificada

Uma fronteira recebe `JUSTIFIED-SEPARATION` somente se houver pelo menos uma diferença
positiva e verificável:

- owner L0 distinto por responsabilidade materializável;
- camada distinta por efeito ou topologia de imports;
- contrato público ou trait diferente;
- observável diferente da linguagem/produto;
- lifecycle, recurso externo ou modo de falha distinto;
- unidade testável e compreensível isoladamente;
- garantia arquitetural que seria perdida pela fusão.

“Está em outro arquivo”, “o linter aceita” e “há menos linhas por arquivo” não justificam
a separação.

## 7. O que conta como composição justificada

Uma composição recebe `JUSTIFIED-COMPOSITION` somente quando a unidade conjunta fornece:

- exaustividade estática;
- ordem/precedência semanticamente necessária;
- transação ou invariantes que precisam mudar juntas;
- visão total de uma decisão fechada;
- coordenação que seria duplicada ou enfraquecida ao separar.

No cristalino, o `match` exaustivo magro de `Content` é candidato natural a esta classe;
a lógica bespoke gorda de várias features no mesmo arquivo não recebe a classificação por
associação. ADR-0109 continua sendo a autoridade.

## 8. Medidas primárias por tarefa

### 8.1 Localização

- tempo até primeiro RED reproduzível;
- arquivos abertos antes de localizar o owner causal;
- owners L0 consultados;
- falsos caminhos investigados;
- número de saltos entre módulo chamador, owner e teste.

### 8.2 Mudança funcional

- owners e consumers produtivos tocados;
- unidades tocadas fora do owner causal;
- diff funcional, excluindo resselo, formatação e fixtures idênticas;
- contratos, defaults, fases ou compatibilidade alterados;
- novos imports entre camadas;
- conhecimento duplicado introduzido.

### 8.3 Verificação

- testes RED necessários;
- tempo RED→GREEN;
- testes focais e globais executados;
- regressões dentro e fora do owner;
- violações arquiteturais criadas/removidas;
- falhas de harness e resultados não executáveis.

### 8.4 Extensão posterior

- owners reabertos para uma segunda extensão equivalente;
- reincidência da mesma classe de defeito;
- proporção da solução anterior reutilizada sem cópia;
- novas fronteiras justificadas e fragmentações candidatas.

Tempo é evidência auxiliar, não veredito isolado: cache, familiaridade, máquina e ordem da
execução devem ser registrados.

## 9. Normalização obrigatória

Não comparar diretamente:

- LOC bruta;
- número bruto de arquivos;
- número bruto de funções;
- número de arms;
- bytes de output;
- estrutura Rust equivalente;
- tempo sem controlar cache e ambiente.

O diff é decomposto em:

```text
funcional | teste/oracle | arquitetura | documentação L0 |
resselo mecânico | formatação | ambiente/harness
```

Somente `funcional`, `teste/oracle` e efeitos arquiteturais causais entram na comparação.
L0 é medido separadamente: é custo deliberado de especificação, não deve ser escondido nem
somado a LOC funcional.

## 10. Classificação de cada tarefa

Cada par termina com uma linha nominal:

```text
pair_id | functional_gate | task | observable | vanilla_revision |
crystalline_state | localization | functional_diff | tests | regressions |
composition_state | separation_state | evidence | inference | refutation |
confounders | verdict
```

Vereditos permitidos:

- `CRYSTALLINE-ADVANTAGE`;
- `VANILLA-ADVANTAGE`;
- `NO-DEMONSTRATED-DIFFERENCE`;
- `TRADEOFF`;
- `NOT-COMPARABLE`;
- `HARNESS-GAP`.

`CRYSTALLINE-ADVANTAGE` exige vantagem em pelo menos uma medida primária, nenhuma piora
material não explicada e classificação completa dos dois livros-razão. Não se infere
vantagem a partir de estética ou conformidade arquitetural isolada.

## 11. Desenho da amostra

Executar no mínimo 12 pares, sem misturar denominadores:

1. três correções em comportamento já suportado;
2. três extensões de superfície ou membro;
3. duas mudanças de layout/morfologia;
4. duas mudanças de export/produto;
5. duas mudanças cross-cutting controladas.

Cada família deve conter ao menos:

- uma tarefa pequena e localizada;
- uma tarefa que atravesse owner/camada;
- quando aplicável, uma tarefa na qual a separação cristalina possa ser custo, não apenas
  benefício.

A seleção é congelada antes dos resultados. Não substituir tarefa desfavorável depois da
execução.

## 12. Oráculos independentes

Usar quatro oráculos separados:

1. **funcional:** linguagem/produto conforme ADR-0107;
2. **arquitetural:** owners, camadas, L0 1:1 e Núcleos Tekt;
3. **processo:** comandos, tempo, arquivos e eventos RED/GREEN;
4. **adversarial:** revisão manual tenta refutar composição/separação justificadas.

Nenhum oráculo deriva sua expectativa da implementação avaliada. O linter pode validar
arquitetura, mas não pode provar sozinho que uma fronteira melhora manutenção.

## 13. Execução em fases

### A — congelar

Registrar pares, programas Typst, observáveis, revisões, ferramentas, ambiente, ordem e
orçamentos. Hash-pin todos os inputs.

### B — provar comparabilidade

Executar os programas nos dois lados e classificar
`FUNCTIONALLY-ALIGNED`/`CONTROLLED-RED`/`NOT-COMPARABLE`/`HARNESS-GAP` antes de medir
arquitetura.

### C — localizar cegamente

Entregar a mesma descrição a sessões segregadas. Registrar navegação e congelar a causa
proposta antes de implementar.

### D — implementar

Aplicar L0→RED→GREEN no cristalino e o fluxo nativo equivalente no vanilla. Gates
ADR-0127 continuam valendo; uma parada obrigatória não é penalidade, mas evento medido.

### E — confrontar composição e separação

Classificar todas as unidades tocadas nos dois livros-razão e tentar refutar cada
`JUSTIFIED-*` com uma alternativa mínima.

### F — extensão retardada

Após pelo menos um cluster intermediário, executar uma segunda extensão sobre metade dos
pares para medir reabertura e reincidência.

### G — fechar

Publicar resultados nominais, matrizes por família, confounders e perguntas abertas. Não
produzir ranking único.

## 14. Critérios de conclusão

O método pode sustentar “vantagem observada do cristalino” somente se:

- todos os pares incluídos forem funcionalmente comparáveis;
- cada número tiver proveniência reproduzível;
- composição e separação forem classificadas independentemente;
- resultados desfavoráveis e `TRADEOFF` permanecerem na amostra;
- houver vantagem repetida em mais de uma família;
- a extensão retardada não inverter a conclusão sem explicação;
- nenhum `HARNESS-GAP` for contado como sucesso;
- a conclusão declarar domínio e limites, nunca “mais manutenível” universalmente.

Com menos de 12 pares, o resultado é `PILOT`, não conclusão arquitetural.

## 15. Saídas previstas

Criar em passo de execução posterior:

- `00_nucleo/diagnosticos/p1211-pares.tsv`;
- `00_nucleo/diagnosticos/p1211-eventos.jsonl`;
- `00_nucleo/diagnosticos/p1211-composicao.tsv`;
- `00_nucleo/diagnosticos/p1211-separacao.tsv`;
- `00_nucleo/diagnosticos/typst-p1211-comparacao-manutenibilidade.md`;
- diretório externo ou `lab/` segregado para patches/artefatos temporários, nunca L1–L4.

## 16. Limites e próximos passos

P1211 define o método; não executa as 12 tarefas e não corrige os gaps P1210. O próximo
passo deve escolher um piloto de dois pares: um caso favorável esperado e um caso em que a
separação cristalina possa impor custo. Se ambos forem comparáveis e os instrumentos forem
reproduzíveis, ampliar a amostra sem alterar o protocolo.

Qualquer instrumentação produtiva, contrato público, default, fase ou compatibilidade exige
L0 e o gate ADR-0127. Instrumentação confinada a `lab/` ou diagnósticos não legitima mudança
em L1–L4.

## 17. Fechamento da execução

P1211 foi executado em 2026-08-26 como passo metodológico. A execução confrontou a nova
ideia de contrapesos do Tekt com ADR-0107, ADR-0108, ADR-0109 e a baseline funcional P1210
antes de fechar o protocolo.

Resultado da auditoria:

- composição e separação possuem estados, evidências e refutações independentes;
- nenhuma contagem produz crédito, cancelamento ou score líquido;
- tarefa pareada, não arquivo/commit/LOC, é a unidade experimental;
- comparabilidade funcional precede qualquer conclusão arquitetural;
- L0, diff funcional, resselo e harness permanecem custos separados;
- resultados desfavoráveis, `TRADEOFF`, `NOT-COMPARABLE` e `HARNESS-GAP` permanecem
  nominais;
- a amostra mínima e a extensão retardada impedem conclusão a partir de um caso
  conveniente;
- o linter é oráculo arquitetural, não oráculo circular de manutenibilidade.

O método está fechado e apto a governar um piloto. Nenhum par foi executado, porque o
escopo deste passo os remete explicitamente ao passo seguinte; por isso P1211 não produz
veredito sobre qual arquitetura é mais manutenível.

Validação documental:

- `git diff --check -- typst-passo-1211.md`: PASS;
- Prompt L0 ou código L1–L4 alterado por P1211: nenhum;
- contrato público/default/fase/compatibilidade alterado: nenhum;
- índice Git alterado por P1211: não.

Estado terminal: `METHOD CLOSED — PILOT REQUIRED`.
